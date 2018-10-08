mod flags;
mod auto_calc_flags;
mod name;
mod fnam;
mod cldt;
mod desc;

pub use self::flags::*;
pub use self::auto_calc_flags::*;
pub use self::name::*;
pub use self::fnam::*;
pub use self::cldt::*;
pub use self::desc::*;

use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use binary::*;
use esx::util::name_to_string;

esx_sub_record! {
  enum ClasSubRecord {
    Name(ClasName) => b"NAME",
    Fnam(ClasFnam) => b"FNAM",
    Cldt(ClasCldt) => b"CLDT",
    Desc(ClasDesc) => b"DESC"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use esx::specialization::Specialization;

  read_write_test!(clas_sub_record_name_read_write, ClasSubRecord::Name(ClasName {
    name: String::from("42")
  }));

  read_write_test!(clas_sub_record_fnam_read_write, ClasSubRecord::Fnam(ClasFnam {
    name: String::from("42")
  }));

  read_write_test!(clas_sub_record_cldt_read_write, ClasSubRecord::Cldt(ClasCldt {
    attribute_id_1: 42,
    attribute_id_2: 42,
    specialization: Specialization::Combat,
    minor_id_1: 42,
    major_id_1: 42,
    minor_id_2: 42,
    major_id_2: 42,
    minor_id_3: 42,
    major_id_3: 42,
    minor_id_4: 42,
    major_id_4: 42,
    minor_id_5: 42,
    major_id_5: 42,
    flags: Flags::Playable,
    auto_calc_flags: AutoCalcFlags::Weapon,
  }));

  read_write_test!(clas_sub_record_desc_read_write, ClasSubRecord::Desc(ClasDesc {
    description: String::from("42")
  }));
}
