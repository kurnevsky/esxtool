mod rank_data;
mod flags;
mod name;
mod fnam;
mod rnam;
mod fadt;
mod anam;
mod intv;

pub use self::rank_data::*;
pub use self::flags::*;
pub use self::name::*;
pub use self::fnam::*;
pub use self::rnam::*;
pub use self::fadt::*;
pub use self::anam::*;
pub use self::intv::*;

use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum FactSubRecord {
    Name(FactName) => b"NAME",
    Fnam(FactFnam) => b"FNAM",
    Rnam(FactRnam) => b"RNAM",
    Fadt(FactFadt) => b"FADT",
    Anam(FactAnam) => b"ANAM",
    Intv(FactIntv) => b"INTV"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(fact_sub_record_name_read_write, FactSubRecord::Name(FactName {
    name: String::from("42")
  }));

  read_write_test!(fact_sub_record_fnam_read_write, FactSubRecord::Fnam(FactFnam {
    name: String::from("42")
  }));

  read_write_test!(fact_sub_record_rnam_read_write, FactSubRecord::Rnam(FactRnam {
    name: String::from("42")
  }));

  read_write_test!(fact_sub_record_fadt_read_write, FactSubRecord::Fadt(FactFadt {
    attribute_id_1: 42,
    attribute_id_2: 42,
    rank_data: [
      RankData {
        attribute_1: 42,
        attribute_2: 42,
        first_skill: 42,
        second_skill: 42,
        faction: 42
      },
      RankData {
        attribute_1: 42,
        attribute_2: 42,
        first_skill: 42,
        second_skill: 42,
        faction: 42
      },
      RankData {
        attribute_1: 42,
        attribute_2: 42,
        first_skill: 42,
        second_skill: 42,
        faction: 42
      },
      RankData {
        attribute_1: 42,
        attribute_2: 42,
        first_skill: 42,
        second_skill: 42,
        faction: 42
      },
      RankData {
        attribute_1: 42,
        attribute_2: 42,
        first_skill: 42,
        second_skill: 42,
        faction: 42
      },
      RankData {
        attribute_1: 42,
        attribute_2: 42,
        first_skill: 42,
        second_skill: 42,
        faction: 42
      },
      RankData {
        attribute_1: 42,
        attribute_2: 42,
        first_skill: 42,
        second_skill: 42,
        faction: 42
      },
      RankData {
        attribute_1: 42,
        attribute_2: 42,
        first_skill: 42,
        second_skill: 42,
        faction: 42
      },
      RankData {
        attribute_1: 42,
        attribute_2: 42,
        first_skill: 42,
        second_skill: 42,
        faction: 42
      },
      RankData {
        attribute_1: 42,
        attribute_2: 42,
        first_skill: 42,
        second_skill: 42,
        faction: 42
      },
    ],
    skill_id: [42, 42, 42, 42, 42, 42, 42],
    flags: Flags::Hidden,
  }));

  read_write_test!(fact_sub_record_anam_read_write, FactSubRecord::Anam(FactAnam {
    name: String::from("42")
  }));

  read_write_test!(fact_sub_record_intv_read_write, FactSubRecord::Intv(FactIntv {
    value: 42
  }));
}
