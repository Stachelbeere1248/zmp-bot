use std::collections::HashSet;
use std::convert::Into;
use std::sync::Arc;

use poise::{async_trait, serenity_prelude as serenity};
use serenity::{client::EventHandler, FullEvent, model::id::UserId};
use serenity::all::ActivityData;
use tokio::sync::RwLock;

mod commands;

struct Data {
    bots: Arc<RwLock<u8>>
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
struct ReadyHandler;

#[async_trait]
impl EventHandler for ReadyHandler {
    async fn ready(
        &self,
        _: poise::serenity_prelude::Context,
        ready: poise::serenity_prelude::Ready,
    ) {
        println!("{} is connected!", ready.user.id);
    }
}

#[tokio::main]
async fn main() {
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::lfg::lfg(),
            commands::lfg::expert(),
            commands::xd::xd(),
            commands::helpstart::helpstart(),
            commands::bots::bots(),
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
        }
        _ => {}
    }
    Ok(())
}
