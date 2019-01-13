mod rank_data;
mod flags;
mod name;
mod fnam;
mod rnam;
mod fadt;
mod anam;
mod intv;

pub use self::rank_data::*;
pub use self::flags::*;
pub use self::name::*;
pub use self::fnam::*;
pub use self::rnam::*;
pub use self::fadt::*;
pub use self::anam::*;
pub use self::intv::*;

use crate::binary::*;

esx_sub_record! {
  enum FactSubRecord {
    Name(FactName) => b"NAME",
    Fnam(FactFnam) => b"FNAM",
    Rnam(FactRnam) => b"RNAM",
    Fadt(FactFadt) => b"FADT",
    Anam(FactAnam) => b"ANAM",
    Intv(FactIntv) => b"INTV"
  }
}
