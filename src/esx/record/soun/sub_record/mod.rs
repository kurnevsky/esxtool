mod name;
mod fnam;
mod data;

pub use self::name::*;
pub use self::fnam::*;
pub use self::data::*;

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum SounSubRecord {
    Name(SounName) => b"NAME",
    Fnam(SounFnam) => b"FNAM",
    Data(SounData) => b"DATA"
  }
}
