use crate::domain::models::comment::{ActiveModel, Model};
use sea_orm::prelude::TimeDateTime;
use sea_orm::ActiveValue::Set;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct CommitVO<T: Validate> {
	#[validate]
	pub commit: T,
}

#[derive(Serialize)]
pub struct ListCommitVO {
	pub comments: Vec<ListCommitBO>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct CreateCommitVO {
	pub parent_id: Option<u64>,
	pub content: String,
	pub content_type: u8,
}

impl CreateCommitVO {
	pub fn into_active(self, user_id: &u64, article_id: &u64) -> ActiveModel {
		ActiveModel {
			parent_id: Set(self.parent_id.to_owned()),
			content: Set(self.content.to_owned()),
			content_type: Set(self.content_type.to_owned()),
			user_id: Set(user_id.to_owned()),
			article_id: Set(article_id.to_owned()),
			..Default::default()
		}
	}
}

impl From<CreateCommitVO> for ActiveModel {
	fn from(creat_commit: CreateCommitVO) -> Self {
		Self {
			parent_id: Set(creat_commit.parent_id.to_owned()),
			content: Set(creat_commit.content.to_owned()),
			content_type: Set(creat_commit.content_type.to_owned()),
			..Default::default()
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CommitBO {
	pub id: u64,
	pub parent_id: Option<u64>,
	pub content: String,
	pub content_type: u8,
}

impl CommitBO {
	pub fn from_model(model: Model) -> CommitBO {
		Self {
			id: model.id.to_owned(),
			parent_id: model.parent_id.to_owned(),
			content: model.content.to_owned(),
			content_type: model.content_type.to_owned(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
pub struct ListCommitBO {
	pub parent_id: u64,
	pub content: String,
	pub content_type: u8,
	pub likes: u64,
	pub user_id: u64,
	pub username: u64,
	pub avatar: String,
	pub create_time: TimeDateTime,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct ListCommitsQueryVO {
	/// page size
	#[serde(default = "default_limit")]
	pub limit: u64,
	/// page
	#[serde(default = "default_offset")]
	pub offset: u64,
}

fn default_limit() -> u64 {
	20
}

fn default_offset() -> u64 {
	1
}
