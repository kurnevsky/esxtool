mod spell_school;
mod flags;
mod indx;
mod medt;
mod itex;
mod ptex;
mod cvfx;
mod bvfx;
mod hvfx;
mod avfx;
mod desc;
mod csnd;
mod bsnd;
mod hsnd;
mod asnd;

pub use self::spell_school::*;
pub use self::flags::*;
pub use self::indx::*;
pub use self::medt::*;
pub use self::itex::*;
pub use self::ptex::*;
pub use self::cvfx::*;
pub use self::bvfx::*;
pub use self::hvfx::*;
pub use self::avfx::*;
pub use self::desc::*;
pub use self::csnd::*;
pub use self::bsnd::*;
pub use self::hsnd::*;
pub use self::asnd::*;

use crate::binary::*;
use crate::esx::util::name_to_string;

esx_sub_record! {
  enum MgefSubRecord {
    Indx(MgefIndx) => b"INDX",
    Medt(MgefMedt) => b"MEDT",
    Itex(MgefItex) => b"ITEX",
    Ptex(MgefPtex) => b"PTEX",
    Cvfx(MgefCvfx) => b"CVFX",
    Bvfx(MgefBvfx) => b"BVFX",
    Hvfx(MgefHvfx) => b"HVFX",
    Avfx(MgefAvfx) => b"AVFX",
    Desc(MgefDesc) => b"DESC",
    Csnd(MgefCsnd) => b"CSND",
    Bsnd(MgefBsnd) => b"BSND",
    Hsnd(MgefHsnd) => b"HSND",
    Asnd(MgefAsnd) => b"ASND"
  }
}
