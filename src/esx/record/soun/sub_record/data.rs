use crate::binary::*;

esx_sub_record_simple! {
  struct SounData {
    volume: u8,
    min_range: u8,
    max_range: u8
  }
}
