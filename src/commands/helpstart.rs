use crate::data::helpstart_api::fetch_all;
use crate::data::helpstart_api::ListType::*;
use crate::data::mojang::name;
use crate::error::Error;
use crate::Context;
use poise::CreateReply;

#[poise::command(
    slash_command,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm",
    ephemeral = "true"
)]
// Check for bots available to you.
pub(crate) async fn helpstart(ctx: Context<'_>) -> Result<(), Error> {
    let links = super::accountv2::get_link(ctx.author(), &ctx.data().sqlite_pool).await?;
    let mc_accounts = links
        .minecraft_accounts
        .into_iter()
        .map(|a| name(&ctx.data().caches, &ctx.data().clients.general, a.uuid))
        .collect::<Vec<_>>();
    let mc_accounts = futures::future::try_join_all(mc_accounts).await?;
    let bots = fetch_all(&ctx.data().clients.local_api_client).await?;
    let usable = bots
        .iter()
        .filter_map(|b| {
            if match b.list_type() {
                Whitelist => b.list().iter().any(|w| mc_accounts.contains(w)),
                Blacklist => mc_accounts.iter().any(|m| !b.list().contains(m)),
            } {
                Some(b.username().as_str())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let s: String = usable.join(", ");

    let reply = CreateReply::default().content(format!(
        "Accounts you can use: {s}\nTotal registered bots: {}",
        bots.len()
    ));
    ctx.send(reply).await?;
    Ok(())
}
