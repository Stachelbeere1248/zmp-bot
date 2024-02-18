use crate::discord::commands::command_helper;
use crate::discord::{Context, Error};

static mut BOTS_AVAILABLE: u8 = 0;

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
    let bots = unsafe { BOTS_AVAILABLE };

    let reply = if bots >= needed_players {
        "Bots available. Please use <@424767825001971715> in the bot-commands channel instead."
            .to_string()
    } else {
        if let Some(value) = command_helper::cooldown(&ctx, 1200, 600) {
            return value;
        };
        format!("<@&1008075054971621448>\nneed: {}", needed_players - bots)
    };

    command_helper::send(ctx, reply).await
}
pub async fn set_bots_available(bots: u8) {
    unsafe {
        BOTS_AVAILABLE = bots;
    }
}
