use crate::Context;
use crate::error::Error;

const XD: &str = "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⡿⠿⠿⠿⠿⠿⠿⠿⢿⣿⣿⣿⣿⡿⠿⠿⠿⠿⠿⠿⠿⢿⡿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n\
                  ⣿⣿⣿⣧⣄⡀⠀⠀⠀⢀⣠⣼⣿⣿⣿⣿⣧⣄⡀⠀⠀⠀⣀⣤⣼⣷⣦⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⢿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣦⠀⠀⠀⠻⣿⣿⣿⣿⣿⣿⠟⠀⠀⢀⣼⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⣶⣦⣄⡀⠀⠀⠙⢿⣿⣿⣿⣿\n\
                  ⣿⣿⣿⣿⣿⣿⣿⣷⡀⠀⠀⠙⣿⣿⣿⣿⠋⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣄⠀⠀⠈⢿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣄⠀⠀⠈⢿⡿⠁⠀⠀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆⠀⠀⢸⣿⣿⣿\n\
                  ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡀⠀⠀⠀⠀⢠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡗⠀⠀⠀⠐⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠀⠀⠀⣿⣿⣿\n\
                  ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠏⠀⠀⣠⡀⠀⠀⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⡿⠃⠀⠀⣰⣿⣷⡄⠀⠀⠘⢿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠀⠀⠀⣾⣿⣿⣿\n\
                  ⣿⣿⣿⣿⣿⣿⣿⡟⠁⠀⢀⣼⣿⣿⣿⣿⣆⠀⠀⠈⠻⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁⠀⢀⣼⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⠋⠀⠀⢠⣾⣿⣿⣿⣿⣿⣿⣷⡀⠀⠀⠙⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⠿⠛⠁⠀⠀⣠⣾⣿⣿⣿⣿⣿\n\
                  ⣿⣿⣿⠿⠛⠁⠀⠀⠀⠙⠻⣿⣿⣿⣿⣿⡿⠟⠋⠀⠀⠀⠈⠛⠻⡿⠟⠛⠁⠀⠀⠈⠉⠉⠉⠉⠀⠀⠀⠀⣀⣴⣾⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣶⣶⣶⣶⣶⣶⣶⣶⣿⣿⣿⣿⣿⣷⣶⣶⣶⣶⣶⣶⣶⣶⣷⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n\
                  ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n";

#[poise::command(
    slash_command,
    owners_only,
    install_context = "User|Guild",
    interaction_context = "Guild|BotDm|PrivateChannel",
    ephemeral = "false",
)]
/// Useless command to check if the bot is online.
pub(crate) async fn xd(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say(XD).await?;
    Ok(())
}
