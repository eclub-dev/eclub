use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use crate::domain::models::user::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileVO<T> {
	pub profile: T,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct ProfileBO {
	pub username: String,
	pub email: String,
	pub bio: String,
	pub avatar: String,
	pub following: bool,
}


impl ProfileBO {
	pub fn from_user(model: Model, following: bool) -> Self {
		Self {
			username: model.username.to_owned(),
			email: model.email.to_owned(),
			bio: model.bio.to_owned(),
			avatar: model.avatar.to_owned(),
			following,
		}
	}
}
