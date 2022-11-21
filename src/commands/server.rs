/// the server command to launch the server
#[derive(Debug, Clone, clap::Parser)]
pub struct ServerCmd {
	/// bind port. Default value is 9988
	#[arg(long, env, default_value_t = 9988)]
	pub server_port: u16,
	/// The connection URL for the database.
	#[arg(long, env)]
	pub database_url: String,
	/// max connections to the database. Default value is 25
	#[arg(long, env, default_value_t = 25)]
	pub max_connections: u32,
}

impl ServerCmd {
	/// run server command
	pub async fn run(&self) -> anyhow::Result<()> {
		Ok(())
	}
}
