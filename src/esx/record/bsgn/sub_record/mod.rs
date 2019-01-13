mod name;
mod fnam;
mod tnam;
mod desc;
mod npcs;

pub use self::name::*;
pub use self::fnam::*;
pub use self::tnam::*;
pub use self::desc::*;
pub use self::npcs::*;

use crate::binary::*;

esx_sub_record! {
  enum BsgnSubRecord {
    Name(BsgnName) => b"NAME",
    Fnam(BsgnFnam) => b"FNAM",
    Tnam(BsgnTnam) => b"TNAM",
    Desc(BsgnDesc) => b"DESC",
    Npcs(BsgnNpcs) => b"NPCS"
  }
}
