pub mod external_memory;
pub mod surface;
pub(super) mod defs {
    use super::*;
    pub use external_memory::defs::*;
    pub use surface::defs::*;
}
#[cfg(feature = "ffi")]
pub(super) mod ffi {
    pub use super::external_memory::ffi::*;
    pub use super::surface::ffi::*;
}
