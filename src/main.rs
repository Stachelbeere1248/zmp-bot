#![feature(integer_sign_cast)]
#![feature(duration_constructors)]

use std::collections::HashSet;
use std::convert::Into;
use std::time::Duration;

use dashmap::DashMap;
use poise::serenity_prelude as serenity;
use serenity::all::{ActivityData, InteractionType, RoleId};
use serenity::prelude::GatewayIntents;
use serenity::{model::id::UserId, FullEvent};
use sqlx::Sqlite;

use error::Error;

mod commands;
mod data;
mod error;
mod handlers;

struct Caches {
    name: DashMap<String, (String, std::time::Instant)>,
    uuid: DashMap<String, (String, std::time::Instant)>,
}

impl Default for Caches {
    fn default() -> Self {
        Self {
            name: DashMap::new(),
            uuid: DashMap::new(),
        }
    }
}

struct ApiClients {
    hypixel_api_client: reqwest::Client,
    local_api_client: reqwest::Client,
    general: reqwest::Client,
}

impl Default for ApiClients {
    fn default() -> Self {
        Self {
            hypixel_api_client: {
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "API-Key",
                    reqwest::header::HeaderValue::try_from(
                        std::env::var("HYPIXEL_API_KEY").unwrap(),
                    )
                    .unwrap(),
                );
                reqwest::ClientBuilder::new()
                    .default_headers(headers)
                    .build()
                    .unwrap()
            },
            local_api_client: reqwest::ClientBuilder::new()
                .danger_accept_invalid_hostnames(true)
                .build()
                .unwrap(),
            general: reqwest::ClientBuilder::default().build().unwrap(),
        }
    }
}

struct Data {
    sqlite_pool: sqlx::Pool<Sqlite>,
    clients: ApiClients,
    caches: Caches,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            sqlite_pool: sqlx::sqlite::SqlitePoolOptions::new()
                .idle_timeout(Duration::from_secs(10))
                .connect_lazy("sqlite:accounts.db")
                .unwrap(),
            caches: Caches::default(),
            clients: ApiClients::default(),
        }
    }
}

type Context<'a> = poise::Context<'a, Data, Error>;
#[tokio::main]
async fn main() {
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::lfg::lfg(),
            commands::lfg::expert(),
            commands::lfg::other(),
            commands::xd::xd(),
            commands::helpstart::helpstart(),
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
        owners: {
            HashSet::from([
                UserId::new(449579075531440128_u64),
                UserId::new(659112817508745216_u64),
            ])
        },
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data::default())
            })
        })
        .build();

    let token = std::env::var("DISCORD_TOKEN").unwrap();
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;
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
            println!("Logged in as '{}'!", data_about_bot.user.name);
        }
        FullEvent::GuildMemberAddition { new_member } => {
            if new_member.guild_id.get() == 1256217633959841853_u64 {
                new_member
                    .add_role(ctx, RoleId::new(1256253358701023232_u64))
                    .await?;
            }
        }
        FullEvent::InteractionCreate { interaction } => {
            if interaction.application_id().get() == 1165594074473037824
                && interaction.kind() == InteractionType::Component
            {
                handlers::bot_interaction::component(ctx, interaction, data).await?;
            }
        }
        FullEvent::Message { new_message } => {
            handlers::message::on_create(ctx, new_message).await?;
        }
        FullEvent::ThreadCreate { thread } => {
            handlers::thread::on_create(ctx, thread).await?;
        }
        _ => {}
    }
    Ok(())
}
