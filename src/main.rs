#![feature(integer_sign_cast)]

use std::collections::HashSet;
use std::convert::Into;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use poise::{async_trait, serenity_prelude as serenity};
use serenity::{client::EventHandler, FullEvent, model::id::UserId};
use serenity::all::{ActivityData, Attachment, ChannelId, CreateAttachment, CreateMessage, Event, Guild, GuildChannel};
use serenity::all::Route::Channel;
use sqlx::{Acquire, ConnectOptions, Executor, Sqlite};
use tokio::sync::RwLock;

mod commands;

struct Data {
    bots: Arc<RwLock<u8>>,
    sqlite_pool: sqlx::Pool<Sqlite>,
    hypixel_api_client: reqwest::Client
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
struct ReadyHandler;

#[tokio::main]
async fn main() {


    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .idle_timeout(Duration::from_secs(10))
        .max_connections(3)
        .connect_lazy("sqlite:accounts.db").unwrap();

    let hypixel_api: String = std::env::var("HYPIXEL_API_KEY").unwrap();
    let hypixel_api_client = {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "API-Key",
            reqwest::header::HeaderValue::try_from(hypixel_api).unwrap(),
        );
        reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build().unwrap()
    };


    let options = poise::FrameworkOptions {
        commands: vec![
            commands::lfg::lfg(),
            commands::lfg::expert(),
            commands::xd::xd(),
            commands::helpstart::helpstart(),
            commands::bots::bots(),
            commands::account::account(),
        ],
        manual_cooldowns: true,
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            ..Default::default()
        },
        on_error: |error| {
            Box::pin(async move {
                match error {
                    other => poise::builtins::on_error(other).await.unwrap(),
                }
            })
        },
        owners: { HashSet::from([UserId::new(449579075531440128_u64)]) },
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(event_handler(_ctx, event, _framework, _data))
        },
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
                    hypixel_api_client
                })
            })
        })
        .build();

    let token = std::env::var("DISCORD_TOKEN").unwrap();
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .activity(ActivityData::playing("arcade_zombies_prison"))
        .await;
    client.unwrap().start().await.unwrap()
}
async fn event_handler(
    _ctx: &serenity::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        },
        _ => {}
    }
    Ok(())
}
