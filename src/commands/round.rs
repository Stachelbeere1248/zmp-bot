use crate::commands::zombies::zombies::*;
use crate::{Context, Error};
use std::convert::Into;

#[poise::command(slash_command)]
pub(crate) async fn round(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
