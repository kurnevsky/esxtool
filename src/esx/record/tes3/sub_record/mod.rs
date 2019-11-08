mod file_type;
mod hedr;
mod mast;
mod data;
mod form;

pub use self::file_type::*;
pub use self::hedr::*;
pub use self::mast::*;
pub use self::data::*;
pub use self::form::*;

use crate::binary::*;

esx_sub_record! {
  enum Tes3SubRecord {
    Hedr(Tes3Hedr) => b"HEDR",
    Mast(Tes3Mast) => b"MAST",
    Data(Tes3Data) => b"DATA",
    Form(Tes3Form) => b"FORM"
  }
}
