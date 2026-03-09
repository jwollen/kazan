//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_video_maintenance1.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_video_maintenance1";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVideoMaintenance1FeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceVideoMaintenance1FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub video_maintenance1: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceVideoMaintenance1FeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVideoMaintenance1FeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("video_maintenance1", &self.video_maintenance1)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVideoMaintenance1FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceVideoMaintenance1FeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceVideoMaintenance1FeaturesKHR<'a> {}

    impl Default for PhysicalDeviceVideoMaintenance1FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                video_maintenance1: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceVideoMaintenance1FeaturesKHR<'a> {
        #[inline]
        pub fn video_maintenance1(mut self, video_maintenance1: bool) -> Self {
            self.video_maintenance1 = video_maintenance1.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoInlineQueryInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoInlineQueryInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub query_pool: QueryPool,
        pub first_query: u32,
        pub query_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoInlineQueryInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoInlineQueryInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("query_pool", &self.query_pool)
                .field("first_query", &self.first_query)
                .field("query_count", &self.query_count)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoInlineQueryInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_INLINE_QUERY_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoDecodeInfoKHR<'a>> for VideoInlineQueryInfoKHR<'a> {}
    unsafe impl<'a> Extends<VideoEncodeInfoKHR<'a>> for VideoInlineQueryInfoKHR<'a> {}

    impl Default for VideoInlineQueryInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                query_pool: Default::default(),
                first_query: Default::default(),
                query_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoInlineQueryInfoKHR<'a> {
        #[inline]
        pub fn query_pool(mut self, query_pool: QueryPool) -> Self {
            self.query_pool = query_pool;
            self
        }

        #[inline]
        pub fn first_query(mut self, first_query: u32) -> Self {
            self.first_query = first_query;
            self
        }

        #[inline]
        pub fn query_count(mut self, query_count: u32) -> Self {
            self.query_count = query_count;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceVideoMaintenance1FeaturesKHR =
        PhysicalDeviceVideoMaintenance1FeaturesKHR<'static>;
    pub type VkVideoInlineQueryInfoKHR = VideoInlineQueryInfoKHR<'static>;
    impl PhysicalDeviceVideoMaintenance1FeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceVideoMaintenance1FeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoInlineQueryInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoInlineQueryInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
