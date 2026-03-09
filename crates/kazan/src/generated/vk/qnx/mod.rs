pub mod external_memory_screen_buffer;
pub mod screen_surface;
pub(super) mod defs {
    use super::*;
    pub use external_memory_screen_buffer::defs::*;
    pub use screen_surface::defs::*;
}
#[cfg(feature = "ffi")]
pub(super) mod ffi {
    pub use super::external_memory_screen_buffer::ffi::*;
    pub use super::screen_surface::ffi::*;
}
