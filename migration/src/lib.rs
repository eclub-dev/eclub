pub use sea_orm_migration::prelude::*;

mod m20221118_181128_user;
mod m20221118_182146_article;
mod m20221118_182150_tag;
mod m20221118_182155_article_favorite;
mod m20221118_182208_article_tag;
mod m20221118_182218_casbin_rule;
mod m20221118_182231_category;
mod m20221118_182242_channel;
mod m20221118_182255_comment;
mod m20221118_182304_comment_favorite;
mod m20221118_182318_follow;
mod m20221118_182326_message;
mod m20221118_182352_user_category;
mod m20221120_160047_email;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
			Box::new(m20221118_181128_user::Migration),
            Box::new(m20221118_182146_article::Migration),
			Box::new(m20221118_182150_tag::Migration),
            Box::new(m20221118_182155_article_favorite::Migration),
            Box::new(m20221118_182208_article_tag::Migration),
            Box::new(m20221118_182218_casbin_rule::Migration),
            Box::new(m20221118_182231_category::Migration),
            Box::new(m20221118_182242_channel::Migration),
            Box::new(m20221118_182255_comment::Migration),
            Box::new(m20221118_182304_comment_favorite::Migration),
            Box::new(m20221118_182318_follow::Migration),
            Box::new(m20221118_182326_message::Migration),
            Box::new(m20221118_182352_user_category::Migration),
            Box::new(m20221120_160047_email::Migration),
        ]
    }
}
