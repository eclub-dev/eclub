#[cfg(feature = "local")]

use crate::{AppState, Config};
use anyhow::Context;
use axum::Router;
use migration::{Migrator, MigratorTrait};
use sea_orm::*;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{trace::TraceLayer};

use crate::controller::{user,tag, category};
use crate::utils::email::Emails;

fn api_router(state: AppState) -> Router<AppState> {
	user::router(state.to_owned())
		.merge(tag::router(state.to_owned()))
		.merge(category::router(state.to_owned()))
}

/// local server run
pub async fn run() -> anyhow::Result<()> {
	dotenv::dotenv().ok();

	let config = Config::from_environment();

	let emails =
		Emails::from_environment(&config.smtp_host, &config.smtp_port, &config.smtp_username, &config.smtp_password);

	let conn = set_up_db(&config.database_url).await?;
	Migrator::up(&conn, None).await?;

	let app = api_router(AppState {
		conn,
		emails,
		config,
	})
	.layer(
		ServiceBuilder::new()
			.layer(TraceLayer::new_for_http())
	);

	let addr = format!("127.0.0.1:{}", 9988);
	println!("listening on {}", addr);

	axum::Server::bind(&"0.0.0.0:9988".parse()?)
		.serve(app.into_make_service())
		.await
		.context("error running HTTP server")
}

/// set up database connection
pub async fn set_up_db(database_url: &str) -> anyhow::Result<DatabaseConnection> {
	//TODO: configure connect options
	let mut opt = ConnectOptions::new(database_url.to_owned());
	opt.max_connections(100)
		.min_connections(5)
		.connect_timeout(Duration::from_secs(8))
		.idle_timeout(Duration::from_secs(8))
		.max_lifetime(Duration::from_secs(8))
		.sqlx_logging(true);

	let db = Database::connect(opt.to_owned()).await?;
	Ok(db)
}
