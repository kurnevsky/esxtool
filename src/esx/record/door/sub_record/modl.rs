use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct DoorModl(model)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(door_modl_read_write, DoorModl {
    model: String::from("42")
  });
}
