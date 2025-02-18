use crate::data::helpstart_api::fetch_all;
use crate::data::helpstart_api::ListType::*;
use crate::data::mojang::name;
use crate::error::Error;
use crate::Context;
use poise::serenity_prelude::ButtonStyle;
use poise::serenity_prelude::ComponentInteractionCollector;
use poise::serenity_prelude::CreateActionRow;
use poise::serenity_prelude::CreateButton;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::CreateInteractionResponse;
use poise::serenity_prelude::CreateInteractionResponseMessage;
use poise::serenity_prelude::EditMessage;
use poise::CreateReply;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    ephemeral = "false"
)]
// Check for bots available to you.
pub(crate) async fn helpstart(
    ctx: Context<'_>,
    user: Option<String>,
    ephemeral: Option<bool>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let ephemeral = ephemeral.unwrap_or(true);
    let links = super::accountv2::get_link(ctx.author(), &ctx.data().sqlite_pool).await?;
    let mc_accounts = match user {
        None => {
            futures::future::try_join_all(
                links
                    .minecraft_accounts
                    .into_iter()
                    .map(|a| name(&ctx.data().caches, &ctx.data().clients.general, a.uuid))
                    .collect::<Vec<_>>(),
            )
            .await?
        }
        Some(name) => vec![name],
    };

    let bots = fetch_all(&ctx.data().clients.local_api_client).await?;
    let usable = bots
        .iter()
        .filter(|b| match b.list_type() {
            Whitelist => b.list().iter().any(|w| mc_accounts.contains(w)),
            Blacklist => mc_accounts.iter().any(|m| !b.list().contains(m)),
        })
        .collect::<Vec<_>>();

    let s: String = usable
        .iter()
        .map(|b| b.username().as_str())
        .collect::<Vec<_>>()
        .join(", ");

    let ready: String = usable
        .iter()
        .filter_map(|b| {
            if !*b.in_party()
                && *b.last_updated()
                    > (SystemTime::now().duration_since(UNIX_EPOCH).ok()? - Duration::from_secs(5))
                        .as_secs_f64()
            {
                Some(b.username().as_str())
            } else {
                println!(
                    "{}, {}",
                    *b.last_updated(),
                    (SystemTime::now().duration_since(UNIX_EPOCH).ok()? - Duration::from_secs(5))
                        .as_secs_f64()
                );
                None
            }
        })
        .collect::<Vec<_>>()
        .join(", ");
    let bid = ctx.id();
    let components = vec![CreateActionRow::Buttons(vec![CreateButton::new(
        bid.to_string(),
    )
    .style(ButtonStyle::Primary)
    .label("📜")])];

    let reply = CreateReply::default()
        .content(format!(
            "Bots that are ready for use: {ready}\nBots you can use: {s}\nTotal registered bots: \
             {}",
            bots.len()
        ))
        .ephemeral(ephemeral)
        .components(components);
    ctx.send(reply).await?;

    while let Some(mut i) = ComponentInteractionCollector::new(ctx)
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(60))
        .filter(move |i| i.data.custom_id == bid.to_string())
        .await
    {
        let embed = CreateEmbed::new().fields(bots.iter().filter_map(|b| {
            if b.note().trim().is_empty() || usable.iter().any(|&u| std::ptr::eq(u, b)) {
                None
            } else {
                Some((b.username(), b.note(), true))
            }
        })).title("Notes").description(
            "Below is the note of each bot that you cannot use. It might help you get whitelisted.",
        );
        i.create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed),
            ),
        )
        .await?;
        i.message
            .edit(ctx, EditMessage::default().components(vec![]))
            .await?;
    }
    Ok(())
}
