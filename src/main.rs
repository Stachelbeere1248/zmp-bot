mod commands;

use poise::{async_trait, Event, serenity_prelude as serenity};
use serenity::model::id::UserId;
use std::collections::HashSet;
use std::convert::Into;
use serenity::client::EventHandler;
use crate::commands::round::round;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {}
struct ReadyHandler;

#[async_trait]
impl EventHandler for ReadyHandler {
    async fn ready(&self, _: poise::serenity_prelude::Context, ready: poise::serenity_prelude::Ready) {
        println!("{} is connected!", ready.user.id);
    }
}


#[tokio::main]
async fn main() {
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::lfg::lfg(),
        ],
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
        owners: {
            HashSet::from([UserId(449579075531440128_u64)])
        },
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(event_handler(_ctx, event, _framework, _data))
        },
        ..Default::default()
    };




    let framework = poise::Framework::builder()

        .options(options)

        .token(std::env::var("DISCORD_TOKEN").unwrap())

        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )

        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
async fn event_handler(
    _ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        Event::Ready { data_about_bot } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        _ => {}
    }
    Ok(())
}