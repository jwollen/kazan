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
