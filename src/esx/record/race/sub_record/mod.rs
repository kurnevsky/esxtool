mod skill_bonus;
mod flags;
mod name;
mod fnam;
mod radt;
mod npcs;
mod desc;

pub use self::skill_bonus::*;
pub use self::flags::*;
pub use self::name::*;
pub use self::fnam::*;
pub use self::radt::*;
pub use self::npcs::*;
pub use self::desc::*;

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum RaceSubRecord {
    Name(RaceName) => b"NAME",
    Fnam(RaceFnam) => b"FNAM",
    Radt(RaceRadt) => b"RADT",
    Npcs(RaceNpcs) => b"NPCS",
    Desc(RaceDesc) => b"DESC"
  }
}
