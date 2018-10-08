mod name;
mod intv;
mod data;

pub use self::name::*;
pub use self::intv::*;
pub use self::data::*;

use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use binary::*;
use esx::util::name_to_string;

esx_sub_record! {
  enum LtexSubRecord {
    Name(LtexName) => b"NAME",
    Intv(LtexIntv) => b"INTV",
    Data(LtexData) => b"DATA"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(ltex_sub_record_name_read_write, LtexSubRecord::Name(LtexName {
    name: String::from("42")
  }));

  read_write_test!(ltex_sub_record_intv_read_write, LtexSubRecord::Intv(LtexIntv {
    value: 42
  }));

  read_write_test!(ltex_sub_record_data_read_write, LtexSubRecord::Data(LtexData {
    name: String::from("42")
  }));
}
