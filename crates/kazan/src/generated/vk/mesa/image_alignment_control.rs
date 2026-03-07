//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_MESA_image_alignment_control.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_MESA_image_alignment_control";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImageAlignmentControlFeaturesMESA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceImageAlignmentControlFeaturesMESA<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image_alignment_control: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceImageAlignmentControlFeaturesMESA<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceImageAlignmentControlFeaturesMESA")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image_alignment_control", &self.image_alignment_control)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageAlignmentControlFeaturesMESA<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceImageAlignmentControlFeaturesMESA<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceImageAlignmentControlFeaturesMESA<'a>
    {
    }

    impl Default for PhysicalDeviceImageAlignmentControlFeaturesMESA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                image_alignment_control: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceImageAlignmentControlFeaturesMESA<'a> {
        #[inline]
        pub fn image_alignment_control(mut self, image_alignment_control: bool) -> Self {
            self.image_alignment_control = image_alignment_control.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImageAlignmentControlPropertiesMESA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceImageAlignmentControlPropertiesMESA<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub supported_image_alignment_mask: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceImageAlignmentControlPropertiesMESA<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceImageAlignmentControlPropertiesMESA")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "supported_image_alignment_mask",
                    &self.supported_image_alignment_mask,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageAlignmentControlPropertiesMESA<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceImageAlignmentControlPropertiesMESA<'a>
    {
    }

    impl Default for PhysicalDeviceImageAlignmentControlPropertiesMESA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                supported_image_alignment_mask: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceImageAlignmentControlPropertiesMESA<'a> {
        #[inline]
        pub fn supported_image_alignment_mask(
            mut self,
            supported_image_alignment_mask: u32,
        ) -> Self {
            self.supported_image_alignment_mask = supported_image_alignment_mask;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageAlignmentControlCreateInfoMESA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageAlignmentControlCreateInfoMESA<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub maximum_requested_alignment: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageAlignmentControlCreateInfoMESA<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageAlignmentControlCreateInfoMESA")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "maximum_requested_alignment",
                    &self.maximum_requested_alignment,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageAlignmentControlCreateInfoMESA<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA;
    }

    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for ImageAlignmentControlCreateInfoMESA<'a> {}

    impl Default for ImageAlignmentControlCreateInfoMESA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                maximum_requested_alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageAlignmentControlCreateInfoMESA<'a> {
        #[inline]
        pub fn maximum_requested_alignment(mut self, maximum_requested_alignment: u32) -> Self {
            self.maximum_requested_alignment = maximum_requested_alignment;
            self
        }
    }
}
