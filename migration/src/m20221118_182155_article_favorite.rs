use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let sql = r#"
			CREATE TABLE `article_favorite` (
			  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
			  `article_id` bigint unsigned NOT NULL,
			  `user_id` bigint unsigned NOT NULL,
			  PRIMARY KEY (`id`),
			  KEY `article_favorite_article_id` (`article_id`),
			  KEY `article_favorite_user_id` (`user_id`),
			  CONSTRAINT `article_favorite_article_id` FOREIGN KEY (`article_id`) REFERENCES `article` (`id`),
			  CONSTRAINT `article_favorite_user_id` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`)
			) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
        "#;
		let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
		manager.get_connection().execute(stmt).await.map(|_| ())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let sql = "DROP TABLE `article_favorite`";
		let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
		manager.get_connection().execute(stmt).await.map(|_| ())
	}
}
