use sea_orm::*;
use crate::error::{Error, Result};
use crate::domain::models::article_favorite::{self, ActiveModel, Entity, Model};

pub struct ArticleFavoriteService;

impl ArticleFavoriteService {
	pub async fn insert(db: &DbConn, article_id: &u64, user_id: &u64) -> Result<Model> {
		Ok(
			ActiveModel {
				article_id: Set(article_id.to_owned()),
				user_id: Set(user_id.to_owned()),
				..Default::default()
			}
				.insert(db)
				.await?
		)
	}

	pub async fn delete(db: &DbConn, article_id: &u64, user_id: &u64) -> Result<DeleteResult> {
		Ok(
			Entity::delete_many()
				.filter(
					Condition::all()
						.add(article_favorite::Column::UserId.eq(user_id.to_owned()))
						.add(article_favorite::Column::ArticleId.eq(article_id.to_owned())),
				)
				.exec(db)
				.await?
		)
	}
}
