mod name;
mod modl;
mod fnam;
mod mcdt;
mod itex;
mod enam;
mod scri;

pub use self::name::*;
pub use self::modl::*;
pub use self::fnam::*;
pub use self::mcdt::*;
pub use self::itex::*;
pub use self::enam::*;
pub use self::scri::*;

use crate::binary::*;

esx_sub_record! {
  enum MiscSubRecord {
    Name(MiscName) => b"NAME",
    Modl(MiscModl) => b"MODL",
    Fnam(MiscFnam) => b"FNAM",
    Mcdt(MiscMcdt) => b"MCDT",
    Itex(MiscItex) => b"ITEX",
    Enam(MiscEnam) => b"ENAM",
    Scri(MiscScri) => b"SCRI"
  }
}
