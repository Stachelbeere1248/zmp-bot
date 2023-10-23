use crate::commands::zombies::rounds::Round::*;
use crate::commands::zombies::rounds::Wave::*;
use crate::commands::zombies::zombies::*;
/*type Wave = Vec<Horde>;
type Round = Vec<Wave>;

fn bad_blood_round1() -> Round {
    let mut round:Round = Vec::with_capacity(2);
    let mut wave1:Wave = Vec::with_capacity(2);
    let mut wave2:Wave = Vec::with_capacity(1);
    wave1.push(Horde {
        zombie: BB_Z_1,
        count: 4
    });
    wave2.push(Horde {
        zombie: BB_Z_1,
        count: 5
    });
    round.push(wave1);
    round.push(wave2);
    round
}*/

pub enum Wave {
    Wave1horde([Horde;1]),
    Wave2hordes([Horde;2]),
    Wave3hordes([Horde;3]),
    Wave4hordes([Horde;4]),
    Wave5hordes([Horde;5]),
    Wave6hordes([Horde;6]),
    Wave7hordes([Horde;7])
}
pub enum Round {
    Round2Waves([Wave;2]),
    Round3Waves([Wave;3]),
    Round4Waves([Wave;4]),
    Round5Waves([Wave;5]),
    Round6Waves([Wave;6]),
    Round7Waves([Wave;7])
}
const DE:[Round;30] = [
    Round2Waves([
        Wave1horde([
            Horde { zombie: BB_Z_1, count: 4 }
        ]),
        Wave1horde([
            Horde { zombie: BB_Z_1, count: 5 }
        ])
    ]);30
];
/*fn t() {
    DE.get(2)
}*/