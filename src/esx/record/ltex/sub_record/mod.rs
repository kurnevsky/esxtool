mod name;
mod intv;
mod data;

pub use self::name::*;
pub use self::intv::*;
pub use self::data::*;

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum LtexSubRecord {
    Name(LtexName) => b"NAME",
    Intv(LtexIntv) => b"INTV",
    Data(LtexData) => b"DATA"
  }
}
