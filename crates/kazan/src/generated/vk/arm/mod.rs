pub mod data_graph;
pub mod format_pack;
pub mod performance_counters_by_region;
pub mod pipeline_opacity_micromap;
pub mod rasterization_order_attachment_access;
pub mod render_pass_striped;
pub mod scheduling_controls;
pub mod shader_core_builtins;
pub mod shader_core_properties;
pub mod tensors;
pub(super) mod defs {
    use super::*;
    pub use data_graph::defs::*;
    pub use format_pack::defs::*;
    pub use performance_counters_by_region::defs::*;
    pub use pipeline_opacity_micromap::defs::*;
    pub use rasterization_order_attachment_access::defs::*;
    pub use render_pass_striped::defs::*;
    pub use scheduling_controls::defs::*;
    pub use shader_core_builtins::defs::*;
    pub use shader_core_properties::defs::*;
    pub use tensors::defs::*;
}
