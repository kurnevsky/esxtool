mod name;
mod fnam;
mod weat;
mod bnam;
mod cnam;
mod snam;

pub use self::name::*;
pub use self::fnam::*;
pub use self::weat::*;
pub use self::bnam::*;
pub use self::cnam::*;
pub use self::snam::*;

use crate::binary::*;

esx_sub_record! {
  enum RegnSubRecord {
    Name(RegnName) => b"NAME",
    Fnam(RegnFnam) => b"FNAM",
    Weat(RegnWeat) => b"WEAT",
    Bnam(RegnBnam) => b"BNAM",
    Cnam(RegnCnam) => b"CNAM",
    Snam(RegnSnam) => b"SNAM"
  }
}
