use axum::Json;
use sea_orm::*;

use crate::domain::helper::email::EmailSendBO;
use crate::domain::helper::user::{verify_password, LoginUserVO, UserBO, UserVO};
use crate::domain::models::email::{self, Model as EmailModel};
use crate::domain::models::user::{self, ActiveModel, Entity, Model};
use crate::service::EmailService;
use crate::AppState;

use crate::error::{Error, Result};

pub struct UserService;

/// Defining a new struct called `UserService` and implementing it.
impl UserService {

	/// It saves the user, sends a confirmation email, and returns the saved user
	///
	/// Arguments:
	///
	/// * `app_state`: &AppState - This is the application state that is passed to the handler. It contains the database
	/// connection pool and other application-wide resources.
	/// * `user`: ActiveModel - This is the user model that was created from the request body.
	/// * `is_create`: true if this is a new user, false if this is an update
	///
	/// Returns:
	///
	/// A `Result` of a `Json` of a `UserVO` of a `UserBO`
	pub async fn create_or_update(
		app_state: &AppState,
		user: ActiveModel,
		is_create: bool,
	) -> Result<Json<UserVO<UserBO>>> {
		let saved_model: Model = user.save(&app_state.conn).await?.try_into_model()?;
		tracing::debug!("create_or_update: {:?}", saved_model);
		if is_create {
			EmailService::send_user_confirm(&app_state, &EmailSendBO::from_user(&saved_model)).await?;
		}
		Ok(Json(UserVO {
			user: UserBO::from(saved_model),
		}))
	}

	/// > This function takes a `LoginUserVO` and returns a `UserVO<UserBO>` if the user exists and the password is correct
	///
	/// Arguments:
	///
	/// * `app_state`: This is the application state that we created in the main.rs file.
	/// * `login_user`: &LoginUserVO
	///
	/// Returns:
	///
	/// A `UserVO`
	pub async fn login_user(app_state: &AppState, login_user: &LoginUserVO) -> Result<Json<UserVO<UserBO>>> {
		let user: Model = Entity::find()
			.filter(user::Column::Email.eq(login_user.email.to_owned()))
			.one(&app_state.conn)
			.await?
			.ok_or(Error::NotFound)?;

		verify_password(login_user.password.to_owned(), user.password.to_owned()).await?;

		tracing::debug!("login_user: {:?}", user);
		Ok(Json(UserVO {
			user: UserBO::from(user),
		}))
	}

	/// `get_current_user` is an asynchronous function that takes an `AppState` and an `id` and returns a
	/// `Result<Json<UserVO<UserBO>>>`
	///
	/// Arguments:
	///
	/// * `app_state`: &AppState - This is the application state that we created in the main.rs file.
	/// * `id`: The id of the user to retrieve.
	///
	/// Returns:
	///
	/// A `UserVO`
	pub async fn get_current_user(app_state: &AppState, id: &u64) -> Result<Json<UserVO<UserBO>>> {
		let user: Model = Entity::find_by_id(id.to_owned()).one(&app_state.conn).await?.ok_or(Error::NotFound)?;
		tracing::debug!("get_current_user: {:?}", user);
		Ok(Json(UserVO {
			user: UserBO::from(user),
		}))
	}

	/// > This function confirms a user by updating the user's `is_valid` field to `1` and returns the user's information
	///
	/// Arguments:
	///
	/// * `app_state`: &AppState - This is the application state that we created in the main.rs file.
	/// * `token`: The token that was sent to the user's email address.
	///
	/// Returns:
	///
	/// A JSON object with the user's information.
	pub async fn confirm_user(app_state: &AppState, token: &str) -> Result<Json<UserVO<UserBO>>> {
		let confirm_res: EmailModel = email::Entity::find()
			.filter(email::Column::Token.eq(token.to_owned()))
			.one(&app_state.conn)
			.await?
			.ok_or(Error::NotFound)?;

		let mut user_active: ActiveModel = Entity::find_by_id(confirm_res.user_id.to_owned())
			.one(&app_state.conn)
			.await?
			.ok_or(Error::NotFound)?
			.into();
		user_active.is_valid = Set(1);
		let user = user_active.update(&app_state.conn).await?;
		Ok(Json(UserVO {
			user: UserBO::from(user),
		}))
	}
}
