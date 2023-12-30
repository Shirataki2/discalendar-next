use ::entity::guild::{self, Entity as Guild};
use poise::serenity_prelude as discord;
use sea_orm::*;

pub struct GuildQuery;

impl GuildQuery {
    pub async fn find_by_guild_id(
        db: &DbConn,
        guild_id: i64,
    ) -> Result<Option<guild::Model>, DbErr> {
        let account = Guild::find()
            .filter(guild::Column::GuildId.eq(guild_id))
            .one(db)
            .await?;
        Ok(account)
    }

    pub async fn search_by_guild_ids(
        db: &DbConn,
        guild_ids: &[i64],
    ) -> Result<Vec<guild::Model>, DbErr> {
        let guilds = Guild::find()
            .filter(guild::Column::GuildId.is_in(guild_ids.to_owned()))
            .all(db)
            .await?;
        Ok(guilds)
    }

    pub async fn create(db: &DbConn, guild: &discord::PartialGuild) -> Result<guild::Model, DbErr> {
        guild::ActiveModel {
            guild_id: Set(guild.id.0 as i64),
            name: Set(guild.name.clone()),
            icon_url: Set(guild.icon_url().unwrap_or("".to_string())),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}
