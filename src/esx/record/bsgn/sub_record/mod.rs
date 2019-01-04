mod name;
mod fnam;
mod tnam;
mod desc;
mod npcs;

pub use self::name::*;
pub use self::fnam::*;
pub use self::tnam::*;
pub use self::desc::*;
pub use self::npcs::*;

use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum BsgnSubRecord {
    Name(BsgnName) => b"NAME",
    Fnam(BsgnFnam) => b"FNAM",
    Tnam(BsgnTnam) => b"TNAM",
    Desc(BsgnDesc) => b"DESC",
    Npcs(BsgnNpcs) => b"NPCS"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(bsgn_sub_record_name_read_write, BsgnSubRecord::Name(BsgnName {
    name: String::from("42")
  }));

  read_write_test!(bsgn_sub_record_fnam_read_write, BsgnSubRecord::Fnam(BsgnFnam {
    name: String::from("42")
  }));

  read_write_test!(bsgn_sub_record_tnam_read_write, BsgnSubRecord::Tnam(BsgnTnam {
    name: String::from("42")
  }));

  read_write_test!(bsgn_sub_record_desc_read_write, BsgnSubRecord::Desc(BsgnDesc {
    description: String::from("42")
  }));

  read_write_test!(bsgn_sub_record_npcs_read_write, BsgnSubRecord::Npcs(BsgnNpcs {
    ability: String::from("42"),
  }));
}
