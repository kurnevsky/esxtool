use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use binary::Binary;

bitflags! {
  pub struct AutoCalcFlags: u32 {
    const Weapon = 0x0000_0001;
    const Armor = 0x0000_0002;
    const Clothing = 0x0000_0004;
    const Books = 0x0000_0008;
    const Ingrediant = 0x0000_0010;
    const Picks = 0x0000_0020;
    const Probes = 0x0000_0040;
    const Lights = 0x0000_0080;
    const Apparatus = 0x0000_0100;
    const Repair = 0x0000_0200;
    const Misc = 0x0000_0400;
    const Spells = 0x0000_0800;
    const MagicItems = 0x0000_1000;
    const Potions = 0x0000_2000;
    const Training = 0x0000_4000;
    const Spellmaking = 0x0000_8000;
    const Enchanting = 0x0001_0000;
    const RepairItem = 0x0002_0000;
  }
}

bitflags_binary!(AutoCalcFlags);
