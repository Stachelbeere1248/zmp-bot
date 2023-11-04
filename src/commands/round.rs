use crate::{Context, Error};
use crate::commands::{
    lfg::{
        *,
        Map::{*}
    },
    zombies::rounds
};
use crate::commands::zombies::rounds::get_round_string;

#[poise::command(slash_command)]
pub(crate) async fn round(
    ctx: Context<'_>,
    map: Map,

    #[min = 1_u8]
    #[max = 105_u8]
    round: u8
) -> Result<(), Error> {

    let t = get_round_string(map,round);

    ctx.say(t).await?;
    Ok(())
}
