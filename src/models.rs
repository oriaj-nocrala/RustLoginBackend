use futures::StreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct UsersModel {
  pub users: HashMap<String, User>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
  pub username: String,
  pub pass: String,
}

pub fn new_user(body: String) -> Result<User, String> {
  let body: User = match serde_json::from_str(&body) {
    Ok(body) => body,
    Err(e) => return Err(format!("Error en el body: {:?}", e)),
  };
  Ok(body)
}

pub async fn find_by_username(
  collection: &mongodb::Collection<User>,
  username: String,
) -> Result<User, String> {
  let doc_in_db = collection
    .find_one(Some(doc! {"username": username.clone()}), None)
    .await
    .map(|doc| {
      doc.ok_or(format!(
        "No se encontró el usuario {} en la base de datos",
        username
      ))
    })
    .map_err(|e| format!("Error al intentar buscar en la base de datos: {:?}", e))?;
  let final_user =
    doc_in_db.map_err(|e| format!("Error al intentar buscar en la base de datos: {:?}", e))?;
  Ok(final_user)
}

pub async fn find_all_users(
  collection: &mongodb::Collection<User>,
) -> Result<HashMap<String, User>, String> {
  let mut users: HashMap<String, User> = HashMap::new();
  let cursor = collection
    .find(None, None)
    .await
    .map_err(|e| format!("Error al intentar buscar en la base de datos: {:?}", e))?;
  cursor
    .for_each(|doc| {
      let user: User = doc
        .map_err(|e| format!("Documento inválido: {:?}", e))
        .unwrap();
      users.insert(user.username.clone(), user);
      futures::future::ready(())
    })
    .await;
  Ok(users)
}

async fn update_user_pass(
  collection: &mongodb::Collection<User>,
  user: User,
  newpass: String,
) -> Result<(), String> {
  let filter = doc! {"username": user.username};
  let update = doc! {"$set": {"pass": newpass}};
  let result = collection
    .update_one(filter, update, None)
    .await
    .map_err(|e| format!("Error al intentar actualizar en la base de datos: {:?}", e))?;
  if result.modified_count == 1 {
    Ok(())
  } else {
    Err("Error al actualizar".to_string())
  }
}

pub async fn delete_user(
  collection: &mongodb::Collection<User>,
  user: User,
) -> Result<bool, String> {
  let filter = doc! {"username": user.username};
  let result = collection
    .delete_one(filter, None)
    .await
    .map_err(|e| format!("Error al intentar borrar en la base de datos: {:?}", e))?;
  if result.deleted_count > 0 {
    Ok(true)
  } else {
    Ok(false)
  }
}

pub async fn insert_one_user(
  collection: &mongodb::Collection<User>,
  user: User,
) -> Result<(), String> {
  let result = collection
    .insert_one(user, None)
    .await
    .map_err(|e| format!("Error al intentar insertar en la base de datos: {:?}", e))?;
  if result.inserted_id.as_object_id().is_some() {
    Ok(())
  } else {
    Err("Error al insertar el usuario, no se pudo obtener el ID insertado".to_string())
  }
}