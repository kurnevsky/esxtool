pub mod sub_record;

use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

use super::record_flags::RecordFlags;
use self::sub_record::*;
use crate::binary::*;

esx_record!(MgefRecord, MgefSubRecord);

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_record_read_write, MgefRecord {
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      MgefSubRecord::Indx(MgefIndx {
        effect_id: 42
      }),
      MgefSubRecord::Medt(MgefMedt {
        spell_school: SpellSchool::Alteration,
        base_cost: 42f32,
        flags: Flags::Spellmaking,
        red: 42,
        blue: 42,
        green: 42,
        speed_x: 42f32,
        size_x: 42f32,
        size_cap: 42f32,
      }),
      MgefSubRecord::Itex(MgefItex {
        effect_icon: String::from("42")
      }),
      MgefSubRecord::Ptex(MgefPtex {
        particle_texture: String::from("42")
      }),
      MgefSubRecord::Cvfx(MgefCvfx {
        casting_visual: String::from("42")
      }),
      MgefSubRecord::Bvfx(MgefBvfx {
        bolt_visual: String::from("42")
      }),
      MgefSubRecord::Hvfx(MgefHvfx {
        hit_visual: String::from("42")
      }),
      MgefSubRecord::Avfx(MgefAvfx {
        area_visual: String::from("42")
      }),
      MgefSubRecord::Desc(MgefDesc {
        description: String::from("42")
      }),
      MgefSubRecord::Csnd(MgefCsnd {
        cast_sound: String::from("42")
      }),
      MgefSubRecord::Bsnd(MgefBsnd {
        bolt_sound: String::from("42")
      }),
      MgefSubRecord::Hsnd(MgefHsnd {
        hit_sound: String::from("42")
      }),
      MgefSubRecord::Asnd(MgefAsnd {
        area_sound: String::from("42")
      }),
    ],
  });
}
