pub mod data_graph;
pub mod data_graph_instruction_set_tosa;
pub mod data_graph_neural_accelerator_statistics;
pub mod data_graph_optical_flow;
pub mod format_pack;
pub mod performance_counters_by_region;
pub mod pipeline_opacity_micromap;
pub mod rasterization_order_attachment_access;
pub mod render_pass_striped;
pub mod scheduling_controls;
pub mod shader_core_builtins;
pub mod shader_core_properties;
pub mod shader_instrumentation;
pub mod tensors;
pub(super) mod defs {
    use super::*;
    pub use data_graph::defs::*;
    pub use data_graph_instruction_set_tosa::defs::*;
    pub use data_graph_neural_accelerator_statistics::defs::*;
    pub use data_graph_optical_flow::defs::*;
    pub use format_pack::defs::*;
    pub use performance_counters_by_region::defs::*;
    pub use pipeline_opacity_micromap::defs::*;
    pub use rasterization_order_attachment_access::defs::*;
    pub use render_pass_striped::defs::*;
    pub use scheduling_controls::defs::*;
    pub use shader_core_builtins::defs::*;
    pub use shader_core_properties::defs::*;
    pub use shader_instrumentation::defs::*;
    pub use tensors::defs::*;
}
#[cfg(feature = "ffi")]
pub(super) mod ffi {
    pub use super::data_graph::ffi::*;
    pub use super::data_graph_instruction_set_tosa::ffi::*;
    pub use super::data_graph_neural_accelerator_statistics::ffi::*;
    pub use super::data_graph_optical_flow::ffi::*;
    pub use super::format_pack::ffi::*;
    pub use super::performance_counters_by_region::ffi::*;
    pub use super::pipeline_opacity_micromap::ffi::*;
    pub use super::rasterization_order_attachment_access::ffi::*;
    pub use super::render_pass_striped::ffi::*;
    pub use super::scheduling_controls::ffi::*;
    pub use super::shader_core_builtins::ffi::*;
    pub use super::shader_core_properties::ffi::*;
    pub use super::shader_instrumentation::ffi::*;
    pub use super::tensors::ffi::*;
}
