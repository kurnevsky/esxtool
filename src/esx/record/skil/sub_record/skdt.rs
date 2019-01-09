use crate::binary::*;
use crate::esx::specialization::Specialization;

esx_sub_record_simple! {
  struct SkilSkdt {
    attribute: u32,
    specialization: Specialization,
    use_value: [f32; 4]
  }
}
