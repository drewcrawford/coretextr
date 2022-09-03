mod ctfontdescriptor;
mod ctfontmanager;

pub use ctfontdescriptor::*;
pub use ctfontmanager::*;

pub type StrongCell<T> = core_foundationr::StrongCell<T>;

#[link(name = "CoreText", kind = "framework")]
extern "C" { }