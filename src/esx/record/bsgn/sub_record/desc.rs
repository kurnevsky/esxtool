use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct BsgnDesc(description)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(bsgn_desc_read_write, BsgnDesc {
    description: String::from("42")
  });
}
