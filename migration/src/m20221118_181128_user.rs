use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let sql = r#"
			CREATE TABLE `user` (
			  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT 'id',
			  `username` varchar(255) NOT NULL COMMENT 'username',
			  `password` varchar(255) NOT NULL COMMENT 'hash password',
			  `email` varchar(255) NOT NULL COMMENT 'email',
			  `role` tinyint unsigned NOT NULL DEFAULT '0' COMMENT 'role',
			  `bio` varchar(1024) NOT NULL DEFAULT '' COMMENT 'bio',
			  `avatar` varchar(256) NOT NULL DEFAULT '' COMMENT 'avatar url',
			  `is_valid` tinyint unsigned NOT NULL DEFAULT '0' COMMENT 'valid flag(0: valid, 1: invalid)',
			  `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'create time',
			  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'update time',
			  PRIMARY KEY (`id`),
			  UNIQUE KEY `uk_username` (`username`),
			  UNIQUE KEY `uk_email` (`email`)
			) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
        "#;
		let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
		manager.get_connection().execute(stmt).await.map(|_| ())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let sql = "DROP TABLE `user`";
		let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
		manager.get_connection().execute(stmt).await.map(|_| ())
	}
}
