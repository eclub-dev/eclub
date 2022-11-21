use axum::extract::rejection::{FormRejection, JsonRejection};
use axum::extract::{FromRequest, FromRequestParts};
use axum::headers::HeaderValue;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use axum::http::Request;
use axum::{async_trait, Form, Json};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::Sha384;
use std::env;
use time::OffsetDateTime;
use validator::Validate;

use crate::error::{Error, FieldValidator, Result};
use crate::utils::constants::{DEFAULT_SESSION_LENGTH, SCHEME_PREFIX};

/// user authentication claims
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthUserClaims {
	/// timestamp
	pub exp: i64,
	/// user id
	pub id: u64,
	/// user name
	pub username: String,
}

impl AuthUserClaims {
	/// generate token for claims
	pub fn generate_token(id: u64, username: String) -> String {
		let exp = (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp();
		Self {
			exp,
			id,
			username,
		}
		.jwt()
	}

	/// get jwt token
	pub fn jwt(&mut self) -> String {
		let hmac_key = env::var("HMAC_KEY").expect("No HMAC_KEY environment variable found");

		let hmac = Hmac::<Sha384>::new_from_slice(hmac_key.as_bytes()).expect("HMAC-SHA-384 can accept any key length");

		self.sign_with_key(&hmac).expect("HMAC signing should be infallible")
	}
	/// from_request header extract
	pub fn from_authorization(auth_header: &HeaderValue) -> Result<Self> {
		let hmac_key = env::var("HMAC_KEY").expect("No HMAC_KEY environment variable found");

		let auth_header = auth_header.to_str().map_err(|_| {
			tracing::debug!("Authorization header is not UTF-8");
			Error::Unauthorized
		})?;

		if !auth_header.starts_with(SCHEME_PREFIX) {
			tracing::debug!("Authorization header is using the wrong scheme: {:?}", auth_header);
			return Err(Error::Unauthorized);
		}

		let token = &auth_header[SCHEME_PREFIX.len()..];

		let jwt = jwt::Token::<jwt::Header, AuthUserClaims, _>::parse_unverified(token).map_err(|e| {
			tracing::debug!("failed to parse Authorization header {:?}: {}", auth_header, e);
			Error::Unauthorized
		})?;

		let hmac = Hmac::<Sha384>::new_from_slice(hmac_key.as_bytes()).expect("HMAC-SHA-384 can accept any key length");

		let jwt = jwt.verify_with_key(&hmac).map_err(|e| {
			tracing::debug!("JWT failed to verify: {}", e);
			Error::Unauthorized
		})?;

		let (_header, claims) = jwt.into();
		if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
			tracing::debug!("token expired");
			return Err(Error::Unauthorized);
		}

		Ok(Self {
			..claims
		})
	}
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUserClaims
where
	S: Send + Sync,
{
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> std::result::Result<Self, Self::Rejection> {
		let auth_header = parts.headers.get(AUTHORIZATION).ok_or(Error::Unauthorized)?;
		Self::from_authorization(auth_header)
	}
}

/// ValidatedForm struct for verified form
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedForm<T>
where
	T: DeserializeOwned + Validate,
	S: Send + Sync,
	Form<T>: FromRequest<S, B, Rejection = FormRejection>,
	B: Send + 'static,
{
	type Rejection = Error;

	async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
		let Form(value) = Form::<T>::from_request(req, state).await?;
		FieldValidator::validate(&value).check()?;
		Ok(ValidatedForm(value))
	}
}

/// ValidatedForm struct for verified json
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
	T: DeserializeOwned + Validate,
	S: Send + Sync,
	Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
	B: Send + 'static,
{
	type Rejection = Error;

	async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
		let Json(value) = Json::<T>::from_request(req, state).await?;
		FieldValidator::validate(&value).check()?;
		Ok(ValidatedJson(value))
	}
}
