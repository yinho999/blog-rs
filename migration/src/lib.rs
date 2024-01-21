#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20231103_114510_notes;

mod m20231231_171853_tags;
mod m20231231_172031_alter_tags_description;
mod m20231231_173417_posts;
mod m20231231_173710_posts_tags;
mod m20231231_231132_add_posts_user_reference;
mod m20240112_065423_series;
mod m20240112_065849_alter_series_title_description;
mod m20240112_100002_posts_series;
mod m20240121_193644_add_post_series_user_reference;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20231103_114510_notes::Migration),
            Box::new(m20231231_171853_tags::Migration),
            Box::new(m20231231_172031_alter_tags_description::Migration),
            Box::new(m20231231_173417_posts::Migration),
            Box::new(m20231231_173710_posts_tags::Migration),
            Box::new(m20231231_231132_add_posts_user_reference::Migration),
            Box::new(m20240112_065423_series::Migration),
            Box::new(m20240112_065849_alter_series_title_description::Migration),
            Box::new(m20240112_100002_posts_series::Migration),
            Box::new(m20240121_193644_add_post_series_user_reference::Migration),
        ]
    }
}