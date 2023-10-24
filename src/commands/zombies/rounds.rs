use crate::commands::zombies::zombies::*;

struct Wave {
    hordes: Vec<Horde>,
}
struct Round {
    waves: Vec<Wave>,
}

fn get_bb_r1() -> Vec<Wave> {
    vec![
        Wave {
            hordes: vec![
                Horde {
                    zombie: BB_Z_1,
                    count: 4,
                }
            ]
        },
        Wave {
            hordes: vec![
                Horde {
                    zombie: BB_Z_1,
                    count: 5,
                }
            ]
        }
    ]
}
pub(crate) fn get_bb_by_round(round: u8) {
    match round {
        1 => t(get_bb_r1()),
        _ => {}
    };
}
fn t(waves:Vec<Wave>) {
    for wave in waves {
        let hordes:Vec<Horde> = wave.hordes;
        for horde in hordes {
            println!("{:?} x {}", horde.zombie, horde.count);
        }
    }
}