use poise::CreateReply;
use reqwest::{Client, Response};
use serde::Deserialize;
use serenity::{
    all::{ChannelId, CreateActionRow, CreateAllowedMentions, CreateButton, CreateMessage, ReactionType, User},
};
use serenity::all::{ButtonStyle};
use sqlx::{Pool, query_as, Sqlite};

use crate::commands::command_helper::cooldown;
use crate::Context;
use crate::error::Error;

#[derive(Deserialize)]
struct Links {
    #[serde(rename = "DISCORD")]
    pub discord: String,
}
#[derive(Deserialize)]
struct SocialMedia {
    pub links: Links,
}
#[derive(Deserialize)]
struct HypixelPlayer {
    #[serde(rename = "socialMedia")]
    pub social_media: SocialMedia,
}
#[derive(Deserialize)]
struct HypixelResponse {
    #[serde(rename = "player")]
    pub player: HypixelPlayer,
}
#[derive(Deserialize)]
struct MojangPlayer {
    pub id: String,
    pub name: String,
}

#[derive(PartialEq, sqlx::FromRow)]
struct Uuid {
    uuid: String,
}
impl Uuid {
    fn get(&self) -> &str {
        self.uuid.as_str()
    }
    async fn for_ign(ign: &str) -> Result<Self, Error> {
        let url: String = format!("https://api.mojang.com/users/profiles/minecraft/{ign}");
        let response_400: Response = reqwest::get(url).await?.error_for_status()?;
        let deserialized = response_400.json::<MojangPlayer>().await?;
        let uuid = Uuid {
            uuid: deserialized.id,
        };
        Ok(uuid)
    }

    async fn ign(&self) -> Result<String, Error> {
        let url: String = format!("https://sessionserver.mojang.com/session/minecraft/profile/{}", self.uuid);
        let response_400: Response = reqwest::get(url).await?.error_for_status()?;
        let deserialized = response_400.json::<MojangPlayer>().await?;
        Ok(deserialized.name)
    }
}
#[derive(PartialEq)]
struct DiscordId {
    id: u64,
}
impl DiscordId {
    async fn matches_fetch(user: &User, uuid: &str, client: &Client) -> Result<bool, Error> {
        let url: String = format!("https://api.hypixel.net/v2/player?uuid={}", uuid);
        let response_400: Response = client.get(url).send().await?.error_for_status()?;
        let deserialized = response_400.json::<HypixelResponse>().await?;
        let matches = deserialized.player.social_media.links.discord == user.name;
        Ok(matches)
    }
}
impl<'a, R: sqlx::Row> sqlx::FromRow<'a, R> for DiscordId
where
    &'a ::std::primitive::str: sqlx::ColumnIndex<R>,
    i64: ::sqlx::decode::Decode<'a, R::Database>,
    i64: ::sqlx::types::Type<R::Database>,
{
    fn from_row(row: &'a R) -> sqlx::Result<Self> {
        let discord_id: i64 = row.try_get("discord_id")?;
        Ok(DiscordId {
            id: discord_id.cast_unsigned(),
        })
    }
}
struct Link {
    link_id: u16,
    discord_ids: Vec<DiscordId>,
    minecraft_accounts: Vec<Uuid>,
}
impl Link {
    fn new(link_id: u16) -> Self {
        Link {
            link_id,
            discord_ids: vec![],
            minecraft_accounts: vec![],
        }
    }
    async fn minecraft(mut self, pool: &Pool<Sqlite>) -> Result<Self, Error> {
        let link_id: i16 = self.link_id.cast_signed();
        self.minecraft_accounts =
            query_as(format!("SELECT minecraft_uuid AS uuid FROM minecraft_links WHERE link_id = {link_id};").as_str())
                .fetch_all(pool)
                .await?;
        Ok(self)
    }
    async fn discord(mut self, pool: &Pool<Sqlite>) -> Result<Self, Error> {
        let link_id: i16 = self.link_id.cast_signed();
        self.discord_ids = query_as(format!("SELECT discord_id FROM discord_links WHERE link_id = {link_id};").as_str())
            .fetch_all(pool)
            .await?;
        Ok(self)
    }
}
#[poise::command(slash_command, subcommands("add", "list"))]
pub(crate) async fn account(_ctx: Context<'_>) -> Result<(), Error> {
    // root of slash-commands is not invokable.
    unreachable!()
}

#[poise::command(
    slash_command,
    install_context = "User|Guild",
    interaction_context = "Guild|BotDm|PrivateChannel",
    ephemeral = "false",
)]
pub(crate) async fn add<'a>(
    ctx: Context<'_>,
    #[description = "Minecraft username"]
    #[min_length = 2]
    #[max_length = 16]
    ign: String,
    #[description = "Discord User"] user: Option<User>,
    #[description = "admin-only"] force: Option<bool>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let user: User = user.unwrap_or(ctx.author().clone());
    let uuid: Uuid = Uuid::for_ign(ign.as_str()).await?;
    let force: bool = force.unwrap_or(false) && ctx.framework().options.owners.contains(&ctx.author().id);
    match DiscordId::matches_fetch(&user, uuid.get(), &ctx.data().hypixel_api_client).await? || force {
        true => {
            let pool: Pool<Sqlite> = ctx.data().sqlite_pool.clone();
            let status: &str = match link_id_from_minecraft(&pool, uuid.get()).await {
                None => match link_id_from_discord(&pool, user.id.get()).await {
                    None => {
                        let id = new_link_id(&pool).await?;
                        sqlx::query(format!("INSERT INTO discord_links VALUES ({}, {});", id.cast_signed(), user.id.get()).as_str())
                            .execute(&pool)
                            .await?;
                        sqlx::query(format!("INSERT INTO minecraft_links VALUES ({}, \"{}\");", id.cast_signed(), uuid.get()).as_str())
                            .execute(&pool)
                            .await?;
                        "Linked your Discord and Minecraft account."
                    }
                    Some(dc_id) => {
                        sqlx::query(format!("INSERT INTO minecraft_links VALUES ({}, \"{}\");", dc_id.cast_signed(), uuid.get()).as_str())
                            .execute(&pool)
                            .await?;
                        "Your Discord account has previously had an account linked. Added the new link."
                    }
                },
                Some(mc_id) => match link_id_from_discord(&pool, user.id.get()).await {
                    None => {
                        sqlx::query(format!("INSERT INTO discord_links VALUES ({}, {});", mc_id.cast_signed(), user.id.get()).as_str())
                            .execute(&pool)
                            .await?;
                        "Your Minecraft account has previously had an account linked. Added the new link."
                    }
                    Some(dc_id) => {
                        sqlx::query(
                            format!(
                                "UPDATE minecraft_links SET link_id = {} WHERE link_id = {};",
                                mc_id.cast_signed(),
                                dc_id.cast_signed()
                            )
                                .as_str(),
                        )
                            .execute(&pool)
                            .await?;
                        sqlx::query(
                            format!(
                                "UPDATE discord_links SET link_id = {} WHERE link_id = {};",
                                mc_id.cast_signed(),
                                dc_id.cast_signed()
                            )
                                .as_str(),
                        )
                            .execute(&pool)
                            .await?;
                        "Both your Discord and Minecraft account had linked accounts. Merged all account links."
                    }
                },
            };
            let s = format!("Verification request for <@{}> with IGN `{}`", user.id.get(), ign);
            ChannelId::new(1257776992497959075)
                .send_message(
                    ctx,
                    CreateMessage::new()
                        .content(s)
                        .allowed_mentions(CreateAllowedMentions::new().empty_roles().all_users(true))
                        .components(vec![CreateActionRow::Buttons(vec![
                            CreateButton::new("accept_verification")
                                .emoji(ReactionType::from('✅'))
                                .style(ButtonStyle::Secondary),
                            CreateButton::new("deny_verification")
                                .emoji(ReactionType::from('❌'))
                                .style(ButtonStyle::Secondary),
                            CreateButton::new("list_accounts")
                                .emoji(ReactionType::from('📜'))
                                .style(ButtonStyle::Primary),
                        ])]),
                )
                .await?;
            ctx.send(CreateReply::default().content(status)).await?;
            Ok(())
        }
        false => Err(Error::Other(format!(
            "The Discord account linked on Hypixel does not match the specified discord account.\nPlease set your linked Discord account \
             on Hypixel to `{}`.",
            user.name
        ))),
    }
}

#[poise::command(
    slash_command,
    install_context = "User|Guild",
    interaction_context = "Guild|BotDm|PrivateChannel",
    ephemeral = "false",
)]
pub(crate) async fn list(ctx: Context<'_>, user: Option<User>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    cooldown(&ctx, 600, 300)?;
    let user: User = user.unwrap_or(ctx.author().clone());
    let pool: &Pool<Sqlite> = &ctx.data().sqlite_pool;
    let s: String = list_string(pool, &user).await?;
    let r: CreateReply = CreateReply::default().ephemeral(false);
    ctx.send(
        r.content(s)
            .allowed_mentions(CreateAllowedMentions::new().empty_roles().empty_users()),
    )
        .await?;
    Ok(())
}

#[poise::command(
    slash_command,
    install_context = "User|Guild",
    interaction_context = "Guild|PrivateChannel",
    ephemeral = "false",
    context_menu_command="Account list"
)]
pub(crate) async fn context_list(ctx: Context<'_>, u: User) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    cooldown(&ctx, 600, 300)?;
    let pool: &Pool<Sqlite> = &ctx.data().sqlite_pool;
    let s: String = list_string(pool, &u).await?;
    ctx.send(
        CreateReply::default()
            .content(s)
            .allowed_mentions(CreateAllowedMentions::new().empty_roles().empty_users())
            .ephemeral(true)
    ).await?;
    Ok(())
}

pub(crate) async fn list_string(pool: &Pool<Sqlite>, user: &User) -> Result<String, Error> {
    let link_id: u16 = link_id_from_discord(pool, user.id.get())
        .await
        .expect("This user has no linked accounts");
    let link: Link = Link::new(link_id).minecraft(pool).await?.discord(pool).await?;
    let mut discord_list = String::from("### Discord:");
    for dc in link.discord_ids {
        discord_list.push_str(format!("\n- <@{}>", dc.id).as_str());
    }
    let mut minecraft_list = String::from("### Minecraft:");
    for mc in link.minecraft_accounts {
        minecraft_list.push_str(format!("\n- `{}`", mc.ign().await?).as_str());
    }
    Ok(format!(
        "## Account list for member #{}:\n{}\n{}",
        link.link_id,
        discord_list.as_str(),
        minecraft_list.as_str()
    ))
}

#[poise::command(slash_command)]
pub(crate) async fn remove(_ctx: Context<'_>) -> Result<(), Error> {
    todo!()
}

async fn link_id_from_minecraft(pool: &Pool<Sqlite>, minecraft_uuid: &str) -> Option<u16> {
    query_as(format!("SELECT link_id FROM minecraft_links WHERE minecraft_uuid = \"{minecraft_uuid}\" LIMIT 1;").as_str())
        .fetch_optional(pool)
        .await
        .expect("Database error: fetching link id by uuid")
        .map(|link_id: LinkId| link_id.link_id.cast_unsigned())
}
async fn link_id_from_discord(pool: &Pool<Sqlite>, snowflake: u64) -> Option<u16> {
    query_as(
        format!(
            "SELECT link_id FROM discord_links WHERE discord_id = {} LIMIT 1;",
            snowflake.cast_signed()
        )
            .as_str(),
    )
        .fetch_optional(pool)
        .await
        .expect("Database error: fetching link_id for discord_id")
        .map(|link_id: LinkId| link_id.link_id.cast_unsigned())
}

#[derive(sqlx::FromRow)]
struct LinkId {
    link_id: i16,
}

async fn new_link_id(pool: &Pool<Sqlite>) -> Result<u16, Error> {
    let result: LinkId = query_as("SELECT MAX(link_id) AS link_id FROM minecraft_links;")
        .fetch_one(pool)
        .await?;
    Ok(result.link_id.cast_unsigned() + 1)
}
