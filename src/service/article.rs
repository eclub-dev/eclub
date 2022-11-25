use crate::domain::helper::article::{ArticleBO, ArticleVO, CreateArticleVO};
use crate::domain::helper::profile::ProfileBO;
use crate::domain::helper::user::UserBO;
use crate::domain::models::article::{self, ActiveModel, Entity, Model};
use crate::domain::models::user;
use crate::error::{Error, Result};
use crate::service::{ArticleCategoryService, ArticleTagService, CategoryService, TagService, UserCategoryService};
use crate::AppState;
use axum::Json;
use sea_orm::*;

pub struct ArticleService;

impl ArticleService {
	pub async fn create_or_update(
		app_state: &AppState,
		create_article: &CreateArticleVO,
		slug: &str,
		user_id: &u64,
	) -> Result<Json<ArticleVO<ArticleBO>>> {
		if let Some(Model {
			id,
			..
		}) = ArticleService::find_article(&app_state.conn, &slug, &user_id).await?
		{
			ArticleTagService::delete_by_article_id(&app_state.conn, &id).await?;
			ArticleCategoryService::delete_by_article_id(&app_state.conn, &id).await?;
		}

		let saved_model: Model = <CreateArticleVO as Into<ActiveModel>>::into(create_article.to_owned())
			.save(&app_state.conn)
			.await?
			.try_into_model()?;

		let tag_list = &create_article.tag_list;
		let category_list = &create_article.category_list;

		TagService::insert_many_by_article_id(&app_state.conn, &saved_model.id, &tag_list).await?;

		CategoryService::insert_many_by_id(&app_state.conn, &user_id, &saved_model.id, &category_list).await?;

		let user_res: user::Model =
			user::Entity::find_by_id(user_id.to_owned()).one(&app_state.conn).await?.ok_or(Error::NotFound)?;

		tracing::debug!("create or update : {:?}", saved_model);
		Ok(Json(ArticleVO {
			article: ArticleBO::from_model(
				saved_model,
				ProfileBO::from_user(user_res, false),
				tag_list.to_owned(),
				category_list.to_owned(),
			),
		}))
	}

	pub async fn get_article(app_state: &AppState, slug: &str, user_id: &u64) -> Result<Json<ArticleVO<ArticleBO>>> {
		let article: Model =
			ArticleService::find_article(&app_state.conn, &slug, &user_id).await?.ok_or(Error::NotFound)?;

		let category_list =
			ArticleCategoryService::find_by_article_id(&app_state.conn, &article.id).await?;
		let tag_list =
			ArticleTagService::find_by_article_id(&app_state.conn, &article.id).await?;

		let user_res: user::Model =
			user::Entity::find_by_id(user_id.to_owned()).one(&app_state.conn).await?.ok_or(Error::NotFound)?;

		tracing::debug!("create or update : {:?}", article);
		Ok(Json(ArticleVO {
			article: ArticleBO::from_model(
				article,
				ProfileBO::from_user(user_res, false),
				tag_list.to_owned(),
				category_list.to_owned(),
			),
		}))
	}

	pub async fn find_article(db: &DbConn, slug: &str, user_id: &u64) -> Result<Option<Model>> {
		Ok(Entity::find()
			.filter(
				Condition::all()
					.add(article::Column::Slug.eq(slug.to_owned()))
					.add(article::Column::UserId.eq(user_id.to_owned())),
			)
			.one(db)
			.await?)
	}
}
