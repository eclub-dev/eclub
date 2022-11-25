use crate::domain::helper::category::QueryAs;
use crate::domain::models::{article_category, article_tag, category};
use crate::error::Result;
use sea_orm::sea_query::{Expr, Query};
use sea_orm::*;

pub struct ArticleCategoryService;

impl ArticleCategoryService {
	pub async fn delete_by_article_id(db: &DbConn, article_id: &u64) -> Result<DeleteResult> {
		// ON DELETE CASCADE()
		Ok(article_category::Entity::delete_many()
			.filter(article_category::Column::ArticleId.eq(article_id.to_owned()))
			.exec(db)
			.await?)
	}

	pub async fn find_by_article_id(db: &DbConn, article_id: &u64) -> Result<Vec<String>> {
		Ok(category::Entity::find()
			.select_only()
			.column_as(category::Column::Name, QueryAs::Category)
			.filter(
				Condition::any().add(
					category::Column::Id.in_subquery(
						Query::select()
							.expr(Expr::col(article_category::Column::CategoryId))
							.from(article_category::Entity)
							.and_where(Expr::col(article_category::Column::ArticleId).eq(article_id.to_owned()))
							.to_owned(),
					),
				),
			)
			.into_values::<_, QueryAs>()
			.all(db)
			.await?)
	}
}
