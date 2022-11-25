use crate::domain::helper::tag::QueryAs;
use crate::domain::models::{article_tag, tag};
use crate::error::Result;
use migration::Expr;
use sea_orm::sea_query::Query;
use sea_orm::*;

pub struct ArticleTagService;

impl ArticleTagService {
	pub async fn delete_by_article_id(db: &DbConn, article_id: &u64) -> Result<DeleteResult> {
		// ON DELETE CASCADE()
		Ok(article_tag::Entity::delete_many()
			.filter(article_tag::Column::ArticleId.eq(article_id.to_owned()))
			.exec(db)
			.await?)
	}

	pub async fn find_by_article_id(db: &DbConn, article_id: &u64) -> Result<Vec<String>> {
		Ok(tag::Entity::find()
			.select_only()
			.column_as(tag::Column::Name, QueryAs::Tag)
			.filter(
				Condition::any().add(
					tag::Column::Id.in_subquery(
						Query::select()
							.expr(Expr::col(article_tag::Column::TagId))
							.from(article_tag::Entity)
							.and_where(Expr::col(article_tag::Column::ArticleId).eq(article_id.to_owned()))
							.to_owned(),
					),
				),
			)
			.into_values::<_, QueryAs>()
			.all(db)
			.await?)
	}
}
