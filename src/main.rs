mod handlers;
mod models;
mod utils;
mod validators;
mod hashing;
use crate::handlers::{create_user_handle, delete_handle, get_users_handle, login_handle};
use crate::utils::mongodb_init;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let collection = mongodb_init().await?;
  let collection = web::Data::new(collection);
  HttpServer::new(move || {
    App::new()
      .app_data(collection.clone())
      .service(get_users_handle)
      .service(create_user_handle)
      .service(login_handle)
      .service(delete_handle)
  })
  .bind(("127.0.0.1", 3333))?
  .run()
  .await
}