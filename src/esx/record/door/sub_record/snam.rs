use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct DoorSnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(door_snam_read_write, DoorSnam {
    name: String::from("42")
  });
}
