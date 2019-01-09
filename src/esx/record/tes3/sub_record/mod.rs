mod file_type;
mod hedr;
mod mast;
mod data;

pub use self::file_type::*;
pub use self::hedr::*;
pub use self::mast::*;
pub use self::data::*;

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum Tes3SubRecord {
    Hedr(Tes3Hedr) => b"HEDR",
    Mast(Tes3Mast) => b"MAST",
    Data(Tes3Data) => b"DATA"
  }
}
