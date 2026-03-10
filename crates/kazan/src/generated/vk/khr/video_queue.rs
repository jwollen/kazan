//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_video_queue.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_video_queue";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    handle_nondispatchable!(
        VideoSessionKHR,
        VIDEO_SESSION_KHR,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoSessionKHR.html>"
    );
    handle_nondispatchable!(
        VideoSessionParametersKHR,
        VIDEO_SESSION_PARAMETERS_KHR,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoSessionParametersKHR.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueFamilyVideoPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct QueueFamilyVideoPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub video_codec_operations: VideoCodecOperationFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for QueueFamilyVideoPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("QueueFamilyVideoPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("video_codec_operations", &self.video_codec_operations)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for QueueFamilyVideoPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::QUEUE_FAMILY_VIDEO_PROPERTIES_KHR;
    }

    unsafe impl Extends<QueueFamilyProperties2<'_>> for QueueFamilyVideoPropertiesKHR<'_> {}

    impl Default for QueueFamilyVideoPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                video_codec_operations: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> QueueFamilyVideoPropertiesKHR<'a> {
        #[inline]
        pub fn video_codec_operations(
            mut self,
            video_codec_operations: VideoCodecOperationFlagsKHR,
        ) -> Self {
            self.video_codec_operations = video_codec_operations;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueFamilyQueryResultStatusPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct QueueFamilyQueryResultStatusPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub query_result_status_support: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for QueueFamilyQueryResultStatusPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("QueueFamilyQueryResultStatusPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "query_result_status_support",
                    &self.query_result_status_support,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for QueueFamilyQueryResultStatusPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR;
    }

    unsafe impl Extends<QueueFamilyProperties2<'_>> for QueueFamilyQueryResultStatusPropertiesKHR<'_> {}

    impl Default for QueueFamilyQueryResultStatusPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                query_result_status_support: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> QueueFamilyQueryResultStatusPropertiesKHR<'a> {
        #[inline]
        pub fn query_result_status_support(mut self, query_result_status_support: bool) -> Self {
            self.query_result_status_support = query_result_status_support.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoProfileListInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoProfileListInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub profile_count: u32,
        pub p_profiles: *const VideoProfileInfoKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoProfileListInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoProfileListInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("profile_count", &self.profile_count)
                .field("p_profiles", &self.p_profiles)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoProfileListInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_PROFILE_LIST_INFO_KHR;
    }

    unsafe impl Extends<PhysicalDeviceImageFormatInfo2<'_>> for VideoProfileListInfoKHR<'_> {}
    unsafe impl Extends<PhysicalDeviceVideoFormatInfoKHR<'_>> for VideoProfileListInfoKHR<'_> {}
    unsafe impl Extends<ImageCreateInfo<'_>> for VideoProfileListInfoKHR<'_> {}
    unsafe impl Extends<BufferCreateInfo<'_>> for VideoProfileListInfoKHR<'_> {}

    impl Default for VideoProfileListInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                profile_count: Default::default(),
                p_profiles: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoProfileListInfoKHR<'a> {
        #[inline]
        pub fn profiles(mut self, profiles: &'a [VideoProfileInfoKHR<'_>]) -> Self {
            self.profile_count = profiles.len().try_into().unwrap();
            self.p_profiles = profiles.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVideoFormatInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceVideoFormatInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image_usage: ImageUsageFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceVideoFormatInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVideoFormatInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image_usage", &self.image_usage)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVideoFormatInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR;
    }

    impl Default for PhysicalDeviceVideoFormatInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                image_usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceVideoFormatInfoKHR<'a> {
        #[inline]
        pub fn image_usage(mut self, image_usage: ImageUsageFlags) -> Self {
            self.image_usage = image_usage;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoFormatPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoFormatPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub format: Format,
        pub component_mapping: ComponentMapping,
        pub image_create_flags: ImageCreateFlags,
        pub image_type: ImageType,
        pub image_tiling: ImageTiling,
        pub image_usage_flags: ImageUsageFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoFormatPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoFormatPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("format", &self.format)
                .field("component_mapping", &self.component_mapping)
                .field("image_create_flags", &self.image_create_flags)
                .field("image_type", &self.image_type)
                .field("image_tiling", &self.image_tiling)
                .field("image_usage_flags", &self.image_usage_flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoFormatPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_FORMAT_PROPERTIES_KHR;
    }

    impl Default for VideoFormatPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                format: Default::default(),
                component_mapping: Default::default(),
                image_create_flags: Default::default(),
                image_type: Default::default(),
                image_tiling: Default::default(),
                image_usage_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoFormatPropertiesKHR<'a> {
        #[inline]
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }

        #[inline]
        pub fn component_mapping(mut self, component_mapping: ComponentMapping) -> Self {
            self.component_mapping = component_mapping;
            self
        }

        #[inline]
        pub fn image_create_flags(mut self, image_create_flags: ImageCreateFlags) -> Self {
            self.image_create_flags = image_create_flags;
            self
        }

        #[inline]
        pub fn image_type(mut self, image_type: ImageType) -> Self {
            self.image_type = image_type;
            self
        }

        #[inline]
        pub fn image_tiling(mut self, image_tiling: ImageTiling) -> Self {
            self.image_tiling = image_tiling;
            self
        }

        #[inline]
        pub fn image_usage_flags(mut self, image_usage_flags: ImageUsageFlags) -> Self {
            self.image_usage_flags = image_usage_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoProfileInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoProfileInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub video_codec_operation: VideoCodecOperationFlagBitsKHR,
        pub chroma_subsampling: VideoChromaSubsamplingFlagsKHR,
        pub luma_bit_depth: VideoComponentBitDepthFlagsKHR,
        pub chroma_bit_depth: VideoComponentBitDepthFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoProfileInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoProfileInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("video_codec_operation", &self.video_codec_operation)
                .field("chroma_subsampling", &self.chroma_subsampling)
                .field("luma_bit_depth", &self.luma_bit_depth)
                .field("chroma_bit_depth", &self.chroma_bit_depth)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoProfileInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_PROFILE_INFO_KHR;
    }

    unsafe impl Extends<QueryPoolCreateInfo<'_>> for VideoProfileInfoKHR<'_> {}

    impl Default for VideoProfileInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                video_codec_operation: Default::default(),
                chroma_subsampling: Default::default(),
                luma_bit_depth: Default::default(),
                chroma_bit_depth: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoProfileInfoKHR<'a> {
        #[inline]
        pub fn video_codec_operation(
            mut self,
            video_codec_operation: VideoCodecOperationFlagBitsKHR,
        ) -> Self {
            self.video_codec_operation = video_codec_operation;
            self
        }

        #[inline]
        pub fn chroma_subsampling(
            mut self,
            chroma_subsampling: VideoChromaSubsamplingFlagsKHR,
        ) -> Self {
            self.chroma_subsampling = chroma_subsampling;
            self
        }

        #[inline]
        pub fn luma_bit_depth(mut self, luma_bit_depth: VideoComponentBitDepthFlagsKHR) -> Self {
            self.luma_bit_depth = luma_bit_depth;
            self
        }

        #[inline]
        pub fn chroma_bit_depth(
            mut self,
            chroma_bit_depth: VideoComponentBitDepthFlagsKHR,
        ) -> Self {
            self.chroma_bit_depth = chroma_bit_depth;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoCapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoCapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: VideoCapabilityFlagsKHR,
        pub min_bitstream_buffer_offset_alignment: DeviceSize,
        pub min_bitstream_buffer_size_alignment: DeviceSize,
        pub picture_access_granularity: Extent2D,
        pub min_coded_extent: Extent2D,
        pub max_coded_extent: Extent2D,
        pub max_dpb_slots: u32,
        pub max_active_reference_pictures: u32,
        pub std_header_version: ExtensionProperties,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoCapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoCapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field(
                    "min_bitstream_buffer_offset_alignment",
                    &self.min_bitstream_buffer_offset_alignment,
                )
                .field(
                    "min_bitstream_buffer_size_alignment",
                    &self.min_bitstream_buffer_size_alignment,
                )
                .field(
                    "picture_access_granularity",
                    &self.picture_access_granularity,
                )
                .field("min_coded_extent", &self.min_coded_extent)
                .field("max_coded_extent", &self.max_coded_extent)
                .field("max_dpb_slots", &self.max_dpb_slots)
                .field(
                    "max_active_reference_pictures",
                    &self.max_active_reference_pictures,
                )
                .field("std_header_version", &self.std_header_version)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoCapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_CAPABILITIES_KHR;
    }

    impl Default for VideoCapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                flags: Default::default(),
                min_bitstream_buffer_offset_alignment: Default::default(),
                min_bitstream_buffer_size_alignment: Default::default(),
                picture_access_granularity: Default::default(),
                min_coded_extent: Default::default(),
                max_coded_extent: Default::default(),
                max_dpb_slots: Default::default(),
                max_active_reference_pictures: Default::default(),
                std_header_version: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoCapabilitiesKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: VideoCapabilityFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn min_bitstream_buffer_offset_alignment(
            mut self,
            min_bitstream_buffer_offset_alignment: DeviceSize,
        ) -> Self {
            self.min_bitstream_buffer_offset_alignment = min_bitstream_buffer_offset_alignment;
            self
        }

        #[inline]
        pub fn min_bitstream_buffer_size_alignment(
            mut self,
            min_bitstream_buffer_size_alignment: DeviceSize,
        ) -> Self {
            self.min_bitstream_buffer_size_alignment = min_bitstream_buffer_size_alignment;
            self
        }

        #[inline]
        pub fn picture_access_granularity(mut self, picture_access_granularity: Extent2D) -> Self {
            self.picture_access_granularity = picture_access_granularity;
            self
        }

        #[inline]
        pub fn min_coded_extent(mut self, min_coded_extent: Extent2D) -> Self {
            self.min_coded_extent = min_coded_extent;
            self
        }

        #[inline]
        pub fn max_coded_extent(mut self, max_coded_extent: Extent2D) -> Self {
            self.max_coded_extent = max_coded_extent;
            self
        }

        #[inline]
        pub fn max_dpb_slots(mut self, max_dpb_slots: u32) -> Self {
            self.max_dpb_slots = max_dpb_slots;
            self
        }

        #[inline]
        pub fn max_active_reference_pictures(mut self, max_active_reference_pictures: u32) -> Self {
            self.max_active_reference_pictures = max_active_reference_pictures;
            self
        }

        #[inline]
        pub fn std_header_version(mut self, std_header_version: ExtensionProperties) -> Self {
            self.std_header_version = std_header_version;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoSessionMemoryRequirementsKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoSessionMemoryRequirementsKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_bind_index: u32,
        pub memory_requirements: MemoryRequirements,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoSessionMemoryRequirementsKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoSessionMemoryRequirementsKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory_bind_index", &self.memory_bind_index)
                .field("memory_requirements", &self.memory_requirements)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoSessionMemoryRequirementsKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR;
    }

    impl Default for VideoSessionMemoryRequirementsKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                memory_bind_index: Default::default(),
                memory_requirements: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoSessionMemoryRequirementsKHR<'a> {
        #[inline]
        pub fn memory_bind_index(mut self, memory_bind_index: u32) -> Self {
            self.memory_bind_index = memory_bind_index;
            self
        }

        #[inline]
        pub fn memory_requirements(mut self, memory_requirements: MemoryRequirements) -> Self {
            self.memory_requirements = memory_requirements;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindVideoSessionMemoryInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BindVideoSessionMemoryInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory_bind_index: u32,
        pub memory: DeviceMemory,
        pub memory_offset: DeviceSize,
        pub memory_size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BindVideoSessionMemoryInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BindVideoSessionMemoryInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory_bind_index", &self.memory_bind_index)
                .field("memory", &self.memory)
                .field("memory_offset", &self.memory_offset)
                .field("memory_size", &self.memory_size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BindVideoSessionMemoryInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_VIDEO_SESSION_MEMORY_INFO_KHR;
    }

    impl Default for BindVideoSessionMemoryInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                memory_bind_index: Default::default(),
                memory: Default::default(),
                memory_offset: Default::default(),
                memory_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BindVideoSessionMemoryInfoKHR<'a> {
        #[inline]
        pub fn memory_bind_index(mut self, memory_bind_index: u32) -> Self {
            self.memory_bind_index = memory_bind_index;
            self
        }

        #[inline]
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }

        #[inline]
        pub fn memory_offset(mut self, memory_offset: DeviceSize) -> Self {
            self.memory_offset = memory_offset;
            self
        }

        #[inline]
        pub fn memory_size(mut self, memory_size: DeviceSize) -> Self {
            self.memory_size = memory_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoPictureResourceInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoPictureResourceInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub coded_offset: Offset2D,
        pub coded_extent: Extent2D,
        pub base_array_layer: u32,
        pub image_view_binding: ImageView,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoPictureResourceInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoPictureResourceInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("coded_offset", &self.coded_offset)
                .field("coded_extent", &self.coded_extent)
                .field("base_array_layer", &self.base_array_layer)
                .field("image_view_binding", &self.image_view_binding)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoPictureResourceInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_PICTURE_RESOURCE_INFO_KHR;
    }

    impl Default for VideoPictureResourceInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                coded_offset: Default::default(),
                coded_extent: Default::default(),
                base_array_layer: Default::default(),
                image_view_binding: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoPictureResourceInfoKHR<'a> {
        #[inline]
        pub fn coded_offset(mut self, coded_offset: Offset2D) -> Self {
            self.coded_offset = coded_offset;
            self
        }

        #[inline]
        pub fn coded_extent(mut self, coded_extent: Extent2D) -> Self {
            self.coded_extent = coded_extent;
            self
        }

        #[inline]
        pub fn base_array_layer(mut self, base_array_layer: u32) -> Self {
            self.base_array_layer = base_array_layer;
            self
        }

        #[inline]
        pub fn image_view_binding(mut self, image_view_binding: ImageView) -> Self {
            self.image_view_binding = image_view_binding;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoReferenceSlotInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoReferenceSlotInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub slot_index: i32,
        pub p_picture_resource: *const VideoPictureResourceInfoKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoReferenceSlotInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoReferenceSlotInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("slot_index", &self.slot_index)
                .field("p_picture_resource", &self.p_picture_resource)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoReferenceSlotInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_REFERENCE_SLOT_INFO_KHR;
    }

    impl Default for VideoReferenceSlotInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                slot_index: Default::default(),
                p_picture_resource: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoReferenceSlotInfoKHR<'a> {
        #[inline]
        pub fn slot_index(mut self, slot_index: i32) -> Self {
            self.slot_index = slot_index;
            self
        }

        #[inline]
        pub fn picture_resource(
            mut self,
            picture_resource: &'a VideoPictureResourceInfoKHR<'a>,
        ) -> Self {
            self.p_picture_resource = picture_resource;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoSessionCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoSessionCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub queue_family_index: u32,
        pub flags: VideoSessionCreateFlagsKHR,
        pub p_video_profile: *const VideoProfileInfoKHR<'a>,
        pub picture_format: Format,
        pub max_coded_extent: Extent2D,
        pub reference_picture_format: Format,
        pub max_dpb_slots: u32,
        pub max_active_reference_pictures: u32,
        pub p_std_header_version: *const ExtensionProperties,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoSessionCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoSessionCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("queue_family_index", &self.queue_family_index)
                .field("flags", &self.flags)
                .field("p_video_profile", &self.p_video_profile)
                .field("picture_format", &self.picture_format)
                .field("max_coded_extent", &self.max_coded_extent)
                .field("reference_picture_format", &self.reference_picture_format)
                .field("max_dpb_slots", &self.max_dpb_slots)
                .field(
                    "max_active_reference_pictures",
                    &self.max_active_reference_pictures,
                )
                .field("p_std_header_version", &self.p_std_header_version)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoSessionCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_SESSION_CREATE_INFO_KHR;
    }

    impl Default for VideoSessionCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                queue_family_index: Default::default(),
                flags: Default::default(),
                p_video_profile: ptr::null(),
                picture_format: Default::default(),
                max_coded_extent: Default::default(),
                reference_picture_format: Default::default(),
                max_dpb_slots: Default::default(),
                max_active_reference_pictures: Default::default(),
                p_std_header_version: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoSessionCreateInfoKHR<'a> {
        #[inline]
        pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
            self.queue_family_index = queue_family_index;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: VideoSessionCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn video_profile(mut self, video_profile: &'a VideoProfileInfoKHR<'a>) -> Self {
            self.p_video_profile = video_profile;
            self
        }

        #[inline]
        pub fn picture_format(mut self, picture_format: Format) -> Self {
            self.picture_format = picture_format;
            self
        }

        #[inline]
        pub fn max_coded_extent(mut self, max_coded_extent: Extent2D) -> Self {
            self.max_coded_extent = max_coded_extent;
            self
        }

        #[inline]
        pub fn reference_picture_format(mut self, reference_picture_format: Format) -> Self {
            self.reference_picture_format = reference_picture_format;
            self
        }

        #[inline]
        pub fn max_dpb_slots(mut self, max_dpb_slots: u32) -> Self {
            self.max_dpb_slots = max_dpb_slots;
            self
        }

        #[inline]
        pub fn max_active_reference_pictures(mut self, max_active_reference_pictures: u32) -> Self {
            self.max_active_reference_pictures = max_active_reference_pictures;
            self
        }

        #[inline]
        pub fn std_header_version(mut self, std_header_version: &'a ExtensionProperties) -> Self {
            self.p_std_header_version = std_header_version;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoSessionParametersCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoSessionParametersCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: VideoSessionParametersCreateFlagsKHR,
        pub video_session_parameters_template: VideoSessionParametersKHR,
        pub video_session: VideoSessionKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoSessionParametersCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoSessionParametersCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field(
                    "video_session_parameters_template",
                    &self.video_session_parameters_template,
                )
                .field("video_session", &self.video_session)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoSessionParametersCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR;
    }

    impl Default for VideoSessionParametersCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                video_session_parameters_template: Default::default(),
                video_session: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoSessionParametersCreateInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: VideoSessionParametersCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn video_session_parameters_template(
            mut self,
            video_session_parameters_template: VideoSessionParametersKHR,
        ) -> Self {
            self.video_session_parameters_template = video_session_parameters_template;
            self
        }

        #[inline]
        pub fn video_session(mut self, video_session: VideoSessionKHR) -> Self {
            self.video_session = video_session;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoSessionParametersUpdateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoSessionParametersUpdateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub update_sequence_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoSessionParametersUpdateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoSessionParametersUpdateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("update_sequence_count", &self.update_sequence_count)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoSessionParametersUpdateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR;
    }

    impl Default for VideoSessionParametersUpdateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                update_sequence_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoSessionParametersUpdateInfoKHR<'a> {
        #[inline]
        pub fn update_sequence_count(mut self, update_sequence_count: u32) -> Self {
            self.update_sequence_count = update_sequence_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoBeginCodingInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoBeginCodingInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: VideoBeginCodingFlagsKHR,
        pub video_session: VideoSessionKHR,
        pub video_session_parameters: VideoSessionParametersKHR,
        pub reference_slot_count: u32,
        pub p_reference_slots: *const VideoReferenceSlotInfoKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoBeginCodingInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoBeginCodingInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("video_session", &self.video_session)
                .field("video_session_parameters", &self.video_session_parameters)
                .field("reference_slot_count", &self.reference_slot_count)
                .field("p_reference_slots", &self.p_reference_slots)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoBeginCodingInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_BEGIN_CODING_INFO_KHR;
    }

    impl Default for VideoBeginCodingInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                video_session: Default::default(),
                video_session_parameters: Default::default(),
                reference_slot_count: Default::default(),
                p_reference_slots: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoBeginCodingInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: VideoBeginCodingFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn video_session(mut self, video_session: VideoSessionKHR) -> Self {
            self.video_session = video_session;
            self
        }

        #[inline]
        pub fn video_session_parameters(
            mut self,
            video_session_parameters: VideoSessionParametersKHR,
        ) -> Self {
            self.video_session_parameters = video_session_parameters;
            self
        }

        #[inline]
        pub fn reference_slots(
            mut self,
            reference_slots: &'a [VideoReferenceSlotInfoKHR<'_>],
        ) -> Self {
            self.reference_slot_count = reference_slots.len().try_into().unwrap();
            self.p_reference_slots = reference_slots.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEndCodingInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEndCodingInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: VideoEndCodingFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEndCodingInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEndCodingInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEndCodingInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_END_CODING_INFO_KHR;
    }

    impl Default for VideoEndCodingInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEndCodingInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: VideoEndCodingFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoCodingControlInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoCodingControlInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: VideoCodingControlFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoCodingControlInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoCodingControlInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoCodingControlInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_CODING_CONTROL_INFO_KHR;
    }

    impl Default for VideoCodingControlInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoCodingControlInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: VideoCodingControlFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueryResultStatusKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct QueryResultStatusKHR(i32);

    impl QueryResultStatusKHR {
        pub const ERROR_KHR: Self = Self(-1);
        pub const NOT_READY_KHR: Self = Self(0);
        pub const COMPLETE_KHR: Self = Self(1);

        // VK_KHR_video_encode_queue
        pub const INSUFFICIENT_BITSTREAM_BUFFER_RANGE_KHR: Self = Self(-1000299000);
    }

    impl fmt::Debug for QueryResultStatusKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ERROR_KHR => Some("ERROR_KHR"),
                Self::NOT_READY_KHR => Some("NOT_READY_KHR"),
                Self::COMPLETE_KHR => Some("COMPLETE_KHR"),
                Self::INSUFFICIENT_BITSTREAM_BUFFER_RANGE_KHR => {
                    Some("INSUFFICIENT_BITSTREAM_BUFFER_RANGE_KHR")
                }
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoCodecOperationFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoCodecOperationFlagsKHR(Flags);
    vk_bitflags_wrapped!(
        VideoCodecOperationFlagsKHR,
        Flags,
        VideoCodecOperationFlagBitsKHR
    );

    impl VideoCodecOperationFlagsKHR {
        pub const NONE: Self = Self(0);
    }

    impl fmt::Debug for VideoCodecOperationFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoCodecOperationFlagBitsKHR::DECODE_AV1_KHR.0,
                    "DECODE_AV1_KHR",
                ),
                (
                    VideoCodecOperationFlagBitsKHR::DECODE_H264_KHR.0,
                    "DECODE_H264_KHR",
                ),
                (
                    VideoCodecOperationFlagBitsKHR::DECODE_H265_KHR.0,
                    "DECODE_H265_KHR",
                ),
                (
                    VideoCodecOperationFlagBitsKHR::DECODE_VP9_KHR.0,
                    "DECODE_VP9_KHR",
                ),
                (
                    VideoCodecOperationFlagBitsKHR::ENCODE_AV1_KHR.0,
                    "ENCODE_AV1_KHR",
                ),
                (
                    VideoCodecOperationFlagBitsKHR::ENCODE_H264_KHR.0,
                    "ENCODE_H264_KHR",
                ),
                (
                    VideoCodecOperationFlagBitsKHR::ENCODE_H265_KHR.0,
                    "ENCODE_H265_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoCodecOperationFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoCodecOperationFlagBitsKHR(u32);

    impl VideoCodecOperationFlagBitsKHR {
        // VK_KHR_video_decode_av1
        pub const DECODE_AV1_KHR: Self = Self(1 << 2);

        // VK_KHR_video_decode_h264
        pub const DECODE_H264_KHR: Self = Self(1 << 0);

        // VK_KHR_video_decode_h265
        pub const DECODE_H265_KHR: Self = Self(1 << 1);

        // VK_KHR_video_decode_vp9
        pub const DECODE_VP9_KHR: Self = Self(1 << 3);

        // VK_KHR_video_encode_av1
        pub const ENCODE_AV1_KHR: Self = Self(1 << 18);

        // VK_KHR_video_encode_h264
        pub const ENCODE_H264_KHR: Self = Self(1 << 16);

        // VK_KHR_video_encode_h265
        pub const ENCODE_H265_KHR: Self = Self(1 << 17);
    }

    impl fmt::Debug for VideoCodecOperationFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DECODE_AV1_KHR => Some("DECODE_AV1_KHR"),
                Self::DECODE_H264_KHR => Some("DECODE_H264_KHR"),
                Self::DECODE_H265_KHR => Some("DECODE_H265_KHR"),
                Self::DECODE_VP9_KHR => Some("DECODE_VP9_KHR"),
                Self::ENCODE_AV1_KHR => Some("ENCODE_AV1_KHR"),
                Self::ENCODE_H264_KHR => Some("ENCODE_H264_KHR"),
                Self::ENCODE_H265_KHR => Some("ENCODE_H265_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoCapabilityFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoCapabilityFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoCapabilityFlagsKHR, Flags, VideoCapabilityFlagBitsKHR);

    impl fmt::Debug for VideoCapabilityFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoCapabilityFlagBitsKHR::PROTECTED_CONTENT_KHR.0,
                    "PROTECTED_CONTENT_KHR",
                ),
                (
                    VideoCapabilityFlagBitsKHR::SEPARATE_REFERENCE_IMAGES_KHR.0,
                    "SEPARATE_REFERENCE_IMAGES_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoCapabilityFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoCapabilityFlagBitsKHR(u32);

    impl VideoCapabilityFlagBitsKHR {
        pub const PROTECTED_CONTENT_KHR: Self = Self(1 << 0);
        pub const SEPARATE_REFERENCE_IMAGES_KHR: Self = Self(1 << 1);
    }

    impl fmt::Debug for VideoCapabilityFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::PROTECTED_CONTENT_KHR => Some("PROTECTED_CONTENT_KHR"),
                Self::SEPARATE_REFERENCE_IMAGES_KHR => Some("SEPARATE_REFERENCE_IMAGES_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoSessionCreateFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoSessionCreateFlagsKHR(Flags);
    vk_bitflags_wrapped!(
        VideoSessionCreateFlagsKHR,
        Flags,
        VideoSessionCreateFlagBitsKHR
    );

    impl fmt::Debug for VideoSessionCreateFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoSessionCreateFlagBitsKHR::PROTECTED_CONTENT_KHR.0,
                    "PROTECTED_CONTENT_KHR",
                ),
                (
                    VideoSessionCreateFlagBitsKHR::ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR.0,
                    "ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR",
                ),
                (
                    VideoSessionCreateFlagBitsKHR::ALLOW_ENCODE_EMPHASIS_MAP_KHR.0,
                    "ALLOW_ENCODE_EMPHASIS_MAP_KHR",
                ),
                (
                    VideoSessionCreateFlagBitsKHR::ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR.0,
                    "ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR",
                ),
                (
                    VideoSessionCreateFlagBitsKHR::INLINE_QUERIES_KHR.0,
                    "INLINE_QUERIES_KHR",
                ),
                (
                    VideoSessionCreateFlagBitsKHR::INLINE_SESSION_PARAMETERS_KHR.0,
                    "INLINE_SESSION_PARAMETERS_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoSessionCreateFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoSessionCreateFlagBitsKHR(u32);

    impl VideoSessionCreateFlagBitsKHR {
        pub const PROTECTED_CONTENT_KHR: Self = Self(1 << 0);
        // VK_KHR_video_encode_quantization_map
        pub const ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self = Self(1 << 3);
        pub const ALLOW_ENCODE_EMPHASIS_MAP_KHR: Self = Self(1 << 4);

        // VK_KHR_video_encode_queue
        pub const ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR: Self = Self(1 << 1);

        // VK_KHR_video_maintenance1
        pub const INLINE_QUERIES_KHR: Self = Self(1 << 2);

        // VK_KHR_video_maintenance2
        pub const INLINE_SESSION_PARAMETERS_KHR: Self = Self(1 << 5);
    }

    impl fmt::Debug for VideoSessionCreateFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::PROTECTED_CONTENT_KHR => Some("PROTECTED_CONTENT_KHR"),
                Self::ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR => {
                    Some("ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR")
                }
                Self::ALLOW_ENCODE_EMPHASIS_MAP_KHR => Some("ALLOW_ENCODE_EMPHASIS_MAP_KHR"),
                Self::ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR => {
                    Some("ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR")
                }
                Self::INLINE_QUERIES_KHR => Some("INLINE_QUERIES_KHR"),
                Self::INLINE_SESSION_PARAMETERS_KHR => Some("INLINE_SESSION_PARAMETERS_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoSessionParametersCreateFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoSessionParametersCreateFlagsKHR(Flags);
    vk_bitflags_wrapped!(
        VideoSessionParametersCreateFlagsKHR,
        Flags,
        VideoSessionParametersCreateFlagBitsKHR
    );

    impl fmt::Debug for VideoSessionParametersCreateFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[(
                VideoSessionParametersCreateFlagBitsKHR::QUANTIZATION_MAP_COMPATIBLE_KHR.0,
                "QUANTIZATION_MAP_COMPATIBLE_KHR",
            )];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoSessionParametersCreateFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoSessionParametersCreateFlagBitsKHR(u32);

    impl VideoSessionParametersCreateFlagBitsKHR {
        // VK_KHR_video_encode_quantization_map
        pub const QUANTIZATION_MAP_COMPATIBLE_KHR: Self = Self(1 << 0);
    }

    impl fmt::Debug for VideoSessionParametersCreateFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::QUANTIZATION_MAP_COMPATIBLE_KHR => Some("QUANTIZATION_MAP_COMPATIBLE_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoBeginCodingFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoBeginCodingFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoBeginCodingFlagsKHR, Flags);

    impl fmt::Debug for VideoBeginCodingFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEndCodingFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEndCodingFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEndCodingFlagsKHR, Flags);

    impl fmt::Debug for VideoEndCodingFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoCodingControlFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoCodingControlFlagsKHR(Flags);
    vk_bitflags_wrapped!(
        VideoCodingControlFlagsKHR,
        Flags,
        VideoCodingControlFlagBitsKHR
    );

    impl fmt::Debug for VideoCodingControlFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (VideoCodingControlFlagBitsKHR::RESET_KHR.0, "RESET_KHR"),
                (
                    VideoCodingControlFlagBitsKHR::ENCODE_RATE_CONTROL_KHR.0,
                    "ENCODE_RATE_CONTROL_KHR",
                ),
                (
                    VideoCodingControlFlagBitsKHR::ENCODE_QUALITY_LEVEL_KHR.0,
                    "ENCODE_QUALITY_LEVEL_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoCodingControlFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoCodingControlFlagBitsKHR(u32);

    impl VideoCodingControlFlagBitsKHR {
        pub const RESET_KHR: Self = Self(1 << 0);
        // VK_KHR_video_encode_queue
        pub const ENCODE_RATE_CONTROL_KHR: Self = Self(1 << 1);
        pub const ENCODE_QUALITY_LEVEL_KHR: Self = Self(1 << 2);
    }

    impl fmt::Debug for VideoCodingControlFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::RESET_KHR => Some("RESET_KHR"),
                Self::ENCODE_RATE_CONTROL_KHR => Some("ENCODE_RATE_CONTROL_KHR"),
                Self::ENCODE_QUALITY_LEVEL_KHR => Some("ENCODE_QUALITY_LEVEL_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoChromaSubsamplingFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoChromaSubsamplingFlagsKHR(Flags);
    vk_bitflags_wrapped!(
        VideoChromaSubsamplingFlagsKHR,
        Flags,
        VideoChromaSubsamplingFlagBitsKHR
    );

    impl VideoChromaSubsamplingFlagsKHR {
        pub const INVALID: Self = Self(0);
    }

    impl fmt::Debug for VideoChromaSubsamplingFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoChromaSubsamplingFlagBitsKHR::MONOCHROME_KHR.0,
                    "MONOCHROME_KHR",
                ),
                (VideoChromaSubsamplingFlagBitsKHR::_420_KHR.0, "_420_KHR"),
                (VideoChromaSubsamplingFlagBitsKHR::_422_KHR.0, "_422_KHR"),
                (VideoChromaSubsamplingFlagBitsKHR::_444_KHR.0, "_444_KHR"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoChromaSubsamplingFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoChromaSubsamplingFlagBitsKHR(u32);

    impl VideoChromaSubsamplingFlagBitsKHR {
        pub const MONOCHROME_KHR: Self = Self(1 << 0);
        pub const _420_KHR: Self = Self(1 << 1);
        pub const _422_KHR: Self = Self(1 << 2);
        pub const _444_KHR: Self = Self(1 << 3);
    }

    impl fmt::Debug for VideoChromaSubsamplingFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::MONOCHROME_KHR => Some("MONOCHROME_KHR"),
                Self::_420_KHR => Some("_420_KHR"),
                Self::_422_KHR => Some("_422_KHR"),
                Self::_444_KHR => Some("_444_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoComponentBitDepthFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoComponentBitDepthFlagsKHR(Flags);
    vk_bitflags_wrapped!(
        VideoComponentBitDepthFlagsKHR,
        Flags,
        VideoComponentBitDepthFlagBitsKHR
    );

    impl VideoComponentBitDepthFlagsKHR {
        pub const INVALID: Self = Self(0);
    }

    impl fmt::Debug for VideoComponentBitDepthFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (VideoComponentBitDepthFlagBitsKHR::_8_KHR.0, "_8_KHR"),
                (VideoComponentBitDepthFlagBitsKHR::_10_KHR.0, "_10_KHR"),
                (VideoComponentBitDepthFlagBitsKHR::_12_KHR.0, "_12_KHR"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoComponentBitDepthFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoComponentBitDepthFlagBitsKHR(u32);

    impl VideoComponentBitDepthFlagBitsKHR {
        pub const _8_KHR: Self = Self(1 << 0);
        pub const _10_KHR: Self = Self(1 << 2);
        pub const _12_KHR: Self = Self(1 << 4);
    }

    impl fmt::Debug for VideoComponentBitDepthFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_8_KHR => Some("_8_KHR"),
                Self::_10_KHR => Some("_10_KHR"),
                Self::_12_KHR => Some("_12_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceVideoCapabilitiesKHR.html>
    pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_video_profile: *const VideoProfileInfoKHR<'_>,
        p_capabilities: *mut VideoCapabilitiesKHR<'_>,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html>
    pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR<'_>,
            p_video_format_property_count: *mut u32,
            p_video_format_properties: *mut VideoFormatPropertiesKHR<'_>,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateVideoSessionKHR.html>
    pub type PFN_vkCreateVideoSessionKHR = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const VideoSessionCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_video_session: *mut VideoSessionKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyVideoSessionKHR.html>
    pub type PFN_vkDestroyVideoSessionKHR = unsafe extern "system" fn(
        device: Device,
        video_session: VideoSessionKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateVideoSessionParametersKHR.html>
    pub type PFN_vkCreateVideoSessionParametersKHR = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const VideoSessionParametersCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_video_session_parameters: *mut VideoSessionParametersKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUpdateVideoSessionParametersKHR.html>
    pub type PFN_vkUpdateVideoSessionParametersKHR = unsafe extern "system" fn(
        device: Device,
        video_session_parameters: VideoSessionParametersKHR,
        p_update_info: *const VideoSessionParametersUpdateInfoKHR<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyVideoSessionParametersKHR.html>
    pub type PFN_vkDestroyVideoSessionParametersKHR = unsafe extern "system" fn(
        device: Device,
        video_session_parameters: VideoSessionParametersKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetVideoSessionMemoryRequirementsKHR.html>
    pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = unsafe extern "system" fn(
        device: Device,
        video_session: VideoSessionKHR,
        p_memory_requirements_count: *mut u32,
        p_memory_requirements: *mut VideoSessionMemoryRequirementsKHR<'_>,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBindVideoSessionMemoryKHR.html>
    pub type PFN_vkBindVideoSessionMemoryKHR = unsafe extern "system" fn(
        device: Device,
        video_session: VideoSessionKHR,
        bind_session_memory_info_count: u32,
        p_bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginVideoCodingKHR.html>
    pub type PFN_vkCmdBeginVideoCodingKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_begin_info: *const VideoBeginCodingInfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdControlVideoCodingKHR.html>
    pub type PFN_vkCmdControlVideoCodingKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_coding_control_info: *const VideoCodingControlInfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndVideoCodingKHR.html>
    pub type PFN_vkCmdEndVideoCodingKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_end_coding_info: *const VideoEndCodingInfoKHR<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkVideoSessionKHR = VideoSessionKHR;
    pub type VkVideoSessionParametersKHR = VideoSessionParametersKHR;
    pub type VkQueueFamilyVideoPropertiesKHR = QueueFamilyVideoPropertiesKHR<'static>;
    pub type VkQueueFamilyQueryResultStatusPropertiesKHR =
        QueueFamilyQueryResultStatusPropertiesKHR<'static>;
    pub type VkVideoProfileListInfoKHR = VideoProfileListInfoKHR<'static>;
    pub type VkPhysicalDeviceVideoFormatInfoKHR = PhysicalDeviceVideoFormatInfoKHR<'static>;
    pub type VkVideoFormatPropertiesKHR = VideoFormatPropertiesKHR<'static>;
    pub type VkVideoProfileInfoKHR = VideoProfileInfoKHR<'static>;
    pub type VkVideoCapabilitiesKHR = VideoCapabilitiesKHR<'static>;
    pub type VkVideoSessionMemoryRequirementsKHR = VideoSessionMemoryRequirementsKHR<'static>;
    pub type VkBindVideoSessionMemoryInfoKHR = BindVideoSessionMemoryInfoKHR<'static>;
    pub type VkVideoPictureResourceInfoKHR = VideoPictureResourceInfoKHR<'static>;
    pub type VkVideoReferenceSlotInfoKHR = VideoReferenceSlotInfoKHR<'static>;
    pub type VkVideoSessionCreateInfoKHR = VideoSessionCreateInfoKHR<'static>;
    pub type VkVideoSessionParametersCreateInfoKHR = VideoSessionParametersCreateInfoKHR<'static>;
    pub type VkVideoSessionParametersUpdateInfoKHR = VideoSessionParametersUpdateInfoKHR<'static>;
    pub type VkVideoBeginCodingInfoKHR = VideoBeginCodingInfoKHR<'static>;
    pub type VkVideoEndCodingInfoKHR = VideoEndCodingInfoKHR<'static>;
    pub type VkVideoCodingControlInfoKHR = VideoCodingControlInfoKHR<'static>;
    pub type VkQueryResultStatusKHR = QueryResultStatusKHR;
    pub type VkVideoCodecOperationFlagsKHR = VideoCodecOperationFlagsKHR;
    pub type VkVideoCodecOperationFlagBitsKHR = VideoCodecOperationFlagBitsKHR;
    pub type VkVideoCapabilityFlagsKHR = VideoCapabilityFlagsKHR;
    pub type VkVideoCapabilityFlagBitsKHR = VideoCapabilityFlagBitsKHR;
    pub type VkVideoSessionCreateFlagsKHR = VideoSessionCreateFlagsKHR;
    pub type VkVideoSessionCreateFlagBitsKHR = VideoSessionCreateFlagBitsKHR;
    pub type VkVideoSessionParametersCreateFlagsKHR = VideoSessionParametersCreateFlagsKHR;
    pub type VkVideoSessionParametersCreateFlagBitsKHR = VideoSessionParametersCreateFlagBitsKHR;
    pub type VkVideoBeginCodingFlagsKHR = VideoBeginCodingFlagsKHR;
    pub type VkVideoEndCodingFlagsKHR = VideoEndCodingFlagsKHR;
    pub type VkVideoCodingControlFlagsKHR = VideoCodingControlFlagsKHR;
    pub type VkVideoCodingControlFlagBitsKHR = VideoCodingControlFlagBitsKHR;
    pub type VkVideoChromaSubsamplingFlagsKHR = VideoChromaSubsamplingFlagsKHR;
    pub type VkVideoChromaSubsamplingFlagBitsKHR = VideoChromaSubsamplingFlagBitsKHR;
    pub type VkVideoComponentBitDepthFlagsKHR = VideoComponentBitDepthFlagsKHR;
    pub type VkVideoComponentBitDepthFlagBitsKHR = VideoComponentBitDepthFlagBitsKHR;
    impl QueueFamilyVideoPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkQueueFamilyVideoPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl QueueFamilyQueryResultStatusPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkQueueFamilyQueryResultStatusPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoProfileListInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoProfileListInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceVideoFormatInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceVideoFormatInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoFormatPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoFormatPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoProfileInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoProfileInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoCapabilitiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoCapabilitiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoSessionMemoryRequirementsKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoSessionMemoryRequirementsKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BindVideoSessionMemoryInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBindVideoSessionMemoryInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoPictureResourceInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoPictureResourceInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoReferenceSlotInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoReferenceSlotInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoSessionCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoSessionCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoSessionParametersCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoSessionParametersCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoSessionParametersUpdateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoSessionParametersUpdateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoBeginCodingInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoBeginCodingInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEndCodingInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEndCodingInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoCodingControlInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoCodingControlInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    get_physical_device_video_capabilities_khr: PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR,
    get_physical_device_video_format_properties_khr:
        PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_video_capabilities_khr: transmute(
                    load(c"vkGetPhysicalDeviceVideoCapabilitiesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_video_format_properties_khr: transmute(
                    load(c"vkGetPhysicalDeviceVideoFormatPropertiesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceVideoCapabilitiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_video_capabilities_khr(
        &self,
        physical_device: PhysicalDevice,
        video_profile: &VideoProfileInfoKHR<'_>,
        capabilities: &mut VideoCapabilitiesKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_physical_device_video_capabilities_khr)(
                physical_device,
                video_profile,
                capabilities,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_video_format_properties_khr<'a>(
        &self,
        physical_device: PhysicalDevice,
        video_format_info: &PhysicalDeviceVideoFormatInfoKHR<'a>,
        mut video_format_properties: impl ExtendUninit<VideoFormatPropertiesKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |video_format_property_count, video_format_properties| {
                let result = (self.get_physical_device_video_format_properties_khr)(
                    physical_device,
                    video_format_info,
                    video_format_property_count,
                    video_format_properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let video_format_properties_buf = video_format_properties.reserve(capacity);
            len = video_format_properties_buf.len().try_into().unwrap();
            let result = call(&mut len, video_format_properties_buf.as_mut_ptr() as *mut _)?;
            video_format_properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}

pub struct DeviceFn {
    create_video_session_khr: PFN_vkCreateVideoSessionKHR,
    destroy_video_session_khr: PFN_vkDestroyVideoSessionKHR,
    get_video_session_memory_requirements_khr: PFN_vkGetVideoSessionMemoryRequirementsKHR,
    bind_video_session_memory_khr: PFN_vkBindVideoSessionMemoryKHR,
    create_video_session_parameters_khr: PFN_vkCreateVideoSessionParametersKHR,
    update_video_session_parameters_khr: PFN_vkUpdateVideoSessionParametersKHR,
    destroy_video_session_parameters_khr: PFN_vkDestroyVideoSessionParametersKHR,
    cmd_begin_video_coding_khr: PFN_vkCmdBeginVideoCodingKHR,
    cmd_end_video_coding_khr: PFN_vkCmdEndVideoCodingKHR,
    cmd_control_video_coding_khr: PFN_vkCmdControlVideoCodingKHR,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_video_session_khr: transmute(
                    load(c"vkCreateVideoSessionKHR").ok_or(MissingEntryPointError)?,
                ),
                destroy_video_session_khr: transmute(
                    load(c"vkDestroyVideoSessionKHR").ok_or(MissingEntryPointError)?,
                ),
                get_video_session_memory_requirements_khr: transmute(
                    load(c"vkGetVideoSessionMemoryRequirementsKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                bind_video_session_memory_khr: transmute(
                    load(c"vkBindVideoSessionMemoryKHR").ok_or(MissingEntryPointError)?,
                ),
                create_video_session_parameters_khr: transmute(
                    load(c"vkCreateVideoSessionParametersKHR").ok_or(MissingEntryPointError)?,
                ),
                update_video_session_parameters_khr: transmute(
                    load(c"vkUpdateVideoSessionParametersKHR").ok_or(MissingEntryPointError)?,
                ),
                destroy_video_session_parameters_khr: transmute(
                    load(c"vkDestroyVideoSessionParametersKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_begin_video_coding_khr: transmute(
                    load(c"vkCmdBeginVideoCodingKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_end_video_coding_khr: transmute(
                    load(c"vkCmdEndVideoCodingKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_control_video_coding_khr: transmute(
                    load(c"vkCmdControlVideoCodingKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateVideoSessionKHR.html>
    #[inline]
    pub unsafe fn create_video_session_khr(
        &self,
        device: Device,
        create_info: &VideoSessionCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<VideoSessionKHR> {
        unsafe {
            let mut video_session = core::mem::MaybeUninit::uninit();
            let result = (self.create_video_session_khr)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                video_session.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(video_session.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyVideoSessionKHR.html>
    #[inline]
    pub unsafe fn destroy_video_session_khr(
        &self,
        device: Device,
        video_session: VideoSessionKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_video_session_khr)(device, video_session, allocator.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetVideoSessionMemoryRequirementsKHR.html>
    #[inline]
    pub unsafe fn get_video_session_memory_requirements_khr<'a>(
        &self,
        device: Device,
        video_session: VideoSessionKHR,
        mut memory_requirements: impl ExtendUninit<VideoSessionMemoryRequirementsKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |memory_requirements_count, memory_requirements| {
                let result = (self.get_video_session_memory_requirements_khr)(
                    device,
                    video_session,
                    memory_requirements_count,
                    memory_requirements as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let memory_requirements_buf = memory_requirements.reserve(capacity);
            len = memory_requirements_buf.len().try_into().unwrap();
            let result = call(&mut len, memory_requirements_buf.as_mut_ptr() as *mut _)?;
            memory_requirements.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBindVideoSessionMemoryKHR.html>
    #[inline]
    pub unsafe fn bind_video_session_memory_khr(
        &self,
        device: Device,
        video_session: VideoSessionKHR,
        bind_session_memory_infos: &[BindVideoSessionMemoryInfoKHR<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.bind_video_session_memory_khr)(
                device,
                video_session,
                bind_session_memory_infos.len().try_into().unwrap(),
                bind_session_memory_infos.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateVideoSessionParametersKHR.html>
    #[inline]
    pub unsafe fn create_video_session_parameters_khr(
        &self,
        device: Device,
        create_info: &VideoSessionParametersCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<VideoSessionParametersKHR> {
        unsafe {
            let mut video_session_parameters = core::mem::MaybeUninit::uninit();
            let result = (self.create_video_session_parameters_khr)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                video_session_parameters.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(video_session_parameters.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUpdateVideoSessionParametersKHR.html>
    #[inline]
    pub unsafe fn update_video_session_parameters_khr(
        &self,
        device: Device,
        video_session_parameters: VideoSessionParametersKHR,
        update_info: &VideoSessionParametersUpdateInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.update_video_session_parameters_khr)(
                device,
                video_session_parameters,
                update_info,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyVideoSessionParametersKHR.html>
    #[inline]
    pub unsafe fn destroy_video_session_parameters_khr(
        &self,
        device: Device,
        video_session_parameters: VideoSessionParametersKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_video_session_parameters_khr)(
                device,
                video_session_parameters,
                allocator.to_raw_ptr(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginVideoCodingKHR.html>
    #[inline]
    pub unsafe fn cmd_begin_video_coding_khr(
        &self,
        command_buffer: CommandBuffer,
        begin_info: &VideoBeginCodingInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_begin_video_coding_khr)(command_buffer, begin_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndVideoCodingKHR.html>
    #[inline]
    pub unsafe fn cmd_end_video_coding_khr(
        &self,
        command_buffer: CommandBuffer,
        end_coding_info: &VideoEndCodingInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_end_video_coding_khr)(command_buffer, end_coding_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdControlVideoCodingKHR.html>
    #[inline]
    pub unsafe fn cmd_control_video_coding_khr(
        &self,
        command_buffer: CommandBuffer,
        coding_control_info: &VideoCodingControlInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_control_video_coding_khr)(command_buffer, coding_control_info) }
    }
}
