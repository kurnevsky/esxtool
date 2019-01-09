use crate::binary::*;
use crate::esx::color_ref::ColorRef;

esx_sub_record_simple! {
  struct RegnCnam {
    color_ref: ColorRef
  }
}
