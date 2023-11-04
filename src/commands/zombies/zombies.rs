use crate::commands::zombies::gear::{
    *,
    ArmorMaterial::*,
    ArmorPiece::*,
    Weapon::*
};

type ChildrenCount = u8;
type RespawningChildren = bool;
#[derive(Debug)]
pub struct Family<'a> {
    name: &'a str,
    damage: u8,
    health: u16,
    follow_range: u8,
    base_armor: u8,
}
#[derive(Debug)]
pub struct Zombie<'a> {
    family: &'a Family<'a>,
    pub tier: u8,

    pub speed: f32,
    armor: &'a Armor,
    weapon: &'a Weapon,
    children: Children<'a>,
    damage_type: DamageType,
}
#[derive(Debug)]
pub enum Children<'a> {
    None,
    Single(&'a Horde<'a>, RespawningChildren),
}
#[derive(Debug)]
pub enum Ability {
    Single(*const str),
    Double(*const str, *const str),
}
#[derive(Debug)]
pub enum DamageType {
    Melee,
    Ranged,
    Ability(Ability),
    MeleeRanged,
    MeleeAbility(Ability),
    RangedAbility(Ability),
}
impl Zombie<'_> {
    pub fn health(&self) -> u16 {
        self.family.health
    }
    pub fn damage(&self) -> (&DamageType, Damage) {
        (&self.damage_type, self.family.damage + self.weapon.damage())
    }
    pub fn follow_range(&self) -> u8 {
        self.family.follow_range
    }
    pub fn armor_value(&self) -> (ArmorValue) {
        let mut armor = self.family.base_armor;
        for piece in self.armor {
            armor += piece.armor_value();
        }
        (armor)
    }
    pub fn armor(&self) -> [(&ArmorMaterial, &SkinId, &LeatherColor);4] {
        let mut t:[(&ArmorMaterial, &SkinId, &LeatherColor);4] = [(&ArmorMaterial::None, &0, &0);4];
        let mut index = 0;
        for piece in self.armor {
            t[index] = piece.info();
            index +=1;
        }
        t
    }
    pub fn speed(&self) -> f32 {
        self.speed
    }
    pub fn family(&self) -> &str {
        self.family.name
    }
}

#[derive(Debug)]
pub struct Horde<'a> {
    pub zombie: Zombie<'a>,
    pub count: u8,
}

pub const BB_BASIC: Family = Family {
    name: "basic",
    damage: 3,
    health: 22,
    follow_range: 35,
    base_armor: 2,
};
pub const BB_SLIME_ZOMBIE: Family = Family {
    name: "slime_zombie",
    damage: 3,
    health: 22,
    follow_range: 35,
    base_armor: 2,
};
pub const BB_SLIME: Family = Family {
    name: "slime",
    damage: 0,
    health: 4,
    follow_range: 16,
    base_armor: 2,
};
pub const BB_WITCH: Family = Family {
    name: "witch",
    damage: 2,
    health: 20,
    follow_range: 16,
    base_armor: 0,
};
pub const BB_WOLF: Family = Family {
    name: "wolf",
    damage: 4,
    health: 16,
    follow_range: 16,
    base_armor: 0,
};
pub const BB_WEREWOLF: Family = Family {
    name: "werewolf",
    damage: 3,
    health: 32,
    follow_range: 35,
    base_armor: 2,
};
pub const BB_PIGMAN: Family = Family {
    name: "pigman",
    damage: 5,
    health: 20,
    follow_range: 35,
    base_armor: 2,
};

pub const BB_Z_1: Zombie = Zombie {
    family: &BB_BASIC,
    tier: 1,

    speed: 0.25,
    armor: &[
        NO_HELMET,
        NO_CHESTPLATE,
        NO_LEGGINGS,
        NO_BOOTS,
    ],
    weapon: &NO_WEAPON,
    children: Children::None,
    damage_type: DamageType::Melee,
};
pub const BB_Z_2: Zombie = Zombie {
    family: &BB_BASIC,
    tier: 2,

    speed: 0.25,
    armor: &[
        NO_HELMET,
        NO_CHESTPLATE,
        Leggings(2,false,Leather(0x000000)),
        NO_BOOTS,
    ],
    weapon: &NO_WEAPON,
    children: Children::None,
    damage_type: DamageType::Melee,
};
pub const BB_Z_3: Zombie = Zombie {
    family: &BB_BASIC,
    tier: 3,

    speed: 0.25,
    armor: &[
        NO_HELMET,
        NO_CHESTPLATE,
        Leggings(2,false,Leather(0x000000)),
        NO_BOOTS,
    ],
    weapon: &WOODEN_AXE,
    children: Children::None,
    damage_type: DamageType::Melee,
};
pub const BB_Z_4: Zombie = Zombie {
    family: &BB_BASIC,
    tier: 4,

    speed: 0.25,
    armor: &[
        NO_HELMET,
        Chestplate(3,false,Leather(0x000000)),
        Leggings(2,false,Leather(0x000000)),
        NO_BOOTS,
    ],
    weapon: &WOODEN_AXE,
    children: Children::None,
    damage_type: DamageType::Melee,
};
pub const BB_Z_5: Zombie = Zombie {
    family: &BB_BASIC,
    tier: 5,

    speed: 0.25,
    armor: &[
        NO_HELMET,
        Chestplate(3,false,Leather(0x000000)),
        Leggings(2,false,Leather(0x000000)),
        Boots(1,false,Leather(0x000000)),
    ],
    weapon: &WOODEN_AXE,
    children: Children::None,
    damage_type: DamageType::Melee,
};
pub const BB_Z_6: Zombie = Zombie {
    family: &BB_BASIC,
    tier: 6,

    speed: 0.26,
    armor: &[
        NO_HELMET,
        Chestplate(3,false,Leather(0x000000)),
        Leggings(2,false,Leather(0x000000)),
        Boots(1,false,Leather(0x000000)),
    ],
    weapon: &STONE_AXE,
    children: Children::None,
    damage_type: DamageType::Melee,
};

pub const BB_SZ_1: Zombie = Zombie {
    family: &BB_SLIME_ZOMBIE,
    tier: 1,

    speed: 0.3,
    armor: &[
        SLIME_HEAD,
        Chestplate(3,false,Leather(0x55FF55)),
        Leggings(2,false,Leather(0x55FF55)),
        Boots(1,false,Leather(0x55FF55)),
    ],
    weapon: &SLIME_BALL,
    children: Children::None,
    damage_type: DamageType::Melee,
};

pub const BB_S_1: Zombie = Zombie {
    family: &BB_SLIME,
    tier: 1,

    speed: 0.36,
    armor: &[
        NO_HELMET,
        NO_CHESTPLATE,
        NO_LEGGINGS,
        NO_BOOTS,
    ],
    weapon: &NO_WEAPON,
    children: Children::None,
    damage_type: DamageType::Melee,
};

pub const BB_WI_1: Zombie = Zombie {
    family: &BB_WITCH,
    tier: 1,

    speed: 0.25,
    armor: &[
        NO_HELMET,
        NO_CHESTPLATE,
        NO_LEGGINGS,
        NO_BOOTS,
    ],
    weapon: &NO_WEAPON,
    children: Children::None,
    damage_type: DamageType::Ability(Ability::Double("splash potion of Harming", "splash potion of Poison")),
};

pub const BB_WO_1: Zombie = Zombie {
    family: &BB_WOLF,
    tier: 1,

    speed: 0.36,
    armor: &[
        NO_HELMET,
        NO_CHESTPLATE,
        NO_LEGGINGS,
        NO_BOOTS,
    ],
    weapon: &NO_WEAPON,
    children: Children::None,
    damage_type: DamageType::Melee,
};

pub const BB_WW_1: Zombie = Zombie {
    family: &BB_WEREWOLF,
    tier: 1,

    speed: 0.3,
    armor: &[
        WERE_HEAD,
        Chestplate(3,false,Leather(0x555555)),
        Leggings(2,false,Leather(0x555555)),
        Boots(1,false,Leather(0x555555)),
    ],
    weapon: &STONE_SWORD,
    children: Children::None,
    damage_type: DamageType::Melee,
};
pub const BB_P_1: Zombie = Zombie {
family: &BB_PIGMAN,
    tier: 1,

    speed: 0.33,
    armor: &[
        NO_HELMET,
        GOLDEN_CHESTPLATE,
        GOLDEN_LEGGINGS,
        GOLDEN_BOOTS,
    ],
    weapon: &GOLD_SWORD,
    children: Children::None,
    damage_type: DamageType::Melee,
};



pub const BB_LILY: Zombie = Zombie {
    family: &Family {
        name: "bb_lore",
        damage: 3,
        health: 55,
        follow_range: 35,
        base_armor: 2,
    },
    tier: 1,
    speed: 0.3,
    armor: &[
        LILY_HEAD,
        CHAIN_CHESTPLATE,
        CHAIN_LEGGINGS,
        CHAIN_BOOTS
    ],
    weapon: &STONE_SWORD,
    children: Children::Single(&Horde {
        zombie: BB_ELLIE,
        count: 1,
    }, false),
    damage_type: DamageType::Melee,
};
pub const BB_ELLIE: Zombie = Zombie {
    family: &Family {
        name: "bb_lore",
        damage: 4,
        health: 30,
        follow_range: 16,
        base_armor: 0,
    },
    tier: 0,
    speed: 0.38,
    armor: &[
        NO_HELMET,
        NO_CHESTPLATE,
        NO_LEGGINGS,
        NO_BOOTS,
    ],
    weapon: &NO_WEAPON,
    children: Children::None,
    damage_type: DamageType::MeleeAbility(Ability::Single("poop")),
};
