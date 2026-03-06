#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    handle_nondispatchable!(VideoSessionKHR, VIDEO_SESSION_KHR, doc = "");
    handle_nondispatchable!(
        VideoSessionParametersKHR,
        VIDEO_SESSION_PARAMETERS_KHR,
        doc = ""
    );
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct QueueFamilyVideoPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub video_codec_operations: VideoCodecOperationFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for QueueFamilyVideoPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::QUEUE_FAMILY_VIDEO_PROPERTIES_KHR;
    }
    unsafe impl<'a> Extends<QueueFamilyProperties2<'a>> for QueueFamilyVideoPropertiesKHR<'a> {}
    impl Default for QueueFamilyVideoPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                video_codec_operations: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> QueueFamilyVideoPropertiesKHR<'a> {
        pub fn video_codec_operations(
            mut self,
            video_codec_operations: VideoCodecOperationFlagsKHR,
        ) -> Self {
            self.video_codec_operations = video_codec_operations;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct QueueFamilyQueryResultStatusPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub query_result_status_support: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for QueueFamilyQueryResultStatusPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR;
    }
    unsafe impl<'a> Extends<QueueFamilyProperties2<'a>>
        for QueueFamilyQueryResultStatusPropertiesKHR<'a>
    {
    }
    impl Default for QueueFamilyQueryResultStatusPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                query_result_status_support: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> QueueFamilyQueryResultStatusPropertiesKHR<'a> {
        pub fn query_result_status_support(mut self, query_result_status_support: bool) -> Self {
            self.query_result_status_support = query_result_status_support.into();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoProfileListInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub profile_count: u32,
        pub p_profiles: *const VideoProfileInfoKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoProfileListInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_PROFILE_LIST_INFO_KHR;
    }
    unsafe impl<'a> Extends<PhysicalDeviceImageFormatInfo2<'a>> for VideoProfileListInfoKHR<'a> {}
    unsafe impl<'a> Extends<PhysicalDeviceVideoFormatInfoKHR<'a>> for VideoProfileListInfoKHR<'a> {}
    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for VideoProfileListInfoKHR<'a> {}
    unsafe impl<'a> Extends<BufferCreateInfo<'a>> for VideoProfileListInfoKHR<'a> {}
    impl Default for VideoProfileListInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                profile_count: Default::default(),
                p_profiles: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoProfileListInfoKHR<'a> {
        pub fn profiles(mut self, profiles: &'a [VideoProfileInfoKHR<'a>]) -> Self {
            self.profile_count = profiles.len().try_into().unwrap();
            self.p_profiles = profiles.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVideoFormatInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image_usage: ImageUsageFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVideoFormatInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR;
    }
    impl Default for PhysicalDeviceVideoFormatInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                image_usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceVideoFormatInfoKHR<'a> {
        pub fn image_usage(mut self, image_usage: ImageUsageFlags) -> Self {
            self.image_usage = image_usage;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for VideoFormatPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_FORMAT_PROPERTIES_KHR;
    }
    impl Default for VideoFormatPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
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
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
        pub fn component_mapping(mut self, component_mapping: ComponentMapping) -> Self {
            self.component_mapping = component_mapping;
            self
        }
        pub fn image_create_flags(mut self, image_create_flags: ImageCreateFlags) -> Self {
            self.image_create_flags = image_create_flags;
            self
        }
        pub fn image_type(mut self, image_type: ImageType) -> Self {
            self.image_type = image_type;
            self
        }
        pub fn image_tiling(mut self, image_tiling: ImageTiling) -> Self {
            self.image_tiling = image_tiling;
            self
        }
        pub fn image_usage_flags(mut self, image_usage_flags: ImageUsageFlags) -> Self {
            self.image_usage_flags = image_usage_flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoProfileInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub video_codec_operation: VideoCodecOperationFlagBitsKHR,
        pub chroma_subsampling: VideoChromaSubsamplingFlagsKHR,
        pub luma_bit_depth: VideoComponentBitDepthFlagsKHR,
        pub chroma_bit_depth: VideoComponentBitDepthFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoProfileInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_PROFILE_INFO_KHR;
    }
    unsafe impl<'a> Extends<QueryPoolCreateInfo<'a>> for VideoProfileInfoKHR<'a> {}
    impl Default for VideoProfileInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                video_codec_operation: Default::default(),
                chroma_subsampling: Default::default(),
                luma_bit_depth: Default::default(),
                chroma_bit_depth: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoProfileInfoKHR<'a> {
        pub fn video_codec_operation(
            mut self,
            video_codec_operation: VideoCodecOperationFlagBitsKHR,
        ) -> Self {
            self.video_codec_operation = video_codec_operation;
            self
        }
        pub fn chroma_subsampling(
            mut self,
            chroma_subsampling: VideoChromaSubsamplingFlagsKHR,
        ) -> Self {
            self.chroma_subsampling = chroma_subsampling;
            self
        }
        pub fn luma_bit_depth(mut self, luma_bit_depth: VideoComponentBitDepthFlagsKHR) -> Self {
            self.luma_bit_depth = luma_bit_depth;
            self
        }
        pub fn chroma_bit_depth(
            mut self,
            chroma_bit_depth: VideoComponentBitDepthFlagsKHR,
        ) -> Self {
            self.chroma_bit_depth = chroma_bit_depth;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for VideoCapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_CAPABILITIES_KHR;
    }
    impl Default for VideoCapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
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
        pub fn flags(mut self, flags: VideoCapabilityFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn min_bitstream_buffer_offset_alignment(
            mut self,
            min_bitstream_buffer_offset_alignment: DeviceSize,
        ) -> Self {
            self.min_bitstream_buffer_offset_alignment = min_bitstream_buffer_offset_alignment;
            self
        }
        pub fn min_bitstream_buffer_size_alignment(
            mut self,
            min_bitstream_buffer_size_alignment: DeviceSize,
        ) -> Self {
            self.min_bitstream_buffer_size_alignment = min_bitstream_buffer_size_alignment;
            self
        }
        pub fn picture_access_granularity(mut self, picture_access_granularity: Extent2D) -> Self {
            self.picture_access_granularity = picture_access_granularity;
            self
        }
        pub fn min_coded_extent(mut self, min_coded_extent: Extent2D) -> Self {
            self.min_coded_extent = min_coded_extent;
            self
        }
        pub fn max_coded_extent(mut self, max_coded_extent: Extent2D) -> Self {
            self.max_coded_extent = max_coded_extent;
            self
        }
        pub fn max_dpb_slots(mut self, max_dpb_slots: u32) -> Self {
            self.max_dpb_slots = max_dpb_slots;
            self
        }
        pub fn max_active_reference_pictures(mut self, max_active_reference_pictures: u32) -> Self {
            self.max_active_reference_pictures = max_active_reference_pictures;
            self
        }
        pub fn std_header_version(mut self, std_header_version: ExtensionProperties) -> Self {
            self.std_header_version = std_header_version;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoSessionMemoryRequirementsKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_bind_index: u32,
        pub memory_requirements: MemoryRequirements,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoSessionMemoryRequirementsKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR;
    }
    impl Default for VideoSessionMemoryRequirementsKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                memory_bind_index: Default::default(),
                memory_requirements: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoSessionMemoryRequirementsKHR<'a> {
        pub fn memory_bind_index(mut self, memory_bind_index: u32) -> Self {
            self.memory_bind_index = memory_bind_index;
            self
        }
        pub fn memory_requirements(mut self, memory_requirements: MemoryRequirements) -> Self {
            self.memory_requirements = memory_requirements;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BindVideoSessionMemoryInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory_bind_index: u32,
        pub memory: DeviceMemory,
        pub memory_offset: DeviceSize,
        pub memory_size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BindVideoSessionMemoryInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_VIDEO_SESSION_MEMORY_INFO_KHR;
    }
    impl Default for BindVideoSessionMemoryInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                memory_bind_index: Default::default(),
                memory: Default::default(),
                memory_offset: Default::default(),
                memory_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BindVideoSessionMemoryInfoKHR<'a> {
        pub fn memory_bind_index(mut self, memory_bind_index: u32) -> Self {
            self.memory_bind_index = memory_bind_index;
            self
        }
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
        pub fn memory_offset(mut self, memory_offset: DeviceSize) -> Self {
            self.memory_offset = memory_offset;
            self
        }
        pub fn memory_size(mut self, memory_size: DeviceSize) -> Self {
            self.memory_size = memory_size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoPictureResourceInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub coded_offset: Offset2D,
        pub coded_extent: Extent2D,
        pub base_array_layer: u32,
        pub image_view_binding: ImageView,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoPictureResourceInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_PICTURE_RESOURCE_INFO_KHR;
    }
    impl Default for VideoPictureResourceInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                coded_offset: Default::default(),
                coded_extent: Default::default(),
                base_array_layer: Default::default(),
                image_view_binding: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoPictureResourceInfoKHR<'a> {
        pub fn coded_offset(mut self, coded_offset: Offset2D) -> Self {
            self.coded_offset = coded_offset;
            self
        }
        pub fn coded_extent(mut self, coded_extent: Extent2D) -> Self {
            self.coded_extent = coded_extent;
            self
        }
        pub fn base_array_layer(mut self, base_array_layer: u32) -> Self {
            self.base_array_layer = base_array_layer;
            self
        }
        pub fn image_view_binding(mut self, image_view_binding: ImageView) -> Self {
            self.image_view_binding = image_view_binding;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoReferenceSlotInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub slot_index: i32,
        pub p_picture_resource: *const VideoPictureResourceInfoKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoReferenceSlotInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_REFERENCE_SLOT_INFO_KHR;
    }
    impl Default for VideoReferenceSlotInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                slot_index: Default::default(),
                p_picture_resource: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoReferenceSlotInfoKHR<'a> {
        pub fn slot_index(mut self, slot_index: i32) -> Self {
            self.slot_index = slot_index;
            self
        }
        pub fn picture_resource(
            mut self,
            picture_resource: &'a VideoPictureResourceInfoKHR<'a>,
        ) -> Self {
            self.p_picture_resource = picture_resource;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for VideoSessionCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_SESSION_CREATE_INFO_KHR;
    }
    impl Default for VideoSessionCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                queue_family_index: Default::default(),
                flags: Default::default(),
                p_video_profile: core::ptr::null(),
                picture_format: Default::default(),
                max_coded_extent: Default::default(),
                reference_picture_format: Default::default(),
                max_dpb_slots: Default::default(),
                max_active_reference_pictures: Default::default(),
                p_std_header_version: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoSessionCreateInfoKHR<'a> {
        pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
            self.queue_family_index = queue_family_index;
            self
        }
        pub fn flags(mut self, flags: VideoSessionCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn video_profile(mut self, video_profile: &'a VideoProfileInfoKHR<'a>) -> Self {
            self.p_video_profile = video_profile;
            self
        }
        pub fn picture_format(mut self, picture_format: Format) -> Self {
            self.picture_format = picture_format;
            self
        }
        pub fn max_coded_extent(mut self, max_coded_extent: Extent2D) -> Self {
            self.max_coded_extent = max_coded_extent;
            self
        }
        pub fn reference_picture_format(mut self, reference_picture_format: Format) -> Self {
            self.reference_picture_format = reference_picture_format;
            self
        }
        pub fn max_dpb_slots(mut self, max_dpb_slots: u32) -> Self {
            self.max_dpb_slots = max_dpb_slots;
            self
        }
        pub fn max_active_reference_pictures(mut self, max_active_reference_pictures: u32) -> Self {
            self.max_active_reference_pictures = max_active_reference_pictures;
            self
        }
        pub fn std_header_version(mut self, std_header_version: &'a ExtensionProperties) -> Self {
            self.p_std_header_version = std_header_version;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoSessionParametersCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: VideoSessionParametersCreateFlagsKHR,
        pub video_session_parameters_template: VideoSessionParametersKHR,
        pub video_session: VideoSessionKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoSessionParametersCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR;
    }
    impl Default for VideoSessionParametersCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                video_session_parameters_template: Default::default(),
                video_session: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoSessionParametersCreateInfoKHR<'a> {
        pub fn flags(mut self, flags: VideoSessionParametersCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn video_session_parameters_template(
            mut self,
            video_session_parameters_template: VideoSessionParametersKHR,
        ) -> Self {
            self.video_session_parameters_template = video_session_parameters_template;
            self
        }
        pub fn video_session(mut self, video_session: VideoSessionKHR) -> Self {
            self.video_session = video_session;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoSessionParametersUpdateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub update_sequence_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoSessionParametersUpdateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR;
    }
    impl Default for VideoSessionParametersUpdateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                update_sequence_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoSessionParametersUpdateInfoKHR<'a> {
        pub fn update_sequence_count(mut self, update_sequence_count: u32) -> Self {
            self.update_sequence_count = update_sequence_count;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for VideoBeginCodingInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_BEGIN_CODING_INFO_KHR;
    }
    impl Default for VideoBeginCodingInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                video_session: Default::default(),
                video_session_parameters: Default::default(),
                reference_slot_count: Default::default(),
                p_reference_slots: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoBeginCodingInfoKHR<'a> {
        pub fn flags(mut self, flags: VideoBeginCodingFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn video_session(mut self, video_session: VideoSessionKHR) -> Self {
            self.video_session = video_session;
            self
        }
        pub fn video_session_parameters(
            mut self,
            video_session_parameters: VideoSessionParametersKHR,
        ) -> Self {
            self.video_session_parameters = video_session_parameters;
            self
        }
        pub fn reference_slots(
            mut self,
            reference_slots: &'a [VideoReferenceSlotInfoKHR<'a>],
        ) -> Self {
            self.reference_slot_count = reference_slots.len().try_into().unwrap();
            self.p_reference_slots = reference_slots.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEndCodingInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: VideoEndCodingFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoEndCodingInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_END_CODING_INFO_KHR;
    }
    impl Default for VideoEndCodingInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoEndCodingInfoKHR<'a> {
        pub fn flags(mut self, flags: VideoEndCodingFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoCodingControlInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: VideoCodingControlFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoCodingControlInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_CODING_CONTROL_INFO_KHR;
    }
    impl Default for VideoCodingControlInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoCodingControlInfoKHR<'a> {
        pub fn flags(mut self, flags: VideoCodingControlFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct QueryResultStatusKHR(i32);
    impl QueryResultStatusKHR {
        pub const ERROR_KHR: Self = Self(-1);
        pub const NOT_READY_KHR: Self = Self(0);
        pub const COMPLETE_KHR: Self = Self(1);
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
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoCodecOperationFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoCodecOperationFlagsKHR, Flags);
    impl VideoCodecOperationFlagsKHR {
        pub const DECODE_H264_KHR: Self = Self(VideoCodecOperationFlagBitsKHR::DECODE_H264_KHR.0);
        pub const DECODE_H265_KHR: Self = Self(VideoCodecOperationFlagBitsKHR::DECODE_H265_KHR.0);
        pub const DECODE_AV1_KHR: Self = Self(VideoCodecOperationFlagBitsKHR::DECODE_AV1_KHR.0);
        pub const DECODE_VP9_KHR: Self = Self(VideoCodecOperationFlagBitsKHR::DECODE_VP9_KHR.0);
        pub const ENCODE_H264_KHR: Self = Self(VideoCodecOperationFlagBitsKHR::ENCODE_H264_KHR.0);
        pub const ENCODE_H265_KHR: Self = Self(VideoCodecOperationFlagBitsKHR::ENCODE_H265_KHR.0);
        pub const ENCODE_AV1_KHR: Self = Self(VideoCodecOperationFlagBitsKHR::ENCODE_AV1_KHR.0);
        pub const NONE: Self = Self(0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoCodecOperationFlagBitsKHR(u32);
    impl VideoCodecOperationFlagBitsKHR {
        pub const DECODE_H264_KHR: Self = Self(1 << 0);
        pub const DECODE_H265_KHR: Self = Self(1 << 1);
        pub const DECODE_AV1_KHR: Self = Self(1 << 2);
        pub const DECODE_VP9_KHR: Self = Self(1 << 3);
        pub const ENCODE_H264_KHR: Self = Self(1 << 16);
        pub const ENCODE_H265_KHR: Self = Self(1 << 17);
        pub const ENCODE_AV1_KHR: Self = Self(1 << 18);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoCapabilityFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoCapabilityFlagsKHR, Flags);
    impl VideoCapabilityFlagsKHR {
        pub const PROTECTED_CONTENT_KHR: Self =
            Self(VideoCapabilityFlagBitsKHR::PROTECTED_CONTENT_KHR.0);
        pub const SEPARATE_REFERENCE_IMAGES_KHR: Self =
            Self(VideoCapabilityFlagBitsKHR::SEPARATE_REFERENCE_IMAGES_KHR.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoCapabilityFlagBitsKHR(u32);
    impl VideoCapabilityFlagBitsKHR {
        pub const PROTECTED_CONTENT_KHR: Self = Self(1 << 0);
        pub const SEPARATE_REFERENCE_IMAGES_KHR: Self = Self(1 << 1);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoSessionCreateFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoSessionCreateFlagsKHR, Flags);
    impl VideoSessionCreateFlagsKHR {
        pub const PROTECTED_CONTENT_KHR: Self =
            Self(VideoSessionCreateFlagBitsKHR::PROTECTED_CONTENT_KHR.0);
        pub const ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR: Self =
            Self(VideoSessionCreateFlagBitsKHR::ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR.0);
        pub const INLINE_QUERIES_KHR: Self =
            Self(VideoSessionCreateFlagBitsKHR::INLINE_QUERIES_KHR.0);
        pub const ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self =
            Self(VideoSessionCreateFlagBitsKHR::ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR.0);
        pub const ALLOW_ENCODE_EMPHASIS_MAP_KHR: Self =
            Self(VideoSessionCreateFlagBitsKHR::ALLOW_ENCODE_EMPHASIS_MAP_KHR.0);
        pub const INLINE_SESSION_PARAMETERS_KHR: Self =
            Self(VideoSessionCreateFlagBitsKHR::INLINE_SESSION_PARAMETERS_KHR.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoSessionCreateFlagBitsKHR(u32);
    impl VideoSessionCreateFlagBitsKHR {
        pub const PROTECTED_CONTENT_KHR: Self = Self(1 << 0);
        pub const ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR: Self = Self(1 << 1);
        pub const INLINE_QUERIES_KHR: Self = Self(1 << 2);
        pub const ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self = Self(1 << 3);
        pub const ALLOW_ENCODE_EMPHASIS_MAP_KHR: Self = Self(1 << 4);
        pub const INLINE_SESSION_PARAMETERS_KHR: Self = Self(1 << 5);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoSessionParametersCreateFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoSessionParametersCreateFlagsKHR, Flags);
    impl VideoSessionParametersCreateFlagsKHR {
        pub const QUANTIZATION_MAP_COMPATIBLE_KHR: Self =
            Self(VideoSessionParametersCreateFlagBitsKHR::QUANTIZATION_MAP_COMPATIBLE_KHR.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoSessionParametersCreateFlagBitsKHR(u32);
    impl VideoSessionParametersCreateFlagBitsKHR {
        pub const QUANTIZATION_MAP_COMPATIBLE_KHR: Self = Self(1 << 0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoBeginCodingFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoBeginCodingFlagsKHR, Flags);
    impl VideoBeginCodingFlagsKHR {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEndCodingFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEndCodingFlagsKHR, Flags);
    impl VideoEndCodingFlagsKHR {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoCodingControlFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoCodingControlFlagsKHR, Flags);
    impl VideoCodingControlFlagsKHR {
        pub const RESET_KHR: Self = Self(VideoCodingControlFlagBitsKHR::RESET_KHR.0);
        pub const ENCODE_RATE_CONTROL_KHR: Self =
            Self(VideoCodingControlFlagBitsKHR::ENCODE_RATE_CONTROL_KHR.0);
        pub const ENCODE_QUALITY_LEVEL_KHR: Self =
            Self(VideoCodingControlFlagBitsKHR::ENCODE_QUALITY_LEVEL_KHR.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoCodingControlFlagBitsKHR(u32);
    impl VideoCodingControlFlagBitsKHR {
        pub const RESET_KHR: Self = Self(1 << 0);
        pub const ENCODE_RATE_CONTROL_KHR: Self = Self(1 << 1);
        pub const ENCODE_QUALITY_LEVEL_KHR: Self = Self(1 << 2);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoChromaSubsamplingFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoChromaSubsamplingFlagsKHR, Flags);
    impl VideoChromaSubsamplingFlagsKHR {
        pub const MONOCHROME_KHR: Self = Self(VideoChromaSubsamplingFlagBitsKHR::MONOCHROME_KHR.0);
        pub const _420_KHR: Self = Self(VideoChromaSubsamplingFlagBitsKHR::_420_KHR.0);
        pub const _422_KHR: Self = Self(VideoChromaSubsamplingFlagBitsKHR::_422_KHR.0);
        pub const _444_KHR: Self = Self(VideoChromaSubsamplingFlagBitsKHR::_444_KHR.0);
        pub const INVALID: Self = Self(0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoChromaSubsamplingFlagBitsKHR(u32);
    impl VideoChromaSubsamplingFlagBitsKHR {
        pub const MONOCHROME_KHR: Self = Self(1 << 0);
        pub const _420_KHR: Self = Self(1 << 1);
        pub const _422_KHR: Self = Self(1 << 2);
        pub const _444_KHR: Self = Self(1 << 3);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoComponentBitDepthFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoComponentBitDepthFlagsKHR, Flags);
    impl VideoComponentBitDepthFlagsKHR {
        pub const _8_KHR: Self = Self(VideoComponentBitDepthFlagBitsKHR::_8_KHR.0);
        pub const _10_KHR: Self = Self(VideoComponentBitDepthFlagBitsKHR::_10_KHR.0);
        pub const _12_KHR: Self = Self(VideoComponentBitDepthFlagBitsKHR::_12_KHR.0);
        pub const INVALID: Self = Self(0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoComponentBitDepthFlagBitsKHR(u32);
    impl VideoComponentBitDepthFlagBitsKHR {
        pub const _8_KHR: Self = Self(1 << 0);
        pub const _10_KHR: Self = Self(1 << 2);
        pub const _12_KHR: Self = Self(1 << 4);
    }
    pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_video_profile: *const VideoProfileInfoKHR<'_>,
        p_capabilities: *mut VideoCapabilitiesKHR<'_>,
    )
        -> vk::Result;
    pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR<'_>,
            p_video_format_property_count: *mut u32,
            p_video_format_properties: *mut VideoFormatPropertiesKHR<'_>,
        ) -> vk::Result;
    pub type PFN_vkCreateVideoSessionKHR = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const VideoSessionCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_video_session: *mut VideoSessionKHR,
    ) -> vk::Result;
    pub type PFN_vkDestroyVideoSessionKHR = unsafe extern "system" fn(
        device: Device,
        video_session: VideoSessionKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkCreateVideoSessionParametersKHR = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const VideoSessionParametersCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_video_session_parameters: *mut VideoSessionParametersKHR,
    ) -> vk::Result;
    pub type PFN_vkUpdateVideoSessionParametersKHR = unsafe extern "system" fn(
        device: Device,
        video_session_parameters: VideoSessionParametersKHR,
        p_update_info: *const VideoSessionParametersUpdateInfoKHR<'_>,
    ) -> vk::Result;
    pub type PFN_vkDestroyVideoSessionParametersKHR = unsafe extern "system" fn(
        device: Device,
        video_session_parameters: VideoSessionParametersKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = unsafe extern "system" fn(
        device: Device,
        video_session: VideoSessionKHR,
        p_memory_requirements_count: *mut u32,
        p_memory_requirements: *mut VideoSessionMemoryRequirementsKHR<'_>,
    )
        -> vk::Result;
    pub type PFN_vkBindVideoSessionMemoryKHR = unsafe extern "system" fn(
        device: Device,
        video_session: VideoSessionKHR,
        bind_session_memory_info_count: u32,
        p_bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR<'_>,
    ) -> vk::Result;
    pub type PFN_vkCmdBeginVideoCodingKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_begin_info: *const VideoBeginCodingInfoKHR<'_>,
    );
    pub type PFN_vkCmdControlVideoCodingKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_coding_control_info: *const VideoCodingControlInfoKHR<'_>,
    );
    pub type PFN_vkCmdEndVideoCodingKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_end_coding_info: *const VideoEndCodingInfoKHR<'_>,
    );
}
pub struct InstanceFn {
    get_physical_device_video_capabilities_khr: PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR,
    get_physical_device_video_format_properties_khr:
        PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR,
}
impl InstanceFn {
    pub unsafe fn load(
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
impl DeviceFn {
    pub unsafe fn load(
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
    pub unsafe fn destroy_video_session_khr(
        &self,
        device: Device,
        video_session: VideoSessionKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_video_session_khr)(device, video_session, allocator.to_raw_ptr()) }
    }
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
    pub unsafe fn cmd_begin_video_coding_khr(
        &self,
        command_buffer: CommandBuffer,
        begin_info: &VideoBeginCodingInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_begin_video_coding_khr)(command_buffer, begin_info) }
    }
    pub unsafe fn cmd_end_video_coding_khr(
        &self,
        command_buffer: CommandBuffer,
        end_coding_info: &VideoEndCodingInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_end_video_coding_khr)(command_buffer, end_coding_info) }
    }
    pub unsafe fn cmd_control_video_coding_khr(
        &self,
        command_buffer: CommandBuffer,
        coding_control_info: &VideoCodingControlInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_control_video_coding_khr)(command_buffer, coding_control_info) }
    }
}
