use std::env;

/// environment config
#[derive(Clone)]
pub struct Config {
	/// hmac key for server encode/decode key
	pub hmac_key: String,
	/// server domain
	pub domain: String,
	/// database url for database connection
	pub database_url: String,
	/// smtp host
	pub smtp_host: String,
	/// smtp port
	pub smtp_port: u16,
	/// smtp username
	pub smtp_username: String,
	/// smtp password
	pub smtp_password: String,
}

impl Config {
	/// Load the env environment variable, panic if there is a problem
	pub fn from_environment() -> Self {
		let hmac_key = env::var("HMAC_KEY").expect("DATABASE_URL is not set in .env file");
		let domain = env::var("DOMAIN").expect("DOMAIN is not set in .env file");
		let database_url = env::var("DATABASE_URL").expect("DOMAIN is not set in .env file");
		let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST is not set in .env file");
		let smtp_port = env::var("SMTP_PORT")
			.expect("SMTP_PORT is not set in .env file")
			.parse::<u16>()
			.expect("There is a problem in parsing string to u16 for SMTP_PORT");
		let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME is not set in .env file");
		let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD is not set in .env file");

		Self {
			hmac_key,
			domain,
			database_url,
			smtp_host,
			smtp_port,
			smtp_username,
			smtp_password,
		}
	}
}
