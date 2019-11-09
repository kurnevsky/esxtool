use super::journal_type::JournalType;
use crate::binary::*;

esx_sub_record_simple! {
  struct JourJety {
    journal_type: JournalType
  }
}
