use crate::binary::*;

esx_sub_record_simple! {
  struct GmstFltv {
    value: f32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(gmst_fltv_read_write, GmstFltv {
    value: 42f32
  });
}
