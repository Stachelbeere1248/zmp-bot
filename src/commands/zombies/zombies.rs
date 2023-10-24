use crate::commands::zombies::gear::ArmorMaterial::Leather;
use crate::commands::zombies::gear::HelmetType::Head;
use crate::commands::zombies::gear::*;
use crate::{Context, Error};
use ArmorPiece::Leggings;

#[derive(Debug)]
pub struct Zombie {
    pub(crate) id: u16,
    pub(crate) damage: u8,
    pub(crate) health: u16,
    pub(crate) speed: f32,
    pub(crate) armor: [ArmorPiece; 4],
    pub(crate) weapon: Weapon,
    pub(crate) follow_range: u8,
}
#[derive(Debug)]
pub struct Horde {
    pub(crate) zombie: Zombie,
    pub(crate) count: u8,
}

pub const BB_Z_1: Zombie = Zombie {
    id: 1,
    damage: 3,
    health: 20,
    speed: 0.25,
    armor: [NO_HELMET, NO_CHESTPLATE, NO_LEGGINGS, NO_BOOTS],
    weapon: WOODEN_AXE,
    follow_range: 35,
};
pub const BB_S_1: Zombie = Zombie {
    id: 2,
    damage: 4,
    health: 25,
    speed: 0.25,
    armor: [
        ArmorPiece::Helmet(Head(3)),
        ArmorPiece::Chestplate(Leather(3), false),
        Leggings(Leather(3), false),
        ArmorPiece::Boots(Leather(3), false),
    ],
    weapon: SLIME_BALL,
    follow_range: 35,
};
