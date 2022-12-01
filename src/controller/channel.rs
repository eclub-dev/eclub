use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};

use crate::error::Result;
use crate::AppState;
use crate::domain::helper::channel::ChannelVO;
use crate::service::channel::ChannelService;

pub fn router(state: AppState) -> Router<AppState> {
	Router::with_state(state).route("/api/channel", get(get_channel_list))
}

pub async fn get_channel_list(State(app_state): State<AppState>) -> Result<Json<ChannelVO>> {
	Ok(ChannelService::get_channel_list(&app_state).await?)
}
