use crate::binary::*;

esx_sub_record_simple! {
  struct GlobFltv {
    value: f32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(glob_fltv_read_write, GlobFltv {
    value: 42f32
  });
}
