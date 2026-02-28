pub mod dense_geometry_format;
pub mod shader_enqueue;
pub(super) mod defs {
    use super::*;
    pub use dense_geometry_format::defs::*;
    pub use shader_enqueue::defs::*;
}
