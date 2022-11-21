use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::transport::smtp::SmtpTransport;
use lettre::{Message, Transport};

use crate::error::{Error, Result};

/// Email
#[derive(Clone)]
pub struct Emails {
	backend: EmailBackend,
}

impl Emails {
	/// Create a new instance
	pub fn from_environment(smtp_host: &str, smtp_port: &u16, smtp_username: &str, smtp_password: &str) -> Self {
		let backend = EmailBackend::Smtp {
			host: smtp_host.to_owned(),
			port: smtp_port.to_owned(),
			username: smtp_username.to_owned(),
			password: smtp_password.to_owned(),
		};
		Self {
			backend,
		}
	}

	/// send emails
	pub fn send(&self, recipient: &str, subject: &str, body: &str) -> Result<()> {
		let EmailBackend::Smtp {
			host,
			port,
			username,
			password,
		} = &self.backend;
		let email = Message::builder()
			.to(recipient.parse()?)
			.from(username.parse()?)
			.subject(subject)
			.body(body.to_string())?;
		SmtpTransport::relay(host)
			.and_then(|transport| {
				transport
					.credentials(Credentials::new(username.to_owned(), password.to_owned()))
					.authentication(vec![Mechanism::Plain])
					.port(port.to_owned())
					.build()
					.send(&email)
			})
			.map_err(|e| Error::SMTP(e))?;
		Ok(())
	}
}

#[derive(Clone)]
enum EmailBackend {
	/// SMTP.
	Smtp {
		host: String,
		port: u16,
		username: String,
		password: String,
	},
}
