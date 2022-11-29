use axum::extract::Path;
use axum::{
	extract::State,
	routing::{get, post, put},
	Json, Router,
};

use crate::domain::helper::user::{CreatUserVO, LoginUserVO, UpdateUserVO, UserBO, UserVO};
use crate::error::Result;
use crate::extractor::{AuthUserClaims, ValidatedJson};
use crate::service::UserService;
use crate::AppState;

/// user router
pub fn router(state: AppState) -> Router<AppState> {
	Router::with_state(state)
		.route("/api/users", post(create_user))
		.route("/api/users/login", post(login_user))
		.route("/api/user", get(get_current_user).put(update_user))
		.route("/api/user/confirm/:token", put(confirm_user))
}

async fn create_user(
	State(app_state): State<AppState>,
	ValidatedJson(req): ValidatedJson<UserVO<CreatUserVO>>,
) -> Result<Json<UserVO<UserBO>>> {
	let creat_user = req.user.generate_hash_password().await?;
	Ok(UserService::create_user(&app_state, creat_user.into()).await?)
}

async fn login_user(
	State(app_state): State<AppState>,
	ValidatedJson(req): ValidatedJson<UserVO<LoginUserVO>>,
) -> Result<Json<UserVO<UserBO>>> {
	Ok(UserService::login_user(&app_state, &req.user).await?)
}

async fn get_current_user(
	State(app_state): State<AppState>,
	auth_user: AuthUserClaims,
) -> Result<Json<UserVO<UserBO>>> {
	Ok(UserService::get_current_user(&app_state, &auth_user.id).await?)
}

async fn update_user(
	State(app_state): State<AppState>,
	auth_user: AuthUserClaims,
	ValidatedJson(req): ValidatedJson<UserVO<UpdateUserVO>>,
) -> Result<Json<UserVO<UserBO>>> {
	if req.user == UpdateUserVO::default() {
		return get_current_user(State(app_state.to_owned()), auth_user).await;
	}
	let update_user = req.user.generate_hash_password().await?;
	Ok(UserService::update_user(&app_state, &auth_user.id, &update_user).await?)
}

async fn confirm_user(State(app_state): State<AppState>, Path(token): Path<String>) -> Result<Json<UserVO<UserBO>>> {
	Ok(UserService::confirm_user(&app_state, &token).await?)
}
