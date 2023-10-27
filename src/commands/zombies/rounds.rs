use crate::commands::zombies::zombies::*;

pub type Wave = Vec<Horde>;
pub type Round = Vec<Wave>;

pub(crate) struct BadBlood;

impl BadBlood {
    fn round1() -> Round {
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
    fn round2() -> Round {
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
    fn round3() -> Round {
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
    fn round4() -> Round {
        vec![
            //wave 1
            vec![
                Horde {
                    zombie: BB_Z_2,
                    count: 4,
                },
                Horde {
                    zombie: BB_SZ_1,
                    count: 2,
                },
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
            ],
            //wave 3
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
    /*fn round5() -> Round {

    }*/
    pub(crate) fn get_round(round: u8) -> Round {
        match round {
            1 => Self::round1(),
            2 => Self::round2(),
            3 => Self::round3(),
            4 => Self::round4(),
            _ => panic!("Round {} not found", round)
        }
    }
}