#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_workgroup_memory_explicit_layout";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub workgroup_memory_explicit_layout: Bool32,
        pub workgroup_memory_explicit_layout_scalar_block_layout: Bool32,
        pub workgroup_memory_explicit_layout8_bit_access: Bool32,
        pub workgroup_memory_explicit_layout16_bit_access: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "workgroup_memory_explicit_layout",
                    &self.workgroup_memory_explicit_layout,
                )
                .field(
                    "workgroup_memory_explicit_layout_scalar_block_layout",
                    &self.workgroup_memory_explicit_layout_scalar_block_layout,
                )
                .field(
                    "workgroup_memory_explicit_layout8_bit_access",
                    &self.workgroup_memory_explicit_layout8_bit_access,
                )
                .field(
                    "workgroup_memory_explicit_layout16_bit_access",
                    &self.workgroup_memory_explicit_layout16_bit_access,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'a>
    {
    }

    impl Default for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                workgroup_memory_explicit_layout: Default::default(),
                workgroup_memory_explicit_layout_scalar_block_layout: Default::default(),
                workgroup_memory_explicit_layout8_bit_access: Default::default(),
                workgroup_memory_explicit_layout16_bit_access: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'a> {
        pub fn workgroup_memory_explicit_layout(
            mut self,
            workgroup_memory_explicit_layout: bool,
        ) -> Self {
            self.workgroup_memory_explicit_layout = workgroup_memory_explicit_layout.into();
            self
        }

        pub fn workgroup_memory_explicit_layout_scalar_block_layout(
            mut self,
            workgroup_memory_explicit_layout_scalar_block_layout: bool,
        ) -> Self {
            self.workgroup_memory_explicit_layout_scalar_block_layout =
                workgroup_memory_explicit_layout_scalar_block_layout.into();
            self
        }

        pub fn workgroup_memory_explicit_layout8_bit_access(
            mut self,
            workgroup_memory_explicit_layout8_bit_access: bool,
        ) -> Self {
            self.workgroup_memory_explicit_layout8_bit_access =
                workgroup_memory_explicit_layout8_bit_access.into();
            self
        }

        pub fn workgroup_memory_explicit_layout16_bit_access(
            mut self,
            workgroup_memory_explicit_layout16_bit_access: bool,
        ) -> Self {
            self.workgroup_memory_explicit_layout16_bit_access =
                workgroup_memory_explicit_layout16_bit_access.into();
            self
        }
    }
}
