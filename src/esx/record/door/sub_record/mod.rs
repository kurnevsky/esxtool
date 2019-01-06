mod name;
mod fnam;
mod modl;
mod scri;
mod snam;
mod anam;

pub use self::name::*;
pub use self::fnam::*;
pub use self::modl::*;
pub use self::scri::*;
pub use self::snam::*;
pub use self::anam::*;

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum DoorSubRecord {
    Name(DoorName) => b"NAME",
    Fnam(DoorFnam) => b"FNAM",
    Modl(DoorModl) => b"MODL",
    Scri(DoorScri) => b"SCRI",
    Snam(DoorSnam) => b"SNAM",
    Anam(DoorAnam) => b"ANAM"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(door_sub_record_name_read_write, DoorSubRecord::Name(DoorName {
    name: String::from("42")
  }));

  read_write_test!(door_sub_record_fnam_read_write, DoorSubRecord::Fnam(DoorFnam {
    name: String::from("42")
  }));

  read_write_test!(door_sub_record_modl_read_write, DoorSubRecord::Modl(DoorModl {
    model: String::from("42")
  }));

  read_write_test!(door_sub_record_scri_read_write, DoorSubRecord::Scri(DoorScri {
    script: String::from("42")
  }));

  read_write_test!(door_sub_record_snam_read_write, DoorSubRecord::Snam(DoorSnam {
    name: String::from("42")
  }));

  read_write_test!(door_sub_record_anam_read_write, DoorSubRecord::Anam(DoorAnam {
    name: String::from("42")
  }));
}
