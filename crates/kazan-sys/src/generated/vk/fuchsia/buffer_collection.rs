#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct BufferCollectionFUCHSIA(u64);
#[repr(C)]
pub struct ImportMemoryBufferCollectionFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct BufferCollectionImageCreateInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct BufferCollectionBufferCreateInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct BufferCollectionCreateInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub collection_token: zx_handle_t,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct BufferCollectionPropertiesFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
    pub buffer_count: u32,
    pub create_info_index: u32,
    pub sysmem_pixel_format: u64,
    pub format_features: FormatFeatureFlags,
    pub sysmem_color_space_index: SysmemColorSpaceFUCHSIA<'a>,
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    pub suggested_x_chroma_offset: ChromaLocation,
    pub suggested_y_chroma_offset: ChromaLocation,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct BufferConstraintsInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub create_info: BufferCreateInfo<'a>,
    pub required_format_features: FormatFeatureFlags,
    pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SysmemColorSpaceFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub color_space: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ImageFormatConstraintsInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_create_info: ImageCreateInfo<'a>,
    pub required_format_features: FormatFeatureFlags,
    pub flags: ImageFormatConstraintsFlagsFUCHSIA,
    pub sysmem_pixel_format: u64,
    pub color_space_count: u32,
    pub p_color_spaces: *const SysmemColorSpaceFUCHSIA<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ImageConstraintsInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format_constraints_count: u32,
    pub p_format_constraints: *const ImageFormatConstraintsInfoFUCHSIA<'a>,
    pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA<'a>,
    pub flags: ImageConstraintsInfoFlagsFUCHSIA,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct BufferCollectionConstraintsInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub min_buffer_count: u32,
    pub max_buffer_count: u32,
    pub min_buffer_count_for_camping: u32,
    pub min_buffer_count_for_dedicated_slack: u32,
    pub min_buffer_count_for_shared_slack: u32,
    pub _marker: PhantomData<&'a ()>,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ImageFormatConstraintsFlagsFUCHSIA: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ImageConstraintsInfoFlagsFUCHSIA: Flags {
        const CPU_READ_RARELY_FUCHSIA = ImageConstraintsInfoFlagBitsFUCHSIA::CPU_READ_RARELY_FUCHSIA.0;
        const CPU_READ_OFTEN_FUCHSIA = ImageConstraintsInfoFlagBitsFUCHSIA::CPU_READ_OFTEN_FUCHSIA.0;
        const CPU_WRITE_RARELY_FUCHSIA = ImageConstraintsInfoFlagBitsFUCHSIA::CPU_WRITE_RARELY_FUCHSIA.0;
        const CPU_WRITE_OFTEN_FUCHSIA = ImageConstraintsInfoFlagBitsFUCHSIA::CPU_WRITE_OFTEN_FUCHSIA.0;
        const PROTECTED_OPTIONAL_FUCHSIA = ImageConstraintsInfoFlagBitsFUCHSIA::PROTECTED_OPTIONAL_FUCHSIA.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageConstraintsInfoFlagBitsFUCHSIA(u32);
impl ImageConstraintsInfoFlagBitsFUCHSIA {
    pub const CPU_READ_RARELY_FUCHSIA: Self = Self(1 << 0);
    pub const CPU_READ_OFTEN_FUCHSIA: Self = Self(1 << 1);
    pub const CPU_WRITE_RARELY_FUCHSIA: Self = Self(1 << 2);
    pub const CPU_WRITE_OFTEN_FUCHSIA: Self = Self(1 << 3);
    pub const PROTECTED_OPTIONAL_FUCHSIA: Self = Self(1 << 4);
}
pub type PFN_vkCreateBufferCollectionFUCHSIA = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const BufferCollectionCreateInfoFUCHSIA<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_collection: *mut BufferCollectionFUCHSIA,
) -> Result;
pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA<'_>,
) -> Result;
pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA<'_>,
) -> Result;
pub type PFN_vkDestroyBufferCollectionFUCHSIA = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_properties: *mut BufferCollectionPropertiesFUCHSIA<'_>,
) -> Result;
