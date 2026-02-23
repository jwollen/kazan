#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceMaintenance7FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance7: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceMaintenance7PropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub robust_fragment_shading_rate_attachment_access: Bool32,
    pub separate_depth_stencil_attachment_access: Bool32,
    pub max_descriptor_set_total_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_total_storage_buffers_dynamic: u32,
    pub max_descriptor_set_total_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_total_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_total_storage_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_total_buffers_dynamic: u32,
}
#[repr(C)]
pub struct PhysicalDeviceLayeredApiPropertiesListKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub layered_api_count: u32,
    pub p_layered_apis: *mut PhysicalDeviceLayeredApiPropertiesKHR,
}
#[repr(C)]
pub struct PhysicalDeviceLayeredApiPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub vendor_id: u32,
    pub device_id: u32,
    pub layered_api: PhysicalDeviceLayeredApiKHR,
    pub device_name: [c_char; MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
}
#[repr(C)]
pub struct PhysicalDeviceLayeredApiVulkanPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub properties: PhysicalDeviceProperties2,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalDeviceLayeredApiKHR(i32);
impl PhysicalDeviceLayeredApiKHR {
    pub const VULKAN_KHR: Self = Self(0);
    pub const D3D12_KHR: Self = Self(1);
    pub const METAL_KHR: Self = Self(2);
    pub const OPENGL_KHR: Self = Self(3);
    pub const OPENGLES_KHR: Self = Self(4);
}
