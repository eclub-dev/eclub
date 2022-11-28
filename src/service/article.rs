use crate::domain::helper::article::{ArticleBO, ArticleVO, CreateArticleVO, ListArticleBO, ListArticlesQueryVO};
use crate::domain::helper::profile::ProfileBO;
use crate::domain::helper::user::UserBO;
use crate::domain::helper::ListVO;
use crate::domain::models::article::{self, ActiveModel, Entity, Model};
use crate::domain::models::user;
use crate::error::{Error, Result};
use crate::service::article_favorite::ArticleFavoriteService;
use crate::service::{ArticleCategoryService, ArticleTagService, CategoryService, TagService, UserCategoryService};
use crate::AppState;
use axum::extract::Query;
use axum::Json;
use sea_orm::ActiveValue::Set;
use sea_orm::*;
use validator::HasLen;

pub struct ArticleService;

impl ArticleService {
	pub async fn upset_article(
		app_state: &AppState,
		create_article: &CreateArticleVO,
		user_id: &u64,
	) -> Result<Json<ArticleVO<ArticleBO>>> {
		// upset_article
		let create_article_active: ActiveModel =
			<CreateArticleVO as Into<ActiveModel>>::into(create_article.to_owned());

		let saved_model: Model = create_article_active.clone().try_into_model()?;

		let insert_res: InsertResult<ActiveModel> = Entity::insert(create_article_active)
			.on_conflict(
				sea_query::OnConflict::columns([article::Column::Slug, article::Column::UserId])
					.update_columns([article::Column::HeadImg, article::Column::Content, article::Column::ContentType])
					.to_owned(),
			)
			.exec(&app_state.conn)
			.await?;

		ArticleService::delete_article_related(&app_state.conn, &saved_model.ulid, &user_id).await?;

		let tag_list = &create_article.tag_list;
		let category_list = &create_article.category_list;
		TagService::insert_many_by_article_id(&app_state.conn, &insert_res.last_insert_id, &tag_list).await?;
		CategoryService::insert_many_by_id(&app_state.conn, &user_id, &insert_res.last_insert_id, &category_list)
			.await?;

		let user_res: user::Model =
			user::Entity::find_by_id(user_id.to_owned()).one(&app_state.conn).await?.ok_or(Error::NotFound)?;

		Ok(Json(ArticleVO {
			article: ArticleBO::from_model(
				saved_model,
				ProfileBO::from_user(user_res, false),
				tag_list.to_owned(),
				category_list.to_owned(),
			),
		}))
	}

	pub async fn get_article(app_state: &AppState, ulid: &str) -> Result<Json<ArticleVO<ArticleBO>>> {
		let article: Model =
			ArticleService::find_article_by_ulid(&app_state.conn, &ulid).await?.ok_or(Error::NotFound)?;
		Ok(ArticleService::full_article(&app_state.conn, article)?)
	}

	pub async fn delete_article(app_state: &AppState, ulid: &str, user_id: &u64) -> Result<()> {
		ArticleService::delete_article_related(&app_state.conn, &ulid, &user_id).await?;
		let deleted = ArticleService::delete_article_by_ulid(&app_state.conn, &ulid, &user_id).await?;
		if deleted.rows_affected > 0 {
			return Ok(());
		}
		Err(Error::NotFound)
	}

	pub async fn favorite_article(
		app_state: &AppState,
		ulid: &str,
		user_id: &u64,
	) -> Result<Json<ArticleVO<ArticleBO>>> {
		let article = ArticleService::find_article_by_ulid(&app_state.conn, &ulid).await?.ok_or(Error::NotFound)?;
		ArticleFavoriteService::insert(&app_state.conn, &article.user_id, &user_id).await?;
		Ok(ArticleService::full_article(&app_state.conn, article)?)
	}

	pub async fn unfavorite_article(
		app_state: &AppState,
		ulid: &str,
		user_id: &u64,
	) -> Result<Json<ArticleVO<ArticleBO>>> {
		let article = ArticleService::find_article_by_ulid(&app_state.conn, &ulid).await?.ok_or(Error::NotFound)?;
		ArticleFavoriteService::delete(&app_state.conn, &article.user_id, &user_id).await?;
		Ok(ArticleService::full_article(&app_state.conn, article)?)
	}

	pub async fn view_article(app_state: &AppState, ulid: &str) -> Result<()> {
		if let Some(model) = ArticleService::find_article_by_ulid(&app_state.conn, &ulid).await? {
			let new_views = &model.views + 1;
			let mut article_active: article::ActiveModel = model.into();
			article_active.views = Set(new_views);
			article_active.update(&app_state.conn).await?;
			return Ok(());
		}
		Err(Error::NotFound)
	}

	pub async fn list_articles(app_state: &AppState, query: &Query<ListArticlesQueryVO>) -> Result<Json<ListVO>> {
		let list_article: Vec<ListArticleBO> = <ListArticleBO>::find_by_statement(Statement::from_sql_and_values(
			DbBackend::MySql,
			r#"
					select
						ulid,
						title,
						head_img,
						content,
						content_type,
						views,
						likes,
						is_pin,
						is_official,
						create_time,
						update_time
					from article
					where is_audit=2
					and ($1 < 1 or user_id = $1)
					and (LENGTH(trim($2)) < 1 or title LIKE '%$2%')
					and id >= $3
					limit $4;
					"#,
			vec![
				query.user_id.clone().into(),
				query.title.clone().into(),
				(query.offset.clone() * (query.limit.clone() - 1)).into(),
				query.limit.clone().into(),
			],
		))
		.all(&app_state.conn)
		.await?;

		Ok(Json(ListVO {
			size: list_article.length().clone(),
			rows: list_article,
			limit: query.limit.clone(),
			offset: query.offset.clone(),
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

	pub async fn find_article_by_ulid(db: &DbConn, ulid: &str) -> Result<Option<Model>> {
		Ok(Entity::find().filter(article::Column::Ulid.eq(ulid.to_owned())).one(db).await?)
	}

	pub async fn find_article_by_id(db: &DbConn, ulid: &str, user_id: &u64) -> Result<Option<Model>> {
		Ok(Entity::find()
			.filter(
				Condition::all()
					.add(article::Column::UserId.eq(user_id.to_owned()))
					.add(article::Column::Ulid.eq(ulid.to_owned())),
			)
			.one(db)
			.await?)
	}

	pub async fn delete_article_related(db: &DbConn, ulid: &str, user_id: &u64) -> Result<()> {
		if let Some(Model {
			id,
			..
		}) = ArticleService::find_article_by_id(&db, &ulid, &user_id).await?
		{
			ArticleTagService::delete_by_article_id(&db, &id).await?;
			ArticleCategoryService::delete_by_article_id(&db, &id).await?;
		}
		Err(Error::NotFound)
	}

	pub async fn delete_article_by_ulid(db: &DbConn, ulid: &str, user_id: &u64) -> Result<DeleteResult> {
		Ok(Entity::delete_many()
			.filter(
				Condition::all()
					.add(article::Column::UserId.eq(user_id.to_owned()))
					.add(article::Column::Ulid.eq(ulid.to_owned())),
			)
			.exec(db)
			.await?)
	}

	pub async fn full_article(db: &DbConn, article: Model) -> Result<Json<ArticleVO<ArticleBO>>> {
		let category_list = ArticleCategoryService::find_by_article_id(&db, &article.id).await?;
		let tag_list = ArticleTagService::find_by_article_id(&db, &article.id).await?;
		let user_res: user::Model =
			user::Entity::find_by_id(&article.user_id.to_owned()).one(&db).await?.ok_or(Error::NotFound)?;

		Ok(Json(ArticleVO {
			article: ArticleBO::from_model(
				article,
				ProfileBO::from_user(user_res, false),
				tag_list.to_owned(),
				category_list.to_owned(),
			),
		}))
	}
}
