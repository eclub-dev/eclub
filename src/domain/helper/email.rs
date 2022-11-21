use crate::domain::models::user::Model as UserModel;
use serde::{Deserialize, Serialize};

/// EmailSend Business Object
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct EmailSendBO {
	#[allow(missing_docs)]
	pub username: String,
	#[allow(missing_docs)]
	pub user_id: u64,
	#[allow(missing_docs)]
	pub email: String,
	#[allow(missing_docs)]
	pub token: String,
}

impl EmailSendBO {
	/// `from_user` takes a `UserModel` and returns a `EmailSendBO`
	///
	/// Arguments:
	///
	/// * `model`: &UserModel - User model.
	///
	/// Returns:
	///
	/// A new instance of the EmailSendBO struct.
	pub fn from_user(model: &UserModel) -> Self {
		let token = ulid::Ulid::new().to_string();
		Self {
			username: model.username.to_owned(),
			user_id: model.id.to_owned(),
			email: model.email.to_owned(),
			token,
		}
	}
}
