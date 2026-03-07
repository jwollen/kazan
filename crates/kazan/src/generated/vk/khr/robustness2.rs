#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_robustness2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRobustness2FeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceRobustness2FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub robust_buffer_access2: Bool32,
        pub robust_image_access2: Bool32,
        pub null_descriptor: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceRobustness2FeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceRobustness2FeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("robust_buffer_access2", &self.robust_buffer_access2)
                .field("robust_image_access2", &self.robust_image_access2)
                .field("null_descriptor", &self.null_descriptor)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRobustness2FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceRobustness2FeaturesKHR<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceRobustness2FeaturesKHR<'a> {}

    impl Default for PhysicalDeviceRobustness2FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                robust_buffer_access2: Default::default(),
                robust_image_access2: Default::default(),
                null_descriptor: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRobustness2FeaturesKHR<'a> {
        pub fn robust_buffer_access2(mut self, robust_buffer_access2: bool) -> Self {
            self.robust_buffer_access2 = robust_buffer_access2.into();
            self
        }

        pub fn robust_image_access2(mut self, robust_image_access2: bool) -> Self {
            self.robust_image_access2 = robust_image_access2.into();
            self
        }

        pub fn null_descriptor(mut self, null_descriptor: bool) -> Self {
            self.null_descriptor = null_descriptor.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRobustness2PropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceRobustness2PropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub robust_storage_buffer_access_size_alignment: DeviceSize,
        pub robust_uniform_buffer_access_size_alignment: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceRobustness2PropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceRobustness2PropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "robust_storage_buffer_access_size_alignment",
                    &self.robust_storage_buffer_access_size_alignment,
                )
                .field(
                    "robust_uniform_buffer_access_size_alignment",
                    &self.robust_uniform_buffer_access_size_alignment,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRobustness2PropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceRobustness2PropertiesKHR<'a>
    {
    }

    impl Default for PhysicalDeviceRobustness2PropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                robust_storage_buffer_access_size_alignment: Default::default(),
                robust_uniform_buffer_access_size_alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRobustness2PropertiesKHR<'a> {
        pub fn robust_storage_buffer_access_size_alignment(
            mut self,
            robust_storage_buffer_access_size_alignment: DeviceSize,
        ) -> Self {
            self.robust_storage_buffer_access_size_alignment =
                robust_storage_buffer_access_size_alignment;
            self
        }

        pub fn robust_uniform_buffer_access_size_alignment(
            mut self,
            robust_uniform_buffer_access_size_alignment: DeviceSize,
        ) -> Self {
            self.robust_uniform_buffer_access_size_alignment =
                robust_uniform_buffer_access_size_alignment;
            self
        }
    }
}
