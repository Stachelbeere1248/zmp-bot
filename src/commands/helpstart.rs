use poise::CreateReply;
use serenity::all::CreateAllowedMentions;

use crate::commands::command_helper;
use crate::{Context, Error};

#[poise::command(slash_command, guild_only)]
pub(crate) async fn helpstart(
    ctx: Context<'_>,
    #[min = 1_u8]
    #[max = 3_u8]
    #[description = "amount of players in your party, DO NOT include bots"]
    #[rename = "current"]
    current_players: u8,
) -> Result<(), Error> {
    let needed_players = 4 - current_players;
    let bots = *ctx.data().bots.read().await;

    let mut reply = CreateReply::default();

    reply = if bots >= needed_players {
        reply.content("Bots available. Please use <@424767825001971715> in the bot-commands channel instead.")
            .ephemeral(true)
    } else {
        match command_helper::cooldown(&ctx, 1200, 600) {
            Ok(_) => reply
                .content(format!(
                    "<@&1008075054971621448>\nneed: {}",
                    needed_players - bots
                ))
                .ephemeral(false)
                .allowed_mentions(CreateAllowedMentions::new().roles(vec![1008075054971621448])),
            Err(why) => reply.content(why.to_string()).ephemeral(true),
        }
    };
    if let Err(why) = ctx.send(reply).await {
        println!("Error sending message: {why}")
    }
    Ok(())
}
