//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "channel")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: u64,
	pub key: Option<String>,
	pub value: Option<String>,
	pub is_valid: u8,
	pub weight: u8,
	pub create_time: TimeDateTime,
	pub update_time: TimeDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
