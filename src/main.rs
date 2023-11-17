mod commands;

use poise::{async_trait, serenity_prelude as serenity, Event};
use serenity::client::EventHandler;
use serenity::model::id::UserId;
use std::collections::HashSet;
use std::convert::Into;
use shuttle_secrets::SecretStore;
use shuttle_poise::ShuttlePoise;
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

#[shuttle_runtime::main]
async fn poise(
    #[shuttle_secrets::Secrets]
    secret_store: SecretStore
) -> ShuttlePoise<Data, Error> {

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::lfg::lfg(),
            commands::xd::xd(),
            commands::helpstart::helpstart()
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
        owners: { HashSet::from([UserId(449579075531440128_u64)]) },
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(event_handler(_ctx, event, _framework, _data))
        },
        ..Default::default()
    };
    let discord_token = secret_store.get("DISCORD_TOKEN").unwrap();

    let framework = poise::Framework::builder()
        .options(options)
        .token(discord_token)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;
    Ok(framework.into())
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
