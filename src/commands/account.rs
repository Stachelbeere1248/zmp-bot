use poise::CreateReply;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, CreateActionRow, CreateButton, CreateMessage, ReactionType, User};
use serenity::builder::CreateAllowedMentions;
use sqlx::{Pool, query_as, Sqlite};

use crate::{Context, Error};

#[poise::command(slash_command, subcommands("add", "list"))]
pub(crate) async fn account(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command)]
pub(crate) async fn add(ctx: Context<'_>, ign: String) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let pool = ctx.data().sqlite_pool.clone();
    let minecraft_uuid = minecraft_uuid_for_username(ign.clone()).await?;
    let hypixel_linked_discord = linked_discord_for_uuid(
        ctx.data().hypixel_api_client.clone(),
        minecraft_uuid.as_str(),
    )
    .await?;
    if hypixel_linked_discord.eq(ctx.author().name.as_str()) {
        link(
            ctx.author().id.get(),
            minecraft_uuid.as_str(),
            &pool,
        )
        .await;
        let s = format!("## User <@{}> added an account:\n### added:\n- name: {}\n- uuid: {}",
                        ctx.author().id.get(),
                        ign.clone(),
                        minecraft_uuid
        );
        ChannelId::new(1257776992497959075).send_message(ctx,
            CreateMessage::new()
                .content(s)
                .allowed_mentions(CreateAllowedMentions::new().empty_roles().all_users(true))
                .components(vec![CreateActionRow::Buttons(vec![
                    CreateButton::new("accept_verification").emoji(ReactionType::from('✅')),
                    CreateButton::new("deny_verification").emoji(ReactionType::from('❌')),
                ])])
        ).await?;
        ctx.send(CreateReply::default().content("Linked accounts.")).await?;
    } else {
        ctx.send(CreateReply::default().content("This Minecraft account's link doesn't seem to match your discord username. Be sure to not link using the display name and remove the @.")).await?;
    }
    Ok(())
}

#[poise::command(slash_command)]
pub(crate) async fn remove(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("hi").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub(crate) async fn list(ctx: Context<'_>, user: Option<User>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let user_id = user.clone().map(|user| user.id.get());
    let user_name = user.clone().map(|user| user.name);
    let author_name = ctx.author().name.clone();
    let pool = ctx.data().sqlite_pool.clone();
    let link_id = link_id_from_discord(&pool, user_id.unwrap_or(ctx.author().id.get())).await;

    let t = match link_id {
        Some(id) => minecraft_uuids(&pool, id).await,
        None => Vec::new(),
    };
    let mut content = format!(
        "## {}'s linked accounts: ",
        user_name.unwrap_or(author_name)
    );
    for l in t {
        content.push_str(format!("\nuuid: {}", l).as_str())
    }
    let reply = CreateReply::default().ephemeral(true).content(content);
    if let Err(why) = ctx.send(reply).await {
        println!("Error sending message: {why}");
    }
    Ok(())
}

async fn link(discord_id: u64, uuid: &str, pool: &Pool<Sqlite>) {
    let link_id = match link_id_from_minecraft(pool, uuid.to_string()).await {
        None => new_link_id(pool).await,
        Some(link_id_mc_old) => {
            // merge sets
            let new_link_id_discord = link_id_from_discord(pool, discord_id)
                .await
                .unwrap_or(u16::MAX)
                .cast_signed();
            sqlx::query(format!("UPDATE minecraft_links SET link_id = {} WHERE link_id = {new_link_id_discord};", link_id_mc_old.cast_signed()).as_str()).execute(pool).await.expect("Database Error: linking previously linked accounts by another user");
            sqlx::query(
                format!(
                    "UPDATE discord_links SET link_id = {} WHERE link_id = {new_link_id_discord};",
                    link_id_mc_old.cast_signed()
                )
                .as_str(),
            )
            .execute(pool)
            .await.expect("Database Error: linking previously linked accounts by another user");
            link_id_mc_old
        }
    };

    let link_id = link_id.cast_signed();
    let discord_id = discord_id.cast_signed();
    sqlx::query(
        format!("INSERT INTO minecraft_links VALUES ({link_id}, \"{uuid}\");").as_str(),
    )
    .execute(pool)
    .await.expect("Database Error: inserting new minecraft value");
    sqlx::query(
        format!("INSERT INTO discord_links VALUES ({link_id}, \"{discord_id}\");").as_str(),
    )
    .execute(pool)
    .await.expect("Database Error: inserting new discord value");
}

#[derive(Serialize, Deserialize)]
struct Links {
    #[serde(rename = "DISCORD")]
    pub discord: String,
}
#[derive(Serialize, Deserialize)]
struct SocialMedia {
    pub links: Links,
    pub prompt: bool,
}
#[derive(Serialize, Deserialize)]
struct HypixelPlayer {
    #[serde(rename = "socialMedia")]
    pub social_media: SocialMedia,
}

#[derive(Serialize, Deserialize)]
struct HypixelResponse {
    #[serde(rename = "player")]
    pub player: HypixelPlayer,
}

#[derive(Serialize, Deserialize)]
struct MojangPlayer {
    pub id: String,
    pub name: String,
}

async fn minecraft_uuid_for_username(name: String) -> Result<String, serde_json::Error> {
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{name}");
    let response = reqwest::get(url).await.expect(format!("Failed retrieving hypixel response for {name}").as_str());
    let response_text = response.text().await.unwrap();
    return (serde_json::from_str(response_text.as_str())
        as Result<MojangPlayer, serde_json::Error>)
        .map(|mojang_player: MojangPlayer| mojang_player.id);
}

async fn linked_discord_for_uuid(
    hypixel_client: reqwest::Client,
    uuid: &str,
) -> Result<String, Error> {
    let hypixel_url = format!("https://api.hypixel.net/v2/player?uuid={uuid}");
    return match hypixel_client.get(hypixel_url).send().await {
        Ok(response) => {
            let response_text = response.text().await.unwrap();
            match (serde_json::from_str(response_text.as_str())
                as Result<HypixelResponse, serde_json::Error>)
                .map(|hypixel_response: HypixelResponse| {
                    hypixel_response.player.social_media.links.discord
                }) {
                Ok(discord) => Ok(discord),
                Err(why) => Err(Error::try_from(why).unwrap()),
            }
        }
        Err(why) => Err(Error::try_from(why).unwrap()),
    };
}

#[derive(sqlx::FromRow)]
struct DiscordLink {
    link_id: i16,
    discord_id: i64,
}

#[derive(sqlx::FromRow)]
struct MinecraftLink {
    link_id: i16,
    minecraft_uuid: String,
}
#[derive(sqlx::FromRow)]
struct LinkId {
    link_id: i16,
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
async fn link_id_from_minecraft(pool: &Pool<Sqlite>, minecraft_uuid: String) -> Option<u16> {
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

async fn new_link_id(pool: &Pool<Sqlite>) -> u16 {
    let result: Result<LinkId, sqlx::Error> = query_as("SELECT link_id FROM minecraft_links WHERE link_id = (SELECT MAX(link_id) FROM minecraft_links) LIMIT 1;")
        .fetch_one(pool)
        .await;
    result
        .expect("Database error: fetching new id")
        .link_id.cast_unsigned() + 1
}

async fn minecraft_uuids(pool: &Pool<Sqlite>, link_id: u16) -> Vec<String> {
    let link_id: i16 = link_id.cast_signed();
    let link_result: Result<Vec<MinecraftLink>, sqlx::Error> =
        query_as(format!("SELECT * FROM minecraft_links WHERE link_id = {link_id};").as_str())
            .fetch_all(pool)
            .await;
    return match link_result {
        Ok(links) => links
            .into_iter()
            .map(|minecraft_link: MinecraftLink| minecraft_link.minecraft_uuid)
            .collect(),
        Err(why) => {
            println!("Error: {}", why);
            Vec::new()
        }
    };
}
/*
async fn discord_ids(pool: &Pool<Sqlite>, link_id: u16) -> Vec<u64> {
    let link_id: i16 = link_id.cast_signed();
    let link_result: Result<Vec<DiscordLink>, sqlx::Error> = query_as(format!("SELECT * FROM discord_links WHERE link_id = {link_id}").as_str())
        .fetch_all(pool)
        .await;
    return match link_result {
        Ok(links) => links.into_iter().map(|discord_link: DiscordLink| discord_link.discord_id.cast_unsigned()).collect(),
        Err(why) => {
            println!("Error: {}", why);
            Vec::new()
        }
    }
}*/
