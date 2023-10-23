use std::convert::Into;
use crate::{Context, Error};
use crate::commands::zombies::zombies::*;

#[poise::command(slash_command)]
pub(crate) async fn round(
    ctx: Context<'_>
) -> Result<(), Error> {
    Ok(())
}