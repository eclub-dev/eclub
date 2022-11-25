use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let sql = r#"
			CREATE TABLE `message` (
			  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT 'id',
			  `sender` bigint unsigned NOT NULL COMMENT 'sender',
			  `receiver` bigint unsigned NOT NULL COMMENT 'receiver',
			  `content_type` tinyint unsigned NOT NULL DEFAULT '0' COMMENT 'content type（0:markdown、1:html、2:image、3:video）',
			  `content` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT 'content',
			  `is_read` tinyint unsigned NOT NULL DEFAULT '0' COMMENT 'read flag (0: unread, 1: read)',
			  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'create time',
			  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'update time',
			  PRIMARY KEY (`id`),
			  KEY `idx_sender` (`sender`),
			  KEY `message_receiver` (`receiver`),
			  CONSTRAINT `message_receiver` FOREIGN KEY (`receiver`) REFERENCES `user` (`id`) ON DELETE CASCADE ON UPDATE RESTRICT,
			  CONSTRAINT `message_sender` FOREIGN KEY (`sender`) REFERENCES `user` (`id`) ON DELETE CASCADE ON UPDATE RESTRICT
			) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='message';
        "#;
		let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
		manager.get_connection().execute(stmt).await.map(|_| ())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let sql = "DROP TABLE `message`";
		let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
		manager.get_connection().execute(stmt).await.map(|_| ())
	}
}
