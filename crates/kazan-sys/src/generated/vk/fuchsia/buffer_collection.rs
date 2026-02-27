#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct BufferCollectionFUCHSIA(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMemoryBufferCollectionFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportMemoryBufferCollectionFUCHSIA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA,
            p_next: core::ptr::null(),
            collection: Default::default(),
            index: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ImportMemoryBufferCollectionFUCHSIA<'a> {
    pub fn collection(mut self, collection: BufferCollectionFUCHSIA) -> Self {
        self.collection = collection;
        self
    }
    pub fn index(mut self, index: u32) -> Self {
        self.index = index;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCollectionImageCreateInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BufferCollectionImageCreateInfoFUCHSIA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA,
            p_next: core::ptr::null(),
            collection: Default::default(),
            index: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> BufferCollectionImageCreateInfoFUCHSIA<'a> {
    pub fn collection(mut self, collection: BufferCollectionFUCHSIA) -> Self {
        self.collection = collection;
        self
    }
    pub fn index(mut self, index: u32) -> Self {
        self.index = index;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCollectionBufferCreateInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub collection: BufferCollectionFUCHSIA,
    pub index: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BufferCollectionBufferCreateInfoFUCHSIA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA,
            p_next: core::ptr::null(),
            collection: Default::default(),
            index: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> BufferCollectionBufferCreateInfoFUCHSIA<'a> {
    pub fn collection(mut self, collection: BufferCollectionFUCHSIA) -> Self {
        self.collection = collection;
        self
    }
    pub fn index(mut self, index: u32) -> Self {
        self.index = index;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCollectionCreateInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub collection_token: zx_handle_t,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BufferCollectionCreateInfoFUCHSIA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BUFFER_COLLECTION_CREATE_INFO_FUCHSIA,
            p_next: core::ptr::null(),
            collection_token: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> BufferCollectionCreateInfoFUCHSIA<'a> {
    pub fn collection_token(mut self, collection_token: zx_handle_t) -> Self {
        self.collection_token = collection_token;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for BufferCollectionPropertiesFUCHSIA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BUFFER_COLLECTION_PROPERTIES_FUCHSIA,
            p_next: core::ptr::null_mut(),
            memory_type_bits: Default::default(),
            buffer_count: Default::default(),
            create_info_index: Default::default(),
            sysmem_pixel_format: Default::default(),
            format_features: Default::default(),
            sysmem_color_space_index: Default::default(),
            sampler_ycbcr_conversion_components: Default::default(),
            suggested_ycbcr_model: Default::default(),
            suggested_ycbcr_range: Default::default(),
            suggested_x_chroma_offset: Default::default(),
            suggested_y_chroma_offset: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> BufferCollectionPropertiesFUCHSIA<'a> {
    pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
        self.memory_type_bits = memory_type_bits;
        self
    }
    pub fn buffer_count(mut self, buffer_count: u32) -> Self {
        self.buffer_count = buffer_count;
        self
    }
    pub fn create_info_index(mut self, create_info_index: u32) -> Self {
        self.create_info_index = create_info_index;
        self
    }
    pub fn sysmem_pixel_format(mut self, sysmem_pixel_format: u64) -> Self {
        self.sysmem_pixel_format = sysmem_pixel_format;
        self
    }
    pub fn format_features(mut self, format_features: FormatFeatureFlags) -> Self {
        self.format_features = format_features;
        self
    }
    pub fn sysmem_color_space_index(
        mut self,
        sysmem_color_space_index: SysmemColorSpaceFUCHSIA<'a>,
    ) -> Self {
        self.sysmem_color_space_index = sysmem_color_space_index;
        self
    }
    pub fn sampler_ycbcr_conversion_components(
        mut self,
        sampler_ycbcr_conversion_components: ComponentMapping,
    ) -> Self {
        self.sampler_ycbcr_conversion_components = sampler_ycbcr_conversion_components;
        self
    }
    pub fn suggested_ycbcr_model(
        mut self,
        suggested_ycbcr_model: SamplerYcbcrModelConversion,
    ) -> Self {
        self.suggested_ycbcr_model = suggested_ycbcr_model;
        self
    }
    pub fn suggested_ycbcr_range(mut self, suggested_ycbcr_range: SamplerYcbcrRange) -> Self {
        self.suggested_ycbcr_range = suggested_ycbcr_range;
        self
    }
    pub fn suggested_x_chroma_offset(mut self, suggested_x_chroma_offset: ChromaLocation) -> Self {
        self.suggested_x_chroma_offset = suggested_x_chroma_offset;
        self
    }
    pub fn suggested_y_chroma_offset(mut self, suggested_y_chroma_offset: ChromaLocation) -> Self {
        self.suggested_y_chroma_offset = suggested_y_chroma_offset;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferConstraintsInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub create_info: BufferCreateInfo<'a>,
    pub required_format_features: FormatFeatureFlags,
    pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BufferConstraintsInfoFUCHSIA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BUFFER_CONSTRAINTS_INFO_FUCHSIA,
            p_next: core::ptr::null(),
            create_info: Default::default(),
            required_format_features: Default::default(),
            buffer_collection_constraints: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> BufferConstraintsInfoFUCHSIA<'a> {
    pub fn create_info(mut self, create_info: BufferCreateInfo<'a>) -> Self {
        self.create_info = create_info;
        self
    }
    pub fn required_format_features(
        mut self,
        required_format_features: FormatFeatureFlags,
    ) -> Self {
        self.required_format_features = required_format_features;
        self
    }
    pub fn buffer_collection_constraints(
        mut self,
        buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA<'a>,
    ) -> Self {
        self.buffer_collection_constraints = buffer_collection_constraints;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SysmemColorSpaceFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub color_space: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SysmemColorSpaceFUCHSIA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SYSMEM_COLOR_SPACE_FUCHSIA,
            p_next: core::ptr::null(),
            color_space: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SysmemColorSpaceFUCHSIA<'a> {
    pub fn color_space(mut self, color_space: u32) -> Self {
        self.color_space = color_space;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for ImageFormatConstraintsInfoFUCHSIA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA,
            p_next: core::ptr::null(),
            image_create_info: Default::default(),
            required_format_features: Default::default(),
            flags: Default::default(),
            sysmem_pixel_format: Default::default(),
            color_space_count: Default::default(),
            p_color_spaces: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ImageFormatConstraintsInfoFUCHSIA<'a> {
    pub fn image_create_info(mut self, image_create_info: ImageCreateInfo<'a>) -> Self {
        self.image_create_info = image_create_info;
        self
    }
    pub fn required_format_features(
        mut self,
        required_format_features: FormatFeatureFlags,
    ) -> Self {
        self.required_format_features = required_format_features;
        self
    }
    pub fn flags(mut self, flags: ImageFormatConstraintsFlagsFUCHSIA) -> Self {
        self.flags = flags;
        self
    }
    pub fn sysmem_pixel_format(mut self, sysmem_pixel_format: u64) -> Self {
        self.sysmem_pixel_format = sysmem_pixel_format;
        self
    }
    pub fn color_spaces(mut self, color_spaces: &'a [SysmemColorSpaceFUCHSIA<'a>]) -> Self {
        self.color_space_count = color_spaces.len().try_into().unwrap();
        self.p_color_spaces = color_spaces.as_ptr();
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageConstraintsInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format_constraints_count: u32,
    pub p_format_constraints: *const ImageFormatConstraintsInfoFUCHSIA<'a>,
    pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA<'a>,
    pub flags: ImageConstraintsInfoFlagsFUCHSIA,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageConstraintsInfoFUCHSIA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_CONSTRAINTS_INFO_FUCHSIA,
            p_next: core::ptr::null(),
            format_constraints_count: Default::default(),
            p_format_constraints: core::ptr::null(),
            buffer_collection_constraints: Default::default(),
            flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ImageConstraintsInfoFUCHSIA<'a> {
    pub fn format_constraints(
        mut self,
        format_constraints: &'a [ImageFormatConstraintsInfoFUCHSIA<'a>],
    ) -> Self {
        self.format_constraints_count = format_constraints.len().try_into().unwrap();
        self.p_format_constraints = format_constraints.as_ptr();
        self
    }
    pub fn buffer_collection_constraints(
        mut self,
        buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA<'a>,
    ) -> Self {
        self.buffer_collection_constraints = buffer_collection_constraints;
        self
    }
    pub fn flags(mut self, flags: ImageConstraintsInfoFlagsFUCHSIA) -> Self {
        self.flags = flags;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for BufferCollectionConstraintsInfoFUCHSIA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA,
            p_next: core::ptr::null(),
            min_buffer_count: Default::default(),
            max_buffer_count: Default::default(),
            min_buffer_count_for_camping: Default::default(),
            min_buffer_count_for_dedicated_slack: Default::default(),
            min_buffer_count_for_shared_slack: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> BufferCollectionConstraintsInfoFUCHSIA<'a> {
    pub fn min_buffer_count(mut self, min_buffer_count: u32) -> Self {
        self.min_buffer_count = min_buffer_count;
        self
    }
    pub fn max_buffer_count(mut self, max_buffer_count: u32) -> Self {
        self.max_buffer_count = max_buffer_count;
        self
    }
    pub fn min_buffer_count_for_camping(mut self, min_buffer_count_for_camping: u32) -> Self {
        self.min_buffer_count_for_camping = min_buffer_count_for_camping;
        self
    }
    pub fn min_buffer_count_for_dedicated_slack(
        mut self,
        min_buffer_count_for_dedicated_slack: u32,
    ) -> Self {
        self.min_buffer_count_for_dedicated_slack = min_buffer_count_for_dedicated_slack;
        self
    }
    pub fn min_buffer_count_for_shared_slack(
        mut self,
        min_buffer_count_for_shared_slack: u32,
    ) -> Self {
        self.min_buffer_count_for_shared_slack = min_buffer_count_for_shared_slack;
        self
    }
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
