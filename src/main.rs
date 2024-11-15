#![feature(integer_sign_cast)]

use std::collections::HashSet;
use std::convert::Into;
use std::sync::Arc;
use std::time::Duration;

use poise::serenity_prelude as serenity;
use serenity::{FullEvent, model::id::UserId};
use serenity::all::{ActivityData, InteractionType, RoleId};
use serenity::prelude::GatewayIntents;
use sqlx::Sqlite;
use tokio::sync::RwLock;

use error::Error;

mod commands;
mod error;
mod handlers;

struct Data {
    bots: Arc<RwLock<u8>>,
    sqlite_pool: sqlx::Pool<Sqlite>,
    hypixel_api_client: reqwest::Client,
} // User data, which is stored and accessible in all command invocations

type Context<'a> = poise::Context<'a, Data, Error>;
#[tokio::main]
async fn main() {
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .idle_timeout(Duration::from_secs(10))
        .connect_lazy("sqlite:accounts.db")
        .unwrap();

    let hypixel_api: String = std::env::var("HYPIXEL_API_KEY").unwrap();
    let hypixel_api_client = {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("API-Key", reqwest::header::HeaderValue::try_from(hypixel_api).unwrap());
        reqwest::ClientBuilder::new().default_headers(headers).build().unwrap()
    };

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::lfg::lfg(),
            commands::lfg::expert(),
            commands::lfg::other(),
            commands::xd::xd(),
            commands::helpstart::helpstart(),
            commands::bots::bots(),
            commands::accountv2::account(),
        ],
        manual_cooldowns: true,
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            ..Default::default()
        },
        on_error: |error| {
            Box::pin(async move {
                error::handle_error(error).await;
            })
        },
        owners: { HashSet::from([UserId::new(449579075531440128_u64), UserId::new(659112817508745216_u64)]) },
        event_handler: |ctx, event, framework, data| Box::pin(event_handler(ctx, event, framework, data)),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    bots: Arc::new(RwLock::new(0)),
                    sqlite_pool,
                    hypixel_api_client,
                })
            })
        })
        .build();

    let token = std::env::var("DISCORD_TOKEN").unwrap();
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MEMBERS;
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .activity(ActivityData::custom("NPC moment..."))
        .await;
    client.unwrap().start_autosharded().await.unwrap()
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        },
        FullEvent::GuildMemberAddition { new_member } => {
            println!("join event");
            if new_member.guild_id.get() == 1256217633959841853_u64 {
                new_member.add_role(ctx, RoleId::new(1256253358701023232_u64)).await?;
                println!("gave member role");
            }
        }
        FullEvent::InteractionCreate { interaction } => {
            if interaction.application_id().get() == 1165594074473037824 && interaction.kind() == InteractionType::Component {
                handlers::bot_interaction::component(ctx, interaction, data).await?;
            }
        },
        FullEvent::Message { new_message } => {
            handlers::message::on_create(ctx, new_message).await?;
        },
        FullEvent::ThreadCreate { thread } => {
            handlers::thread::on_create(ctx, thread).await?;
        },
        _ => {}
    }
    Ok(())
}
