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
