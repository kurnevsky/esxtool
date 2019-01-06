use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct DoorName(name)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(door_name_read_write, DoorName {
    name: String::from("42")
  });
}
