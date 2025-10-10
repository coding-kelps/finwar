use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS pgcrypto;")
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Bot::Table)
                    .if_not_exists()
                    .col(pk_auto(Bot::Id))
                    .col(uuid(Bot::Uuid).not_null().unique_key().default(Expr::cust("gen_random_uuid()")))
                    .col(string(Bot::Name).not_null().unique_key())
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
