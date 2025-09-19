use crate::m20250918_163000_create_bots_table::Bot;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Wallet::Table)
                    .if_not_exists()
                    .col(pk_auto(Wallet::Id))
                    .col(integer(Wallet::BotId))
                    .col(integer(Wallet::Cash).not_null())
                    .col(decimal_len(Wallet::Asset, 5, 2))
                    .col(timestamp_with_time_zone(Wallet::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(Wallet::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-wallet-bot_id")
                            .from(Wallet::Table, Wallet::BotId)
                            .to(Bot::Table, Bot::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Wallet::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Wallet {
    Table,
    Id,
    BotId,
    Cash,
    Asset,
    CreatedAt,
    UpdatedAt,
}
