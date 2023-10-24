use crate::commands::zombies::gear::ArmorMaterial::Leather;
use crate::commands::zombies::gear::ArmorPiece::Leggings;
use crate::commands::zombies::gear::Weapon::{Axe, Sword};
use crate::commands::zombies::gear::WeaponMaterial::{Diamond, Gold, Wood};
#[derive(Debug)]
pub enum ArmorPiece {
    None,
    Helmet(HelmetType),
    Chestplate(ArmorMaterial, Enchanted),
    Leggings(ArmorMaterial, Enchanted),
    Boots(ArmorMaterial, Enchanted),
}
#[derive(Debug)]
pub enum WeaponMaterial {
    Wood,
    Stone,
    Gold,
    Iron,
    Diamond,
}
#[derive(Debug)]
pub enum HelmetType {
    Head(u32),
    Helmet(ArmorMaterial, Enchanted),
}
#[derive(Debug)]
pub enum ArmorMaterial {
    Leather(u32),
    Gold,
    Chainmail,
    Iron,
    Diamond,
}
#[derive(Debug)]
pub enum Weapon {
    None,
    Axe(WeaponMaterial, Enchanted),
    Sword(WeaponMaterial, Enchanted),

    SlimeBall(Enchanted),
}
pub type Enchanted = bool;
pub const NO_WEAPON: Weapon = Weapon::None;
pub const WOODEN_AXE: Weapon = Axe(Wood, false);
pub const DIAMOND_AXE: Weapon = Axe(Diamond, false);
pub const GOLD_SWORD: Weapon = Sword(Gold, false);
pub const DIAMOND_SWORD: Weapon = Sword(Diamond, false);
pub const SLIME_BALL: Weapon = Weapon::SlimeBall(true);

pub const NO_HELMET: ArmorPiece = ArmorPiece::None;
pub const NO_CHESTPLATE: ArmorPiece = ArmorPiece::None;
pub const NO_LEGGINGS: ArmorPiece = ArmorPiece::None;
pub const NO_BOOTS: ArmorPiece = ArmorPiece::None;
