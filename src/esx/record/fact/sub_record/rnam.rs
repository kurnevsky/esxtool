use crate::binary::*;

esx_sub_record_fixed_string! {
  struct FactRnam(name: 32)
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(fact_rnam_read_write, FactRnam {
    name: String::from("42")
  });
}
