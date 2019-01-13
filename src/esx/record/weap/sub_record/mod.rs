mod flags;
mod weapon_type;
mod name;
mod modl;
mod fnam;
mod wpdt;
mod itex;
mod enam;
mod scri;

pub use self::flags::*;
pub use self::weapon_type::*;
pub use self::name::*;
pub use self::modl::*;
pub use self::fnam::*;
pub use self::wpdt::*;
pub use self::itex::*;
pub use self::enam::*;
pub use self::scri::*;

use crate::binary::*;

esx_sub_record! {
  enum WeapSubRecord {
    Name(WeapName) => b"NAME",
    Modl(WeapModl) => b"MODL",
    Fnam(WeapFnam) => b"FNAM",
    Wpdt(WeapWpdt) => b"WPDT",
    Itex(WeapItex) => b"ITEX",
    Enam(WeapEnam) => b"ENAM",
    Scri(WeapScri) => b"SCRI"
  }
}
