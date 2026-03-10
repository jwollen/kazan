//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_mutable_descriptor_type.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_mutable_descriptor_type";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

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

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'_> {}

    impl Default for PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                mutable_descriptor_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'a> {
        #[inline]
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
                p_descriptor_types: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MutableDescriptorTypeListEXT<'a> {
        #[inline]
        pub fn descriptor_types(mut self, descriptor_types: &'a [DescriptorType]) -> Self {
            self.descriptor_type_count = descriptor_types.len().try_into().unwrap();
            self.p_descriptor_types = descriptor_types.as_ptr() as _;
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

    unsafe impl Extends<DescriptorSetLayoutCreateInfo<'_>> for MutableDescriptorTypeCreateInfoEXT<'_> {}
    unsafe impl Extends<DescriptorPoolCreateInfo<'_>> for MutableDescriptorTypeCreateInfoEXT<'_> {}

    impl Default for MutableDescriptorTypeCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                mutable_descriptor_type_list_count: Default::default(),
                p_mutable_descriptor_type_lists: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MutableDescriptorTypeCreateInfoEXT<'a> {
        #[inline]
        pub fn mutable_descriptor_type_lists(
            mut self,
            mutable_descriptor_type_lists: &'a [MutableDescriptorTypeListEXT<'_>],
        ) -> Self {
            self.mutable_descriptor_type_list_count =
                mutable_descriptor_type_lists.len().try_into().unwrap();
            self.p_mutable_descriptor_type_lists = mutable_descriptor_type_lists.as_ptr() as _;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT =
        PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'static>;
    pub type VkMutableDescriptorTypeListEXT = MutableDescriptorTypeListEXT<'static>;
    pub type VkMutableDescriptorTypeCreateInfoEXT = MutableDescriptorTypeCreateInfoEXT<'static>;
    impl PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MutableDescriptorTypeListEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMutableDescriptorTypeListEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MutableDescriptorTypeCreateInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMutableDescriptorTypeCreateInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
