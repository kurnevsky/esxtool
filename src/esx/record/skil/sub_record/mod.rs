mod indx;
mod skdt;
mod desc;

pub use self::indx::*;
pub use self::skdt::*;
pub use self::desc::*;

use crate::binary::*;

esx_sub_record! {
  enum SkilSubRecord {
    Indx(SkilIndx) => b"INDX",
    Skdt(SkilSkdt) => b"SKDT",
    Desc(SkilDesc) => b"DESC"
  }
}
