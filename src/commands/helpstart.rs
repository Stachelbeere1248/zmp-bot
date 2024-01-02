use crate::{Context, Error};
use crate::commands::command_helper;

static mut BOTS_AVAILABLE: u8 = 0;

#[poise::command(slash_command,guild_only)]
pub(crate) async fn helpstart(
    ctx: Context<'_>,

    #[min = 1_u8]
    #[max = 3_u8]
    #[description = "default: 3"]
    #[rename = "needed"]
    needed_players: u8
) -> Result<(), Error> {
    let bots = unsafe { BOTS_AVAILABLE };

    let reply= if bots >= needed_players {
        "Bots available. Please use the Zombies Helpstart v2 Bot in the bot-commands channel instead.".to_string()
    } else {
        if let Some(value) = command_helper::cooldown(
            &ctx,
            1200,
            600
        ) {
            return value;
        };
        format!(
            "<@&1008075054971621448>\nneed: {}"
            , needed_players - bots
        )
    };

    command_helper::send(ctx, reply).await
}
pub async fn set_bots_available(bots: u8) {
    unsafe {
        BOTS_AVAILABLE = bots;
    }
}