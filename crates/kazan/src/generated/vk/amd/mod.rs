pub mod anti_lag;
pub mod buffer_marker;
pub mod device_coherent_memory;
pub mod display_native_hdr;
pub mod draw_indirect_count;
pub mod gcn_shader;
pub mod gpu_shader_half_float;
pub mod gpu_shader_int16;
pub mod memory_overallocation_behavior;
pub mod mixed_attachment_samples;
pub mod negative_viewport_height;
pub mod pipeline_compiler_control;
pub mod rasterization_order;
pub mod shader_ballot;
pub mod shader_core_properties;
pub mod shader_core_properties2;
pub mod shader_early_and_late_fragment_tests;
pub mod shader_explicit_vertex_parameter;
pub mod shader_fragment_mask;
pub mod shader_image_load_store_lod;
pub mod shader_info;
pub mod shader_trinary_minmax;
pub mod texture_gather_bias_lod;
pub(super) mod defs {
    use super::*;
    pub use anti_lag::defs::*;
    pub use buffer_marker::defs::*;
    pub use device_coherent_memory::defs::*;
    pub use display_native_hdr::defs::*;
    pub use draw_indirect_count::defs::*;
    pub use memory_overallocation_behavior::defs::*;
    pub use mixed_attachment_samples::defs::*;
    pub use pipeline_compiler_control::defs::*;
    pub use rasterization_order::defs::*;
    pub use shader_core_properties::defs::*;
    pub use shader_core_properties2::defs::*;
    pub use shader_early_and_late_fragment_tests::defs::*;
    pub use shader_info::defs::*;
    pub use texture_gather_bias_lod::defs::*;
}
#[cfg(feature = "ffi")]
pub(super) mod ffi {
    pub use super::anti_lag::ffi::*;
    pub use super::device_coherent_memory::ffi::*;
    pub use super::display_native_hdr::ffi::*;
    pub use super::memory_overallocation_behavior::ffi::*;
    pub use super::mixed_attachment_samples::ffi::*;
    pub use super::pipeline_compiler_control::ffi::*;
    pub use super::rasterization_order::ffi::*;
    pub use super::shader_core_properties::ffi::*;
    pub use super::shader_core_properties2::ffi::*;
    pub use super::shader_early_and_late_fragment_tests::ffi::*;
    pub use super::shader_info::ffi::*;
    pub use super::texture_gather_bias_lod::ffi::*;
}
