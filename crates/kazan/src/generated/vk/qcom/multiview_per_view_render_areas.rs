#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub multiview_per_view_render_areas: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type:
                    StructureType::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM,
                p_next: core::ptr::null_mut(),
                multiview_per_view_render_areas: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM<'a> {
        pub fn multiview_per_view_render_areas(
            mut self,
            multiview_per_view_render_areas: Bool32,
        ) -> Self {
            self.multiview_per_view_render_areas = multiview_per_view_render_areas;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub per_view_render_area_count: u32,
        pub p_per_view_render_areas: *const Rect2D,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM,
                p_next: core::ptr::null(),
                per_view_render_area_count: Default::default(),
                p_per_view_render_areas: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM<'a> {
        pub fn per_view_render_areas(mut self, per_view_render_areas: &'a [Rect2D]) -> Self {
            self.per_view_render_area_count = per_view_render_areas.len().try_into().unwrap();
            self.p_per_view_render_areas = per_view_render_areas.as_ptr();
            self
        }
    }
}
