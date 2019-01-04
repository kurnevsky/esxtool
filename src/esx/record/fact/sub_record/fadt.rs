use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use super::rank_data::RankData;
use super::flags::Flags;
use crate::binary::*;

esx_sub_record_simple! {
  struct FactFadt {
    attribute_id_1: u32,
    attribute_id_2: u32,
    rank_data: [RankData; 10],
    skill_id: [i32; 7],
    flags: Flags
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(fact_fadt_read_write, FactFadt {
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
  });
}
