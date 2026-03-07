//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_QCOM_multiview_per_view_render_areas.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_QCOM_multiview_per_view_render_areas";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub multiview_per_view_render_areas: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "multiview_per_view_render_areas",
                    &self.multiview_per_view_render_areas,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'a>
    {
    }

    impl Default for PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                multiview_per_view_render_areas: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'a> {
        #[inline]
        pub fn multiview_per_view_render_areas(
            mut self,
            multiview_per_view_render_areas: bool,
        ) -> Self {
            self.multiview_per_view_render_areas = multiview_per_view_render_areas.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub per_view_render_area_count: u32,
        pub p_per_view_render_areas: *const Rect2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "per_view_render_area_count",
                    &self.per_view_render_area_count,
                )
                .field("p_per_view_render_areas", &self.p_per_view_render_areas)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM;
    }

    unsafe impl<'a> Extends<RenderPassBeginInfo<'a>>
        for MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'a>
    {
    }
    unsafe impl<'a> Extends<RenderingInfo<'a>>
        for MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'a>
    {
    }

    impl Default for MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                per_view_render_area_count: Default::default(),
                p_per_view_render_areas: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'a> {
        #[inline]
        pub fn per_view_render_areas(mut self, per_view_render_areas: &'a [Rect2D]) -> Self {
            self.per_view_render_area_count = per_view_render_areas.len().try_into().unwrap();
            self.p_per_view_render_areas = per_view_render_areas.as_ptr();
            self
        }
    }
}
