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
    pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub per_view_position_all_components: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'a>
    {
    }
    impl Default for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                per_view_position_all_components: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'a> {
        pub fn per_view_position_all_components(
            mut self,
            per_view_position_all_components: Bool32,
        ) -> Self {
            self.per_view_position_all_components = per_view_position_all_components;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MultiviewPerViewAttributesInfoNVX<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub per_view_attributes: Bool32,
        pub per_view_attributes_position_x_only: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MultiviewPerViewAttributesInfoNVX<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX;
    }
    unsafe impl<'a> Extends<CommandBufferInheritanceInfo<'a>>
        for MultiviewPerViewAttributesInfoNVX<'a>
    {
    }
    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>> for MultiviewPerViewAttributesInfoNVX<'a> {}
    unsafe impl<'a> Extends<RenderingInfo<'a>> for MultiviewPerViewAttributesInfoNVX<'a> {}
    impl Default for MultiviewPerViewAttributesInfoNVX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                per_view_attributes: Default::default(),
                per_view_attributes_position_x_only: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MultiviewPerViewAttributesInfoNVX<'a> {
        pub fn per_view_attributes(mut self, per_view_attributes: Bool32) -> Self {
            self.per_view_attributes = per_view_attributes;
            self
        }
        pub fn per_view_attributes_position_x_only(
            mut self,
            per_view_attributes_position_x_only: Bool32,
        ) -> Self {
            self.per_view_attributes_position_x_only = per_view_attributes_position_x_only;
            self
        }
    }
}
