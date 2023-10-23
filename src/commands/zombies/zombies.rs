use crate::commands::zombies::gear::*;
use crate::{Context, Error};
#[derive(Debug)]
pub struct Zombie {
    id: u16,
    damage: u8,
    health: u16,
    speed: f32,
    armor: [ArmorPiece;4],
    weapon: Weapon,
    follow_range: u8
}
#[derive(Debug)]
pub struct Horde {
    pub zombie: Zombie,
    pub count: u8
}

pub const BB_Z_1: Zombie = Zombie {
    id: 1,
    damage: 3,
    health: 20,
    speed: 0.25,
    armor: [NO_HELMET,NO_CHESTPLATE,LEATHER_LEGGINGS,NO_BOOTS],
    weapon: WOODEN_AXE,
    follow_range: 35
};