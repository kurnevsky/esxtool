use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct DoorFnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(door_fnam_read_write, DoorFnam {
    name: String::from("42")
  });
}
