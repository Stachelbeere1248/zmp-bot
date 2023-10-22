mod commands;

use std::collections::HashSet;
use std::convert::Into;
use poise::serenity_prelude as serenity;
use serenity::model::id::UserId;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
pub struct Data {}

#[tokio::main]
async fn main() {
    let mut owners = HashSet::new();
    owners.insert(UserId(449579075531440128_u64));
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::lfg::lfg()],
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
            },owners,
            ..Default::default()
        })
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
