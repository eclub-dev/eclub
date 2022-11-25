use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let sql = r#"
			CREATE TABLE `email` (
			  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT 'id',
			  `user_id` bigint unsigned NOT NULL COMMENT 'user id',
			  `email` varchar(255) NOT NULL COMMENT 'email',
			  `token` varchar(255) NOT NULL COMMENT 'token',
			  `is_verified` tinyint unsigned NOT NULL DEFAULT '0' COMMENT 'verified flag(0: verified, 1: unverified)',
			  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'create time',
			  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'update time',
			  PRIMARY KEY (`id`),
			  UNIQUE KEY `uk_email_token` (`email`,`token`),
			  KEY `email_user_id` (`user_id`),
			  CONSTRAINT `email_user_id` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`) ON DELETE CASCADE ON UPDATE RESTRICT
			) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
        "#;
		let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
		manager.get_connection().execute(stmt).await.map(|_| ())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let sql = "DROP TABLE `email`";
		let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
		manager.get_connection().execute(stmt).await.map(|_| ())
	}
}
