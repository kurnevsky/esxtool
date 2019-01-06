use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct DoorAnam(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(door_anam_read_write, DoorAnam {
    name: String::from("42")
  });
}
