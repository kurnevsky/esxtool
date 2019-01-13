mod schd;
mod scvr;
mod scdt;
mod sctx;

pub use self::schd::*;
pub use self::scvr::*;
pub use self::scdt::*;
pub use self::sctx::*;

use crate::binary::*;

esx_sub_record! {
  enum ScptSubRecord {
    Schd(ScptSchd) => b"SCHD",
    Scvr(ScptScvr) => b"SCVR",
    Scdt(ScptScdt) => b"SCDT",
    Sctx(ScptSctx) => b"SCTX"
  }
}
