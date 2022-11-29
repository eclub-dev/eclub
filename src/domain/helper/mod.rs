use crate::domain::helper::article::ListArticleBO;

pub mod user;
pub mod email;
pub mod tag;
pub mod category;
pub mod profile;
pub mod article;
pub mod commit;


#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListVO<T = ListArticleBO> {
	pub rows: Vec<T>,
	/// page size
	pub limit: u64,
	/// page
	pub offset: u64,
	/// page
	pub size: u64,
}
