
use crate::models::User;
use crate::hashing::{validate_password};
use mongodb::bson::doc;

pub fn parse_json_body(body: String) -> Result<User, String> {
  let body:User = match serde_json::from_str(&body) {
    Ok(body) => body,
    Err(e) => {
      return Err(format!("Error en el body: {:?}", e));
    }
  };
  Ok(body)
}



pub async fn verify_user_against_db(
  collection: &mongodb::Collection<User>,
  user: User,
) -> Result<bool, String> {
  let doc_in_db = collection
    .find_one(Some(doc! {"username": user.username}), None)
    .await
    .map_err(|e| format!("Error al intentar buscar en la base de datos: {:?}", e))?;
  let is_valid = match doc_in_db {
    Some(user_from_db) => validate_password(user_from_db, user.pass)
      .map_err(|e| format!("Error al intentar validar la contraseña: {:?}", e))?,
    None => return Err("No existe el usuario".to_string()),
  };
  Ok(is_valid)
}