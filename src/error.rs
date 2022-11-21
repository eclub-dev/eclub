use axum::http::header::WWW_AUTHENTICATE;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use validator::{Validate, ValidationErrors};

use axum::extract::rejection::{FormRejection, JsonRejection};
use axum::{Json};
use lettre::address::AddressError;
use lettre::error::Error as LetError;
use lettre::transport::smtp::Error as SMTPError;
use sea_orm::{DbErr, RuntimeErr};
use std::borrow::Cow;
use std::collections::HashMap;
use thiserror::Error as ThisError;

use crate::utils::constants::SCHEME_PREFIX;
use crate::utils::re::extract_key;

/// custom result
pub type Result<T, E = Error> = std::result::Result<T, E>;

///  error
#[derive(ThisError, Debug)]
pub enum Error {
	/// Return `400 Unauthorized`
	#[error("client error")]
	BadRequest,

	/// Return `401 Unauthorized`
	#[error("authentication required")]
	Unauthorized,

	/// Return `403 Forbidden`
	#[error("not accept that authentication")]
	Forbidden,

	/// Return `404 Not Found`
	#[error("resource not found")]
	NotFound,

	/// Return `500 Internal Server Error`
	#[error("Internal Server Error")]
	InternalServerError,

	/// Return `422 Unprocessable Entity`
	#[error("error in the request body")]
	UnprocessableEntity {
		/// errors map
		errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
	},

	/// validator
	#[error(transparent)]
	ValidationError(#[from] validator::ValidationErrors),

	/// axum FormRejection
	#[error(transparent)]
	AxumFormRejection(#[from] FormRejection),

	/// axum JsonRejection
	#[error(transparent)]
	AxumJsonRejection(#[from] JsonRejection),

	/// email error
	#[error("Email error: {0}")]
	Email(LetError),

	/// smtp error
	#[error("service error: {0}")]
	SMTP(SMTPError),

	/// database error
	#[error("Database error: {0}")]
	Database(DbErr),

	/// Source and Display delegate to anyhow::Error
	#[error(transparent)]
	Anyhow(#[from] anyhow::Error),
}

impl From<DbErr> for Error {
	fn from(error: DbErr) -> Error {
		match error {
			DbErr::RecordNotFound(_) => Error::NotFound,
			DbErr::Custom(_) => Error::NotFound,
			_ => Error::Database(error),
		}
	}
}

impl From<AddressError> for Error {
	fn from(error: AddressError) -> Self {
		match error {
			_ => Error::InternalServerError,
		}
	}
}

impl From<LetError> for Error {
	fn from(error: LetError) -> Self {
		match error {
			_ => Error::Email(error),
		}
	}
}

impl Error {
	/// unprocessable entity
	pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
	where
		K: Into<Cow<'static, str>>,
		V: Into<Cow<'static, str>>,
	{
		let mut error_map = HashMap::new();

		for (key, val) in errors {
			error_map.entry(key.into()).or_insert_with(Vec::new).push(val.into());
		}

		Self::UnprocessableEntity {
			errors: error_map,
		}
	}

	fn status_code(&self) -> StatusCode {
		match self {
			Self::Unauthorized => StatusCode::UNAUTHORIZED,
			Self::Forbidden => StatusCode::FORBIDDEN,
			Self::NotFound => StatusCode::NOT_FOUND,
			Self::UnprocessableEntity {
				..
			} => StatusCode::UNPROCESSABLE_ENTITY,
			Self::Database(_) => StatusCode::UNPROCESSABLE_ENTITY,
			Self::Anyhow(_) | Self::SMTP(_) => StatusCode::INTERNAL_SERVER_ERROR,
			_ => StatusCode::SERVICE_UNAVAILABLE,
		}
	}

	/// map error
	pub fn map_err(self) -> Error {
		match self {
			_ => Error::NotFound,
		}
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		match self {
			Self::UnprocessableEntity {
				errors,
			} => {
				#[derive(serde::Serialize)]
				struct Errors {
					errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
				}

				return (
					StatusCode::UNPROCESSABLE_ENTITY,
					Json(Errors {
						errors,
					}),
				)
					.into_response();
			}
			Self::Unauthorized => {
				return (
					self.status_code(),
					[(WWW_AUTHENTICATE, HeaderValue::from_static(SCHEME_PREFIX.trim()))]
						.into_iter()
						.collect::<HeaderMap>(),
					self.to_string(),
				)
					.into_response();
			}

			Self::SMTP(ref e) => {
				tracing::error!("SMTP error: {:?}", e);
				return (self.status_code(), "").into_response();
			}

			Self::Database(ref e) => {
				match e {
					// FIXME: more precise error
					// https://dev.mysql.com/doc/mysql-errors/8.0/en/server-error-reference.html
					DbErr::Exec(RuntimeErr::SqlxError(sqlx::Error::Database(dbe))) => {
						// tracing::error!("Database error: {:?}", e);
						// let mysql_err = dbe.downcast_ref::<MySqlDatabaseError>();
						//
						// if mysql_err.number() == 1062 {
						// 	// Message: Duplicate entry '%s' for key %d
						//
						// }

						let key = extract_key(&dbe.message().to_owned()).unwrap_or("".to_string());

						return (self.status_code(), key).into_response();
					}
					_ => (),
				}
			}

			Self::Anyhow(ref e) => {
				tracing::error!("Generic error: {:?}", e);
			}

			_ => (),
		}

		(self.status_code(), self.to_string()).into_response()
	}
}

/// Field Validator
pub struct FieldValidator {
	errors: ValidationErrors,
}

impl Default for FieldValidator {
	fn default() -> Self {
		Self {
			errors: ValidationErrors::new(),
		}
	}
}

impl FieldValidator {
	/// validate the model and get  errors
	pub fn validate<T: Validate>(model: &T) -> Self {
		Self {
			errors: model.validate().err().unwrap_or_else(ValidationErrors::new),
		}
	}
	/// check errors and map errors
	pub fn check(self) -> Result<(), Error> {
		if self.errors.is_empty() {
			Ok(())
		} else {
			use validator::ValidationErrorsKind::{Field, List, Struct};

			let mut error_map = HashMap::new();
			// FIXME: optimize logic
			for (key, val) in self.errors.into_errors() {
				match val {
					Field(field_errors) => {
						error_map.insert(
							Cow::Borrowed(key),
							field_errors.into_iter().map(|field_error| field_error.code).collect(),
						);
					}
					List(list_errors) => {
						for (_error_key, error_val) in list_errors {
							for (list_key, list_val) in error_val.into_errors() {
								if let Field(field_errors) = list_val {
									error_map.insert(
										Cow::Borrowed(list_key),
										field_errors.into_iter().map(|field_error| field_error.code).collect(),
									);
								}
							}
						}
					}
					Struct(struct_errors) => {
						for (struct_key, struct_val) in struct_errors.into_errors() {
							if let Field(field_errors) = struct_val {
								error_map.insert(
									Cow::Borrowed(struct_key),
									field_errors.into_iter().map(|field_error| field_error.code).collect(),
								);
							}
						}
					}
				}
			}

			Err(Error::UnprocessableEntity {
				errors: error_map,
			})
		}
	}
}

/// Result handler function
pub trait ResultExt<T> {
	/// map result error, return T.
	fn to_api(self) -> Result<T, Error>;
}

impl<T> ResultExt<T> for Result<T> {
	#[inline]
	fn to_api(self) -> Result<T, Error> {
		self.map_err(Error::map_err)
	}
}
