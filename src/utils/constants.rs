/// Token session duration
pub const DEFAULT_SESSION_LENGTH: time::Duration = time::Duration::weeks(2);

/// Header token for the Authorization
pub const SCHEME_PREFIX: &str = "Token ";

/// email constants
pub mod email {
	/// email confirm body
	pub const CONFIRM_BODY: &str = "Hello {}! Welcome. Please click the
									link below to verify your email address. Thank you!\n
									https://{}/confirm/{}";
	/// email confirm subject
	pub const CONFIRM_SUBJECT: &str = "Please confirm your email address";
}

/// regex constants
pub mod re {
	/// mysql error extract key
	pub const DUPLICATE_KEY: &str = r"['](?P<key>.*?)[']";
}
