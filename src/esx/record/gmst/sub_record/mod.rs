mod name;
mod strv;
mod intv;
mod fltv;

pub use self::name::*;
pub use self::strv::*;
pub use self::intv::*;
pub use self::fltv::*;

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum GmstSubRecord {
    Name(GmstName) => b"NAME",
    Strv(GmstStrv) => b"STRV",
    Intv(GmstIntv) => b"INTV",
    Fltv(GmstFltv) => b"FLTV"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(gmst_sub_record_name_read_write, GmstSubRecord::Name(GmstName {
    name: String::from("42")
  }));

  read_write_test!(gmst_sub_record_strv_read_write, GmstSubRecord::Strv(GmstStrv {
    value: String::from("42")
  }));

  read_write_test!(gmst_sub_record_intv_read_write, GmstSubRecord::Intv(GmstIntv {
    value: 42
  }));

  read_write_test!(gmst_sub_record_fltvv_read_write, GmstSubRecord::Fltv(GmstFltv {
    value: 42f32
  }));
}
