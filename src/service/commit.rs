use crate::domain::helper::commit::{CommitBO, CommitVO, CreateCommitVO, ListCommitBO, ListCommitVO};
use crate::domain::models::comment::{self, Entity, Model};
use crate::error::{Error, Result};
use crate::service::{ArticleService};
use crate::AppState;
use axum::Json;
use sea_orm::*;

pub struct CommitService;

impl CommitService {
	pub async fn add_comment(
		app_state: &AppState,
		user_id: &u64,
		ulid: &str,
		create_commit: CreateCommitVO,
	) -> Result<Json<CommitVO<CommitBO>>> {
		let article = ArticleService::find_article_by_ulid(&app_state.conn, &ulid).await?.ok_or(Error::NotFound)?;

		let commit: Model =
			create_commit.into_active(user_id, &article.id).save(&app_state.conn).await?.try_into_model()?;


		tracing::debug!("create_or_update: {:?}", &commit);


		// 返回commit总体的结构体
		Ok(Json(CommitVO {
			commit: CommitBO::from_model(commit)
		}))
	}

	pub async fn delete_comment(app_state: &AppState, user_id: &u64, ulid: &str, commit_id: &u64) -> Result<()> {
		let article = ArticleService::find_article_by_ulid(&app_state.conn, &ulid).await?.ok_or(Error::NotFound)?;

		let res: DeleteResult = Entity::delete_many()
			.filter(
				Condition::all()
					.add(comment::Column::UserId.eq(user_id.to_owned()))
					.add(comment::Column::ArticleId.eq(article.id.to_owned()))
					.add(comment::Column::Id.eq(commit_id.to_owned())),
			)
			.exec(&app_state.conn)
			.await?;
		if res.rows_affected < 1 {
			return Err(Error::NotFound)
		}
		Ok(())
	}

	pub async fn list_comment(app_state: &AppState, ulid: &str) -> Result<Json<ListCommitVO>> {
		let article = ArticleService::find_article_by_ulid(&app_state.conn, &ulid).await?.ok_or(Error::NotFound)?;

		let list_commit: Vec<ListCommitBO> = <ListCommitBO>::find_by_statement(Statement::from_sql_and_values(
			DbBackend::MySql,
			r#"
					select
						comment.parent_id as parent_id,
						comment.content as content,
						comment.content_type as content_type,
						user.id as user_id,
						user.username as username,
						user.avatar as avatar,
						comment.create_time as create_time,
						count(comment_favorite.id) as likes
					from comment
					inner join user
					on comment.user_id=user.id
					where comment.article_id = ?
					right join comment_favorite
					on user.id=comment_favorite.user_id
					GROUP BY parent_id,content,content_type,user_id,username,avatar,create_time
					"#,
			vec![article.id.into()],
		))
		.all(&app_state.conn)
		.await?;

		Ok(Json(ListCommitVO {
			comments: list_commit,
		}))
	}
}
