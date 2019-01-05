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
use crate::esx::util::name_to_string;

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

#[cfg(test)]
mod tests {
  use super::*;

  use crate::esx::color_ref::ColorRef;

  read_write_test!(regn_sub_record_name_read_write, RegnSubRecord::Name(RegnName {
    name: String::from("42")
  }));

  read_write_test!(regn_sub_record_fnam_read_write, RegnSubRecord::Fnam(RegnFnam {
    name: String::from("42")
  }));

  read_write_test!(regn_sub_record_weat_read_write, RegnSubRecord::Weat(RegnWeat {
    clear: 42,
    cloudy: 42,
    foggy: 42,
    overcast: 42,
    rain: 42,
    thunder: 42,
    ash: 42,
    blight: 42,
    snow: 42,
    blizard: 42,
  }));

  read_write_test!(regn_sub_record_bnam_read_write, RegnSubRecord::Bnam(RegnBnam {
    name: String::from("42")
  }));

  read_write_test!(regn_sub_record_cnam_read_write, RegnSubRecord::Cnam(RegnCnam {
    color_ref: ColorRef {
      red: 42,
      green: 42,
      blue: 42,
      null: 42,
    }
  }));

  read_write_test!(regn_sub_record_snam_read_write, RegnSubRecord::Snam(RegnSnam {
    sound_name: String::from("42"),
    chance: 42,
  }));
}
