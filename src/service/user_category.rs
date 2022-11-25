use crate::domain::models::user_category;
use crate::error::Result;
use sea_orm::*;

pub struct UserCategoryService;

impl UserCategoryService {
	pub async fn delete_by_article_id(db: &DbConn, category_id: &u64) -> Result<DeleteResult> {
		// ON DELETE CASCADE()
		Ok(user_category::Entity::delete_many()
			.filter(user_category::Column::CategoryId.eq(category_id.to_owned()))
			.exec(db)
			.await?)
	}
}
