use crate::{
  types::{ApiKey, PartialUser, User, UserCredendials},
  Backend, Error, Result, Tokenizer,
};
use rocket::{
  response::status::{Created, NoContent},
  serde::json::{self, Json},
  State,
};

#[post("/authenticate", data = "<credentials>")]
pub fn authenticate_user(
  credentials: std::result::Result<Json<UserCredendials>, json::Error<'_>>,
  tokenizer: &State<Tokenizer>,
  backend: &State<Backend>,
) -> Result<Json<ApiKey>> {
  let credentials = credentials?;

  backend
    .find_user(credentials.into_inner())
    .and_then(|user| {
      tokenizer.generate().map(|token| User {
        token: Some(token),
        ..user
      })
    })
    .and_then(|user| backend.update_user(user))
    .map(|user| {
      Json(ApiKey {
        token: user.token.unwrap(),
      })
    })
}

#[post("/users", data = "<user>")]
pub fn add_user(
  user: std::result::Result<Json<User>, json::Error<'_>>,
  api_key: std::result::Result<ApiKey, Error>,
  backend: &State<Backend>,
) -> Result<Created<()>> {
  let user = user?;
  let _ = api_key?;

  let username = &user.username.clone();

  backend
    .add_user(user.into_inner())
    .map(|_| Created::new(format!("/users/{}", username)))
}

#[delete("/users/<username>")]
pub fn delete_user(
  username: String,
  api_key: std::result::Result<ApiKey, Error>,
  backend: &State<Backend>,
) -> Result<NoContent> {
  let _ = api_key?;

  backend.delete_user(&username).map(|_| NoContent)
}

#[get("/users")]
pub fn get_all_users(
  backend: &State<Backend>,
  api_key: std::result::Result<ApiKey, Error>,
) -> Result<Json<Vec<PartialUser>>> {
  let _ = api_key?;

  Ok(Json(
    backend
      .list_users()?
      .into_iter()
      .map(PartialUser::from)
      .collect(),
  ))
}