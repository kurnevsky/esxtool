mod name;
mod fnam;
mod data;

pub use self::name::*;
pub use self::fnam::*;
pub use self::data::*;

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum SounSubRecord {
    Name(SounName) => b"NAME",
    Fnam(SounFnam) => b"FNAM",
    Data(SounData) => b"DATA"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(soun_sub_record_name_read_write, SounSubRecord::Name(SounName {
    name: String::from("42")
  }));

  read_write_test!(soun_sub_record_fnam_read_write, SounSubRecord::Fnam(SounFnam {
    name: String::from("42")
  }));

  read_write_test!(soun_sub_record_data_read_write, SounSubRecord::Data(SounData {
    volume: 42,
    min_range: 42,
    max_range: 42,
  }));
}
