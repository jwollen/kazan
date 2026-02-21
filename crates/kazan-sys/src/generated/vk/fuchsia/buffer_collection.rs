#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCollectionFUCHSIA(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMemoryBufferCollectionFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCollectionImageCreateInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCollectionBufferCreateInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCollectionCreateInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub collection_token: zx_handle_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCollectionPropertiesFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
    pub buffer_count: u32,
    pub create_info_index: u32,
    pub sysmem_pixel_format: u64,
    pub format_features: FormatFeatureFlags,
    pub sysmem_color_space_index: SysmemColorSpaceFUCHSIA,
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    pub suggested_x_chroma_offset: ChromaLocation,
    pub suggested_y_chroma_offset: ChromaLocation,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferConstraintsInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub create_info: BufferCreateInfo,
    pub required_format_features: FormatFeatureFlags,
    pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SysmemColorSpaceFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub color_space: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageFormatConstraintsInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_create_info: ImageCreateInfo,
    pub required_format_features: FormatFeatureFlags,
    pub flags: ImageFormatConstraintsFlagsFUCHSIA,
    pub sysmem_pixel_format: u64,
    pub color_space_count: u32,
    pub p_color_spaces: *const SysmemColorSpaceFUCHSIA,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageConstraintsInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format_constraints_count: u32,
    pub p_format_constraints: *const ImageFormatConstraintsInfoFUCHSIA,
    pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA,
    pub flags: ImageConstraintsInfoFlagsFUCHSIA,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCollectionConstraintsInfoFUCHSIA {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub min_buffer_count: u32,
    pub max_buffer_count: u32,
    pub min_buffer_count_for_camping: u32,
    pub min_buffer_count_for_dedicated_slack: u32,
    pub min_buffer_count_for_shared_slack: u32,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ImageFormatConstraintsFlagsFUCHSIA: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ImageConstraintsInfoFlagsFUCHSIA: Flags {
        const CPU_READ_RARELY_FUCHSIA = 1 << 0;
        const CPU_READ_OFTEN_FUCHSIA = 1 << 1;
        const CPU_WRITE_RARELY_FUCHSIA = 1 << 2;
        const CPU_WRITE_OFTEN_FUCHSIA = 1 << 3;
        const PROTECTED_OPTIONAL_FUCHSIA = 1 << 4;
    }
}
pub type PFN_vkCreateBufferCollectionFUCHSIA = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const BufferCollectionCreateInfoFUCHSIA,
    p_allocator: *const AllocationCallbacks,
    p_collection: *mut BufferCollectionFUCHSIA,
) -> Result;
pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA,
) -> Result;
pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA,
) -> Result;
pub type PFN_vkDestroyBufferCollectionFUCHSIA = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_properties: *mut BufferCollectionPropertiesFUCHSIA,
) -> Result;
