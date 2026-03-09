//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_FUCHSIA_buffer_collection.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_FUCHSIA_buffer_collection";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    handle_nondispatchable!(
        BufferCollectionFUCHSIA,
        BUFFER_COLLECTION_FUCHSIA,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferCollectionFUCHSIA.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportMemoryBufferCollectionFUCHSIA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImportMemoryBufferCollectionFUCHSIA<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub collection: BufferCollectionFUCHSIA,
        pub index: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImportMemoryBufferCollectionFUCHSIA<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImportMemoryBufferCollectionFUCHSIA")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("collection", &self.collection)
                .field("index", &self.index)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImportMemoryBufferCollectionFUCHSIA<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA;
    }

    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for ImportMemoryBufferCollectionFUCHSIA<'a> {}

    impl Default for ImportMemoryBufferCollectionFUCHSIA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                collection: Default::default(),
                index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportMemoryBufferCollectionFUCHSIA<'a> {
        #[inline]
        pub fn collection(mut self, collection: BufferCollectionFUCHSIA) -> Self {
            self.collection = collection;
            self
        }

        #[inline]
        pub fn index(mut self, index: u32) -> Self {
            self.index = index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferCollectionImageCreateInfoFUCHSIA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BufferCollectionImageCreateInfoFUCHSIA<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub collection: BufferCollectionFUCHSIA,
        pub index: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BufferCollectionImageCreateInfoFUCHSIA<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BufferCollectionImageCreateInfoFUCHSIA")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("collection", &self.collection)
                .field("index", &self.index)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BufferCollectionImageCreateInfoFUCHSIA<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA;
    }

    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for BufferCollectionImageCreateInfoFUCHSIA<'a> {}

    impl Default for BufferCollectionImageCreateInfoFUCHSIA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                collection: Default::default(),
                index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BufferCollectionImageCreateInfoFUCHSIA<'a> {
        #[inline]
        pub fn collection(mut self, collection: BufferCollectionFUCHSIA) -> Self {
            self.collection = collection;
            self
        }

        #[inline]
        pub fn index(mut self, index: u32) -> Self {
            self.index = index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferCollectionBufferCreateInfoFUCHSIA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BufferCollectionBufferCreateInfoFUCHSIA<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub collection: BufferCollectionFUCHSIA,
        pub index: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BufferCollectionBufferCreateInfoFUCHSIA<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BufferCollectionBufferCreateInfoFUCHSIA")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("collection", &self.collection)
                .field("index", &self.index)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BufferCollectionBufferCreateInfoFUCHSIA<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA;
    }

    unsafe impl<'a> Extends<BufferCreateInfo<'a>> for BufferCollectionBufferCreateInfoFUCHSIA<'a> {}

    impl Default for BufferCollectionBufferCreateInfoFUCHSIA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                collection: Default::default(),
                index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BufferCollectionBufferCreateInfoFUCHSIA<'a> {
        #[inline]
        pub fn collection(mut self, collection: BufferCollectionFUCHSIA) -> Self {
            self.collection = collection;
            self
        }

        #[inline]
        pub fn index(mut self, index: u32) -> Self {
            self.index = index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferCollectionCreateInfoFUCHSIA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BufferCollectionCreateInfoFUCHSIA<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub collection_token: zx_handle_t,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BufferCollectionCreateInfoFUCHSIA<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BufferCollectionCreateInfoFUCHSIA")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("collection_token", &self.collection_token)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BufferCollectionCreateInfoFUCHSIA<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_COLLECTION_CREATE_INFO_FUCHSIA;
    }

    impl Default for BufferCollectionCreateInfoFUCHSIA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                collection_token: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BufferCollectionCreateInfoFUCHSIA<'a> {
        #[inline]
        pub fn collection_token(mut self, collection_token: zx_handle_t) -> Self {
            self.collection_token = collection_token;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferCollectionPropertiesFUCHSIA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for BufferCollectionPropertiesFUCHSIA<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BufferCollectionPropertiesFUCHSIA")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory_type_bits", &self.memory_type_bits)
                .field("buffer_count", &self.buffer_count)
                .field("create_info_index", &self.create_info_index)
                .field("sysmem_pixel_format", &self.sysmem_pixel_format)
                .field("format_features", &self.format_features)
                .field("sysmem_color_space_index", &self.sysmem_color_space_index)
                .field(
                    "sampler_ycbcr_conversion_components",
                    &self.sampler_ycbcr_conversion_components,
                )
                .field("suggested_ycbcr_model", &self.suggested_ycbcr_model)
                .field("suggested_ycbcr_range", &self.suggested_ycbcr_range)
                .field("suggested_x_chroma_offset", &self.suggested_x_chroma_offset)
                .field("suggested_y_chroma_offset", &self.suggested_y_chroma_offset)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BufferCollectionPropertiesFUCHSIA<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_COLLECTION_PROPERTIES_FUCHSIA;
    }

    impl Default for BufferCollectionPropertiesFUCHSIA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
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
        #[inline]
        pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
            self.memory_type_bits = memory_type_bits;
            self
        }

        #[inline]
        pub fn buffer_count(mut self, buffer_count: u32) -> Self {
            self.buffer_count = buffer_count;
            self
        }

        #[inline]
        pub fn create_info_index(mut self, create_info_index: u32) -> Self {
            self.create_info_index = create_info_index;
            self
        }

        #[inline]
        pub fn sysmem_pixel_format(mut self, sysmem_pixel_format: u64) -> Self {
            self.sysmem_pixel_format = sysmem_pixel_format;
            self
        }

        #[inline]
        pub fn format_features(mut self, format_features: FormatFeatureFlags) -> Self {
            self.format_features = format_features;
            self
        }

        #[inline]
        pub fn sysmem_color_space_index(
            mut self,
            sysmem_color_space_index: SysmemColorSpaceFUCHSIA<'a>,
        ) -> Self {
            self.sysmem_color_space_index = sysmem_color_space_index;
            self
        }

        #[inline]
        pub fn sampler_ycbcr_conversion_components(
            mut self,
            sampler_ycbcr_conversion_components: ComponentMapping,
        ) -> Self {
            self.sampler_ycbcr_conversion_components = sampler_ycbcr_conversion_components;
            self
        }

        #[inline]
        pub fn suggested_ycbcr_model(
            mut self,
            suggested_ycbcr_model: SamplerYcbcrModelConversion,
        ) -> Self {
            self.suggested_ycbcr_model = suggested_ycbcr_model;
            self
        }

        #[inline]
        pub fn suggested_ycbcr_range(mut self, suggested_ycbcr_range: SamplerYcbcrRange) -> Self {
            self.suggested_ycbcr_range = suggested_ycbcr_range;
            self
        }

        #[inline]
        pub fn suggested_x_chroma_offset(
            mut self,
            suggested_x_chroma_offset: ChromaLocation,
        ) -> Self {
            self.suggested_x_chroma_offset = suggested_x_chroma_offset;
            self
        }

        #[inline]
        pub fn suggested_y_chroma_offset(
            mut self,
            suggested_y_chroma_offset: ChromaLocation,
        ) -> Self {
            self.suggested_y_chroma_offset = suggested_y_chroma_offset;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferConstraintsInfoFUCHSIA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BufferConstraintsInfoFUCHSIA<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub create_info: BufferCreateInfo<'a>,
        pub required_format_features: FormatFeatureFlags,
        pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BufferConstraintsInfoFUCHSIA<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BufferConstraintsInfoFUCHSIA")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("create_info", &self.create_info)
                .field("required_format_features", &self.required_format_features)
                .field(
                    "buffer_collection_constraints",
                    &self.buffer_collection_constraints,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BufferConstraintsInfoFUCHSIA<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_CONSTRAINTS_INFO_FUCHSIA;
    }

    impl Default for BufferConstraintsInfoFUCHSIA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                create_info: Default::default(),
                required_format_features: Default::default(),
                buffer_collection_constraints: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BufferConstraintsInfoFUCHSIA<'a> {
        #[inline]
        pub fn create_info(mut self, create_info: BufferCreateInfo<'a>) -> Self {
            self.create_info = create_info;
            self
        }

        #[inline]
        pub fn required_format_features(
            mut self,
            required_format_features: FormatFeatureFlags,
        ) -> Self {
            self.required_format_features = required_format_features;
            self
        }

        #[inline]
        pub fn buffer_collection_constraints(
            mut self,
            buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA<'a>,
        ) -> Self {
            self.buffer_collection_constraints = buffer_collection_constraints;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSysmemColorSpaceFUCHSIA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SysmemColorSpaceFUCHSIA<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub color_space: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SysmemColorSpaceFUCHSIA<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SysmemColorSpaceFUCHSIA")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("color_space", &self.color_space)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SysmemColorSpaceFUCHSIA<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SYSMEM_COLOR_SPACE_FUCHSIA;
    }

    impl Default for SysmemColorSpaceFUCHSIA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                color_space: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SysmemColorSpaceFUCHSIA<'a> {
        #[inline]
        pub fn color_space(mut self, color_space: u32) -> Self {
            self.color_space = color_space;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageFormatConstraintsInfoFUCHSIA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageFormatConstraintsInfoFUCHSIA<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageFormatConstraintsInfoFUCHSIA")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image_create_info", &self.image_create_info)
                .field("required_format_features", &self.required_format_features)
                .field("flags", &self.flags)
                .field("sysmem_pixel_format", &self.sysmem_pixel_format)
                .field("color_space_count", &self.color_space_count)
                .field("p_color_spaces", &self.p_color_spaces)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageFormatConstraintsInfoFUCHSIA<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA;
    }

    impl Default for ImageFormatConstraintsInfoFUCHSIA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                image_create_info: Default::default(),
                required_format_features: Default::default(),
                flags: Default::default(),
                sysmem_pixel_format: Default::default(),
                color_space_count: Default::default(),
                p_color_spaces: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageFormatConstraintsInfoFUCHSIA<'a> {
        #[inline]
        pub fn image_create_info(mut self, image_create_info: ImageCreateInfo<'a>) -> Self {
            self.image_create_info = image_create_info;
            self
        }

        #[inline]
        pub fn required_format_features(
            mut self,
            required_format_features: FormatFeatureFlags,
        ) -> Self {
            self.required_format_features = required_format_features;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: ImageFormatConstraintsFlagsFUCHSIA) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn sysmem_pixel_format(mut self, sysmem_pixel_format: u64) -> Self {
            self.sysmem_pixel_format = sysmem_pixel_format;
            self
        }

        #[inline]
        pub fn color_spaces(mut self, color_spaces: &'a [SysmemColorSpaceFUCHSIA<'a>]) -> Self {
            self.color_space_count = color_spaces.len().try_into().unwrap();
            self.p_color_spaces = color_spaces.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageConstraintsInfoFUCHSIA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageConstraintsInfoFUCHSIA<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub format_constraints_count: u32,
        pub p_format_constraints: *const ImageFormatConstraintsInfoFUCHSIA<'a>,
        pub buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA<'a>,
        pub flags: ImageConstraintsInfoFlagsFUCHSIA,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageConstraintsInfoFUCHSIA<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageConstraintsInfoFUCHSIA")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("format_constraints_count", &self.format_constraints_count)
                .field("p_format_constraints", &self.p_format_constraints)
                .field(
                    "buffer_collection_constraints",
                    &self.buffer_collection_constraints,
                )
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageConstraintsInfoFUCHSIA<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_CONSTRAINTS_INFO_FUCHSIA;
    }

    impl Default for ImageConstraintsInfoFUCHSIA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                format_constraints_count: Default::default(),
                p_format_constraints: ptr::null(),
                buffer_collection_constraints: Default::default(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageConstraintsInfoFUCHSIA<'a> {
        #[inline]
        pub fn format_constraints(
            mut self,
            format_constraints: &'a [ImageFormatConstraintsInfoFUCHSIA<'a>],
        ) -> Self {
            self.format_constraints_count = format_constraints.len().try_into().unwrap();
            self.p_format_constraints = format_constraints.as_ptr();
            self
        }

        #[inline]
        pub fn buffer_collection_constraints(
            mut self,
            buffer_collection_constraints: BufferCollectionConstraintsInfoFUCHSIA<'a>,
        ) -> Self {
            self.buffer_collection_constraints = buffer_collection_constraints;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: ImageConstraintsInfoFlagsFUCHSIA) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferCollectionConstraintsInfoFUCHSIA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for BufferCollectionConstraintsInfoFUCHSIA<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BufferCollectionConstraintsInfoFUCHSIA")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("min_buffer_count", &self.min_buffer_count)
                .field("max_buffer_count", &self.max_buffer_count)
                .field(
                    "min_buffer_count_for_camping",
                    &self.min_buffer_count_for_camping,
                )
                .field(
                    "min_buffer_count_for_dedicated_slack",
                    &self.min_buffer_count_for_dedicated_slack,
                )
                .field(
                    "min_buffer_count_for_shared_slack",
                    &self.min_buffer_count_for_shared_slack,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BufferCollectionConstraintsInfoFUCHSIA<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA;
    }

    impl Default for BufferCollectionConstraintsInfoFUCHSIA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
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
        #[inline]
        pub fn min_buffer_count(mut self, min_buffer_count: u32) -> Self {
            self.min_buffer_count = min_buffer_count;
            self
        }

        #[inline]
        pub fn max_buffer_count(mut self, max_buffer_count: u32) -> Self {
            self.max_buffer_count = max_buffer_count;
            self
        }

        #[inline]
        pub fn min_buffer_count_for_camping(mut self, min_buffer_count_for_camping: u32) -> Self {
            self.min_buffer_count_for_camping = min_buffer_count_for_camping;
            self
        }

        #[inline]
        pub fn min_buffer_count_for_dedicated_slack(
            mut self,
            min_buffer_count_for_dedicated_slack: u32,
        ) -> Self {
            self.min_buffer_count_for_dedicated_slack = min_buffer_count_for_dedicated_slack;
            self
        }

        #[inline]
        pub fn min_buffer_count_for_shared_slack(
            mut self,
            min_buffer_count_for_shared_slack: u32,
        ) -> Self {
            self.min_buffer_count_for_shared_slack = min_buffer_count_for_shared_slack;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageFormatConstraintsFlagsFUCHSIA.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ImageFormatConstraintsFlagsFUCHSIA(Flags);
    vk_bitflags_wrapped!(ImageFormatConstraintsFlagsFUCHSIA, Flags);

    impl fmt::Debug for ImageFormatConstraintsFlagsFUCHSIA {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageConstraintsInfoFlagsFUCHSIA.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ImageConstraintsInfoFlagsFUCHSIA(Flags);
    vk_bitflags_wrapped!(ImageConstraintsInfoFlagsFUCHSIA, Flags);

    impl ImageConstraintsInfoFlagsFUCHSIA {
        pub const CPU_READ_RARELY_FUCHSIA: Self =
            Self(ImageConstraintsInfoFlagBitsFUCHSIA::CPU_READ_RARELY_FUCHSIA.0);
        pub const CPU_READ_OFTEN_FUCHSIA: Self =
            Self(ImageConstraintsInfoFlagBitsFUCHSIA::CPU_READ_OFTEN_FUCHSIA.0);
        pub const CPU_WRITE_RARELY_FUCHSIA: Self =
            Self(ImageConstraintsInfoFlagBitsFUCHSIA::CPU_WRITE_RARELY_FUCHSIA.0);
        pub const CPU_WRITE_OFTEN_FUCHSIA: Self =
            Self(ImageConstraintsInfoFlagBitsFUCHSIA::CPU_WRITE_OFTEN_FUCHSIA.0);
        pub const PROTECTED_OPTIONAL_FUCHSIA: Self =
            Self(ImageConstraintsInfoFlagBitsFUCHSIA::PROTECTED_OPTIONAL_FUCHSIA.0);
    }

    impl fmt::Debug for ImageConstraintsInfoFlagsFUCHSIA {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    ImageConstraintsInfoFlagsFUCHSIA::CPU_READ_RARELY_FUCHSIA.0,
                    "CPU_READ_RARELY_FUCHSIA",
                ),
                (
                    ImageConstraintsInfoFlagsFUCHSIA::CPU_READ_OFTEN_FUCHSIA.0,
                    "CPU_READ_OFTEN_FUCHSIA",
                ),
                (
                    ImageConstraintsInfoFlagsFUCHSIA::CPU_WRITE_RARELY_FUCHSIA.0,
                    "CPU_WRITE_RARELY_FUCHSIA",
                ),
                (
                    ImageConstraintsInfoFlagsFUCHSIA::CPU_WRITE_OFTEN_FUCHSIA.0,
                    "CPU_WRITE_OFTEN_FUCHSIA",
                ),
                (
                    ImageConstraintsInfoFlagsFUCHSIA::PROTECTED_OPTIONAL_FUCHSIA.0,
                    "PROTECTED_OPTIONAL_FUCHSIA",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageConstraintsInfoFlagBitsFUCHSIA.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ImageConstraintsInfoFlagBitsFUCHSIA(u32);

    impl ImageConstraintsInfoFlagBitsFUCHSIA {
        pub const CPU_READ_RARELY_FUCHSIA: Self = Self(1 << 0);
        pub const CPU_READ_OFTEN_FUCHSIA: Self = Self(1 << 1);
        pub const CPU_WRITE_RARELY_FUCHSIA: Self = Self(1 << 2);
        pub const CPU_WRITE_OFTEN_FUCHSIA: Self = Self(1 << 3);
        pub const PROTECTED_OPTIONAL_FUCHSIA: Self = Self(1 << 4);
    }

    impl fmt::Debug for ImageConstraintsInfoFlagBitsFUCHSIA {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::CPU_READ_RARELY_FUCHSIA => Some("CPU_READ_RARELY_FUCHSIA"),
                Self::CPU_READ_OFTEN_FUCHSIA => Some("CPU_READ_OFTEN_FUCHSIA"),
                Self::CPU_WRITE_RARELY_FUCHSIA => Some("CPU_WRITE_RARELY_FUCHSIA"),
                Self::CPU_WRITE_OFTEN_FUCHSIA => Some("CPU_WRITE_OFTEN_FUCHSIA"),
                Self::PROTECTED_OPTIONAL_FUCHSIA => Some("PROTECTED_OPTIONAL_FUCHSIA"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateBufferCollectionFUCHSIA.html>
    pub type PFN_vkCreateBufferCollectionFUCHSIA = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const BufferCollectionCreateInfoFUCHSIA<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_collection: *mut BufferCollectionFUCHSIA,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetBufferCollectionBufferConstraintsFUCHSIA.html>
    pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA =
        unsafe extern "system" fn(
            device: Device,
            collection: BufferCollectionFUCHSIA,
            p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA<'_>,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetBufferCollectionImageConstraintsFUCHSIA.html>
    pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA =
        unsafe extern "system" fn(
            device: Device,
            collection: BufferCollectionFUCHSIA,
            p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA<'_>,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyBufferCollectionFUCHSIA.html>
    pub type PFN_vkDestroyBufferCollectionFUCHSIA = unsafe extern "system" fn(
        device: Device,
        collection: BufferCollectionFUCHSIA,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetBufferCollectionPropertiesFUCHSIA.html>
    pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA = unsafe extern "system" fn(
        device: Device,
        collection: BufferCollectionFUCHSIA,
        p_properties: *mut BufferCollectionPropertiesFUCHSIA<'_>,
    )
        -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkBufferCollectionFUCHSIA = BufferCollectionFUCHSIA;
    pub type VkImportMemoryBufferCollectionFUCHSIA = ImportMemoryBufferCollectionFUCHSIA<'static>;
    pub type VkBufferCollectionImageCreateInfoFUCHSIA =
        BufferCollectionImageCreateInfoFUCHSIA<'static>;
    pub type VkBufferCollectionBufferCreateInfoFUCHSIA =
        BufferCollectionBufferCreateInfoFUCHSIA<'static>;
    pub type VkBufferCollectionCreateInfoFUCHSIA = BufferCollectionCreateInfoFUCHSIA<'static>;
    pub type VkBufferCollectionPropertiesFUCHSIA = BufferCollectionPropertiesFUCHSIA<'static>;
    pub type VkBufferConstraintsInfoFUCHSIA = BufferConstraintsInfoFUCHSIA<'static>;
    pub type VkSysmemColorSpaceFUCHSIA = SysmemColorSpaceFUCHSIA<'static>;
    pub type VkImageFormatConstraintsInfoFUCHSIA = ImageFormatConstraintsInfoFUCHSIA<'static>;
    pub type VkImageConstraintsInfoFUCHSIA = ImageConstraintsInfoFUCHSIA<'static>;
    pub type VkBufferCollectionConstraintsInfoFUCHSIA =
        BufferCollectionConstraintsInfoFUCHSIA<'static>;
    pub type VkImageFormatConstraintsFlagsFUCHSIA = ImageFormatConstraintsFlagsFUCHSIA;
    pub type VkImageConstraintsInfoFlagsFUCHSIA = ImageConstraintsInfoFlagsFUCHSIA;
    pub type VkImageConstraintsInfoFlagBitsFUCHSIA = ImageConstraintsInfoFlagBitsFUCHSIA;
    impl ImportMemoryBufferCollectionFUCHSIA<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImportMemoryBufferCollectionFUCHSIA {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BufferCollectionImageCreateInfoFUCHSIA<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBufferCollectionImageCreateInfoFUCHSIA {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BufferCollectionBufferCreateInfoFUCHSIA<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBufferCollectionBufferCreateInfoFUCHSIA {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BufferCollectionCreateInfoFUCHSIA<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBufferCollectionCreateInfoFUCHSIA {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BufferCollectionPropertiesFUCHSIA<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBufferCollectionPropertiesFUCHSIA {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BufferConstraintsInfoFUCHSIA<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBufferConstraintsInfoFUCHSIA {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SysmemColorSpaceFUCHSIA<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSysmemColorSpaceFUCHSIA {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageFormatConstraintsInfoFUCHSIA<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageFormatConstraintsInfoFUCHSIA {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageConstraintsInfoFUCHSIA<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageConstraintsInfoFUCHSIA {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BufferCollectionConstraintsInfoFUCHSIA<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBufferCollectionConstraintsInfoFUCHSIA {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    create_buffer_collection_fuchsia: PFN_vkCreateBufferCollectionFUCHSIA,
    set_buffer_collection_image_constraints_fuchsia:
        PFN_vkSetBufferCollectionImageConstraintsFUCHSIA,
    set_buffer_collection_buffer_constraints_fuchsia:
        PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA,
    destroy_buffer_collection_fuchsia: PFN_vkDestroyBufferCollectionFUCHSIA,
    get_buffer_collection_properties_fuchsia: PFN_vkGetBufferCollectionPropertiesFUCHSIA,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_buffer_collection_fuchsia: transmute(
                    load(c"vkCreateBufferCollectionFUCHSIA").ok_or(MissingEntryPointError)?,
                ),
                set_buffer_collection_image_constraints_fuchsia: transmute(
                    load(c"vkSetBufferCollectionImageConstraintsFUCHSIA")
                        .ok_or(MissingEntryPointError)?,
                ),
                set_buffer_collection_buffer_constraints_fuchsia: transmute(
                    load(c"vkSetBufferCollectionBufferConstraintsFUCHSIA")
                        .ok_or(MissingEntryPointError)?,
                ),
                destroy_buffer_collection_fuchsia: transmute(
                    load(c"vkDestroyBufferCollectionFUCHSIA").ok_or(MissingEntryPointError)?,
                ),
                get_buffer_collection_properties_fuchsia: transmute(
                    load(c"vkGetBufferCollectionPropertiesFUCHSIA")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateBufferCollectionFUCHSIA.html>
    #[inline]
    pub unsafe fn create_buffer_collection_fuchsia(
        &self,
        device: Device,
        create_info: &BufferCollectionCreateInfoFUCHSIA<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<BufferCollectionFUCHSIA> {
        unsafe {
            let mut collection = core::mem::MaybeUninit::uninit();
            let result = (self.create_buffer_collection_fuchsia)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                collection.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(collection.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetBufferCollectionImageConstraintsFUCHSIA.html>
    #[inline]
    pub unsafe fn set_buffer_collection_image_constraints_fuchsia(
        &self,
        device: Device,
        collection: BufferCollectionFUCHSIA,
        image_constraints_info: &ImageConstraintsInfoFUCHSIA<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.set_buffer_collection_image_constraints_fuchsia)(
                device,
                collection,
                image_constraints_info,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetBufferCollectionBufferConstraintsFUCHSIA.html>
    #[inline]
    pub unsafe fn set_buffer_collection_buffer_constraints_fuchsia(
        &self,
        device: Device,
        collection: BufferCollectionFUCHSIA,
        buffer_constraints_info: &BufferConstraintsInfoFUCHSIA<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.set_buffer_collection_buffer_constraints_fuchsia)(
                device,
                collection,
                buffer_constraints_info,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyBufferCollectionFUCHSIA.html>
    #[inline]
    pub unsafe fn destroy_buffer_collection_fuchsia(
        &self,
        device: Device,
        collection: BufferCollectionFUCHSIA,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_buffer_collection_fuchsia)(device, collection, allocator.to_raw_ptr())
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetBufferCollectionPropertiesFUCHSIA.html>
    #[inline]
    pub unsafe fn get_buffer_collection_properties_fuchsia(
        &self,
        device: Device,
        collection: BufferCollectionFUCHSIA,
        properties: &mut BufferCollectionPropertiesFUCHSIA<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result =
                (self.get_buffer_collection_properties_fuchsia)(device, collection, properties);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
