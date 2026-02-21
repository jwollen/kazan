#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalImageFormatPropertiesNV {
    pub image_format_properties: ImageFormatProperties,
    pub external_memory_features: ExternalMemoryFeatureFlagsNV,
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlagsNV,
    pub compatible_handle_types: ExternalMemoryHandleTypeFlagsNV,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ExternalMemoryHandleTypeFlagsNV: Flags {
        const OPAQUE_WIN32_NV = 1 << 0;
        const OPAQUE_WIN32_KMT_NV = 1 << 1;
        const D3D11_IMAGE_NV = 1 << 2;
        const D3D11_IMAGE_KMT_NV = 1 << 3;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ExternalMemoryFeatureFlagsNV: Flags {
        const DEDICATED_ONLY_NV = 1 << 0;
        const EXPORTABLE_NV = 1 << 1;
        const IMPORTABLE_NV = 1 << 2;
    }
}
pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        external_handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_external_image_format_properties: *mut ExternalImageFormatPropertiesNV,
    ) -> Result;
