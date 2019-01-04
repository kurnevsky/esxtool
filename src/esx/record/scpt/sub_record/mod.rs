mod schd;
mod scvr;
mod scdt;
mod sctx;

pub use self::schd::*;
pub use self::scvr::*;
pub use self::scdt::*;
pub use self::sctx::*;

use std::io::{Error, ErrorKind, Read, Result, Seek, Write};

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum ScptSubRecord {
    Schd(ScptSchd) => b"SCHD",
    Scvr(ScptScvr) => b"SCVR",
    Scdt(ScptScdt) => b"SCDT",
    Sctx(ScptSctx) => b"SCTX"
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  read_write_test!(scpt_sub_record_schd_read_write, ScptSubRecord::Schd(ScptSchd {
    name: String::from("42"),
    num_shorts: 42,
    num_longs: 42,
    num_floats: 42,
    script_data_size: 42,
    local_var_size: 42,
  }));

  read_write_test!(scpt_sub_record_scvr_read_write, ScptSubRecord::Scvr(ScptScvr {
    variables: vec![
      String::from("42"),
      String::from("43"),
    ],
  }));

  read_write_test!(scpt_sub_record_scdt_read_write, ScptSubRecord::Scdt(ScptScdt {
    data: vec![42; 123],
  }));

  read_write_test!(scpt_sub_record_sctx_read_write, ScptSubRecord::Sctx(ScptSctx {
    text: String::from("42")
  }));
}
