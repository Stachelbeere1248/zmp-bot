use crate::{Context, Error};
use crate::commands::{
    lfg::{
        *,
        Map::{*}
    },
    zombies::{
        rounds
    }
};

#[poise::command(slash_command)]
pub(crate) async fn round(
    ctx: Context<'_>,
    map: Map,

    #[min = 1_u8]
    #[max = 105_u8]
    round: u8
) -> Result<(), Error> {
    match map {
        DeadEnd => {}
        BadBlood => {rounds::BadBlood::get_round(round);}
        AlienArcadium => {}
    }
    ctx.say(format!("hi")).await?;
    Ok(())
}
