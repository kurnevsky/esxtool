use crate::binary::*;

esx_sub_record_simple! {
  struct MiscMcdt {
    weight: f32,
    value: u32,
    is_key: u32
  }
}
