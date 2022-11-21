use crate::utils::constants::re::DUPLICATE_KEY;
use lazy_static::lazy_static;
use regex::Regex;

pub fn extract_key(input: &str) -> Option<String> {
	lazy_static! {
		static ref RE: Regex = Regex::new(DUPLICATE_KEY).unwrap();
	}
	RE.captures(input).and_then(|cap| cap.name("key").map(|key| key.as_str().to_string()))
}
