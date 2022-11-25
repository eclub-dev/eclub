use crate::domain::helper::profile::ProfileBO;
use crate::domain::models::article::{ActiveModel, Model};
use sea_orm::prelude::TimeDateTime;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use std::convert::{From, Into};
use validator::Validate;

#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct ArticleVO<T: Validate> {
	#[validate]
	pub article: T,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct CreateArticleVO {
	pub title: String,
	pub head_img: String,
	pub content: String,
	pub content_type: u8,
	pub tag_list: Vec<String>,
	pub category_list: Vec<String>,
	// TODO: custom theme/css/js
}

impl From<CreateArticleVO> for ActiveModel {
	fn from(creat_article: CreateArticleVO) -> Self {

		Self {
			ulid: Set(ulid::Ulid::new().to_string()),
			slug: Set(slug::slugify(creat_article.title.to_owned())),
			title: Set(creat_article.title.to_owned()),
			head_img: Set(creat_article.head_img.to_owned()),
			content: Set(creat_article.content.to_owned()),
			content_type: Set(creat_article.content_type.to_owned()),
			..Default::default()
		}
	}
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct UpdateArticleVO {
	pub title: String,
	pub head_img: String,
	pub content: String,
	pub content_type: u8,
	pub tag_list: Vec<String>,
	pub category_list: Vec<String>,
	// TODO: custom theme/css/js
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ArticleBO {
	pub ulid: String,
	pub title: String,
	pub head_img: String,
	pub content: String,
	pub content_type: u8,
	pub tag_list: Vec<String>,
	pub category_list: Vec<String>,
	pub views: u64,
	pub likes: u64,
	pub is_pin: u8,
	pub is_official: u8,
	pub create_time: TimeDateTime,
	pub update_time: TimeDateTime,
	pub author: ProfileBO,
}

impl ArticleBO {
	pub fn from_model(model: Model, author: ProfileBO, tag_list: Vec<String>, category_list: Vec<String>) -> ArticleBO {
		Self {
			ulid: model.ulid.to_owned(),
			title: model.title.to_owned(),
			head_img: model.head_img.to_owned(),
			content: model.content.to_owned(),
			content_type: model.content_type.to_owned(),
			tag_list,
			category_list,
			views: model.views.to_owned(),
			likes: model.likes.to_owned(),
			is_pin: model.is_pin.to_owned(),
			is_official: model.is_official.to_owned(),
			create_time: model.create_time.to_owned(),
			update_time: model.update_time.to_owned(),
			author,
		}
	}
}
