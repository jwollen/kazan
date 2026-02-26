#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance7FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance7: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMaintenance7FeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            maintenance7: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance7PropertiesKHR<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMaintenance7PropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            robust_fragment_shading_rate_attachment_access: Default::default(),
            separate_depth_stencil_attachment_access: Default::default(),
            max_descriptor_set_total_uniform_buffers_dynamic: Default::default(),
            max_descriptor_set_total_storage_buffers_dynamic: Default::default(),
            max_descriptor_set_total_buffers_dynamic: Default::default(),
            max_descriptor_set_update_after_bind_total_uniform_buffers_dynamic: Default::default(),
            max_descriptor_set_update_after_bind_total_storage_buffers_dynamic: Default::default(),
            max_descriptor_set_update_after_bind_total_buffers_dynamic: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLayeredApiPropertiesListKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub layered_api_count: u32,
    pub p_layered_apis: *mut PhysicalDeviceLayeredApiPropertiesKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceLayeredApiPropertiesListKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR,
            p_next: core::ptr::null_mut(),
            layered_api_count: Default::default(),
            p_layered_apis: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLayeredApiPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub vendor_id: u32,
    pub device_id: u32,
    pub layered_api: PhysicalDeviceLayeredApiKHR,
    pub device_name: [c_char; MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceLayeredApiPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            vendor_id: Default::default(),
            device_id: Default::default(),
            layered_api: Default::default(),
            device_name: [Default::default(); _],
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLayeredApiVulkanPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub properties: PhysicalDeviceProperties2<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceLayeredApiVulkanPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            properties: Default::default(),
            _marker: PhantomData,
        }
    }
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
