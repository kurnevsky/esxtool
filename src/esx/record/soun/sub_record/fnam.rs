use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct SounFnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(soun_fnam_read_write, SounFnam {
    name: String::from("42")
  });
}
