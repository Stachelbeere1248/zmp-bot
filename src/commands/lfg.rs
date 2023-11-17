//
use crate::commands::lfg::Difficulty::Normal;
use crate::commands::lfg::Map::*;
use crate::commands::lfg::Mode::*;
//from main.rs
use crate::{Context, Error};
//
use serenity::model::id::RoleId;
use serenity::model::mention::Mention;
use serenity::model::mention::Mention::Role;
use crate::commands::command_helper;

#[derive(Debug, poise::ChoiceParameter, PartialEq)]
pub enum Map {
    #[name = "Dead End"]
    DeadEnd,
    #[name = "Bad Blood"]
    BadBlood,
    #[name = "Alien Arcadium"]
    AlienArcadium,
}
#[derive(Debug, poise::ChoiceParameter)]
pub enum Mode {
    #[name = "Casual"]
    Casual,
    #[name = "Speedrun"]
    Speedrun,
    #[name = "Challenge"]
    Challenge,
    #[name = "Event/Challenge"]
    Event,
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
#[poise::command(slash_command)]
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
    if let Some(value) = command_helper::cooldown(
        &ctx,
        600,
        300
    ) {
        return value;
    }



    let current = current_players.unwrap_or(1);
    let mut desired = desired_players.unwrap_or(4);
    if current >= desired { desired = 4 }
    let map_name: &str = map.name();
    let ping: Mention;
    match mode {
        Casual => match map {
            DeadEnd => ping = Role(RoleId(1005837123921915914)),
            BadBlood => ping = Role(RoleId(1140190470698438666)),
            AlienArcadium => ping = Role(RoleId(1105917281898336356)),
        },
        Speedrun => ping = Role(RoleId(1005836989595144243)),
        Challenge => ping = Role(RoleId(1005836864680361994)),
        Event => ping = Role(RoleId(1175116511095050331))
    }
    let diff_name: &str = if map != AlienArcadium {
        difficulty.unwrap_or(Normal).name()
    } else {
        Normal.name()
    };



    let mut reply = format!(
        "{c}/{d} {e} {f} {b}",
        b = ping,
        c = current,
        d = desired,
        e = map_name,
        f = diff_name
    );

    if note.is_some() {
        let t = note.unwrap();
        let regex = regex::Regex::new("(<@&?[0-9]*>)|(@everyone|@here)").unwrap();
        if regex.is_match(&t) {
            reply = String::from("Your Note seems to match a ping <:Maark:1128577127931985950>");
        } else {
            reply.push_str(format!("\nNote: {}", t).as_str());
        }
    }


    command_helper::send(ctx, reply).await
}
