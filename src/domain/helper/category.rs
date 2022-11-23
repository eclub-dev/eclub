use sea_orm::{DeriveColumn, EnumIter, IdenStatic};
use serde::{Serialize};

#[derive(Serialize)]
pub struct CategoryVO {
	pub category_list: Vec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum QueryAs {
	Category,
}


// #[derive(Debug, FromQueryResult)]
// pub struct CategoryBO {
// 	name: String,
// }
