use axum::extract::State;
use axum::Json;
use sea_orm::*;

use crate::domain::helper::profile::{ProfileBO, ProfileVO};
use crate::domain::models::user::{self, Entity as UserEntity, Model as UserModel};
use crate::error::{Error, Result};
use crate::service::FollowService;
use crate::AppState;

/// Profile ServiceBuilder
pub struct ProfileService;

impl ProfileService {
	pub async fn get_user_profile(
		app_state: &AppState,
		username: &str,
		user_id: &u64,
	) -> Result<Json<ProfileVO<ProfileBO>>> {
		let profile: ProfileBO = <ProfileBO>::find_by_statement(Statement::from_sql_and_values(
			DbBackend::MySql,
			r#"
						select
							username,
							email,
							bio,
							avatar,
							exists(
								select 1 from follow
								where follow_id = $2
							) "following"
						from user
						where username = $1
					"#,
			vec![username.into(), user_id.to_string().into()],
		))
		.one(&app_state.conn)
		.await?
		.ok_or(Error::NotFound)?;

		Ok(Json(ProfileVO {
			profile,
		}))
	}

	pub async fn follow_user(
		app_state: &AppState,
		username: &str,
		user_id: &u64,
	) -> Result<Json<ProfileVO<ProfileBO>>> {
		let user: UserModel = UserEntity::find()
			.filter(user::Column::Username.eq(username.to_owned()))
			.one(&app_state.conn)
			.await?
			.ok_or(Error::NotFound)?;

		FollowService::insert(&app_state.conn, &user.id, user_id).await?;
		Ok(Json(ProfileVO {
			profile: ProfileBO::from_user(user, true),
		}))
	}

	pub async fn unfollow_user(
		State(app_state): State<AppState>,
		username: &str,
		user_id: &u64,
	) -> Result<Json<ProfileVO<ProfileBO>>> {
		let user: UserModel = UserEntity::find()
			.filter(user::Column::Username.eq(username.to_owned()))
			.one(&app_state.conn)
			.await?
			.ok_or(Error::NotFound)?;

		FollowService::delete(&app_state.conn, &user.id, user_id).await?;
		Ok(Json(ProfileVO {
			profile: ProfileBO::from_user(user, false),
		}))
	}
}
