use crate::commands::zombies::gear::{
    *,
    ArmorMaterial::*,
    ArmorPiece::*,
    Weapon::*
};
use crate::commands::zombies::rounds::{Round, Wave};

#[derive(Debug)]
pub struct Family {
    family: *const str,
    damage: u8,
    health: u16,
    follow_range: u8,
    base_armor: u8,
}
#[derive(Debug)]
pub struct Zombie {
    family: Family,
    pub tier: u8,

    pub speed: f32,
    armor: Armor,
    weapon: Weapon,
}

impl Zombie {
    pub fn health(&self) -> u16 {
        self.family.health
    }
    pub fn damage(&self) -> Damage {
        self.family.damage+ self.weapon.damage()
    }
    pub fn follow_range(&self) -> u8 {
        self.family.follow_range
    }
    pub fn armor_value(&self) -> ArmorValue {
        let mut armor = self.family.base_armor;
        for piece in &self.armor {
            armor += piece.armor_value();
        }
        armor
    }
    pub fn family(&self) -> *const str {
        self.family.family
    }
}

#[derive(Debug)]
pub struct Horde {
    pub zombie: Zombie,
    pub count: u8,
}

pub const BB_BASIC: Family = Family {
    family: "basic",
    damage: 3,
    health: 22,
    follow_range: 35,
    base_armor: 2,
};
pub const BB_SLIME_ZOMBIE: Family = Family {
    family: "slime_zombie",
    damage: 3,
    health: 22,
    follow_range: 35,
    base_armor: 2,
};
pub const BB_SLIME: Family = Family {
    family: "slime",
    damage: 0,
    health: 4,
    follow_range: 16,
    base_armor: 2,
};

pub const BB_Z_1: Zombie = Zombie {
    family: BB_BASIC,
    tier: 1,

    speed: 0.25,
    armor: [
        NO_HELMET,
        NO_CHESTPLATE,
        NO_LEGGINGS,
        NO_BOOTS,
    ],
    weapon: NO_WEAPON,
};
pub const BB_Z_2: Zombie = Zombie {
    family: BB_BASIC,
    tier: 2,

    speed: 0.25,
    armor: [
        NO_HELMET,
        NO_CHESTPLATE,
        Leggings(2,false,Leather(0x000000)),
        NO_BOOTS,
    ],
    weapon: NO_WEAPON,
};
pub const BB_Z_3: Zombie = Zombie {
    family: BB_BASIC,
    tier: 3,

    speed: 0.25,
    armor: [
        NO_HELMET,
        NO_CHESTPLATE,
        Leggings(2,false,Leather(0x000000)),
        NO_BOOTS,
    ],
    weapon: WOODEN_AXE,
};

pub const BB_SZ_1: Zombie = Zombie {
    family: BB_SLIME_ZOMBIE,
    tier: 1,

    speed: 0.3,
    armor: [
        SLIME_HEAD,
        Chestplate(3,false,Leather(0x55FF55)),
        Leggings(2,false,Leather(0x55FF55)),
        Boots(1,false,Leather(0x55FF55)),
    ],
    weapon: SLIME_BALL,
};
pub const BB_S_1: Zombie = Zombie {
    family: BB_SLIME,
    tier: 1,

    speed: 0.36,
    armor: [
        NO_HELMET,
        NO_CHESTPLATE,
        NO_LEGGINGS,
        NO_BOOTS,
    ],
    weapon: NO_WEAPON,
};
