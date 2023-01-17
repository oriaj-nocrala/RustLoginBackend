use std::io::Error;

use mongodb::{options::ClientOptions, Client, Collection};
use crate::models::User;

pub async fn mongodb_init() -> Result<Collection<User>, Error> {
  let client_options = match ClientOptions::parse("mongodb://localhost:27017").await {
    Ok(client_options) => client_options,
    Err(e) => {
      return Err(mongodb_error_to_io_error(e));
    }
  };
  let client = match Client::with_options(client_options) {
    Ok(client) => client,
    Err(e) => {
      return Err(mongodb_error_to_io_error(e));
    }
  };
  let db = client.database("login_rust");
  let collection: Collection<User> = db.collection("users");
  Ok(collection)
}

fn mongodb_error_to_io_error(err: mongodb::error::Error) -> std::io::Error {
  std::io::Error::new(std::io::ErrorKind::Other, err)
}