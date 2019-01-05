pub mod sub_record;

use super::record_flags::RecordFlags;
use self::sub_record::*;
use crate::binary::*;

esx_record!(ScptRecord, ScptSubRecord);

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(scpt_record_read_write, ScptRecord {
    unknown: 42,
    flags: RecordFlags::Persistent,
    sub_records: vec![
      ScptSubRecord::Schd(ScptSchd {
        name: String::from("42"),
        num_shorts: 42,
        num_longs: 42,
        num_floats: 42,
        script_data_size: 42,
        local_var_size: 42,
      }),
      ScptSubRecord::Scvr(ScptScvr {
        variables: vec![
          String::from("42"),
          String::from("43"),
        ],
      }),
      ScptSubRecord::Scdt(ScptScdt {
        data: vec![42; 123],
      }),
      ScptSubRecord::Sctx(ScptSctx {
        text: String::from("42")
      }),
    ],
  });
}
