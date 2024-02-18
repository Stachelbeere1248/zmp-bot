mod commands;

use poise::{async_trait, serenity_prelude as serenity};
use serenity::{client::EventHandler, model::id::UserId, FullEvent};
use std::collections::HashSet;
use std::convert::Into;

struct Data {} // User data, which is stored and accessible in all command invocations
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

pub(crate) async fn main() {
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::lfg::lfg(),
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
                    poise::FrameworkError::ArgumentParse { error, .. } => {
                        if let Some(error) = error.downcast_ref::<serenity::RoleParseError>() {
                            println!("Found a RoleParseError: {:?}", error);
                        } else {
                            println!("Not a RoleParseError :(");
                        }
                    }
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
                Ok(Data {})
            })
        })
        .build();

    let token = std::env::var("DISCORD_TOKEN").unwrap();
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    if let Err(why) = client.unwrap().start().await
    {
        println!("Error starting serenity framework: {:?}", why)
    };
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
