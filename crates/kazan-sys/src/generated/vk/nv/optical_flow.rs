#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct OpticalFlowSessionNV(u64);
#[repr(C)]
pub struct PhysicalDeviceOpticalFlowFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optical_flow: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceOpticalFlowPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_output_grid_sizes: OpticalFlowGridSizeFlagsNV,
    pub supported_hint_grid_sizes: OpticalFlowGridSizeFlagsNV,
    pub hint_supported: Bool32,
    pub cost_supported: Bool32,
    pub bidirectional_flow_supported: Bool32,
    pub global_flow_supported: Bool32,
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub max_num_regions_of_interest: u32,
}
#[repr(C)]
pub struct OpticalFlowImageFormatInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub usage: OpticalFlowUsageFlagsNV,
}
#[repr(C)]
pub struct OpticalFlowImageFormatPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format: Format,
}
#[repr(C)]
pub struct OpticalFlowSessionCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub width: u32,
    pub height: u32,
    pub image_format: Format,
    pub flow_vector_format: Format,
    pub cost_format: Format,
    pub output_grid_size: OpticalFlowGridSizeFlagsNV,
    pub hint_grid_size: OpticalFlowGridSizeFlagsNV,
    pub performance_level: OpticalFlowPerformanceLevelNV,
    pub flags: OpticalFlowSessionCreateFlagsNV,
}
#[repr(C)]
pub struct OpticalFlowSessionCreatePrivateDataInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub id: u32,
    pub size: u32,
    pub p_private_data: *const c_void,
}
#[repr(C)]
pub struct OpticalFlowExecuteInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: OpticalFlowExecuteFlagsNV,
    pub region_count: u32,
    pub p_regions: *const Rect2D,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpticalFlowPerformanceLevelNV(i32);
impl OpticalFlowPerformanceLevelNV {
    pub const UNKNOWN_NV: Self = Self(0);
    pub const SLOW_NV: Self = Self(1);
    pub const MEDIUM_NV: Self = Self(2);
    pub const FAST_NV: Self = Self(3);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpticalFlowSessionBindingPointNV(i32);
impl OpticalFlowSessionBindingPointNV {
    pub const UNKNOWN_NV: Self = Self(0);
    pub const INPUT_NV: Self = Self(1);
    pub const REFERENCE_NV: Self = Self(2);
    pub const HINT_NV: Self = Self(3);
    pub const FLOW_VECTOR_NV: Self = Self(4);
    pub const BACKWARD_FLOW_VECTOR_NV: Self = Self(5);
    pub const COST_NV: Self = Self(6);
    pub const BACKWARD_COST_NV: Self = Self(7);
    pub const GLOBAL_FLOW_NV: Self = Self(8);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct OpticalFlowGridSizeFlagsNV: Flags {
        const _1X1_NV = OpticalFlowGridSizeFlagBitsNV::_1X1_NV.0;
        const _2X2_NV = OpticalFlowGridSizeFlagBitsNV::_2X2_NV.0;
        const _4X4_NV = OpticalFlowGridSizeFlagBitsNV::_4X4_NV.0;
        const _8X8_NV = OpticalFlowGridSizeFlagBitsNV::_8X8_NV.0;
        const UNKNOWN = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpticalFlowGridSizeFlagBitsNV(u32);
impl OpticalFlowGridSizeFlagBitsNV {
    pub const _1X1_NV: Self = Self(1 << 0);
    pub const _2X2_NV: Self = Self(1 << 1);
    pub const _4X4_NV: Self = Self(1 << 2);
    pub const _8X8_NV: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct OpticalFlowUsageFlagsNV: Flags {
        const INPUT_NV = OpticalFlowUsageFlagBitsNV::INPUT_NV.0;
        const OUTPUT_NV = OpticalFlowUsageFlagBitsNV::OUTPUT_NV.0;
        const HINT_NV = OpticalFlowUsageFlagBitsNV::HINT_NV.0;
        const COST_NV = OpticalFlowUsageFlagBitsNV::COST_NV.0;
        const GLOBAL_FLOW_NV = OpticalFlowUsageFlagBitsNV::GLOBAL_FLOW_NV.0;
        const UNKNOWN = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpticalFlowUsageFlagBitsNV(u32);
impl OpticalFlowUsageFlagBitsNV {
    pub const INPUT_NV: Self = Self(1 << 0);
    pub const OUTPUT_NV: Self = Self(1 << 1);
    pub const HINT_NV: Self = Self(1 << 2);
    pub const COST_NV: Self = Self(1 << 3);
    pub const GLOBAL_FLOW_NV: Self = Self(1 << 4);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct OpticalFlowSessionCreateFlagsNV: Flags {
        const ENABLE_HINT_NV = OpticalFlowSessionCreateFlagBitsNV::ENABLE_HINT_NV.0;
        const ENABLE_COST_NV = OpticalFlowSessionCreateFlagBitsNV::ENABLE_COST_NV.0;
        const ENABLE_GLOBAL_FLOW_NV = OpticalFlowSessionCreateFlagBitsNV::ENABLE_GLOBAL_FLOW_NV.0;
        const ALLOW_REGIONS_NV = OpticalFlowSessionCreateFlagBitsNV::ALLOW_REGIONS_NV.0;
        const BOTH_DIRECTIONS_NV = OpticalFlowSessionCreateFlagBitsNV::BOTH_DIRECTIONS_NV.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpticalFlowSessionCreateFlagBitsNV(u32);
impl OpticalFlowSessionCreateFlagBitsNV {
    pub const ENABLE_HINT_NV: Self = Self(1 << 0);
    pub const ENABLE_COST_NV: Self = Self(1 << 1);
    pub const ENABLE_GLOBAL_FLOW_NV: Self = Self(1 << 2);
    pub const ALLOW_REGIONS_NV: Self = Self(1 << 3);
    pub const BOTH_DIRECTIONS_NV: Self = Self(1 << 4);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct OpticalFlowExecuteFlagsNV: Flags {
        const DISABLE_TEMPORAL_HINTS_NV = OpticalFlowExecuteFlagBitsNV::DISABLE_TEMPORAL_HINTS_NV.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpticalFlowExecuteFlagBitsNV(u32);
impl OpticalFlowExecuteFlagBitsNV {
    pub const DISABLE_TEMPORAL_HINTS_NV: Self = Self(1 << 0);
}
pub type PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_optical_flow_image_format_info: *const OpticalFlowImageFormatInfoNV,
    p_format_count: *mut u32,
    p_image_format_properties: *mut OpticalFlowImageFormatPropertiesNV,
) -> Result;
pub type PFN_vkCreateOpticalFlowSessionNV = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const OpticalFlowSessionCreateInfoNV,
    p_allocator: *const AllocationCallbacks,
    p_session: *mut OpticalFlowSessionNV,
) -> Result;
pub type PFN_vkDestroyOpticalFlowSessionNV = unsafe extern "system" fn(
    device: Device,
    session: OpticalFlowSessionNV,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkBindOpticalFlowSessionImageNV = unsafe extern "system" fn(
    device: Device,
    session: OpticalFlowSessionNV,
    binding_point: OpticalFlowSessionBindingPointNV,
    view: ImageView,
    layout: ImageLayout,
) -> Result;
pub type PFN_vkCmdOpticalFlowExecuteNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    session: OpticalFlowSessionNV,
    p_execute_info: *const OpticalFlowExecuteInfoNV,
);
