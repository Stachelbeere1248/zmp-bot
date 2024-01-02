use crate::{Context, Error};
use crate::commands::{command_helper, helpstart};
use std::string::String;

#[poise::command(slash_command)]
pub(crate) async fn bots(
    ctx: Context<'_>,

    #[min = 0_u8]
    #[description = "default: 0"]
    bots: u8
) -> Result<(), Error> {
    helpstart::set_bots_available(bots).await;

    let reply = format!("{} bots are now registered as available", bots).to_string();
    command_helper::send(ctx, reply).await
}