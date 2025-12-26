pub use sea_orm_migration::prelude::*;

mod init_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(init_tables::Migration)]
    }
}
