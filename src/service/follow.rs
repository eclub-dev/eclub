use sea_orm::*;

use crate::domain::models::follow::{self, ActiveModel, Entity, Model};
use crate::error::{Result};

pub struct FollowService;

impl FollowService {
	pub async fn is_following(db: &DbConn, user_id: &u64, follow_id: &u64) -> bool {
		Entity::find()
			.filter(
				Condition::all()
					.add(follow::Column::UserId.eq(user_id.to_owned()))
					.add(follow::Column::FollowId.eq(follow_id.to_owned())),
			)
			.one(db)
			.await
			.is_ok()
	}

	pub async fn following_authors(db: &DbConn, follow_id: &u64) -> Result<Vec<Model>> {
		Ok(
			Entity::find()
				.select_only()
				.column(follow::Column::UserId)
				.filter(follow::Column::FollowId.eq(follow_id.to_owned()))
				.all(db)
				.await?
		)
	}

	pub async fn followed_users(db: &DbConn, user_id: &u64) -> Result<Vec<Model>> {
		Ok(
			Entity::find()
				.select_only()
				.column(follow::Column::FollowId)
				.filter(follow::Column::UserId.eq(user_id.to_owned()))
				.all(db)
				.await?
		)
	}

	pub async fn insert(db: &DbConn, user_id: &u64, follow_id: &u64) -> Result<Model> {
		let follow = ActiveModel {
			user_id: Set(user_id.to_owned()),
			follow_id: Set(follow_id.to_owned()),
			..Default::default()
		};
		Ok(follow.insert(db).await?)
	}

	pub async fn delete(db: &DbConn, user_id: &u64, follow_id: &u64) -> Result<DeleteResult> {
		Ok(
			Entity::delete_many()
				.filter(
					Condition::all()
						.add(follow::Column::UserId.eq(user_id.to_owned()))
						.add(follow::Column::FollowId.eq(follow_id.to_owned())),
				)
				.exec(db)
				.await?
		)
	}
}
