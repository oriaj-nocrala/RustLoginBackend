use crate::models::{UsersModel, User};
use crate::hashing::{user_hash_password, verify_password};
use crate::validators::{parse_json_body, verify_user_against_db};
use crate::models::{delete_user, find_all_users, find_by_username, insert_one_user };
use actix_web::{post, get, delete, web, HttpResponse, Responder, };
use mongodb::{Collection};


#[get("/")]
async fn get_users_handle(collection: web::Data<Collection<User>>) -> impl Responder {
  let collection = collection.get_ref();
  match find_all_users(collection).await {
    Ok(users) => HttpResponse::Ok().json(UsersModel { users }),
    Err(e) => HttpResponse::InternalServerError().body(e),
  }

}

#[post("/")]
async fn create_user_handle(collection: web::Data<Collection<User>>, body: String) -> impl Responder {
  let collection = collection.get_ref();
  let body = match parse_json_body(body) {
    Ok(body) => body,
    Err(e) => return HttpResponse::BadRequest().body(e),
  };
  let user = match user_hash_password(body) {
    Ok(user) => user,
    Err(e) => return HttpResponse::InternalServerError().body(e),
  };
  match insert_one_user(collection, user).await {
    Ok(_) => (),
    Err(e) => return HttpResponse::InternalServerError().body(e),
  };

  HttpResponse::Ok().body("Usuario creado")
}

#[post("/login")]
async fn login_handle(collection: web::Data<Collection<User>>, body: String) -> impl Responder {
  let collection = collection.get_ref();

  let body_user = match parse_json_body(body) {
    Ok(body) => body,
    Err(e) => return HttpResponse::BadRequest().body(e),
  };
  let is_valid = match verify_user_against_db(collection, body_user).await {
    Ok(is_valid) => is_valid,
    Err(e) => return HttpResponse::InternalServerError().body(e),
  };
  if is_valid {
    HttpResponse::Ok().body("Login correcto")
  } else {
    HttpResponse::Ok().body("Login incorrecto")
  }
}

#[delete("/")]
async fn delete_handle(collection: web::Data<Collection<User>>, body: String) -> impl Responder {
  let collection = collection.get_ref();

  let body_user = match parse_json_body(body) {
    Ok(body) => body,
    Err(e) => return HttpResponse::BadRequest().body(e),
  };
  match find_by_username(collection, body_user.username).await {
    Ok(user) => {
      if verify_password(body_user.pass, user.pass.clone()){
        match delete_user(collection, user).await {
          Ok(_) => HttpResponse::Ok().body("Usuario borrado"),
          Err(e) => HttpResponse::InternalServerError().body(e),
        }
      }
      else {
        HttpResponse::Ok().body("Contraseña incorrecta")
      }
    }
    Err(e) => HttpResponse::InternalServerError().body(e),
  }
}