use crate::AppState;
use axum::Json;
use sea_orm::*;

use crate::domain::helper::tag::{QueryAs, TagVO};
use crate::domain::models::tag::{self, ActiveModel, Entity, Model};
use crate::error::Result;

/// tag ServiceBuilder
pub struct TagService;

impl TagService {
	/// > Find a tag by name
	///
	/// Arguments:
	///
	/// * `db`: &DbConn - this is the database connection that is passed in from the route.
	/// * `tag_name`: The name of the tag to find.
	///
	/// Returns:
	///
	/// A Result<Option<Model>>
	pub async fn find_tag_by_name(db: &DbConn, tag_name: &str) -> Result<Option<Model>> {
		Ok(Entity::find().filter(tag::Column::Name.eq(tag_name.to_owned())).one(db).await?)
	}

	/// > Inserts a list of tags into the database, if they don't already exist
	///
	/// Arguments:
	///
	/// * `db`: &DbConn - This is the database connection that we're using.
	/// * `tag_list`: &Vec<String>
	///
	/// Returns:
	///
	/// The return type is a `Result<InsertResult<ActiveModel>>`.
	pub async fn insert_many(db: &DbConn, tag_list: &Vec<String>) -> Result<InsertResult<ActiveModel>> {
		Ok(Entity::insert_many(
			tag_list
				.iter()
				.map(|tag_name| ActiveModel {
					name: Set(tag_name.to_owned()),
					..Default::default()
				})
				.collect::<Vec<ActiveModel>>(),
		)
		.on_conflict(sea_query::OnConflict::column(tag::Column::Name).update_column(tag::Column::Name).to_owned())
		.exec(db)
		.await?)
	}

	/// > Get all the tags from the database and return them as a JSON object
	///
	/// Arguments:
	///
	/// * `app_state`: &AppState - This is the application state that we created in the main.rs file.
	///
	/// Returns:
	///
	/// A vector of strings.
	pub async fn get_tag_list(app_state: &AppState) -> Result<Json<TagVO>> {
		let tag_list: Vec<String> = Entity::find()
			.select_only()
			.column_as(tag::Column::Name, QueryAs::Tag)
			.into_values::<_, QueryAs>()
			.all(&app_state.conn)
			.await?;

		Ok(Json(TagVO {
			tag_list,
		}))
	}

}
