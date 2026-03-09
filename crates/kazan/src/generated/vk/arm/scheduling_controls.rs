//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_ARM_scheduling_controls.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_ARM_scheduling_controls";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceQueueShaderCoreControlCreateInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceQueueShaderCoreControlCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_core_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceQueueShaderCoreControlCreateInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceQueueShaderCoreControlCreateInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_core_count", &self.shader_core_count)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceQueueShaderCoreControlCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM;
    }

    unsafe impl<'a> Extends<DeviceQueueCreateInfo<'a>>
        for DeviceQueueShaderCoreControlCreateInfoARM<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for DeviceQueueShaderCoreControlCreateInfoARM<'a> {}

    impl Default for DeviceQueueShaderCoreControlCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_core_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceQueueShaderCoreControlCreateInfoARM<'a> {
        #[inline]
        pub fn shader_core_count(mut self, shader_core_count: u32) -> Self {
            self.shader_core_count = shader_core_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSchedulingControlsFeaturesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceSchedulingControlsFeaturesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub scheduling_controls: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceSchedulingControlsFeaturesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceSchedulingControlsFeaturesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("scheduling_controls", &self.scheduling_controls)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSchedulingControlsFeaturesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceSchedulingControlsFeaturesARM<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceSchedulingControlsFeaturesARM<'a> {}

    impl Default for PhysicalDeviceSchedulingControlsFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                scheduling_controls: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceSchedulingControlsFeaturesARM<'a> {
        #[inline]
        pub fn scheduling_controls(mut self, scheduling_controls: bool) -> Self {
            self.scheduling_controls = scheduling_controls.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSchedulingControlsPropertiesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceSchedulingControlsPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub scheduling_controls_flags: PhysicalDeviceSchedulingControlsFlagsARM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceSchedulingControlsPropertiesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceSchedulingControlsPropertiesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("scheduling_controls_flags", &self.scheduling_controls_flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSchedulingControlsPropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceSchedulingControlsPropertiesARM<'a>
    {
    }

    impl Default for PhysicalDeviceSchedulingControlsPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                scheduling_controls_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceSchedulingControlsPropertiesARM<'a> {
        #[inline]
        pub fn scheduling_controls_flags(
            mut self,
            scheduling_controls_flags: PhysicalDeviceSchedulingControlsFlagsARM,
        ) -> Self {
            self.scheduling_controls_flags = scheduling_controls_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSchedulingControlsFlagsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PhysicalDeviceSchedulingControlsFlagsARM(Flags64);
    vk_bitflags_wrapped!(PhysicalDeviceSchedulingControlsFlagsARM, Flags64);

    impl PhysicalDeviceSchedulingControlsFlagsARM {
        pub const SHADER_CORE_COUNT_ARM: Self =
            Self(PhysicalDeviceSchedulingControlsFlagBitsARM::SHADER_CORE_COUNT_ARM.0);
    }

    impl fmt::Debug for PhysicalDeviceSchedulingControlsFlagsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags64, &str)] = &[(
                PhysicalDeviceSchedulingControlsFlagsARM::SHADER_CORE_COUNT_ARM.0,
                "SHADER_CORE_COUNT_ARM",
            )];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSchedulingControlsFlagBitsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PhysicalDeviceSchedulingControlsFlagBitsARM(u64);

    impl PhysicalDeviceSchedulingControlsFlagBitsARM {
        pub const SHADER_CORE_COUNT_ARM: Self = Self(1 << 0);
    }

    impl fmt::Debug for PhysicalDeviceSchedulingControlsFlagBitsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SHADER_CORE_COUNT_ARM => Some("SHADER_CORE_COUNT_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkDeviceQueueShaderCoreControlCreateInfoARM =
        DeviceQueueShaderCoreControlCreateInfoARM<'static>;
    pub type VkPhysicalDeviceSchedulingControlsFeaturesARM =
        PhysicalDeviceSchedulingControlsFeaturesARM<'static>;
    pub type VkPhysicalDeviceSchedulingControlsPropertiesARM =
        PhysicalDeviceSchedulingControlsPropertiesARM<'static>;
    pub type VkPhysicalDeviceSchedulingControlsFlagsARM = PhysicalDeviceSchedulingControlsFlagsARM;
    pub type VkPhysicalDeviceSchedulingControlsFlagBitsARM =
        PhysicalDeviceSchedulingControlsFlagBitsARM;
    impl DeviceQueueShaderCoreControlCreateInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceQueueShaderCoreControlCreateInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceSchedulingControlsFeaturesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceSchedulingControlsFeaturesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceSchedulingControlsPropertiesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceSchedulingControlsPropertiesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
}
