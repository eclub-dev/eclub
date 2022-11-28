use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};

use crate::domain::helper::article::{ArticleBO, ArticleVO, CreateArticleVO, ListArticlesQueryVO, UpdateArticleVO};
use crate::domain::helper::ListVO;
use crate::extractor::{AuthUserClaims, ValidatedJson};
use crate::service::ArticleService;
use crate::AppState;

use crate::error::Result;

pub fn router(state: AppState) -> Router<AppState> {
	Router::with_state(state)
		.route("/api/articles", post(upset_article).put(upset_article).get(list_articles))
		.route("/api/articles/:ulid", get(get_article).delete(delete_article).put(view_article))
		.route("/api/articles/:ulid/favorite", post(favorite_article).delete(unfavorite_article))
}

/// for writer
async fn upset_article(
	State(app_state): State<AppState>,
	auth_user: AuthUserClaims,
	ValidatedJson(mut req): ValidatedJson<ArticleVO<CreateArticleVO>>,
) -> Result<Json<ArticleVO<ArticleBO>>> {
	Ok(ArticleService::upset_article(&app_state, &req.article, &auth_user.id).await?)
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
	Path(ulid): Path<String>,
) -> Result<()> {
	Ok(ArticleService::delete_article(&app_state, &ulid[..], &auth_user.id).await?)
}

async fn favorite_article(
	State(app_state): State<AppState>,
	auth_user: AuthUserClaims,
	Path(ulid): Path<String>,
) -> Result<Json<ArticleVO<ArticleBO>>> {
	Ok(ArticleService::favorite_article(&app_state, &ulid[..], &auth_user.id).await?)
}

async fn unfavorite_article(
	State(app_state): State<AppState>,
	auth_user: AuthUserClaims,
	Path(ulid): Path<String>,
) -> Result<Json<ArticleVO<ArticleBO>>> {
	Ok(ArticleService::unfavorite_article(&app_state, &ulid[..], &auth_user.id).await?)
}

async fn view_article(State(app_state): State<AppState>, Path(ulid): Path<String>) -> Result<()> {
	Ok(ArticleService::view_article(&app_state, &ulid[..]).await?)
}

async fn list_articles(State(app_state): State<AppState>, query: Query<ListArticlesQueryVO>) -> Result<Json<ListVO>> {
	Ok(ArticleService::list_articles(&app_state, &query).await?)
}
