mod spell_school;
mod flags;
mod indx;
mod medt;
mod itex;
mod ptex;
mod cvfx;
mod bvfx;
mod hvfx;
mod avfx;
mod desc;
mod csnd;
mod bsnd;
mod hsnd;
mod asnd;

pub use self::spell_school::*;
pub use self::flags::*;
pub use self::indx::*;
pub use self::medt::*;
pub use self::itex::*;
pub use self::ptex::*;
pub use self::cvfx::*;
pub use self::bvfx::*;
pub use self::hvfx::*;
pub use self::avfx::*;
pub use self::desc::*;
pub use self::csnd::*;
pub use self::bsnd::*;
pub use self::hsnd::*;
pub use self::asnd::*;

use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum MgefSubRecord {
    Indx(MgefIndx) => b"INDX",
    Medt(MgefMedt) => b"MEDT",
    Itex(MgefItex) => b"ITEX",
    Ptex(MgefPtex) => b"PTEX",
    Cvfx(MgefCvfx) => b"CVFX",
    Bvfx(MgefBvfx) => b"BVFX",
    Hvfx(MgefHvfx) => b"HVFX",
    Avfx(MgefAvfx) => b"AVFX",
    Desc(MgefDesc) => b"DESC",
    Csnd(MgefCsnd) => b"CSND",
    Bsnd(MgefBsnd) => b"BSND",
    Hsnd(MgefHsnd) => b"HSND",
    Asnd(MgefAsnd) => b"ASND"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_sub_record_indx_read_write, MgefSubRecord::Indx(MgefIndx {
    effect_id: 42,
  }));

  read_write_test!(mgef_sub_record_medt_read_write, MgefSubRecord::Medt(MgefMedt {
    spell_school: SpellSchool::Alteration,
    base_cost: 42f32,
    flags: Flags::Spellmaking,
    red: 42,
    blue: 42,
    green: 42,
    speed_x: 42f32,
    size_x: 42f32,
    size_cap: 42f32,
  }));

  read_write_test!(mgef_sub_record_itex_read_write, MgefSubRecord::Itex(MgefItex {
    effect_icon: String::from("42")
  }));

  read_write_test!(mgef_sub_record_ptex_read_write, MgefSubRecord::Ptex(MgefPtex {
    particle_texture: String::from("42")
  }));

  read_write_test!(mgef_sub_record_cvfx_read_write, MgefSubRecord::Cvfx(MgefCvfx {
    casting_visual: String::from("42")
  }));

  read_write_test!(mgef_sub_record_bvfx_read_write, MgefSubRecord::Bvfx(MgefBvfx {
    bolt_visual: String::from("42")
  }));

  read_write_test!(mgef_sub_record_hvfx_read_write, MgefSubRecord::Hvfx(MgefHvfx {
    hit_visual: String::from("42")
  }));

  read_write_test!(mgef_sub_record_avfx_read_write, MgefSubRecord::Avfx(MgefAvfx {
    area_visual: String::from("42")
  }));

  read_write_test!(mgef_sub_record_desc_read_write, MgefSubRecord::Desc(MgefDesc {
    description: String::from("42")
  }));

  read_write_test!(mgef_sub_record_csnd_read_write, MgefSubRecord::Csnd(MgefCsnd {
    cast_sound: String::from("42")
  }));

  read_write_test!(mgef_sub_record_bsnd_read_write, MgefSubRecord::Bsnd(MgefBsnd {
    bolt_sound: String::from("42")
  }));

  read_write_test!(mgef_sub_record_hsnd_read_write, MgefSubRecord::Hsnd(MgefHsnd {
    hit_sound: String::from("42")
  }));

  read_write_test!(mgef_sub_record_asnd_read_write, MgefSubRecord::Asnd(MgefAsnd {
    area_sound: String::from("42")
  }));
}
