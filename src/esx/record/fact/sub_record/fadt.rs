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
