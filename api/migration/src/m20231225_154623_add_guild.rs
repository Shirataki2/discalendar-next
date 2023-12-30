use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Guild::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Guild::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Guild::Name).string().not_null())
                    .col(ColumnDef::new(Guild::GuildId).big_integer().unique_key().not_null())
                    .col(ColumnDef::new(Guild::IconUrl).string().not_null())
                    .to_owned()
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-guilds-guild-id")
                    .table(Guild::Table)
                    .col(Guild::GuildId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx-guilds-guild-id").to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Guild::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Guild {
    Table,
    Id,
    Name,
    GuildId,
    IconUrl,
}
