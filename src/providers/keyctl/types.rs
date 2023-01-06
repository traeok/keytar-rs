use crate::keytar::error::Error;
use crate::providers::keyctl;
use crate::utils::u8_slice_to_array;

pub struct Keyring {
  pub id: nc::key_serial_t,
}

#[derive(Debug)]
pub struct Key(nc::key_serial_t);

impl Key {
  pub fn describe(&self) -> Result<String, Error> {
    let mut vec = vec![0u8; 0];

    let mut ret;
    unsafe {
      ret = keyctl::call!(keyctl::Commands::Describe as i32, self.0 as usize, 0, 0)?
        as nc::key_serial_t;
    }

    let mut klen;
    loop {
      klen = ret;
      vec.resize((klen + 1) as usize, 0u8);

      unsafe {
        ret = keyctl::call!(
          keyctl::Commands::Describe as i32,
          self.0 as usize,
          <Vec<u8> as AsMut<[u8]>>::as_mut(&mut vec).as_mut_ptr() as _,
          klen as usize
        )? as nc::key_serial_t;
      }
      if ret < 0 {
        return Ok(String::default());
      }

      if klen >= ret {
        break;
      }
      vec.clear();
    }

    Ok(String::from_utf8(vec).unwrap())
  }
  pub fn read_bytes<T: AsMut<[u8]>>(&self, buffer: &mut T) -> Result<usize, Error> {
    let len: usize;
    unsafe {
      len = keyctl::call!(
        keyctl::Commands::Read as i32,
        self.0 as usize,
        buffer.as_mut().as_mut_ptr() as _,
        buffer.as_mut().len() as _
      )?;
    }

    Ok(len)
  }

  pub fn read(&self) -> Result<Vec<u8>, Error> {
    let mut buffer: Vec<u8> = vec![0u8; 65535 as usize];

    let bytes_read = self.read_bytes(&mut buffer)?;
    unsafe {
      buffer.set_len(bytes_read);
    }
    Ok(buffer)
  }

  pub fn invalidate(&self) -> Result<(), Error> {
    unsafe {
      keyctl::call!(keyctl::Commands::Invalidate as i32, self.0 as usize)?;
    }

    Ok(())
  }
}

impl Keyring {
  pub fn from_id(id: nc::key_serial_t) -> Self {
    Keyring { id }
  }

  pub fn from_special_id(id: keyctl::SpecialId, should_create: bool) -> Result<Self, Error> {
    let real_id: nc::key_serial_t;
    unsafe {
      real_id = keyctl::call!(
        keyctl::Commands::GetKeyringId as i32,
        id as usize,
        should_create as usize
      )? as nc::key_serial_t;
    }
    Ok(Self { id: real_id })
  }

  pub fn keys(&self) -> Result<Vec<Key>, Error> {
    let mut keys: Vec<Key> = Vec::default();
    let mut vec: Vec<u8> = vec![0u8; 0];

    let mut ret;

    unsafe {
      ret =
        keyctl::call!(keyctl::Commands::Read as i32, self.id as usize, 0, 0)? as nc::key_serial_t;
    }

    let mut kring_len;
    loop {
      kring_len = ret;
      vec.resize((kring_len + 1) as usize, 0u8);
      unsafe {
        ret = keyctl::call!(
          keyctl::Commands::Read as i32,
          self.id as usize,
          <Vec<u8> as AsMut<[u8]>>::as_mut(&mut vec).as_mut_ptr() as _,
          kring_len as usize
        )? as nc::key_serial_t;
      }

      if ret < 0 {
        vec.clear();
        return Ok(keys);
      }

      if kring_len >= ret {
        break;
      }
      vec.clear();
    }

    vec[ret as usize] = 0;
    let mut i = 0;

    loop {
      if i + 3 > vec.len() {
        break;
      }

      let as_int = nc::key_serial_t::from_ne_bytes(u8_slice_to_array(&vec[i..=i + 3]));
      keys.push(Key(as_int));

      i += 4;
    }

    Ok(keys)
  }

  pub fn add_key(&self, description: &str, secret: &str) -> Result<Key, Error> {
    let id: nc::key_serial_t;
    unsafe {
      match keyctl::add_key(
        keyctl::KeyType::User,
        description,
        Some(secret.as_ref()),
        self.id,
      ) {
        Ok(key) => id = key,
        Err(err) => return Err(Error::from(err)),
      };
    }

    Ok(Key(id))
  }

  pub fn search(&self, description: &str) -> Result<Key, Error> {
    let desc = std::ffi::CString::new(description).or(Err(Error::from_details(
      "Invalid description for Keyring::search",
    )))?;

    let key_id: nc::key_serial_t;
    unsafe {
      key_id = keyctl::call!(
        keyctl::Commands::Search as i32,
        self.id as usize,
        keyctl::KeyType::User.as_str().as_ptr() as _,
        desc.as_ptr() as _,
        0
      )? as nc::key_serial_t;
    }

    Ok(Key(key_id))
  }
}
