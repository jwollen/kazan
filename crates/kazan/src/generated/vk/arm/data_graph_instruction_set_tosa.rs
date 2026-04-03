//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_ARM_data_graph_instruction_set_tosa.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_ARM_data_graph_instruction_set_tosa";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    pub const MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM: u32 = 128;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphTOSANameQualityARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DataGraphTOSANameQualityARM {
        pub name: ArrayCStr<{ MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM as usize }>,
        pub quality_flags: DataGraphTOSAQualityFlagsARM,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DataGraphTOSANameQualityARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DataGraphTOSANameQualityARM")
                .field("name", &self.name)
                .field("quality_flags", &self.quality_flags)
                .finish()
        }
    }

    impl Default for DataGraphTOSANameQualityARM {
        fn default() -> Self {
            Self {
                name: Default::default(),
                quality_flags: Default::default(),
            }
        }
    }

    impl DataGraphTOSANameQualityARM {
        #[inline]
        pub fn name(
            mut self,
            name: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            self.name.write_c_str(name)?;
            Ok(self)
        }

        #[inline]
        pub fn quality_flags(mut self, quality_flags: DataGraphTOSAQualityFlagsARM) -> Self {
            self.quality_flags = quality_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueFamilyDataGraphTOSAPropertiesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct QueueFamilyDataGraphTOSAPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub profile_count: u32,
        pub p_profiles: *const DataGraphTOSANameQualityARM,
        pub extension_count: u32,
        pub p_extensions: *const DataGraphTOSANameQualityARM,
        pub level: DataGraphTOSALevelARM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for QueueFamilyDataGraphTOSAPropertiesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("QueueFamilyDataGraphTOSAPropertiesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("profile_count", &self.profile_count)
                .field("p_profiles", &self.p_profiles)
                .field("extension_count", &self.extension_count)
                .field("p_extensions", &self.p_extensions)
                .field("level", &self.level)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for QueueFamilyDataGraphTOSAPropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::QUEUE_FAMILY_DATA_GRAPH_TOSA_PROPERTIES_ARM;
    }

    impl Default for QueueFamilyDataGraphTOSAPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                profile_count: Default::default(),
                p_profiles: ptr::null(),
                extension_count: Default::default(),
                p_extensions: ptr::null(),
                level: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> QueueFamilyDataGraphTOSAPropertiesARM<'a> {
        #[inline]
        pub fn profiles(mut self, profiles: &'a [DataGraphTOSANameQualityARM]) -> Self {
            self.profile_count = profiles.len().try_into().unwrap();
            self.p_profiles = profiles.as_ptr() as _;
            self
        }

        #[inline]
        pub fn extensions(mut self, extensions: &'a [DataGraphTOSANameQualityARM]) -> Self {
            self.extension_count = extensions.len().try_into().unwrap();
            self.p_extensions = extensions.as_ptr() as _;
            self
        }

        #[inline]
        pub fn level(mut self, level: DataGraphTOSALevelARM) -> Self {
            self.level = level;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphTOSALevelARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataGraphTOSALevelARM(i32);

    impl DataGraphTOSALevelARM {
        pub const NONE_ARM: Self = Self(0);
        pub const _8K_ARM: Self = Self(1);
    }

    impl fmt::Debug for DataGraphTOSALevelARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::NONE_ARM => Some("NONE_ARM"),
                Self::_8K_ARM => Some("_8K_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphTOSAQualityFlagsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DataGraphTOSAQualityFlagsARM(Flags);
    vk_bitflags_wrapped!(
        DataGraphTOSAQualityFlagsARM,
        Flags,
        DataGraphTOSAQualityFlagBitsARM
    );

    impl fmt::Debug for DataGraphTOSAQualityFlagsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    DataGraphTOSAQualityFlagBitsARM::ACCELERATED_ARM.0,
                    "ACCELERATED_ARM",
                ),
                (
                    DataGraphTOSAQualityFlagBitsARM::CONFORMANT_ARM.0,
                    "CONFORMANT_ARM",
                ),
                (
                    DataGraphTOSAQualityFlagBitsARM::EXPERIMENTAL_ARM.0,
                    "EXPERIMENTAL_ARM",
                ),
                (
                    DataGraphTOSAQualityFlagBitsARM::DEPRECATED_ARM.0,
                    "DEPRECATED_ARM",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphTOSAQualityFlagBitsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DataGraphTOSAQualityFlagBitsARM(u32);

    impl DataGraphTOSAQualityFlagBitsARM {
        pub const ACCELERATED_ARM: Self = Self(1 << 0);
        pub const CONFORMANT_ARM: Self = Self(1 << 1);
        pub const EXPERIMENTAL_ARM: Self = Self(1 << 2);
        pub const DEPRECATED_ARM: Self = Self(1 << 3);
    }

    impl fmt::Debug for DataGraphTOSAQualityFlagBitsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ACCELERATED_ARM => Some("ACCELERATED_ARM"),
                Self::CONFORMANT_ARM => Some("CONFORMANT_ARM"),
                Self::EXPERIMENTAL_ARM => Some("EXPERIMENTAL_ARM"),
                Self::DEPRECATED_ARM => Some("DEPRECATED_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM.html>
    pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
            p_queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM<'_>,
            p_properties: *mut BaseOutStructure<'_>,
        ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkDataGraphTOSANameQualityARM = DataGraphTOSANameQualityARM;
    pub type VkQueueFamilyDataGraphTOSAPropertiesARM =
        QueueFamilyDataGraphTOSAPropertiesARM<'static>;
    pub type VkDataGraphTOSALevelARM = DataGraphTOSALevelARM;
    pub type VkDataGraphTOSAQualityFlagsARM = DataGraphTOSAQualityFlagsARM;
    pub type VkDataGraphTOSAQualityFlagBitsARM = DataGraphTOSAQualityFlagBitsARM;
    impl QueueFamilyDataGraphTOSAPropertiesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkQueueFamilyDataGraphTOSAPropertiesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    get_physical_device_queue_family_data_graph_engine_operation_properties:
        PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_queue_family_data_graph_engine_operation_properties: transmute(
                    load(c"vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM.html>
    #[inline]
    pub unsafe fn get_physical_device_queue_family_data_graph_engine_operation_properties(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM<'_>,
        properties: &mut BaseOutStructure<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self
                .get_physical_device_queue_family_data_graph_engine_operation_properties)(
                physical_device,
                queue_family_index,
                queue_family_data_graph_properties,
                properties,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
