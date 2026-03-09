pub mod descriptor_set_host_mapping;
pub mod fragment_density_map_layered;
pub mod mutable_descriptor_type;
pub mod video_encode_rgb_conversion;
pub(super) mod defs {
    use super::*;
    pub use descriptor_set_host_mapping::defs::*;
    pub use fragment_density_map_layered::defs::*;
    pub use mutable_descriptor_type::defs::*;
    pub use video_encode_rgb_conversion::defs::*;
}
#[cfg(feature = "ffi")]
pub(super) mod ffi {
    pub use super::descriptor_set_host_mapping::ffi::*;
    pub use super::fragment_density_map_layered::ffi::*;
    pub use super::mutable_descriptor_type::ffi::*;
    pub use super::video_encode_rgb_conversion::ffi::*;
}
