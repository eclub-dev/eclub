//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "casbin_rule")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: i32,
	pub ptype: String,
	pub v0: String,
	pub v1: String,
	pub v2: String,
	pub v3: String,
	pub v4: String,
	pub v5: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
