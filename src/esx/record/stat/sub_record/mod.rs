mod name;
mod modl;

pub use self::name::*;
pub use self::modl::*;

use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum StatSubRecord {
    Name(StatName) => b"NAME",
    Modl(StatModl) => b"MODL"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(stat_sub_record_name_read_write, StatSubRecord::Name(StatName {
    name: String::from("42")
  }));

  read_write_test!(stat_sub_record_modl_read_write, StatSubRecord::Modl(StatModl {
    model: String::from("42")
  }));
}
