use poise::CreateReply;
use reqwest::Response;
use serde::Deserialize;
use serenity::{
    all::{
        CreateAllowedMentions,
        CreateButton,
        CreateActionRow,
        User,
        ReactionType,
        ChannelId,
        CreateMessage
    },
    json::JsonError
};
use sqlx::{Pool, query_as, Sqlite};

use crate::{Context, Error};

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
    uuid: String
}
impl Uuid {
    fn new(uuid: &str) -> Self {
        let uuid: String = uuid.to_string();
        Uuid {
            uuid
        }
    }
    fn get(&self) -> &str {
        self.uuid.as_str()
    }
    async fn fetch(ign: &str) -> Result<Self, Error> {
        let url: String = format!("https://api.mojang.com/users/profiles/minecraft/{ign}");
        let response: Response = reqwest::get(url).await?;
        match response.error_for_status() {
            Ok(res) => {
                let response_text = res.text().await.unwrap();
                let uuid = (serde_json::from_str(response_text.as_str())
                    as Result<MojangPlayer, JsonError>)
                    .map(|mojang_player: MojangPlayer| Uuid {uuid: mojang_player.id})?;
                Ok(uuid)
            },
            Err(why) => Err(Error::from(format!(
                "Mojang returned an error. Please make sure to enter a valid Minecraft username.\n\n\
                Details: {}", why).as_str())),
        }
    }
}
#[derive(PartialEq)]
struct DiscordId {
    id: u64
}
impl DiscordId {
    fn new(id: u64) -> Self {
        DiscordId {
            id
        }
    }
    fn new_from_unsigned(id: i64) -> Self {
        DiscordId {
            id: id.cast_unsigned()
        }
    }
    async fn matches_fetch(user: &User, uuid: &str) -> Result<bool, Error> {
        let url: String = format!("https://api.hypixel.net/v2/player?uuid={}", uuid);
        let response: Response = reqwest::get(url).await?;
        match response.error_for_status() {
            Ok(res) => {
                let response_text = res.text().await.unwrap();
                let matches = (serde_json::from_str(response_text.as_str())
                    as Result<HypixelPlayer, JsonError>)
                    .map(|hypixel_player: HypixelPlayer| user.name == hypixel_player.social_media.links.discord)?;
                Ok(matches)
            },
            Err(why) => {
                println!("Hypixel issue: {}", why);
                Err(Error::from("Hypixel returned an error."))
            }
        }
    }
}
impl<'a, R: sqlx::Row> sqlx::FromRow<'a, R> for DiscordId
where &'a ::std::primitive::str: sqlx::ColumnIndex<R>,
    i64: ::sqlx::decode::Decode<'a, R::Database>,
    i64: ::sqlx::types::Type<R::Database> {
    fn from_row(row: &'a R) -> sqlx::Result<Self> {
        let discord_id: i64 = row.try_get("discord_id")?;
        Ok(DiscordId {
            id: discord_id.cast_unsigned()
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
    async fn minecraft(mut self, pool: &Pool<Sqlite>) -> Self {
        let link_id: i16 = self.link_id.cast_signed();
        self.minecraft_accounts = query_as(format!("SELECT minecraft_uuid AS uuid FROM minecraft_links WHERE link_id = {link_id};").as_str())
            .fetch_all(pool).await.expect("Error getting Minecraft UUIDs.");
        self
    }
    async fn discord(mut self, pool: &Pool<Sqlite>) -> Self {
        let link_id: i16 = self.link_id.cast_signed();
        self.discord_ids = query_as(format!("SELECT discord_id FROM discord_links WHERE link_id = {link_id};").as_str())
                .fetch_all(pool).await.expect("Error getting Discord IDs.");
        self
    }
}
#[poise::command(slash_command, subcommands("add", "list"))]
pub(crate) async fn account(_ctx: Context<'_>) -> Result<(), Error> {
    // root of slash-commands is not invokable.
    unreachable!()
}


#[poise::command(slash_command)]
pub(crate) async fn add<'a>(
    ctx: Context<'_>,

    #[description = "Minecraft username"]
    #[min_length = 2]
    #[max_length = 16]
    ign: String,

    user: Option<User>,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let user = user.unwrap_or(ctx.author().clone());
    let uuid = Uuid::fetch(ign.as_str()).await?;
    let valid = DiscordId::matches_fetch(&user, uuid.get()).await
        .expect("This Minecraft account does not have a Discord account linked.");
    match valid {
        true => {
            let r = CreateReply::default().ephemeral(false);
            let pool: Pool<Sqlite> = ctx.data().sqlite_pool.clone();
            let (status, link_id) = match link_id_from_minecraft(&pool, uuid.get()).await {
                None => { match link_id_from_discord(&pool, user.id.get()).await {
                        None => {
                            let id = new_link_id(&pool).await;
                            sqlx::query(format!("INSERT INTO discord_links VALUES ({}, {});", id.cast_signed(), user.id.get()).as_str())
                                .execute(&pool).await.expect("Database Error: inserting new minecraft value");
                            sqlx::query(format!("INSERT INTO minecraft_links VALUES ({}, \"{}\");", id.cast_signed(), uuid.get()).as_str())
                                .execute(&pool).await.expect("Database Error: inserting new minecraft value");
                            ("Linked your Discord and Minecraft account.", id)
                        }
                        Some(dc_id) => {
                            sqlx::query(format!("INSERT INTO minecraft_links VALUES ({}, \"{}\");", dc_id.cast_signed(), uuid.get()).as_str())
                                .execute(&pool).await.expect("Database Error: inserting new minecraft value");
                            ("Your Discord account has previously had an account linked. Added the new link.", dc_id)
                        }
                    }
                }, Some(mc_id) => {
                    match link_id_from_discord(&pool, user.id.get()).await {
                        None => {
                            sqlx::query(format!("INSERT INTO discord_links VALUES ({}, {});", mc_id.cast_signed(), user.id.get()).as_str())
                                .execute(&pool).await.expect("Database Error: inserting new minecraft value");
                            ("Your Minecraft account has previously had an account linked. Added the new link.", mc_id)
                        },
                        Some(dc_id) => {
                            sqlx::query(format!("UPDATE minecraft_links SET link_id = {} WHERE link_id = {};", mc_id.cast_signed(), dc_id.cast_signed()).as_str())
                                .execute(&pool).await.expect("Database Error: Merging Minecraft Accounts.");
                            sqlx::query(format!("UPDATE discord_links SET link_id = {} WHERE link_id = {};", mc_id.cast_signed(), dc_id.cast_signed()).as_str())
                                .execute(&pool).await.expect("Database Error: Merging Discord Accounts.");
                            ("Both your Discord and Minecraft account had linked accounts. Merged all account links.", mc_id)
                        }
                    }
                }
            };
            ctx.send(r.content(status)).await?;
            let link = Link::new(link_id).minecraft(&pool).await.discord(&pool).await;
            let s = list_string(link, user.id.get()).await;
            ChannelId::new(1257776992497959075).send_message(
                ctx,
                CreateMessage::new()
                    .content(s)
                    .allowed_mentions(CreateAllowedMentions::new().empty_roles().all_users(true))
                    .components(vec![CreateActionRow::Buttons(vec![
                        CreateButton::new("accept_verification").emoji(ReactionType::from('✅')),
                        CreateButton::new("deny_verification").emoji(ReactionType::from('❌')),
                    ])])
            ).await?;
        }
        false => {
            let r = CreateReply::default().ephemeral(true)
                .content(format!("The Discord account linked on Hypixel does not seem to match the specified account.\n\
                Expected account link: `{}`", user.name));
            ctx.send(r).await?;
        }
    }
    Ok(())
}

#[poise::command(slash_command)]
pub(crate) async fn list(
    ctx: Context<'_>,
    user: Option<User>
) -> Result<(), Error> {
    ctx.defer().await?;
    let user = user.unwrap_or(ctx.author().clone());
    let r = CreateReply::default().ephemeral(false);
    let pool: Pool<Sqlite> = ctx.data().sqlite_pool.clone();
    let link_id = link_id_from_discord(&pool, user.id.get()).await.expect("This user has no linked accounts");
    let link = Link::new(link_id).minecraft(&pool).await.discord(&pool).await;
    let s = list_string(link, user.id.get()).await;
    ctx.send(r.content(s).allowed_mentions(CreateAllowedMentions::new().empty_roles().empty_users())).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub(crate) async fn remove(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

async fn list_string(link: Link, user_id: u64) -> String {
    let mut discord_list = String::from("### Discord:");
    for dc in link.discord_ids {
        discord_list.push_str(format!("\n- <@{}>", dc.id).as_str());
    }
    let mut minecraft_list = String::from("### Minecraft:");
    for mc in link.minecraft_accounts {
        minecraft_list.push_str(format!("\n- `{}`>", mc.get()).as_str());
    }
    format!("## Account list for <@{user_id}>:\n{}\n{}", discord_list.as_str(), minecraft_list.as_str())
}

#[derive(sqlx::FromRow)]
struct MinecraftLink {
    link_id: i16,
    minecraft_uuid: String,
}

async fn link_id_from_minecraft(pool: &Pool<Sqlite>, minecraft_uuid: &str) -> Option<u16> {
    return query_as(
        format!(
            "SELECT * FROM minecraft_links WHERE minecraft_uuid = \"{minecraft_uuid}\" LIMIT 1;"
        )
            .as_str(),
    )
        .fetch_optional(pool)
        .await
        .expect("Database error: fetching link id by uuid")
        .map(|minecraft_link: MinecraftLink| minecraft_link.link_id.cast_unsigned());
}

#[derive(sqlx::FromRow)]
struct DiscordLink {
    link_id: i16,
    discord_id: i64,
}
async fn link_id_from_discord(pool: &Pool<Sqlite>, snowflake: u64) -> Option<u16> {
    let discord_id: i64 = snowflake.cast_signed();
    return query_as(
        format!("SELECT * FROM discord_links WHERE discord_id = {discord_id} LIMIT 1;").as_str(),
    )
        .fetch_optional(pool)
        .await
        .expect("Database error: fetching link id by discord")
        .map(|discord_link: DiscordLink| discord_link.link_id.cast_unsigned());
}

#[derive(sqlx::FromRow)]
struct LinkId {
    link_id: i16,
}

async fn new_link_id(pool: &Pool<Sqlite>) -> u16 {
    let result: Result<LinkId, sqlx::Error> = query_as("SELECT MAX(link_id) AS link_id FROM minecraft_links;")
        .fetch_one(pool)
        .await;
    result
        .expect("Database error: fetching new id")
        .link_id.cast_unsigned() + 1
}