use crate::binary::Binary;

esx_bitflags! {
  struct AutoCalcFlags: u32 {
    const WEAPON = 0x0000_0001;
    const ARMOR = 0x0000_0002;
    const CLOTHING = 0x0000_0004;
    const BOOKS = 0x0000_0008;
    const INGREDIANT = 0x0000_0010;
    const PICKS = 0x0000_0020;
    const PROBES = 0x0000_0040;
    const LIGHTS = 0x0000_0080;
    const APPARATUS = 0x0000_0100;
    const REPAIR = 0x0000_0200;
    const MISC = 0x0000_0400;
    const SPELLS = 0x0000_0800;
    const MAGICITEMS = 0x0000_1000;
    const POTIONS = 0x0000_2000;
    const TRAINING = 0x0000_4000;
    const SPELLMAKING = 0x0000_8000;
    const ENCHANTING = 0x0001_0000;
    const REPAIRITEM = 0x0002_0000;
  }
}
