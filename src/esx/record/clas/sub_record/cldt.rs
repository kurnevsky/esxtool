use super::flags::Flags;
use super::auto_calc_flags::AutoCalcFlags;
use crate::binary::*;
use crate::esx::specialization::Specialization;

esx_sub_record_simple! {
  struct ClasCldt {
    attribute_id_1: u32,
    attribute_id_2: u32,
    specialization: Specialization,
    minor_id_1: u32,
    major_id_1: u32,
    minor_id_2: u32,
    major_id_2: u32,
    minor_id_3: u32,
    major_id_3: u32,
    minor_id_4: u32,
    major_id_4: u32,
    minor_id_5: u32,
    major_id_5: u32,
    flags: Flags,
    auto_calc_flags: AutoCalcFlags
  }
}
