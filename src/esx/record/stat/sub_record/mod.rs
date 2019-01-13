mod name;
mod modl;

pub use self::name::*;
pub use self::modl::*;

use crate::binary::*;

esx_sub_record! {
  enum StatSubRecord {
    Name(StatName) => b"NAME",
    Modl(StatModl) => b"MODL"
  }
}
