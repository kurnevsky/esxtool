use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct MgefBvfx(bolt_visual)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(mgef_bvfx_read_write, MgefBvfx {
    bolt_visual: String::from("42")
  });
}
