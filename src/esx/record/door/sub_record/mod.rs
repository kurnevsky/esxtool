mod name;
mod fnam;
mod modl;
mod scri;
mod snam;
mod anam;

pub use self::name::*;
pub use self::fnam::*;
pub use self::modl::*;
pub use self::scri::*;
pub use self::snam::*;
pub use self::anam::*;

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum DoorSubRecord {
    Name(DoorName) => b"NAME",
    Fnam(DoorFnam) => b"FNAM",
    Modl(DoorModl) => b"MODL",
    Scri(DoorScri) => b"SCRI",
    Snam(DoorSnam) => b"SNAM",
    Anam(DoorAnam) => b"ANAM"
  }
}
