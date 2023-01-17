use crate::models::User;
use pwhash::bcrypt::{verify, hash};

fn from_pwhash_error(err: pwhash::error::Error) -> std::string::String {
  format!("Error desde pwhash: {:?}", err)
}

pub fn hash_password(password: String) -> Result<String, String> {
  hash(password).map_err(from_pwhash_error)
}

pub fn verify_password(password: String, hash: String) -> bool {
  verify(password, &hash)
}

pub fn user_hash_password(user: User) -> Result<User, String> {
  let hashed = hash_password(user.pass)?;
  let user = User {
    username: user.username,
    pass: hashed,
  };
  Ok(user)
}

pub fn validate_password(user_from_db: User, pass: String) -> Result<bool, String> {
  let pass_in_db = user_from_db.pass;
  let is_valid = verify(pass, &pass_in_db);
  Ok(is_valid)
}