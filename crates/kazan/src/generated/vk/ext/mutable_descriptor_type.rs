#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_mutable_descriptor_type";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub mutable_descriptor_type: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMutableDescriptorTypeFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("mutable_descriptor_type", &self.mutable_descriptor_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                mutable_descriptor_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'a> {
        pub fn mutable_descriptor_type(mut self, mutable_descriptor_type: bool) -> Self {
            self.mutable_descriptor_type = mutable_descriptor_type.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMutableDescriptorTypeListEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MutableDescriptorTypeListEXT<'a> {
        pub descriptor_type_count: u32,
        pub p_descriptor_types: *const DescriptorType,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MutableDescriptorTypeListEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MutableDescriptorTypeListEXT")
                .field("descriptor_type_count", &self.descriptor_type_count)
                .field("p_descriptor_types", &self.p_descriptor_types)
                .finish()
        }
    }

    impl Default for MutableDescriptorTypeListEXT<'_> {
        fn default() -> Self {
            Self {
                descriptor_type_count: Default::default(),
                p_descriptor_types: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MutableDescriptorTypeListEXT<'a> {
        pub fn descriptor_types(mut self, descriptor_types: &'a [DescriptorType]) -> Self {
            self.descriptor_type_count = descriptor_types.len().try_into().unwrap();
            self.p_descriptor_types = descriptor_types.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMutableDescriptorTypeCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MutableDescriptorTypeCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub mutable_descriptor_type_list_count: u32,
        pub p_mutable_descriptor_type_lists: *const MutableDescriptorTypeListEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MutableDescriptorTypeCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MutableDescriptorTypeCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "mutable_descriptor_type_list_count",
                    &self.mutable_descriptor_type_list_count,
                )
                .field(
                    "p_mutable_descriptor_type_lists",
                    &self.p_mutable_descriptor_type_lists,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MutableDescriptorTypeCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<DescriptorSetLayoutCreateInfo<'a>>
        for MutableDescriptorTypeCreateInfoEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DescriptorPoolCreateInfo<'a>> for MutableDescriptorTypeCreateInfoEXT<'a> {}

    impl Default for MutableDescriptorTypeCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                mutable_descriptor_type_list_count: Default::default(),
                p_mutable_descriptor_type_lists: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MutableDescriptorTypeCreateInfoEXT<'a> {
        pub fn mutable_descriptor_type_lists(
            mut self,
            mutable_descriptor_type_lists: &'a [MutableDescriptorTypeListEXT<'a>],
        ) -> Self {
            self.mutable_descriptor_type_list_count =
                mutable_descriptor_type_lists.len().try_into().unwrap();
            self.p_mutable_descriptor_type_lists = mutable_descriptor_type_lists.as_ptr();
            self
        }
    }
}
