#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub const ATTACHMENT_UNUSED: u32 = !0;
pub const SUBPASS_EXTERNAL: u32 = !0;
pub type SampleMask = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Framebuffer(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPass(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClearRect {
    pub rect: Rect2D,
    pub base_array_layer: u32,
    pub layer_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageBlit {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offsets: [Offset3D; 2],
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offsets: [Offset3D; 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageResolve {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VertexInputRate,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineVertexInputStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineVertexInputStateCreateFlags,
    pub vertex_binding_description_count: u32,
    pub p_vertex_binding_descriptions: *const VertexInputBindingDescription,
    pub vertex_attribute_description_count: u32,
    pub p_vertex_attribute_descriptions: *const VertexInputAttributeDescription,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineInputAssemblyStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineInputAssemblyStateCreateFlags,
    pub topology: PrimitiveTopology,
    pub primitive_restart_enable: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineTessellationStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineTessellationStateCreateFlags,
    pub patch_control_points: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineViewportStateCreateFlags,
    pub viewport_count: u32,
    pub p_viewports: *const Viewport,
    pub scissor_count: u32,
    pub p_scissors: *const Rect2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineRasterizationStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineRasterizationStateCreateFlags,
    pub depth_clamp_enable: Bool32,
    pub rasterizer_discard_enable: Bool32,
    pub polygon_mode: PolygonMode,
    pub cull_mode: CullModeFlags,
    pub front_face: FrontFace,
    pub depth_bias_enable: Bool32,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineMultisampleStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineMultisampleStateCreateFlags,
    pub rasterization_samples: SampleCountFlagBits,
    pub sample_shading_enable: Bool32,
    pub min_sample_shading: f32,
    pub p_sample_mask: *const SampleMask,
    pub alpha_to_coverage_enable: Bool32,
    pub alpha_to_one_enable: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineColorBlendAttachmentState {
    pub blend_enable: Bool32,
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
    pub color_write_mask: ColorComponentFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineColorBlendStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineColorBlendStateCreateFlags,
    pub logic_op_enable: Bool32,
    pub logic_op: LogicOp,
    pub attachment_count: u32,
    pub p_attachments: *const PipelineColorBlendAttachmentState,
    pub blend_constants: [f32; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineDynamicStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDynamicStateCreateFlags,
    pub dynamic_state_count: u32,
    pub p_dynamic_states: *const DynamicState,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StencilOpState {
    pub fail_op: StencilOp,
    pub pass_op: StencilOp,
    pub depth_fail_op: StencilOp,
    pub compare_op: CompareOp,
    pub compare_mask: u32,
    pub write_mask: u32,
    pub reference: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineDepthStencilStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDepthStencilStateCreateFlags,
    pub depth_test_enable: Bool32,
    pub depth_write_enable: Bool32,
    pub depth_compare_op: CompareOp,
    pub depth_bounds_test_enable: Bool32,
    pub stencil_test_enable: Bool32,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub min_depth_bounds: f32,
    pub max_depth_bounds: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GraphicsPipelineCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo,
    pub p_vertex_input_state: *const PipelineVertexInputStateCreateInfo,
    pub p_input_assembly_state: *const PipelineInputAssemblyStateCreateInfo,
    pub p_tessellation_state: *const PipelineTessellationStateCreateInfo,
    pub p_viewport_state: *const PipelineViewportStateCreateInfo,
    pub p_rasterization_state: *const PipelineRasterizationStateCreateInfo,
    pub p_multisample_state: *const PipelineMultisampleStateCreateInfo,
    pub p_depth_stencil_state: *const PipelineDepthStencilStateCreateInfo,
    pub p_color_blend_state: *const PipelineColorBlendStateCreateInfo,
    pub p_dynamic_state: *const PipelineDynamicStateCreateInfo,
    pub layout: PipelineLayout,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub render_pass: RenderPass,
    pub framebuffer: Framebuffer,
    pub render_area: Rect2D,
    pub clear_value_count: u32,
    pub p_clear_values: *const ClearValue,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClearAttachment {
    pub aspect_mask: ImageAspectFlags,
    pub color_attachment: u32,
    pub clear_value: ClearValue,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentDescription {
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlagBits,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub stencil_load_op: AttachmentLoadOp,
    pub stencil_store_op: AttachmentStoreOp,
    pub initial_layout: ImageLayout,
    pub final_layout: ImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentReference {
    pub attachment: u32,
    pub layout: ImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassDescription {
    pub flags: SubpassDescriptionFlags,
    pub pipeline_bind_point: PipelineBindPoint,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const AttachmentReference,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const AttachmentReference,
    pub p_resolve_attachments: *const AttachmentReference,
    pub p_depth_stencil_attachment: *const AttachmentReference,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassDependency {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: PipelineStageFlags,
    pub dst_stage_mask: PipelineStageFlags,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub dependency_flags: DependencyFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderPassCreateFlags,
    pub attachment_count: u32,
    pub p_attachments: *const AttachmentDescription,
    pub subpass_count: u32,
    pub p_subpasses: *const SubpassDescription,
    pub dependency_count: u32,
    pub p_dependencies: *const SubpassDependency,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FramebufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: FramebufferCreateFlags,
    pub render_pass: RenderPass,
    pub attachment_count: u32,
    pub p_attachments: *const ImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ClearValue {
    pub color: ClearColorValue,
    pub depth_stencil: ClearDepthStencilValue,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttachmentLoadOp(i32);
impl AttachmentLoadOp {
    pub const LOAD: Self = Self(0);
    pub const CLEAR: Self = Self(1);
    pub const DONT_CARE: Self = Self(2);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttachmentStoreOp(i32);
impl AttachmentStoreOp {
    pub const STORE: Self = Self(0);
    pub const DONT_CARE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrimitiveTopology(i32);
impl PrimitiveTopology {
    pub const POINT_LIST: Self = Self(0);
    pub const LINE_LIST: Self = Self(1);
    pub const LINE_STRIP: Self = Self(2);
    pub const TRIANGLE_LIST: Self = Self(3);
    pub const TRIANGLE_STRIP: Self = Self(4);
    pub const TRIANGLE_FAN: Self = Self(5);
    pub const LINE_LIST_WITH_ADJACENCY: Self = Self(6);
    pub const LINE_STRIP_WITH_ADJACENCY: Self = Self(7);
    pub const TRIANGLE_LIST_WITH_ADJACENCY: Self = Self(8);
    pub const TRIANGLE_STRIP_WITH_ADJACENCY: Self = Self(9);
    pub const PATCH_LIST: Self = Self(10);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompareOp(i32);
impl CompareOp {
    pub const NEVER: Self = Self(0);
    pub const LESS: Self = Self(1);
    pub const EQUAL: Self = Self(2);
    pub const LESS_OR_EQUAL: Self = Self(3);
    pub const GREATER: Self = Self(4);
    pub const NOT_EQUAL: Self = Self(5);
    pub const GREATER_OR_EQUAL: Self = Self(6);
    pub const ALWAYS: Self = Self(7);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolygonMode(i32);
impl PolygonMode {
    pub const FILL: Self = Self(0);
    pub const LINE: Self = Self(1);
    pub const POINT: Self = Self(2);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrontFace(i32);
impl FrontFace {
    pub const COUNTER_CLOCKWISE: Self = Self(0);
    pub const CLOCKWISE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlendFactor(i32);
impl BlendFactor {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const SRC_COLOR: Self = Self(2);
    pub const ONE_MINUS_SRC_COLOR: Self = Self(3);
    pub const DST_COLOR: Self = Self(4);
    pub const ONE_MINUS_DST_COLOR: Self = Self(5);
    pub const SRC_ALPHA: Self = Self(6);
    pub const ONE_MINUS_SRC_ALPHA: Self = Self(7);
    pub const DST_ALPHA: Self = Self(8);
    pub const ONE_MINUS_DST_ALPHA: Self = Self(9);
    pub const CONSTANT_COLOR: Self = Self(10);
    pub const ONE_MINUS_CONSTANT_COLOR: Self = Self(11);
    pub const CONSTANT_ALPHA: Self = Self(12);
    pub const ONE_MINUS_CONSTANT_ALPHA: Self = Self(13);
    pub const SRC_ALPHA_SATURATE: Self = Self(14);
    pub const SRC1_COLOR: Self = Self(15);
    pub const ONE_MINUS_SRC1_COLOR: Self = Self(16);
    pub const SRC1_ALPHA: Self = Self(17);
    pub const ONE_MINUS_SRC1_ALPHA: Self = Self(18);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlendOp(i32);
impl BlendOp {
    pub const ADD: Self = Self(0);
    pub const SUBTRACT: Self = Self(1);
    pub const REVERSE_SUBTRACT: Self = Self(2);
    pub const MIN: Self = Self(3);
    pub const MAX: Self = Self(4);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StencilOp(i32);
impl StencilOp {
    pub const KEEP: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const REPLACE: Self = Self(2);
    pub const INCREMENT_AND_CLAMP: Self = Self(3);
    pub const DECREMENT_AND_CLAMP: Self = Self(4);
    pub const INVERT: Self = Self(5);
    pub const INCREMENT_AND_WRAP: Self = Self(6);
    pub const DECREMENT_AND_WRAP: Self = Self(7);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogicOp(i32);
impl LogicOp {
    pub const CLEAR: Self = Self(0);
    pub const AND: Self = Self(1);
    pub const AND_REVERSE: Self = Self(2);
    pub const COPY: Self = Self(3);
    pub const AND_INVERTED: Self = Self(4);
    pub const NO_OP: Self = Self(5);
    pub const XOR: Self = Self(6);
    pub const OR: Self = Self(7);
    pub const NOR: Self = Self(8);
    pub const EQUIVALENT: Self = Self(9);
    pub const INVERT: Self = Self(10);
    pub const OR_REVERSE: Self = Self(11);
    pub const COPY_INVERTED: Self = Self(12);
    pub const OR_INVERTED: Self = Self(13);
    pub const NAND: Self = Self(14);
    pub const SET: Self = Self(15);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexInputRate(i32);
impl VertexInputRate {
    pub const VERTEX: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubpassContents(i32);
impl SubpassContents {
    pub const INLINE: Self = Self(0);
    pub const SECONDARY_COMMAND_BUFFERS: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DynamicState(i32);
impl DynamicState {
    pub const VIEWPORT: Self = Self(0);
    pub const SCISSOR: Self = Self(1);
    pub const LINE_WIDTH: Self = Self(2);
    pub const DEPTH_BIAS: Self = Self(3);
    pub const BLEND_CONSTANTS: Self = Self(4);
    pub const DEPTH_BOUNDS: Self = Self(5);
    pub const STENCIL_COMPARE_MASK: Self = Self(6);
    pub const STENCIL_WRITE_MASK: Self = Self(7);
    pub const STENCIL_REFERENCE: Self = Self(8);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct FramebufferCreateFlags: Flags {
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FramebufferCreateFlagBits(u32);
impl FramebufferCreateFlagBits {}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct RenderPassCreateFlags: Flags {
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderPassCreateFlagBits(u32);
impl RenderPassCreateFlagBits {}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineDepthStencilStateCreateFlags: Flags {
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineDepthStencilStateCreateFlagBits(u32);
impl PipelineDepthStencilStateCreateFlagBits {}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineDynamicStateCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineColorBlendStateCreateFlags: Flags {
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineColorBlendStateCreateFlagBits(u32);
impl PipelineColorBlendStateCreateFlagBits {}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineMultisampleStateCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineRasterizationStateCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineViewportStateCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineTessellationStateCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineInputAssemblyStateCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineVertexInputStateCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ColorComponentFlags: Flags {
        const R = ColorComponentFlagBits::R.0;
        const G = ColorComponentFlagBits::G.0;
        const B = ColorComponentFlagBits::B.0;
        const A = ColorComponentFlagBits::A.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorComponentFlagBits(u32);
impl ColorComponentFlagBits {
    pub const R: Self = Self(1 << 0);
    pub const G: Self = Self(1 << 1);
    pub const B: Self = Self(1 << 2);
    pub const A: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SubpassDescriptionFlags: Flags {
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubpassDescriptionFlagBits(u32);
impl SubpassDescriptionFlagBits {}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct AttachmentDescriptionFlags: Flags {
        const MAY_ALIAS = AttachmentDescriptionFlagBits::MAY_ALIAS.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttachmentDescriptionFlagBits(u32);
impl AttachmentDescriptionFlagBits {
    pub const MAY_ALIAS: Self = Self(1 << 0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct StencilFaceFlags: Flags {
        const FRONT = StencilFaceFlagBits::FRONT.0;
        const BACK = StencilFaceFlagBits::BACK.0;
        const FRONT_AND_BACK = 0x00000003;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StencilFaceFlagBits(u32);
impl StencilFaceFlagBits {
    pub const FRONT: Self = Self(1 << 0);
    pub const BACK: Self = Self(1 << 1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct CullModeFlags: Flags {
        const FRONT = CullModeFlagBits::FRONT.0;
        const BACK = CullModeFlagBits::BACK.0;
        const NONE = 0;
        const FRONT_AND_BACK = 0x00000003;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CullModeFlagBits(u32);
impl CullModeFlagBits {
    pub const FRONT: Self = Self(1 << 0);
    pub const BACK: Self = Self(1 << 1);
}
pub type PFN_vkCreateGraphicsPipelines = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const GraphicsPipelineCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> Result;
pub type PFN_vkCreateFramebuffer = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const FramebufferCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_framebuffer: *mut Framebuffer,
) -> Result;
pub type PFN_vkDestroyFramebuffer = unsafe extern "system" fn(
    device: Device,
    framebuffer: Framebuffer,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCreateRenderPass = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const RenderPassCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_render_pass: *mut RenderPass,
) -> Result;
pub type PFN_vkDestroyRenderPass = unsafe extern "system" fn(
    device: Device,
    render_pass: RenderPass,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkGetRenderAreaGranularity = unsafe extern "system" fn(
    device: Device,
    render_pass: RenderPass,
    p_granularity: *mut Extent2D,
);
pub type PFN_vkCmdSetViewport = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewports: *const Viewport,
);
pub type PFN_vkCmdSetScissor = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_scissor: u32,
    scissor_count: u32,
    p_scissors: *const Rect2D,
);
pub type PFN_vkCmdSetLineWidth =
    unsafe extern "system" fn(command_buffer: CommandBuffer, line_width: f32);
pub type PFN_vkCmdSetDepthBias = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    depth_bias_constant_factor: f32,
    depth_bias_clamp: f32,
    depth_bias_slope_factor: f32,
);
pub type PFN_vkCmdSetBlendConstants =
    unsafe extern "system" fn(command_buffer: CommandBuffer, blend_constants: *const [f32; 4]);
pub type PFN_vkCmdSetDepthBounds = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    min_depth_bounds: f32,
    max_depth_bounds: f32,
);
pub type PFN_vkCmdSetStencilCompareMask = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    compare_mask: u32,
);
pub type PFN_vkCmdSetStencilWriteMask = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    write_mask: u32,
);
pub type PFN_vkCmdSetStencilReference = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    reference: u32,
);
pub type PFN_vkCmdBindIndexBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    index_type: IndexType,
);
pub type PFN_vkCmdBindVertexBuffers = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const Buffer,
    p_offsets: *const DeviceSize,
);
pub type PFN_vkCmdDraw = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
);
pub type PFN_vkCmdDrawIndexed = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
);
pub type PFN_vkCmdDrawIndirect = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
pub type PFN_vkCmdDrawIndexedIndirect = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
pub type PFN_vkCmdBlitImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageBlit,
    filter: Filter,
);
pub type PFN_vkCmdClearDepthStencilImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    p_depth_stencil: *const ClearDepthStencilValue,
    range_count: u32,
    p_ranges: *const ImageSubresourceRange,
);
pub type PFN_vkCmdClearAttachments = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    attachment_count: u32,
    p_attachments: *const ClearAttachment,
    rect_count: u32,
    p_rects: *const ClearRect,
);
pub type PFN_vkCmdResolveImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageResolve,
);
pub type PFN_vkCmdBeginRenderPass = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_render_pass_begin: *const RenderPassBeginInfo,
    contents: SubpassContents,
);
pub type PFN_vkCmdNextSubpass =
    unsafe extern "system" fn(command_buffer: CommandBuffer, contents: SubpassContents);
pub type PFN_vkCmdEndRenderPass = unsafe extern "system" fn(command_buffer: CommandBuffer);
