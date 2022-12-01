use crate::AppState;
use axum::Json;
use sea_orm::*;

use crate::domain::helper::channel::{QueryAs, ChannelVO};
use crate::domain::models::channel::{self, Entity};
use crate::error::Result;

/// channel ServiceBuilder
pub struct ChannelService;

impl ChannelService {
	pub async fn get_channel_list(app_state: &AppState) -> Result<Json<ChannelVO>> {
		let channel_list: Vec<String> = Entity::find()
			.select_only()
			.column_as(channel::Column::Value, QueryAs::Channel)
			.order_by(channel::Column::Weight, Order::Desc)
			.into_values::<_, QueryAs>()
			.all(&app_state.conn)
			.await?;

		Ok(Json(ChannelVO {
			channel_list,
		}))
	}
}
