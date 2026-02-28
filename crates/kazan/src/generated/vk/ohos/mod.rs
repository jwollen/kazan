pub mod external_memory;
pub mod surface;
pub(super) mod defs {
    use super::*;
    pub use external_memory::defs::*;
    pub use surface::defs::*;
}
