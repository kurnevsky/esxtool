use crate::binary::*;

esx_sub_record_string! {
  struct ScptSctx(text)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(scpt_sctx_read_write, ScptSctx {
    text: String::from("42")
  });
}
