#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVideoMaintenance2FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub video_maintenance2: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVideoMaintenance2FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_VIDEO_MAINTENANCE_2_FEATURES_KHR;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceVideoMaintenance2FeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceVideoMaintenance2FeaturesKHR<'a> {}
    impl Default for PhysicalDeviceVideoMaintenance2FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                video_maintenance2: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceVideoMaintenance2FeaturesKHR<'a> {
        pub fn video_maintenance2(mut self, video_maintenance2: Bool32) -> Self {
            self.video_maintenance2 = video_maintenance2;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoDecodeH264InlineSessionParametersInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_sps: *const StdVideoH264SequenceParameterSet<'a>,
        pub p_std_pps: *const StdVideoH264PictureParameterSet<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH264InlineSessionParametersInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_DECODE_H264_INLINE_SESSION_PARAMETERS_INFO_KHR;
    }
    unsafe impl<'a> Extends<VideoDecodeInfoKHR<'a>>
        for VideoDecodeH264InlineSessionParametersInfoKHR<'a>
    {
    }
    impl Default for VideoDecodeH264InlineSessionParametersInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_std_sps: core::ptr::null(),
                p_std_pps: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoDecodeH264InlineSessionParametersInfoKHR<'a> {
        pub fn std_sps(mut self, std_sps: &'a StdVideoH264SequenceParameterSet<'a>) -> Self {
            self.p_std_sps = std_sps;
            self
        }
        pub fn std_pps(mut self, std_pps: &'a StdVideoH264PictureParameterSet<'a>) -> Self {
            self.p_std_pps = std_pps;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoDecodeH265InlineSessionParametersInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_vps: *const StdVideoH265VideoParameterSet<'a>,
        pub p_std_sps: *const StdVideoH265SequenceParameterSet<'a>,
        pub p_std_pps: *const StdVideoH265PictureParameterSet<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH265InlineSessionParametersInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_DECODE_H265_INLINE_SESSION_PARAMETERS_INFO_KHR;
    }
    unsafe impl<'a> Extends<VideoDecodeInfoKHR<'a>>
        for VideoDecodeH265InlineSessionParametersInfoKHR<'a>
    {
    }
    impl Default for VideoDecodeH265InlineSessionParametersInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_std_vps: core::ptr::null(),
                p_std_sps: core::ptr::null(),
                p_std_pps: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoDecodeH265InlineSessionParametersInfoKHR<'a> {
        pub fn std_vps(mut self, std_vps: &'a StdVideoH265VideoParameterSet<'a>) -> Self {
            self.p_std_vps = std_vps;
            self
        }
        pub fn std_sps(mut self, std_sps: &'a StdVideoH265SequenceParameterSet<'a>) -> Self {
            self.p_std_sps = std_sps;
            self
        }
        pub fn std_pps(mut self, std_pps: &'a StdVideoH265PictureParameterSet<'a>) -> Self {
            self.p_std_pps = std_pps;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoDecodeAV1InlineSessionParametersInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_sequence_header: *const StdVideoAV1SequenceHeader<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeAV1InlineSessionParametersInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_DECODE_AV1_INLINE_SESSION_PARAMETERS_INFO_KHR;
    }
    unsafe impl<'a> Extends<VideoDecodeInfoKHR<'a>>
        for VideoDecodeAV1InlineSessionParametersInfoKHR<'a>
    {
    }
    impl Default for VideoDecodeAV1InlineSessionParametersInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_std_sequence_header: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoDecodeAV1InlineSessionParametersInfoKHR<'a> {
        pub fn std_sequence_header(
            mut self,
            std_sequence_header: &'a StdVideoAV1SequenceHeader<'a>,
        ) -> Self {
            self.p_std_sequence_header = std_sequence_header;
            self
        }
    }
}
