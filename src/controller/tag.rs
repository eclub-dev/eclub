use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};

use crate::domain::helper::tag::TagVO;
use crate::error::Result;
use crate::service::TagService;
use crate::AppState;

pub fn router(state: AppState) -> Router<AppState> {
	Router::with_state(state).route("/api/tag", get(get_tag_list))
}

pub async fn get_tag_list(State(app_state): State<AppState>) -> Result<Json<TagVO>> {
	Ok(TagService::get_tag_list(&app_state).await?)
}
