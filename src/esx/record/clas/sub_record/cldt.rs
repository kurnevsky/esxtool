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

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(clas_cldt_read_write, ClasCldt {
    attribute_id_1: 42,
    attribute_id_2: 42,
    specialization: Specialization::Combat,
    minor_id_1: 42,
    major_id_1: 42,
    minor_id_2: 42,
    major_id_2: 42,
    minor_id_3: 42,
    major_id_3: 42,
    minor_id_4: 42,
    major_id_4: 42,
    minor_id_5: 42,
    major_id_5: 42,
    flags: Flags::Playable,
    auto_calc_flags: AutoCalcFlags::Weapon,
  });
}
