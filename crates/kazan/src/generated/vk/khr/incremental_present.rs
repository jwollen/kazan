//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_incremental_present.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_incremental_present";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentRegionsKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PresentRegionsKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain_count: u32,
        pub p_regions: *const PresentRegionKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PresentRegionsKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PresentRegionsKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("swapchain_count", &self.swapchain_count)
                .field("p_regions", &self.p_regions)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PresentRegionsKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PRESENT_REGIONS_KHR;
    }

    unsafe impl Extends<PresentInfoKHR<'_>> for PresentRegionsKHR<'_> {}

    impl Default for PresentRegionsKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                swapchain_count: Default::default(),
                p_regions: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PresentRegionsKHR<'a> {
        #[inline]
        pub fn regions(mut self, regions: &'a [PresentRegionKHR<'_>]) -> Self {
            self.swapchain_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentRegionKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PresentRegionKHR<'a> {
        pub rectangle_count: u32,
        pub p_rectangles: *const RectLayerKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PresentRegionKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PresentRegionKHR")
                .field("rectangle_count", &self.rectangle_count)
                .field("p_rectangles", &self.p_rectangles)
                .finish()
        }
    }

    impl Default for PresentRegionKHR<'_> {
        fn default() -> Self {
            Self {
                rectangle_count: Default::default(),
                p_rectangles: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PresentRegionKHR<'a> {
        #[inline]
        pub fn rectangles(mut self, rectangles: &'a [RectLayerKHR]) -> Self {
            self.rectangle_count = rectangles.len().try_into().unwrap();
            self.p_rectangles = rectangles.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRectLayerKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct RectLayerKHR {
        pub offset: Offset2D,
        pub extent: Extent2D,
        pub layer: u32,
    }

    impl RectLayerKHR {
        #[inline]
        pub fn offset(mut self, offset: Offset2D) -> Self {
            self.offset = offset;
            self
        }

        #[inline]
        pub fn extent(mut self, extent: Extent2D) -> Self {
            self.extent = extent;
            self
        }

        #[inline]
        pub fn layer(mut self, layer: u32) -> Self {
            self.layer = layer;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPresentRegionsKHR = PresentRegionsKHR<'static>;
    pub type VkPresentRegionKHR = PresentRegionKHR<'static>;
    pub type VkRectLayerKHR = RectLayerKHR;
    impl PresentRegionsKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPresentRegionsKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PresentRegionKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPresentRegionKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
