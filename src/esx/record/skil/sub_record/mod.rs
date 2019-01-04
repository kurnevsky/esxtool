mod indx;
mod skdt;
mod desc;

pub use self::indx::*;
pub use self::skdt::*;
pub use self::desc::*;

use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum SkilSubRecord {
    Indx(SkilIndx) => b"INDX",
    Skdt(SkilSkdt) => b"SKDT",
    Desc(SkilDesc) => b"DESC"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::esx::specialization::Specialization;

  read_write_test!(skil_sub_record_indx_read_write, SkilSubRecord::Indx(SkilIndx {
    skill_id: 42,
  }));

  read_write_test!(skil_sub_record_skdt_read_write, SkilSubRecord::Skdt(SkilSkdt {
    attribute: 42,
    specialization: Specialization::Combat,
    use_value: [42f32, 42f32, 42f32, 42f32],
  }));

  read_write_test!(skil_sub_record_desc_read_write, SkilSubRecord::Desc(SkilDesc {
    description: String::from("42")
  }));
}
