use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(AccountStatus::Table)
                    .values(AccountStatus::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Account::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Account::DiscordId)
                            .big_integer()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Account::Name).string().not_null())
                    .col(
                        ColumnDef::new(Account::Status)
                            .enumeration(AccountStatus::Table, AccountStatus::iter().skip(1))
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-users-discord-id")
                    .table(Account::Table)
                    .col(Account::DiscordId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx-users-discord-id").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(AccountStatus::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
    DiscordId,
    Name,
    Status,
}

#[derive(Iden, EnumIter)]
pub enum AccountStatus {
    Table,
    Enabled,
    Disabled,
}
