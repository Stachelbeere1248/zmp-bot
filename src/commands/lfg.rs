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
    let guild_id = ctx.guild_id().unwrap().get();
    let mut reply: CreateReply = CreateReply::default();
    reply = match cooldown(&ctx, 600, 300) {
        Ok(_) => {
            let current: u8 = current_players.unwrap_or(1);
            let mut desired: u8 = desired_players.unwrap_or(4);
            if current >= desired {
                desired = 4
            }
            let map_name: &str = map.name();
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
enum ExpertMap {
    #[name = "Dead End"]
    DeadEnd,
    #[name = "Bad Blood"]
    BadBlood,
    #[name = "Alien Arcadium"]
    AlienArcadium,
    //#[name = "Prison"]
    //Prison,
    #[name = "Speedrun"]
    Speedrun,
}
#[poise::command(slash_command, guild_only, rename = "expert-lfg")]
pub(crate) async fn expert(
    ctx: Context<'_>,

    #[rename = "map"] mode: ExpertMap,

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
                desired = 4;
            };
            let (ping, allowed_roles): (u64, Vec<u64>) = match mode {
                ExpertMap::Speedrun => (
                    1295322375637958716,
                    ROLE_LIST.iter().skip(2).map(|tier| [tier[4], tier[5]]).flatten().collect()
                ),
                ExpertMap::DeadEnd => (
                    1295321319344177172,
                    ROLE_LIST.iter().skip(2).map(|tier| [tier[1], tier[5]]).flatten().collect()
                ),
                ExpertMap::BadBlood => (
                    1295322259631640607,
                    ROLE_LIST.iter().skip(2).map(|tier| [tier[2], tier[5]]).flatten().collect()
                ),
                ExpertMap::AlienArcadium => (
                    1295322327910842441,
                    ROLE_LIST.iter().skip(2).map(|tier| [tier[3], tier[5]]).flatten().collect()
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

const ROLE_LIST: [[u64;6]; 9] = [ // [[basic, de, bb, aa, sr, star]; 9]
    [1256229103678259311, 1256229192744304670, 1256229223450935377, 1256229498899271754, 1256229540900900996, 1256229575269154866], //novice
    [1256230831131983932, 1256230750827577447, 1256230776828334143, 1256230793630715975, 1256230818444214333, 1256230734642024468], //seasoned
    [1256230723455553556, 1256230653083521045, 1256230666786443310, 1256230686214324255, 1256230704061353995, 1256230636721537097], //expert
    [1256230625635995718, 1256230573203128370, 1256230582908747776, 1256230600025706506, 1256230610998005872, 1256230557897986068], //pro
    [1256230543532626002, 1256230480823582861, 1256230502273126421, 1256230515355160597, 1256230531478065243, 1256230463241191494], //master
    [1256230442907074703, 1256230359419588700, 1256230396719403141, 1256230416516649012, 1256230429212545025, 1256230346848997396], //grandmaster
    [1256230332169060362, 1256230266889044020, 1256230288888168458, 1256230416516649012, 1256230316528631870, 1256230242943766651], //legend
    [1256230231732387950, 1256230157967163495, 1256230181199151254, 1256230194499420223, 1256230207099244646, 1256230102627258449], //divine
    [1256230002597302322, 1256229873064869939, 1256229929247440906, 1256229963166646314, 1256229982569627792, 1256229672598110218], //goat
];
