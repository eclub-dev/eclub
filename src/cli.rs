use crate::commands::ServerCmd;
use clap::{crate_authors, crate_description, crate_version, CommandFactory, FromArgMatches};
#[cfg(feature = "local")]
use crate::app;


#[allow(missing_docs)]
#[derive(Debug, clap::Parser)]
pub struct Cli {
	#[allow(missing_docs)]
	#[command(subcommand)]
	pub subcommand: Option<Subcommand>,
}

impl Cli {
	fn args() -> Self {
		let app = <Self as CommandFactory>::command();
		let app = app.version(crate_version!()).author(crate_authors!()).about(crate_description!());
		let matches = app.try_get_matches_from(&mut std::env::args_os()).unwrap_or_else(|e| e.exit());
		<Self as FromArgMatches>::from_arg_matches(&matches).unwrap_or_else(|e| e.exit())
	}
}

/// sub command
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
	/// launch server
	Server(ServerCmd),
}

/// run handler
pub async fn run() -> anyhow::Result<()> {
	let cli = Cli::args();
	dotenv::dotenv().ok();

	#[cfg(feature = "local")]
	tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_test_writer().init();

	match &cli.subcommand {
		Some(Subcommand::Server(cmd)) => cmd.run().await,
		#[cfg(feature = "local")]
		None => app::run().await,
		#[cfg(not(feature = "local"))]
		None => async { println!("not ready for product"); Ok(()) }.await,

	}
}
