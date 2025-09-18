use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Bot::Table)
                    .if_not_exists()
                    .col(pk_auto(Bot::Id))
                    .col(uuid(Bot::Uuid).unique_key())
                    .col(string(Bot::Name).not_null())
                    .col(
                        timestamp_with_time_zone(Bot::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Bot::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Bot::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub enum Bot {
    Table,
    Id,
    Uuid,
    Name,
    CreatedAt,
    UpdatedAt,
}
