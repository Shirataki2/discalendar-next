use crate::prelude::*;

#[derive(Debug, Serialize)]
pub struct GuildListResponse {
    pub invited: Vec<discord::GuildInfo>,
    pub invitable: Vec<discord::GuildInfo>,
    pub not_invitable: Vec<discord::GuildInfo>,
}

#[get("/list")]
async fn list(token: arguments::Token, req: HttpRequest) -> Result<HttpResponse, Error> {
    let client = DiscordClient::from_token(token.inner())?;
    let guilds = client.fetch_current_user_guilds().await?;
    let guild_ids = guilds.iter().map(|guild| guild.id.0 as i64).collect::<Vec<_>>();

    let db = get_data::<DbConn>(&req)?;
    let invited_guilds = service::GuildQuery::search_by_guild_ids(db, &guild_ids).await?;

    let mut resp = GuildListResponse {
        invited: vec![],
        invitable: vec![],
        not_invitable: vec![],
    };

    for guild in guilds {
        if invited_guilds.iter().any(|g| g.guild_id == guild.id.0 as i64) {
            resp.invited.push(guild);
        } else if is_invitable(&guild) {
            resp.invitable.push(guild);
        } else {
            resp.not_invitable.push(guild);
        }
    }

    Ok(HttpResponse::Ok().json(resp))
}

fn is_invitable(guild: &discord::GuildInfo) -> bool {
    if guild.owner {
        return true;
    }
    let permissions = guild.permissions;
    permissions.administrator() || permissions.manage_guild()
}
