use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let sql = r#"
			CREATE TABLE `casbin_rule` (
			  `id` int NOT NULL AUTO_INCREMENT,
			  `ptype` varchar(12) NOT NULL,
			  `v0` varchar(128) NOT NULL,
			  `v1` varchar(128) NOT NULL,
			  `v2` varchar(128) NOT NULL,
			  `v3` varchar(128) NOT NULL,
			  `v4` varchar(128) NOT NULL,
			  `v5` varchar(128) NOT NULL,
			  PRIMARY KEY (`id`),
			  UNIQUE KEY `uk_casbin_adapter` (`ptype`,`v0`,`v1`,`v2`,`v3`,`v4`,`v5`)
			) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3;
        "#;
		let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
		manager.get_connection().execute(stmt).await.map(|_| ())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let sql = "DROP TABLE `casbin_rule`";
		let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
		manager.get_connection().execute(stmt).await.map(|_| ())
	}
}
