use axum::extract::{Path, State};
use axum::routing::{post,get};
use axum::{Json, Router};

use crate::domain::helper::article::{ArticleBO, ArticleVO, CreateArticleVO, UpdateArticleVO};
use crate::extractor::{AuthUserClaims, ValidatedJson};
use crate::service::ArticleService;
use crate::AppState;

use crate::error::Result;

pub fn router(state: AppState) -> Router<AppState> {
	Router::with_state(state)
		.route("/api/articles", post(create_or_update_article))
		.route("/api/articles/:slug", get(get_article).put(create_or_update_article))
}

/// for writer
async fn create_or_update_article(
	State(app_state): State<AppState>,
	auth_user: AuthUserClaims,
	Path(slug): Path<String>,
	ValidatedJson(mut req): ValidatedJson<ArticleVO<CreateArticleVO>>,
) -> Result<Json<ArticleVO<ArticleBO>>> {
	Ok(ArticleService::create_or_update(&app_state, &req.article, &slug[..], &auth_user.id).await?)
}

async fn get_article(
	State(app_state): State<AppState>,
	Path(ulid): Path<String>,
) -> Result<Json<ArticleVO<ArticleBO>>> {
	Ok(ArticleService::get_article(&app_state, &ulid[..]).await?)
}

/// for writer
async fn delete_article(
	State(app_state): State<AppState>,
	auth_user: AuthUserClaims,
	Path(slug): Path<String>,
) -> Result<()> {
	Ok(ArticleService::delete_article(&app_state, &slug[..], &auth_user.id).await?)
}

async fn favorite_article(
	State(app_state): State<AppState>,
	auth_user: AuthUserClaims,
	Path(ulid): Path<String>,
) -> Result<()> {
	Ok(ArticleService::favorite_article(&app_state, &ulid[..], &auth_user.id).await?)
}

async fn unfavorite_article(
	State(app_state): State<AppState>,
	auth_user: AuthUserClaims,
	Path(ulid): Path<String>,
) -> Result<()> {
	Ok(ArticleService::unfavorite_article(&app_state, &ulid[..], &auth_user.id).await?)
}

async fn view_article(
	State(app_state): State<AppState>,
	auth_user: AuthUserClaims,
	Path(ulid): Path<String>,
) -> Result<()> {
	Ok(ArticleService::view_article(&app_state, &ulid[..], &auth_user.id).await?)
}




