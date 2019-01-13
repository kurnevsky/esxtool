mod flags;
mod auto_calc_flags;
mod name;
mod fnam;
mod cldt;
mod desc;

pub use self::flags::*;
pub use self::auto_calc_flags::*;
pub use self::name::*;
pub use self::fnam::*;
pub use self::cldt::*;
pub use self::desc::*;

use crate::binary::*;

esx_sub_record! {
  enum ClasSubRecord {
    Name(ClasName) => b"NAME",
    Fnam(ClasFnam) => b"FNAM",
    Cldt(ClasCldt) => b"CLDT",
    Desc(ClasDesc) => b"DESC"
  }
}
