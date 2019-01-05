use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct BsgnFnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(bsgn_fnam_read_write, BsgnFnam {
    name: String::from("42")
  });
}
