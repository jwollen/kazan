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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentRegionsKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PresentRegionsKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain_count: u32,
        pub p_regions: *const PresentRegionKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PresentRegionsKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PRESENT_REGIONS_KHR;
    }
    unsafe impl<'a> Extends<PresentInfoKHR<'a>> for PresentRegionsKHR<'a> {}
    impl Default for PresentRegionsKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                swapchain_count: Default::default(),
                p_regions: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PresentRegionsKHR<'a> {
        pub fn regions(mut self, regions: &'a [PresentRegionKHR<'a>]) -> Self {
            self.swapchain_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentRegionKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PresentRegionKHR<'a> {
        pub rectangle_count: u32,
        pub p_rectangles: *const RectLayerKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PresentRegionKHR<'_> {
        fn default() -> Self {
            Self {
                rectangle_count: Default::default(),
                p_rectangles: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PresentRegionKHR<'a> {
        pub fn rectangles(mut self, rectangles: &'a [RectLayerKHR]) -> Self {
            self.rectangle_count = rectangles.len().try_into().unwrap();
            self.p_rectangles = rectangles.as_ptr();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRectLayerKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct RectLayerKHR {
        pub offset: Offset2D,
        pub extent: Extent2D,
        pub layer: u32,
    }
    impl RectLayerKHR {
        pub fn offset(mut self, offset: Offset2D) -> Self {
            self.offset = offset;
            self
        }
        pub fn extent(mut self, extent: Extent2D) -> Self {
            self.extent = extent;
            self
        }
        pub fn layer(mut self, layer: u32) -> Self {
            self.layer = layer;
            self
        }
    }
}
