use crate::binary::*;

esx_sub_record_null_terminated_string! {
  struct DoorScri(script)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(door_scri_read_write, DoorScri {
    script: String::from("42")
  });
}
