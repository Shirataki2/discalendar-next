use ::entity::{
    account::{self, Entity as Account},
    sea_orm_active_enums::AccountStatus,
};
use sea_orm::*;

pub struct AccountQuery;

impl AccountQuery {
    pub async fn find_by_discord_id(
        db: &DbConn,
        discord_id: i64,
    ) -> Result<Option<account::Model>, DbErr> {
        let account = Account::find()
            .filter(account::Column::DiscordId.eq(discord_id))
            .one(db)
            .await?;
        Ok(account)
    }

    pub async fn create(db: &DbConn, discord_id: i64, name: &str) -> Result<account::Model, DbErr> {
        account::ActiveModel {
            discord_id: Set(discord_id),
            name: Set(name.to_string()),
            status: Set(AccountStatus::Enabled),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn upsert(db: &DbConn, discord_id: i64, name: &str) -> Result<account::Model, DbErr> {
        let account = Self::find_by_discord_id(db, discord_id).await?;
        match account {
            Some(account) => {
                account::ActiveModel {
                    id: Set(account.id),
                    name: Set(name.to_string()),
                    ..Default::default()
                }
                .update(db)
                .await
            }
            None => Self::create(db, discord_id, name).await,
        }
    }
}
