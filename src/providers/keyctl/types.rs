use crate::keytar::error::Error;
use crate::providers::keyctl;

pub struct Keyring {
  pub id: nc::key_serial_t,
}

pub struct Key(nc::key_serial_t);

impl Key {
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
