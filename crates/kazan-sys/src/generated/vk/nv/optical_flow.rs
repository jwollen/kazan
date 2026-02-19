#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpticalFlowSessionNV(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceOpticalFlowFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optical_flow: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct OpticalFlowImageFormatInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub usage: OpticalFlowUsageFlagsNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpticalFlowImageFormatPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format: Format,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct OpticalFlowSessionCreatePrivateDataInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub id: u32,
    pub size: u32,
    pub p_private_data: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpticalFlowExecuteInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: OpticalFlowExecuteFlagsNV,
    pub region_count: u32,
    pub p_regions: *const Rect2D,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpticalFlowPerformanceLevelNV(i32);
impl OpticalFlowPerformanceLevelNV {
    pub const UNKNOWN_NV: Self = Self(0);
    pub const SLOW_NV: Self = Self(1);
    pub const MEDIUM_NV: Self = Self(2);
    pub const FAST_NV: Self = Self(3);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        const _1X1_NV = 1 << 0;
        const _2X2_NV = 1 << 1;
        const _4X4_NV = 1 << 2;
        const _8X8_NV = 1 << 3;
        const UNKNOWN = 0;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct OpticalFlowUsageFlagsNV: Flags {
        const INPUT_NV = 1 << 0;
        const OUTPUT_NV = 1 << 1;
        const HINT_NV = 1 << 2;
        const COST_NV = 1 << 3;
        const GLOBAL_FLOW_NV = 1 << 4;
        const UNKNOWN = 0;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct OpticalFlowSessionCreateFlagsNV: Flags {
        const ENABLE_HINT_NV = 1 << 0;
        const ENABLE_COST_NV = 1 << 1;
        const ENABLE_GLOBAL_FLOW_NV = 1 << 2;
        const ALLOW_REGIONS_NV = 1 << 3;
        const BOTH_DIRECTIONS_NV = 1 << 4;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct OpticalFlowExecuteFlagsNV: Flags {
        const DISABLE_TEMPORAL_HINTS_NV = 1 << 0;
    }
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
