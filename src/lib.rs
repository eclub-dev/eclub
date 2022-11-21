//! `eclub` is an modern community platform built in Rust
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_imports)]
use sea_orm::DatabaseConnection;
use crate::utils::config::Config;
use crate::utils::email::Emails;

/// This is a app run for local develop.
#[cfg(feature = "local")]
pub mod app;
/// This is the command line interface for the application.
pub mod cli;
/// This is the module that contains all the commands that we'll use to run the application.
pub mod commands;
/// This is controller module.
pub mod controller;
/// This is domain module.
pub mod domain;
/// This is error module.
pub mod error;
/// This is middleware module.
pub mod middleware;
/// This is service module.
pub mod service;
/// This is utils module.
pub mod utils;
/// This is extractor module that contains all the extractor that we'll use to request.
pub mod extractor;


/// AppState struct.
///
/// Properties:
///
/// * `conn`: DatabaseConnection - this is the database connection that we'll use to connect to the database.
/// * `emails`: Emails - This is the email service that we'll use to send emails.
/// * `config`: This is the configuration for the application env value.
#[derive(Clone)]
pub struct AppState {
	/// database connection
	conn: DatabaseConnection,
	/// email
	emails: Emails,
	/// environment config
	config: Config,
}
