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
    fn round5() -> Round {
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
    fn round6() -> Round {
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
    fn round7() -> Round {
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
    /*fn round8() -> Round {

    }*/
    pub(crate) fn get_round(round: u8) -> Option<Round> {
        match round {
            1 => Some(Self::round1()),
            2 => Some(Self::round2()),
            3 => Some(Self::round3()),
            4 => Some(Self::round4()),
            5 => Some(Self::round5()),
            6 => Some(Self::round6()),
            7 => Some(Self::round7()),
            _ => None
        }
    }
}