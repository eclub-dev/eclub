use crate::domain::helper::email::EmailSendBO;
use crate::error::{Result};
use crate::utils::constants::email::{CONFIRM_SUBJECT};
use crate::AppState;
use sea_orm::ActiveValue::Set;
use sea_orm::*;

use crate::domain::models::email::{ActiveModel as EmailActiveModel};

/// Email ServiceBuilder
pub struct EmailService;

impl EmailService {
	pub async fn send_user_confirm(app_state: &AppState, email_send: &EmailSendBO) -> Result<()> {
		let emails = app_state.emails.to_owned();
		let conn = app_state.conn.to_owned();
		let config = app_state.config.to_owned();

		let body = format!(
			"Hello {}! Welcome. Please click the link below to verify your email address. Thank you!\n
									https://{}/confirm/{}",
			email_send.username.to_owned(),
			config.domain,
			email_send.token.to_owned(),
		);

		emails.send(&email_send.email.to_owned(), CONFIRM_SUBJECT, &body)?;
		EmailActiveModel {
			user_id: Set(email_send.user_id.to_owned()),
			email: Set(email_send.email.to_owned()),
			token: Set(email_send.token.to_owned()),
			..Default::default()
		}
		.insert(&conn)
		.await?;
		Ok(())
	}
}
