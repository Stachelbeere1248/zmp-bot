use poise::{ChoiceParameter, CreateReply};
use serenity::all::{CreateAllowedMentions, RoleId};

//from main.rs
use crate::commands::command_helper::cooldown;
use crate::{Context, Error};
//
use crate::commands::lfg::Difficulty::Normal;
use crate::commands::lfg::Map::*;
use crate::commands::lfg::Mode::*;

#[derive(Debug, poise::ChoiceParameter, PartialEq)]
pub enum Map {
    #[name = "Dead End"]
    DeadEnd,
    #[name = "Bad Blood"]
    BadBlood,
    #[name = "Alien Arcadium"]
    AlienArcadium,
    #[name = "Prison"]
    Prison,
}
#[derive(Debug, poise::ChoiceParameter)]
pub enum Mode {
    #[name = "Casual"]
    Casual,
    #[name = "Speedrun"]
    Speedrun,
    #[name = "Challenge"]
    Challenge,
    #[name = "Challenge of the week"]
    Event,
    //#[name = "Tournament Practice"]
    //Tournament,
}
#[derive(Debug, poise::ChoiceParameter)]
pub enum Difficulty {
    #[name = "Normal"]
    Normal,
    #[name = "Hard"]
    Hard,
    #[name = "R.I.P."]
    Rip,
}
#[poise::command(slash_command, guild_only)]
pub(crate) async fn lfg(
    ctx: Context<'_>,

    #[rename = "map"] map: Map,

    #[description = "Normal"]
    #[rename = "difficulty"]
    difficulty: Option<Difficulty>,

    #[rename = "mode"]
    #[description = "play-style"]
    mode: Mode,

    #[min = 1_u8]
    #[max = 3_u8]
    #[description = "default: 1"]
    #[rename = "current"]
    current_players: Option<u8>,

    #[min = 2_u8]
    #[max = 4_u8]
    #[description = "default: 4"]
    #[rename = "desired"]
    desired_players: Option<u8>,

    #[description = "optional extra message"]
    #[rename = "message"]
    note: Option<String>,
) -> Result<(), Error> {
    let mut reply: CreateReply = CreateReply::default();
    let guild_id = ctx.guild_id().unwrap().get();
    reply = match cooldown(&ctx, 600, 300) {
        Ok(_) => {
            let current: u8 = current_players.unwrap_or(1);
            let mut desired: u8 = desired_players.unwrap_or(4);
            if current >= desired {
                desired = 4
            }
            let map_name: &str = map.name();
            let old_ping: u64 = match mode {
                Casual => match map {
                    DeadEnd => 1005837123921915914,
                    BadBlood => 1140190470698438666,
                    AlienArcadium => 1105917281898336356,
                    Prison => 1253440747454333009,
                },
                Speedrun => 1005836989595144243,
                Challenge => 1005836864680361994,
                Event => 1175116511095050331,
                //Tournament => 1210508966036242445,
            };
            let new_ping: u64 = match mode {
                Casual => match map {
                    DeadEnd => 1257408106783178752,
                    BadBlood => 1257408198541836459,
                    AlienArcadium => 1257408233757343815,
                    Prison => 1257408303835644029,
                },
                Speedrun => 1257408362367287367,
                Challenge => 1257408398631370762,
                Event => 1257408432063905915,
                //Tournament => 1210508966036242445,
            };
            let ping = match guild_id {
                1256217633959841853 => new_ping,
                995300932164276234 => old_ping,
                _ => 0,
            };
            let difficulty: Difficulty = match map {
                DeadEnd | BadBlood | Prison => difficulty.unwrap_or(Normal),
                AlienArcadium => Normal,
            };

            let mut reply_content: String = format!("<@&{ping}> {current}/{desired} {map_name}",);
            match difficulty {
                Normal => {}
                Difficulty::Hard | Difficulty::Rip => {
                    reply_content.push(' ');
                    reply_content.push_str(difficulty.name());
                }
            }
            match note {
                None => {}
                Some(note) => {
                    reply_content.push_str(format!("\nNote: {note}").as_str());
                }
            }
            reply
                .content(reply_content)
                .ephemeral(false)
                .allowed_mentions(CreateAllowedMentions::new().roles(vec![ping]))
        }
        Err(why) => reply.content(why.to_string()).ephemeral(true),
    };

    if let Err(why) = ctx.send(reply).await {
        println!("Error sending message: {why}");
    }
    Ok(())
}
#[derive(Debug, poise::ChoiceParameter)]
enum Expert {
    #[name = "Dead End"]
    DeadEnd,
    #[name = "Bad Blood"]
    BadBlood,
    #[name = "Alien Arcadium"]
    AlienArcadium,
    #[name = "Speedrun"]
    Speedrun,
}
#[poise::command(slash_command, guild_only, rename = "expert-lfg")]
pub(crate) async fn expert(
    ctx: Context<'_>,

    #[rename = "map"] mode: Expert,

    #[min = 1_u8]
    #[max = 3_u8]
    #[description = "default: 1"]
    #[rename = "current"]
    current_players: Option<u8>,

    #[min = 2_u8]
    #[max = 4_u8]
    #[description = "default: 4"]
    #[rename = "desired"]
    desired_players: Option<u8>,

    #[description = "extra message"]
    #[rename = "message"]
    note: String,
) -> Result<(), Error> {
    let mut reply: CreateReply = CreateReply::default();

    reply = match cooldown(&ctx, 600, 300) {
        Ok(_) => {
            let current: u8 = current_players.unwrap_or(1);
            let mut desired: u8 = desired_players.unwrap_or(4);
            if current >= desired {
                desired = 4
            }
            //TODO ZMPv2
            let (ping, allowed_roles): (u64, Vec<u64>) = match mode {
                Expert::Speedrun => (
                    1243676538067488828,
                    vec![
                        1235975301461184513,
                        1235975620400386172,
                        1235975954413654167,
                        1235976165345329162,
                        1235976366109753404,
                        1235967416605872179,
                        1235967676048740434,
                        1235968713354907648,
                        1235968958511841351,
                        1235969159679180860,
                        1236226368052658217,
                        1173396223592513647,
                    ],
                ),
                Expert::DeadEnd => (
                    1243675610895880366,
                    vec![
                        1235975562498015354,
                        1235975873187020900,
                        1235976093517746227,
                        1235976301702156359,
                        1235976469683896441,
                        1235967416605872179,
                        1235967676048740434,
                        1235968713354907648,
                        1235968958511841351,
                        1235969159679180860,
                        1236226368052658217,
                        1173396223592513647,
                    ],
                ),
                Expert::BadBlood => (
                    1243676387634708491,
                    vec![
                        1235975518529261599,
                        1235975747219357706,
                        1235976055769268274,
                        1235976257414631464,
                        1235976434724376728,
                        1235967416605872179,
                        1235967676048740434,
                        1235968713354907648,
                        1235968958511841351,
                        1235969159679180860,
                        1236226368052658217,
                        1173396223592513647,
                    ],
                ),
                Expert::AlienArcadium => (
                    1243676465829249116,
                    vec![
                        1235975471968157851,
                        1235975697487364237,
                        1235975991617130567,
                        1235976216469835926,
                        1235976398380863549,
                        1235967416605872179,
                        1235967676048740434,
                        1235968713354907648,
                        1235968958511841351,
                        1235969159679180860,
                        1236226368052658217,
                        1173396223592513647,
                    ],
                ),
            };
            let is_expert: bool = ctx
                .author_member()
                .await
                .unwrap()
                .roles
                .iter()
                .any(|user_role: &RoleId| allowed_roles.contains(&user_role.get()));
            let reply_content: String = format!("{current}/{desired} <@&{ping}>: {note}");
            match is_expert {
                true => reply
                    .content(reply_content)
                    .ephemeral(false)
                    .allowed_mentions(CreateAllowedMentions::new().roles(vec![ping])),
                false => reply
                    .content("You do not have any of the required expert ranks.")
                    .ephemeral(true),
            }
        }
        Err(why) => reply.content(why.to_string()).ephemeral(true),
    };

    if let Err(why) = ctx.send(reply).await {
        println!("Error sending message: {why}");
    }
    Ok(())
}
