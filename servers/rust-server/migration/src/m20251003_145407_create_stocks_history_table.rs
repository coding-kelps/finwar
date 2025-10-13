use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StocksHistory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StocksHistory::Time)
                            .timestamp_with_time_zone()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(StocksHistory::Symbol)
                            .text()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(StocksHistory::Open)
                            .double()
                    )
                    .col(
                        ColumnDef::new(StocksHistory::High)
                            .double()
                    )
                    .col(
                        ColumnDef::new(StocksHistory::Low)
                            .double()
                    )
                    .col(
                        ColumnDef::new(StocksHistory::Close)
                            .double()
                    )
                    .col(
                        ColumnDef::new(StocksHistory::Volume)
                            .big_integer()
                    )
                    .primary_key(
                        Index::create()
                            .col(StocksHistory::Time)
                            .col(StocksHistory::Symbol)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StocksHistory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StocksHistory {
    Table,
    Time,
    Symbol,
    Open,
    High,
    Low,
    Close,
    Volume,
}
