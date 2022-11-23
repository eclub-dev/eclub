use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};

use crate::domain::helper::category::CategoryVO;
use crate::error::Result;
use crate::service::CategoryService;
use crate::AppState;

pub fn router(state: AppState) -> Router<AppState> {
	Router::with_state(state)
		.route("/api/category", get(get_tag_list))
		.route("/api/category/:username", get(get_user_category))
}

pub async fn get_tag_list(State(app_state): State<AppState>) -> Result<Json<CategoryVO>> {
	Ok(CategoryService::get_category_list(&app_state).await?)
}


async fn get_user_category(State(app_state): State<AppState>, Path(username): Path<String>) -> Result<Json<CategoryVO>> {
	Ok(CategoryService::get_user_category(&app_state, &username).await?)
}
