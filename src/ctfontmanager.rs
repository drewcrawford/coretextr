use core_foundationr::{CFData, StrongCell};
use crate::CTFontDescriptor;

extern "C" {
    fn CTFontManagerCreateFontDescriptorFromData(data: &CFData) -> *const CTFontDescriptor;
}

pub struct CTFontManager {
    //currently unimplemented
}
#[allow(non_snake_case)]
impl CTFontManager {
    pub fn CreateFontDescriptorFromData(data: &CFData) -> Option<StrongCell<CTFontDescriptor>> {
        let raw = unsafe{ CTFontManagerCreateFontDescriptorFromData(data) };
        if raw.is_null() {
            None
        }
        else {
            Some(unsafe{ StrongCell::assuming_retained(raw) })
        }
    }
}

#[test] fn create_font() {
    let data = CFData::copy_slice(include_bytes!("../test_data/SquadaOne-Regular.ttf"));
    let font = CTFontManager::CreateFontDescriptorFromData(&data);
    assert!(font.is_some());
}

