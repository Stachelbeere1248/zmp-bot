use crate::commands::zombies::gear::{
    Weapon::{*},
    WeaponMaterial::{*}
};

pub type Enchanted = bool;
pub type Armor = [ArmorPiece; 4];
pub type Damage = u8;
pub type ArmorValue = u8;
pub type SkinId = u8;
pub type LeatherColor = u32;
#[derive(Debug)]
pub enum ArmorPiece {
    None(ArmorValue),
    Helmet(ArmorValue, Enchanted, ArmorMaterial),
    Head(ArmorValue, Enchanted, SkinId),
    Chestplate(ArmorValue, Enchanted, ArmorMaterial),
    Leggings(ArmorValue, Enchanted, ArmorMaterial),
    Boots(ArmorValue, Enchanted, ArmorMaterial)
}

impl ArmorPiece {
    pub fn armor_value(&self) -> &ArmorValue {
        match self {
            ArmorPiece::None(value) => value,
            ArmorPiece::Helmet(value, _, _) => value,
            ArmorPiece::Head(value, _, _) => value,
            ArmorPiece::Chestplate(value, _, _) => value,
            ArmorPiece::Leggings(value, _, _) => value,
            ArmorPiece::Boots(value, _, _) => value
        }
    }
}

#[derive(Debug)]
pub enum WeaponMaterial {
    Wood,
    Stone,
    Gold,
    Iron,
    Diamond
}
#[derive(Debug)]
pub enum ArmorMaterial {
    Leather(LeatherColor),
    Gold,
    Chainmail,
    Iron,
    Diamond
}
#[derive(Debug)]
pub enum Weapon {
    None(Damage),
    Axe(Damage, Enchanted, WeaponMaterial),
    Sword(Damage, Enchanted, WeaponMaterial),

    Other(Damage, Enchanted, *const str)
}

impl Weapon {
    pub fn damage(&self) -> &Damage {
        match self {
            None(damage) => damage,
            Axe(damage, _, _) => damage,
            Sword(damage, _, _) => damage,
            Other(damage, _, _) => damage
        }
    }
}


//No Gear
pub const NO_HELMET: ArmorPiece = ArmorPiece::None(0);
pub const NO_CHESTPLATE: ArmorPiece = ArmorPiece::None(0);
pub const NO_LEGGINGS: ArmorPiece = ArmorPiece::None(0);
pub const NO_BOOTS: ArmorPiece = ArmorPiece::None(0);
pub const SLIME_HEAD: ArmorPiece = ArmorPiece::Head(0,false,0);


//Weapons
pub const NO_WEAPON: Weapon = Weapon::None(0);
pub const WOODEN_AXE: Weapon = Axe(3,false,Wood);
pub const DIAMOND_AXE: Weapon = Axe(6,false,Diamond);
pub const GOLD_SWORD: Weapon = Sword(4,false,Gold);
pub const DIAMOND_SWORD: Weapon = Sword(7,false,Diamond);
pub const SLIME_BALL: Weapon = Other(0, true, "Slime Ball");

