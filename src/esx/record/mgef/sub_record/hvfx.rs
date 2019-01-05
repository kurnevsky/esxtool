use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct MgefHvfx(hit_visual)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_hvfx_read_write, MgefHvfx {
    hit_visual: String::from("42")
  });
}
