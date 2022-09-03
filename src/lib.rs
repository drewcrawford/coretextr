mod ctfontdescriptor;
mod ctfontmanager;

pub use ctfontdescriptor::*;
pub use ctfontmanager::*;

#[link(name = "CoreText", kind = "framework")]
extern "C" { }