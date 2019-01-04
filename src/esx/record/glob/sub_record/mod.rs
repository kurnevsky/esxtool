mod global_type;
mod name;
mod fnam;
mod fltv;

pub use self::global_type::*;
pub use self::name::*;
pub use self::fnam::*;
pub use self::fltv::*;

use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum GlobSubRecord {
    Name(GlobName) => b"NAME",
    Fnam(GlobFnam) => b"FNAM",
    Fltv(GlobFltv) => b"FLTV"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(glob_sub_record_name_read_write, GlobSubRecord::Name(GlobName {
    name: String::from("42")
  }));

  read_write_test!(glob_sub_record_fnam_read_write, GlobSubRecord::Fnam(GlobFnam {
    global_type: GlobalType::Short
  }));

  read_write_test!(glob_sub_record_fltv_read_write, GlobSubRecord::Fltv(GlobFltv {
    value: 42f32
  }));
}
