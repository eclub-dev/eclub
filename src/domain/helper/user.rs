use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use std::convert::{From, Into};
use validator::Validate;

use crate::domain::models::user::{ActiveModel, Model};
use crate::error::{Error, Result};
use crate::extractor::AuthUserClaims;

#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct UserVO<T: Validate> {
	#[validate]
	pub user: T,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct CreatUserVO {
	#[validate(length(min = 1, max = 64, code = "username length must be greater than 1 and less than 64"))]
	pub username: String,
	#[validate(email(code = "Email format is incorrect"))]
	pub email: String,
	#[validate(length(min = 6, code = "password length must be greater than 6"))]
	pub password: Option<String>,
}

impl CreatUserVO {
	pub async fn generate_hash_password(self) -> Result<CreatUserVO> {
		let password = hash_password(self.password.unwrap_or_default()).await?;
		Ok(CreatUserVO {
			password: Some(password),
			..self
		})
	}
}

impl From<CreatUserVO> for ActiveModel {
	fn from(creat_user: CreatUserVO) -> Self {
		Self {
			username: Set(creat_user.username.to_owned()),
			email: Set(creat_user.email.to_owned()),
			password: Set(creat_user.password.unwrap_or_default().to_owned()),
			..Default::default()
		}
	}
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct LoginUserVO {
	#[validate(email(code = "Email format is incorrect"))]
	pub email: String,
	#[validate(length(min = 6, max = 64, code = "password length must be greater than 6 and less than 64"))]
	pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate, Default, PartialEq, Eq)]
#[serde(default)]
pub struct UpdateUserVO {
	#[validate(email(code = "Email format is incorrect"))]
	pub email: String,
	#[validate(length(min = 1, max = 64, code = "username length must be greater than 1 and less than 64"))]
	pub username: String,
	#[validate(length(min = 6, max = 64, code = "password length must be greater than 6 and less than 64"))]
	pub password: Option<String>,
	#[validate(length(max = 64, code = "bio length must be less than 64"))]
	pub bio: String,
	#[validate(url)]
	pub avatar: String,
}

impl UpdateUserVO {
	pub async fn generate_hash_password(self) -> Result<UpdateUserVO> {
		let password = hash_password(self.password.unwrap_or_default().to_owned()).await?;
		Ok(UpdateUserVO {
			password: Some(password),
			..self
		})
	}
}

impl From<UpdateUserVO> for ActiveModel {
	fn from(update_user: UpdateUserVO) -> Self {
		Self {
			email: Set(update_user.email.to_owned()),
			username: Set(update_user.username.to_owned()),
			password: Set(update_user.password.unwrap_or_default().to_owned()),
			bio: Set(update_user.bio.to_owned()),
			avatar: Set(update_user.avatar.to_owned()),
			..Default::default()
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Validate, Default, PartialEq, Eq)]
pub struct UserBO {
	pub username: Option<String>,
	pub token: Option<String>,
	pub email: Option<String>,
	pub bio: Option<String>,
	pub avatar: Option<String>,
}

impl From<Model> for UserBO {
	fn from(model: Model) -> Self {
		Self {
			username: Some(model.username.to_owned()),
			token: Some(AuthUserClaims::generate_token(model.id, model.username.to_owned())),
			email: Some(model.email.to_owned()),
			bio: Some(model.bio.to_owned()),
			avatar: Some(model.avatar.to_owned()),
		}
	}
}

pub async fn hash_password(password: String) -> Result<String> {
	Ok(tokio::task::spawn_blocking(move || -> Result<String> {
		let salt = SaltString::generate(rand::thread_rng());
		Ok(PasswordHash::generate(Argon2::default(), password, salt.as_str())
			.map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
			.to_string())
	})
	.await
	.context("panic in generating password hash")??)
}

pub async fn verify_password(password: String, password_hash: String) -> Result<()> {
	Ok(tokio::task::spawn_blocking(move || -> Result<()> {
		let hash = PasswordHash::new(&password_hash).map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

		hash.verify_password(&[&Argon2::default()], password).map_err(|e| match e {
			argon2::password_hash::Error::Password => Error::Unauthorized,
			_ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
		})
	})
	.await
	.context("panic in verifying password hash")??)
}
