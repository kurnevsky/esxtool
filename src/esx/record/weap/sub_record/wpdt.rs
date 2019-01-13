use super::flags::Flags;
use super::weapon_type::WeaponType;
use crate::binary::*;

esx_sub_record_simple! {
  struct WeapWpdt {
    weight: f32,
    value: u32,
    weapon_type: WeaponType,
    health: u16,
    speed: f32,
    reach: f32,
    enchant_pts: u16,
    chop_min: u8,
    chop_max: u8,
    slash_min: u8,
    slash_max: u8,
    trust_min: u8,
    trust_max: u8,
    flags: Flags
  }
}
