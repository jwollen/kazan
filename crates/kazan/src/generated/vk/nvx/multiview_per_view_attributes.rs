//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NVX_multiview_per_view_attributes.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NVX_multiview_per_view_attributes";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub per_view_position_all_components: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "per_view_position_all_components",
                    &self.per_view_position_all_components,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'_>
    {
    }

    impl Default for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                per_view_position_all_components: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'a> {
        #[inline]
        pub fn per_view_position_all_components(
            mut self,
            per_view_position_all_components: bool,
        ) -> Self {
            self.per_view_position_all_components = per_view_position_all_components.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMultiviewPerViewAttributesInfoNVX.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MultiviewPerViewAttributesInfoNVX<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub per_view_attributes: Bool32,
        pub per_view_attributes_position_x_only: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MultiviewPerViewAttributesInfoNVX<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MultiviewPerViewAttributesInfoNVX")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("per_view_attributes", &self.per_view_attributes)
                .field(
                    "per_view_attributes_position_x_only",
                    &self.per_view_attributes_position_x_only,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MultiviewPerViewAttributesInfoNVX<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX;
    }

    unsafe impl Extends<CommandBufferInheritanceInfo<'_>> for MultiviewPerViewAttributesInfoNVX<'_> {}
    unsafe impl Extends<GraphicsPipelineCreateInfo<'_>> for MultiviewPerViewAttributesInfoNVX<'_> {}
    unsafe impl Extends<RenderingInfo<'_>> for MultiviewPerViewAttributesInfoNVX<'_> {}

    impl Default for MultiviewPerViewAttributesInfoNVX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                per_view_attributes: Default::default(),
                per_view_attributes_position_x_only: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MultiviewPerViewAttributesInfoNVX<'a> {
        #[inline]
        pub fn per_view_attributes(mut self, per_view_attributes: bool) -> Self {
            self.per_view_attributes = per_view_attributes.into();
            self
        }

        #[inline]
        pub fn per_view_attributes_position_x_only(
            mut self,
            per_view_attributes_position_x_only: bool,
        ) -> Self {
            self.per_view_attributes_position_x_only = per_view_attributes_position_x_only.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX =
        PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'static>;
    pub type VkMultiviewPerViewAttributesInfoNVX = MultiviewPerViewAttributesInfoNVX<'static>;
    impl PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MultiviewPerViewAttributesInfoNVX<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMultiviewPerViewAttributesInfoNVX {
            unsafe { core::mem::transmute(self) }
        }
    }
}
