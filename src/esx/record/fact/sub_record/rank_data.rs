use crate::binary::*;

esx_data! {
  struct RankData {
    attribute_1: u32,
    attribute_2: u32,
    first_skill: u32,
    second_skill: u32,
    faction: u32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(rank_data_read_write, RankData {
    attribute_1: 42,
    attribute_2: 42,
    first_skill: 42,
    second_skill: 42,
    faction: 42,
  });
}
