use crate::{Context, Error};
use crate::commands::command_helper;

#[poise::command(slash_command)]
pub(crate) async fn helpstart(
    ctx: Context<'_>,

    #[min = 1_u8]
    #[max = 3_u8]
    #[description = "default: 3"]
    #[rename = "needed"]
    needed_players: u8
) -> Result<(), Error> {
    if let Some(value) = command_helper::cooldown(
        &ctx,
        1200,
        600
    ) {
        return value;
    }
    let reply = format!(
        "<@&1008075054971621448>\nneed: {}"
        ,needed_players
    );

    command_helper::send(ctx, reply).await
}