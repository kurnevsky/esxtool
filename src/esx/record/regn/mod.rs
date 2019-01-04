pub mod sub_record;

use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

use super::record_flags::RecordFlags;
use self::sub_record::*;
use crate::binary::*;

esx_record!(RegnRecord, RegnSubRecord);

#[cfg(test)]
mod tests {
  use super::*;

  use crate::esx::color_ref::ColorRef;

  read_write_test!(regn_record_read_write, RegnRecord {
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      RegnSubRecord::Name(RegnName {
        name: String::from("42")
      }),
      RegnSubRecord::Fnam(RegnFnam {
        name: String::from("42")
      }),
      RegnSubRecord::Weat(RegnWeat {
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
      }),
      RegnSubRecord::Bnam(RegnBnam {
        name: String::from("42")
      }),
      RegnSubRecord::Cnam(RegnCnam {
        color_ref: ColorRef {
          red: 42,
          green: 42,
          blue: 42,
          null: 42,
        }
      }),
      RegnSubRecord::Snam(RegnSnam {
        sound_name: String::from("42"),
        chance: 42,
      }),
    ],
  });
}
