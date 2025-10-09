pub use sea_orm_migration::prelude::*;

mod m20250918_163000_create_bots_table;
mod m20250918_170126_create_wallet_table;
mod m20251008_194445_create_orderbook_table;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250918_163000_create_bots_table::Migration),
            Box::new(m20250918_170126_create_wallet_table::Migration),
            Box::new(m20251008_194445_create_orderbook_table::Migration),
        ]
    }
}
