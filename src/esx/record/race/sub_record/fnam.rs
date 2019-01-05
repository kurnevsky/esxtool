use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct RaceFnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(race_fnam_read_write, RaceFnam {
    name: String::from("42")
  });
}
