use crate::Context;
use crate::error::Error;

const XD: &str = "⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⡿⠿⠿⠿⠿⠿⠿⠿⢿⣿⣿⣿⣿⡿⠿⠿⠿⠿⠿⠿⠿⢿⡿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣧⣄⡀⠀⠀⠀⢀⣠⣼⣿⣿⣿⣿⣧⣄⡀⠀⠀⠀⣀⣤⣼⣷⣦⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⢿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣦⠀⠀⠀⠻⣿⣿⣿⣿⣿⣿⠟⠀⠀⢀⣼⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⣶⣦⣄⡀⠀⠀⠙⢿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣷⡀⠀⠀⠙⣿⣿⣿⣿⠋⠀⠀⣠⣾⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣄⠀⠀⠈⢿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣄⠀⠀⠈⢿⡿⠁⠀⠀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡆⠀⠀⢸⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡀⠀⠀⠀⠀⢠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡗⠀⠀⠀⠐⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠀⠀⠀⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠏⠀⠀⣠⡀⠀⠀⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⡿⠃⠀⠀⣰⣿⣷⡄⠀⠀⠘⢿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠀⠀⠀⣾⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⡟⠁⠀⢀⣼⣿⣿⣿⣿⣆⠀⠀⠈⠻⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠁⠀⢀⣼⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⠋⠀⠀⢠⣾⣿⣿⣿⣿⣿⣿⣷⡀⠀⠀⠙⣿⣿⣿⣿⣿⣿⡇⠀⠀⢸⣿⣿⣿⣿⣿⣿⠿⠛⠁⠀⠀⣠⣾⣿⣿⣿⣿⣿\n⣿⣿⣿⠿⠛⠁⠀⠀⠀⠙⠻⣿⣿⣿⣿⣿⡿⠟⠋⠀⠀⠀⠈⠛⠻⡿⠟⠛⠁⠀⠀⠈⠉⠉⠉⠉⠀⠀⠀⠀⣀⣴⣾⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣶⣶⣶⣶⣶⣶⣶⣶⣿⣿⣿⣿⣿⣷⣶⣶⣶⣶⣶⣶⣶⣶⣷⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿\n";

#[poise::command(slash_command)]
pub(crate) async fn xd(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say(XD).await?;
    Ok(())
}
