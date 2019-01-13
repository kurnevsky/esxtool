mod global_type;
mod name;
mod fnam;
mod fltv;

pub use self::global_type::*;
pub use self::name::*;
pub use self::fnam::*;
pub use self::fltv::*;

use crate::binary::*;

esx_sub_record! {
  enum GlobSubRecord {
    Name(GlobName) => b"NAME",
    Fnam(GlobFnam) => b"FNAM",
    Fltv(GlobFltv) => b"FLTV"
  }
}
