use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Orderbook::Table)
                    .if_not_exists()
                    .col(pk_auto(Orderbook::Id))
                    .col(integer(Orderbook::BotId).not_null())
                    .col(string(Orderbook::Symbol).not_null())
                    .col(string(Orderbook::OrderType).not_null())
                    .col(integer(Orderbook::Quantity).not_null())
                    .col(double(Orderbook::Price).not_null())
                    .col(string(Orderbook::Status).not_null().default("pending"))
                    .col(timestamp_with_time_zone(Orderbook::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_orderbook_bot")
                            .from(Orderbook::Table, Orderbook::BotId)
                            .to(Bot::Table, Bot::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Orderbook::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Orderbook {
    Table,
    Id,
    BotId,
    Symbol,
    OrderType,
    Quantity,
    Price,
    Status,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Bot {
    Table,
    Id,
}
