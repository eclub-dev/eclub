use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let sql = r#"
			CREATE TABLE `article` (
			  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT 'id',
			  `ulid` char(26) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'ulid',
			  `user_id` bigint unsigned NOT NULL COMMENT 'user id',
			  `slug` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'artcile slug',
			  `title` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'artcile title',
			  `head_img` varchar(8192) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT 'Article header image',
			  `content` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'content',
			  `content_type` tinyint unsigned NOT NULL DEFAULT '0' COMMENT 'Article content (0: markdown, 1: html)',
			  `views` bigint unsigned NOT NULL DEFAULT '0' COMMENT 'Views',
			  `likes` bigint unsigned NOT NULL DEFAULT '0' COMMENT 'Likes',
			  `audit_content` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '0' COMMENT 'The reason for the audit failure',
			  `is_audit` tinyint unsigned NOT NULL DEFAULT '0' COMMENT 'Audit flag (0: Not audited, 1: Audited but not passed, 2: Audited and passed)',
			  `is_pin` tinyint unsigned NOT NULL DEFAULT '0' COMMENT 'Pin mark (0: not pinned, 1: pinned)',
			  `is_official` tinyint unsigned NOT NULL DEFAULT '0' COMMENT 'Official mark (0: not official, 1: official)',
			  `is_delete` tinyint unsigned NOT NULL DEFAULT '0' COMMENT 'Deletion flag (0: not deleted, 1: deleted)',
			  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'create time',
			  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'update time',
			  PRIMARY KEY (`id`),
			  UNIQUE KEY `uk_user_slug` (`user_id`,`slug`),
			  KEY `idx_user_id` (`user_id`),
			  KEY `idx_ulid` (`ulid`) USING BTREE,
			  CONSTRAINT `article_user_id` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`) ON DELETE CASCADE
			) ENGINE=InnoDB AUTO_INCREMENT=33 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
        "#;
		let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
		manager.get_connection().execute(stmt).await.map(|_| ())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let sql = "DROP TABLE `article`";
		let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
		manager.get_connection().execute(stmt).await.map(|_| ())
	}
}
