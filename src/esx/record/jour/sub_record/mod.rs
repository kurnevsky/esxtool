mod journal_type;
mod jety;
mod yeto;
mod yein;
mod text;
mod jeda;
mod jemo;
mod jedm;
mod act_;

pub use self::journal_type::*;
pub use self::jety::*;
pub use self::yeto::*;
pub use self::yein::*;
pub use self::text::*;
pub use self::jeda::*;
pub use self::jemo::*;
pub use self::jedm::*;
pub use self::act_::*;

use crate::binary::*;

esx_sub_record! {
  enum JourSubRecord {
    Jety(JourJety) => b"JETY",
    Yeto(JourYeto) => b"YETO",
    Yein(JourYein) => b"YEIN",
    Text(JourText) => b"TEXT",
    Jeda(JourJeda) => b"JEDA",
    Jemo(JourJemo) => b"JEMO",
    Jedm(JourJedm) => b"JEDM",
    Act_(JourAct_) => b"ACT_"
  }
}
