pub mod types;

#[allow(dead_code)]
pub enum SpecialId {
  Requestor = -8,
  RequestKeyAuthKey,
  Group,
  UserSession,
  User,
  Session,
  Process,
  Thread,
}

#[allow(dead_code)]
pub enum RequestKeyring {
  Thread,
  Process,
  Session,
  User,
  UserSession,
  Group,
  Requestor,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum KeyType {
  Keyring,
  User,
  Logon,
  BigKey,
}

impl KeyType {
  fn as_str(&self) -> &'static str {
    match self {
      KeyType::Keyring => "keyring\0",
      KeyType::User => "user\0",
      KeyType::Logon => "logon\0",
      KeyType::BigKey => "big_key\0",
    }
  }
}

#[allow(dead_code)]
pub enum Commands {
  GetKeyringId,
  JoinSessionKeyring,
  Update,
  Revoke,
  Chown,
  SetPerm,
  Describe,
  Clear,
  Link,
  Unlink,
  Search,
  Read,
  Instantiate,
  Negate,
  SetRequestKeyring,
  SetTimeout,
  AssumeAuthority,
  GetSecurity,
  SessionToParent,
  Reject,
  InstantiateIov,
  Invalidate,
  GetPersistent,
  DhCompute,
  PkeyQuery,
  PkeyEncrypt,
  PkeyDecrypt,
  PkeySign,
  PkeyVerify,
  RestrictKeyring,
  Move,
  Capabilities,
  WatchKey,
}

macro_rules! call {
  ( $op:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr ) => {
    nc::keyctl($op, $a2, $a3, $a4, $a5)
  };
  ( $op:expr, $a2:expr, $a3:expr, $a4:expr) => {
    nc::keyctl($op, $a2, $a3, $a4, 0)
  };
  ( $op:expr, $a2:expr, $a3:expr ) => {
    nc::keyctl($op, $a2, $a3, 0, 0)
  };
  ( $op:expr, $a2:expr ) => {
    nc::keyctl($op, $a2, 0, 0, 0)
  };
}

pub unsafe fn add_key(
  keytype: KeyType,
  description: &str,
  payload: Option<&[u8]>,
  dest_keyring: nc::key_serial_t,
) -> Result<nc::key_serial_t, nc::Errno> {
  let (secret, secret_len) = match payload {
    Some(payload) => (payload.as_ptr(), payload.len()),
    None => (core::ptr::null(), 0),
  };

  nc::add_key(
    keytype.as_str(),
    description,
    secret as usize,
    secret_len,
    dest_keyring,
  )
}

pub(crate) use call;
