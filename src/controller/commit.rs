use crate::domain::helper::commit::{CommitBO, CommitVO, CreateCommitVO, ListCommitVO};
use crate::error::Result;
use crate::extractor::{AuthUserClaims, ValidatedJson};
use crate::service::commit::CommitService;
use crate::AppState;
use axum::extract::{Path, State};
use axum::routing::{delete, get};
use axum::{Json, Router};

pub fn router(state: AppState) -> Router<AppState> {
	Router::with_state(state)
		.route("/api/articles/:ulid/comments", get(list_comment).post(add_comment))
		.route("/api/articles/:ulid/comments/:comment_id", delete(delete_comment))
}

async fn add_comment(
	State(app_state): State<AppState>,
	auth_user: AuthUserClaims,
	Path(ulid): Path<String>,
	ValidatedJson(req): ValidatedJson<CommitVO<CreateCommitVO>>,
) -> Result<Json<CommitVO<CommitBO>>> {
	Ok(CommitService::add_comment(&app_state, &auth_user.id, &ulid[..], req.commit).await?)
}

async fn delete_comment(
	State(app_state): State<AppState>,
	auth_user: AuthUserClaims,
	Path((ulid, comment_id)): Path<(String, u64)>,
) -> Result<()> {
	Ok(CommitService::delete_comment(&app_state, &auth_user.id, &ulid[..], &comment_id).await?)
}

async fn list_comment(State(app_state): State<AppState>, Path(ulid): Path<String>) -> Result<Json<ListCommitVO>> {
	Ok(CommitService::list_comment(&app_state, &ulid[..]).await?)
}
