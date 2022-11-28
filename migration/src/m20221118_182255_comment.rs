use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let sql = r#"
			CREATE TABLE `comment` (
			  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT 'id',
			  `content` text NOT NULL COMMENT 'content',
			  `content_type` tinyint unsigned NOT NULL DEFAULT '0' COMMENT 'Article content (0: markdown, 1: html)',
			  `article_id` bigint unsigned NOT NULL COMMENT 'article id',
			  `user_id` bigint unsigned NOT NULL COMMENT 'user id',
			  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'create time',
			  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'update time',
			  PRIMARY KEY (`id`),
			  KEY `comment_article_id` (`article_id`),
			  KEY `comment_user_id` (`user_id`),
			  CONSTRAINT `comment_article_id` FOREIGN KEY (`article_id`) REFERENCES `article` (`id`) ON DELETE CASCADE ON UPDATE RESTRICT,
			  CONSTRAINT `comment_user_id` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`) ON DELETE CASCADE ON UPDATE RESTRICT
			) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
        "#;
		let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
		manager.get_connection().execute(stmt).await.map(|_| ())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let sql = "DROP TABLE `comment`";
		let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
		manager.get_connection().execute(stmt).await.map(|_| ())
	}
}
