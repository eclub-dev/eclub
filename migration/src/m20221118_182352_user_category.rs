use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let sql = r#"
			CREATE TABLE `user_category` (
			  `user_id` bigint unsigned NOT NULL COMMENT 'user id',
			  `category_id` bigint unsigned NOT NULL COMMENT 'category id',
			  PRIMARY KEY (`user_id`,`category_id`),
			  KEY `user_category_category_id` (`category_id`),
			  CONSTRAINT `user_category_category_id` FOREIGN KEY (`category_id`) REFERENCES `category` (`id`),
			  CONSTRAINT `user_category_user_id` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`)
			) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
        "#;
		let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
		manager.get_connection().execute(stmt).await.map(|_| ())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let sql = "DROP TABLE `user_category`";
		let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
		manager.get_connection().execute(stmt).await.map(|_| ())
	}
}
