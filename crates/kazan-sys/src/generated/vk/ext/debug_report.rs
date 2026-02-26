#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DebugReportCallbackEXT(u64);
#[repr(C)]
pub struct DebugReportCallbackCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DebugReportFlagsEXT,
    pub pfn_callback: Option<PFN_vkDebugReportCallbackEXT>,
    pub p_user_data: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugReportObjectTypeEXT(i32);
impl DebugReportObjectTypeEXT {
    pub const UNKNOWN_EXT: Self = Self(0);
    pub const INSTANCE_EXT: Self = Self(1);
    pub const PHYSICAL_DEVICE_EXT: Self = Self(2);
    pub const DEVICE_EXT: Self = Self(3);
    pub const QUEUE_EXT: Self = Self(4);
    pub const SEMAPHORE_EXT: Self = Self(5);
    pub const COMMAND_BUFFER_EXT: Self = Self(6);
    pub const FENCE_EXT: Self = Self(7);
    pub const DEVICE_MEMORY_EXT: Self = Self(8);
    pub const BUFFER_EXT: Self = Self(9);
    pub const IMAGE_EXT: Self = Self(10);
    pub const EVENT_EXT: Self = Self(11);
    pub const QUERY_POOL_EXT: Self = Self(12);
    pub const BUFFER_VIEW_EXT: Self = Self(13);
    pub const IMAGE_VIEW_EXT: Self = Self(14);
    pub const SHADER_MODULE_EXT: Self = Self(15);
    pub const PIPELINE_CACHE_EXT: Self = Self(16);
    pub const PIPELINE_LAYOUT_EXT: Self = Self(17);
    pub const RENDER_PASS_EXT: Self = Self(18);
    pub const PIPELINE_EXT: Self = Self(19);
    pub const DESCRIPTOR_SET_LAYOUT_EXT: Self = Self(20);
    pub const SAMPLER_EXT: Self = Self(21);
    pub const DESCRIPTOR_POOL_EXT: Self = Self(22);
    pub const DESCRIPTOR_SET_EXT: Self = Self(23);
    pub const FRAMEBUFFER_EXT: Self = Self(24);
    pub const COMMAND_POOL_EXT: Self = Self(25);
    pub const SURFACE_KHR_EXT: Self = Self(26);
    pub const SWAPCHAIN_KHR_EXT: Self = Self(27);
    pub const DEBUG_REPORT_CALLBACK_EXT_EXT: Self = Self(28);
    pub const DISPLAY_KHR_EXT: Self = Self(29);
    pub const DISPLAY_MODE_KHR_EXT: Self = Self(30);
    pub const VALIDATION_CACHE_EXT_EXT: Self = Self(33);
    pub const ACCELERATION_STRUCTURE_KHR_EXT: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_NV_EXT: Self = Self(1000165000);
    pub const BUFFER_COLLECTION_FUCHSIA_EXT: Self = Self(1000366000);
    pub const CUDA_FUNCTION_NV_EXT: Self = Self(1000307001);
    pub const CUDA_MODULE_NV_EXT: Self = Self(1000307000);
    pub const CU_FUNCTION_NVX_EXT: Self = Self(1000029001);
    pub const CU_MODULE_NVX_EXT: Self = Self(1000029000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_EXT: Self = Self(1000085000);
    pub const SAMPLER_YCBCR_CONVERSION_EXT: Self = Self(1000156000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT: Self = Self::DESCRIPTOR_UPDATE_TEMPLATE_EXT;
    pub const SAMPLER_YCBCR_CONVERSION_KHR_EXT: Self = Self::SAMPLER_YCBCR_CONVERSION_EXT;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DebugReportFlagsEXT: Flags {
        const INFORMATION_EXT = DebugReportFlagBitsEXT::INFORMATION_EXT.0;
        const WARNING_EXT = DebugReportFlagBitsEXT::WARNING_EXT.0;
        const PERFORMANCE_WARNING_EXT = DebugReportFlagBitsEXT::PERFORMANCE_WARNING_EXT.0;
        const ERROR_EXT = DebugReportFlagBitsEXT::ERROR_EXT.0;
        const DEBUG_EXT = DebugReportFlagBitsEXT::DEBUG_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugReportFlagBitsEXT(u32);
impl DebugReportFlagBitsEXT {
    pub const INFORMATION_EXT: Self = Self(1 << 0);
    pub const WARNING_EXT: Self = Self(1 << 1);
    pub const PERFORMANCE_WARNING_EXT: Self = Self(1 << 2);
    pub const ERROR_EXT: Self = Self(1 << 3);
    pub const DEBUG_EXT: Self = Self(1 << 4);
}
pub type PFN_vkDebugReportCallbackEXT = unsafe extern "system" fn(
    flags: DebugReportFlagsEXT,
    object_type: DebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    p_layer_prefix: *const c_char,
    p_message: *const c_char,
    p_user_data: *mut c_void,
) -> Bool32;
pub type PFN_vkCreateDebugReportCallbackEXT = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DebugReportCallbackCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_callback: *mut DebugReportCallbackEXT,
) -> Result;
pub type PFN_vkDestroyDebugReportCallbackEXT = unsafe extern "system" fn(
    instance: Instance,
    callback: DebugReportCallbackEXT,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkDebugReportMessageEXT = unsafe extern "system" fn(
    instance: Instance,
    flags: DebugReportFlagsEXT,
    object_type: DebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    p_layer_prefix: *const c_char,
    p_message: *const c_char,
);
