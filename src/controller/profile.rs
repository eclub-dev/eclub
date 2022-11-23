use crate::domain::helper::profile::{ProfileBO, ProfileVO};
use crate::error::Result;
use crate::extractor::AuthUserClaims;
use crate::service::profile::ProfileService;
use crate::AppState;
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};

pub fn router(state: AppState) -> Router<AppState> {
	Router::with_state(state)
		.route("/api/profile/:username", get(get_user_profile))
		.route("/api/profile/:username/follow", post(follow_user).delete(unfollow_user))
}

/// TODO: consider the user not logged in
async fn get_user_profile(
	State(app_state): State<AppState>,
	auth_user: AuthUserClaims,
	Path(username): Path<String>,
) -> Result<Json<ProfileVO<ProfileBO>>> {
	Ok(ProfileService::get_user_profile(&app_state, &username[..], &auth_user.id).await?)
}

async fn follow_user(
	State(app_state): State<AppState>,
	auth_user: AuthUserClaims,
	Path(username): Path<String>,
) -> Result<Json<ProfileVO<ProfileBO>>> {
	Ok(ProfileService::follow_user(&app_state, &username[..], &auth_user.id).await?)
}

async fn unfollow_user(
	State(app_state): State<AppState>,
	auth_user: AuthUserClaims,
	Path(username): Path<String>,
) -> Result<Json<ProfileVO<ProfileBO>>> {
	Ok(ProfileService::follow_user(&app_state, &username[..], &auth_user.id).await?)
}
