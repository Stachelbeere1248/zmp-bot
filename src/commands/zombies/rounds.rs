use std::fmt::format;
use crate::commands::lfg::Map;
use crate::commands::zombies::zombies::*;

type Wave<'a> = Vec<Horde<'a>>;
pub(crate) type Round<'a> = Vec<Wave<'a>>;

struct BadBlood;

impl BadBlood {
    fn round1<'a>() -> Round<'a> {
        vec![
            //wave 1
            vec![
                Horde {
                    zombie: BB_Z_1,
                    count: 4,
                }
            ],
            //wave 2
            vec![
                Horde {
                    zombie: BB_Z_1,
                    count: 5,
                }
            ]
        ]
    }
    fn round2<'a>() -> Round<'a> {
        vec![
            //wave 1
            vec![
                Horde {
                    zombie: BB_Z_1,
                    count: 4,
                },
                Horde {
                    zombie: BB_SZ_1,
                    count: 1,
                }
            ],
            //wave 2
            vec![
                Horde {
                    zombie: BB_Z_2,
                    count: 4,
                },
                Horde {
                    zombie: BB_SZ_1,
                    count: 2,
                }
            ]
        ]
    }
    fn round3<'a>() -> Round<'a> {
        vec![
            //wave 1
            vec![
                Horde {
                    zombie: BB_Z_2,
                    count: 4,
                },
                Horde {
                    zombie: BB_SZ_1,
                    count: 3,
                },
                Horde {
                    zombie: BB_S_1,
                    count: 2,
                }
            ],
            //wave 2
            vec![
                Horde {
                    zombie: BB_Z_2,
                    count: 3,
                },
                Horde {
                    zombie: BB_SZ_1,
                    count: 2,
                },
                Horde {
                    zombie: BB_S_1,
                    count: 2,
                }
            ]
        ]
    }
    fn round4<'a>() -> Round<'a> {
        vec![
            //wave 1
            vec![
                Horde {
                    zombie: BB_Z_3,
                    count: 4,
                },
                Horde {
                    zombie: BB_SZ_1,
                    count: 2,
                }
            ],
            //wave 2
            vec![
                Horde {
                    zombie: BB_Z_3,
                    count: 3,
                },
                Horde {
                    zombie: BB_SZ_1,
                    count: 1,
                },
                Horde {
                    zombie: BB_WI_1,
                    count: 2,
                }
            ],
            //wave 3
            vec![
                Horde {
                    zombie: BB_Z_3,
                    count: 3,
                },
                Horde {
                    zombie: BB_SZ_1,
                    count: 1,
                },
                Horde {
                    zombie: BB_S_1,
                    count: 2,
                }
            ]
        ]
    }
    fn round5<'a>() -> Round<'a> {
        vec![
            //wave 1
            vec![
                Horde {
                    zombie: BB_WO_1,
                    count: 3,
                },
                Horde {
                    zombie: BB_WW_1,
                    count: 1,
                }
            ],
            //wave 2
            vec![
                Horde {
                    zombie: BB_WO_1,
                    count: 3,
                },
                Horde {
                    zombie: BB_WW_1,
                    count: 1,
                }
            ],
            //wave 3
            vec![
                Horde {
                    zombie: BB_WO_1,
                    count: 2,
                },
                Horde {
                    zombie: BB_WW_1,
                    count: 1,
                },
                Horde {
                    zombie: BB_LILY,
                    count: 1,
                },
                Horde {
                    zombie: BB_ELLIE,
                    count: 1,
                }
            ]
        ]
    }
    fn round6<'a>() -> Round<'a> {
        vec![
            //wave 1
            vec![
                Horde {
                    zombie: BB_Z_4,
                    count: 4,
                },
                Horde {
                    zombie: BB_WI_1,
                    count: 3,
                }
            ],
            //wave 2
            vec![
                Horde {
                    zombie: BB_Z_4,
                    count: 3,
                },
                Horde {
                    zombie: BB_WI_1,
                    count: 2,
                }
            ],
            //wave 3
            vec![
                Horde {
                    zombie: BB_Z_4,
                    count: 3,
                },
                Horde {
                    zombie: BB_WI_1,
                    count: 2,
                },
            ]
        ]
    }
    fn round7<'a>() -> Round<'a> {
        vec![
            //wave 1
            vec![
                Horde {
                    zombie: BB_S_1,
                    count: 3,
                },
                Horde {
                    zombie: BB_WI_1,
                    count: 3,
                }
            ],
            //wave 2
            vec![
                Horde {
                    zombie: BB_S_1,
                    count: 3,
                },
                Horde {
                    zombie: BB_WI_1,
                    count: 3,
                }
            ],
            //wave 3
            vec![
                Horde {
                    zombie: BB_S_1,
                    count: 3,
                },
                Horde {
                    zombie: BB_WI_1,
                    count: 3,
                },
            ]
        ]
    }
    fn round8<'a>() -> Round<'a> {
        vec![
            //wave 1
            vec![
                Horde {
                    zombie: BB_Z_5,
                    count: 4,
                },
                Horde {
                    zombie: BB_P_1,
                    count: 2,
                }
            ],
            //wave 2
            vec![
                Horde {
                    zombie: BB_Z_5,
                    count: 4,
                },
                Horde {
                    zombie: BB_P_1,
                    count: 2,
                }
            ],
            //wave 3
            vec![
                Horde {
                    zombie: BB_Z_5,
                    count: 4,
                },
                Horde {
                    zombie: BB_P_1,
                    count: 1,
                },
            ]
        ]
    }
    fn round9<'a>() -> Round<'a> {
        vec![
            //wave 1
            vec![
                Horde {
                    zombie: BB_SZ_1,
                    count: 3,
                },
                Horde {
                    zombie: BB_S_1,
                    count: 5,
                }
            ],
            //wave 2
            vec![
                Horde {
                    zombie: BB_SZ_1,
                    count: 3,
                },
                Horde {
                    zombie: BB_S_1,
                    count: 4,
                }
            ],
            //wave 3
            vec![
                Horde {
                    zombie: BB_SZ_1,
                    count: 3,
                },
                Horde {
                    zombie: BB_S_1,
                    count: 4,
                },
            ]
        ]
    }
    fn get_round<'b>(round: u8) -> Option<Round<'b>> {
        match round {
            1 => Some(Self::round1()),
            2 => Some(Self::round2()),
            3 => Some(Self::round3()),
            4 => Some(Self::round4()),
            5 => Some(Self::round5()),
            6 => Some(Self::round6()),
            7 => Some(Self::round7()),
            8 => Some(Self::round8()),
            9 => Some(Self::round9()),
            _ => None
        }
    }
}

pub(crate) fn get_round<'a>(
    map: &Map,
    round: u8,
) -> Option<Round<'a>> {
    match map {
        Map::DeadEnd => BadBlood::get_round(round),
        Map::BadBlood => BadBlood::get_round(round),
        Map::AlienArcadium => BadBlood::get_round(round)
    }
}

pub(crate) fn get_round_string (
    map: Map,
    r: u8) -> String {
    let round = get_round(&map, r);
    if round.is_some() {
        let mut string = String::new();
        let mut wave_index:u8 = 0;
        for wave in round.unwrap() {
            wave_index += 1;

            string.push_str(format!("# Wave {}\n", wave_index).as_str());
            for horde in wave {
                string.push_str(format!(
                    "### {}x {} {}\n",
                    horde.count,
                    horde.zombie.family(),
                    horde.zombie.tier
                ).as_str());
                let mut armor_string = String::new();
                let armor = *horde.zombie.armor();
                let (x,y,z) = armor.get(0).unwrap();
                let (x,_,z) = armor.get(1).unwrap();
                let (x,_,z) = armor.get(2).unwrap();
                let (x,_,z) = armor.get(3).unwrap();
                armor_string.push_str("");


                string.push_str(format!(
                    "- Health: {}\n- Damage: {}\n- Armor: {} {:?}\n- Follow Range: {}\n- Speed: {}\n",
                    horde.zombie.health(),
                    horde.zombie.damage().1,
                    horde.zombie.armor_value(),
                    horde.zombie.armor(),
                    horde.zombie.follow_range(),
                    horde.zombie.speed,
                ).as_str());
            }
        }
        string
    } else {
        String::from(format!("There is no Round {} on the Map {}", r, &map))
    }

}














