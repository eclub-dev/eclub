use crate::AppState;
use axum::Json;
use sea_orm::*;

use crate::domain::helper::category::{CategoryVO, QueryAs};
use crate::domain::models::category::{self, ActiveModel, Entity, Model};
use crate::error::Result;

/// Category ServiceBuilder
pub struct CategoryService;

impl CategoryService {
	/// > Find a tag by name
	///
	/// Arguments:
	///
	/// * `db`: &DbConn - this is the database connection that is passed in from the route.
	/// * `category_name`: The name of the category to find.
	///
	/// Returns:
	///
	/// A Result<Option<Model>>
	pub async fn find_tag_by_name(db: &DbConn, category_name: &str) -> Result<Option<Model>> {
		Ok(Entity::find().filter(category::Column::Name.eq(category_name.to_owned())).one(db).await?)
	}

	/// > Inserts a list of categories into the database, if they don't already exist
	///
	/// Arguments:
	///
	/// * `db`: &DbConn - This is the database connection that we created in the previous step.
	/// * `category_list`: A vector of strings that represent the category names to be inserted.
	///
	/// Returns:
	///
	/// The return type is a Result<InsertResult<ActiveModel>>.
	pub async fn insert_many(db: &DbConn, category_list: &Vec<String>) -> Result<InsertResult<ActiveModel>> {
		Ok(Entity::insert_many(
			category_list
				.iter()
				.map(|category_name| ActiveModel {
					name: Set(category_name.to_owned()),
					..Default::default()
				})
				.collect::<Vec<ActiveModel>>(),
		)
		.on_conflict(
			sea_query::OnConflict::column(category::Column::Name).update_column(category::Column::Name).to_owned(),
		)
		.exec(db)
		.await?)
	}

	/// > Get a list of all categories from the database
	///
	/// Arguments:
	///
	/// * `app_state`: &AppState - This is the application state that we created earlier.
	///
	/// Returns:
	///
	/// A vector of strings.
	pub async fn get_category_list(app_state: &AppState) -> Result<Json<CategoryVO>> {
		let category_list: Vec<String> = Entity::find()
			.select_only()
			.column_as(category::Column::Name, QueryAs::Category)
			.into_values::<_, QueryAs>()
			.all(&app_state.conn)
			.await?;

		Ok(Json(CategoryVO {
			category_list,
		}))
	}

	/// > This function returns a list of categories that a user has subscribed to
	///
	/// Arguments:
	///
	/// * `app_state`: &AppState - This is the application state that we created earlier.
	/// * `username`: &str - username form User Model
	///
	/// Returns:
	///
	/// A vector of strings.
	pub async fn get_user_category(app_state: &AppState, username: &str) -> Result<Json<CategoryVO>> {
		let category_list: Vec<String> = <String>::find_by_statement::<QueryAs>(Statement::from_sql_and_values(
			DbBackend::MySql,
			r#"
					select
						category.name as name
					from category
					inner join user_category
					on category.id=user_category.category_id
					inner join user
					on user_category.user_id=user.id
					where user.username=?
				"#,
			vec![username.into()],
		))
		.all(&app_state.conn)
		.await?;

		Ok(Json(CategoryVO {
			category_list,
		}))
	}
}
