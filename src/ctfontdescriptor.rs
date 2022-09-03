use std::ffi::c_void;
use core_foundationr::CFType;

#[repr(C)]
pub struct CTFontDescriptor(c_void);
impl CFType for CTFontDescriptor {}