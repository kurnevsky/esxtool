mod name;
mod modl;

pub use self::name::*;
pub use self::modl::*;

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum StatSubRecord {
    Name(StatName) => b"NAME",
    Modl(StatModl) => b"MODL"
  }
}
