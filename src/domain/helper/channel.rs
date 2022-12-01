use sea_orm::{DeriveColumn, EnumIter, IdenStatic};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ChannelVO {
	pub channel_list: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum QueryAs {
	Channel,
}
