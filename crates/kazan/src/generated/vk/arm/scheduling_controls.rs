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

    unsafe impl Extends<DeviceQueueCreateInfo<'_>> for DeviceQueueShaderCoreControlCreateInfoARM<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for DeviceQueueShaderCoreControlCreateInfoARM<'_> {}

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

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceSchedulingControlsFeaturesARM<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceSchedulingControlsFeaturesARM<'_> {}

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

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceSchedulingControlsPropertiesARM<'_>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub scheduling_controls_max_warps_count: u32,
        pub scheduling_controls_max_queued_batches_count: u32,
        pub scheduling_controls_max_work_group_batch_size: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "scheduling_controls_max_warps_count",
                    &self.scheduling_controls_max_warps_count,
                )
                .field(
                    "scheduling_controls_max_queued_batches_count",
                    &self.scheduling_controls_max_queued_batches_count,
                )
                .field(
                    "scheduling_controls_max_work_group_batch_size",
                    &self.scheduling_controls_max_work_group_batch_size,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_PROPERTIES_ARM;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM<'_>
    {
    }

    impl Default for PhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                scheduling_controls_max_warps_count: Default::default(),
                scheduling_controls_max_queued_batches_count: Default::default(),
                scheduling_controls_max_work_group_batch_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM<'a> {
        #[inline]
        pub fn scheduling_controls_max_warps_count(
            mut self,
            scheduling_controls_max_warps_count: u32,
        ) -> Self {
            self.scheduling_controls_max_warps_count = scheduling_controls_max_warps_count;
            self
        }

        #[inline]
        pub fn scheduling_controls_max_queued_batches_count(
            mut self,
            scheduling_controls_max_queued_batches_count: u32,
        ) -> Self {
            self.scheduling_controls_max_queued_batches_count =
                scheduling_controls_max_queued_batches_count;
            self
        }

        #[inline]
        pub fn scheduling_controls_max_work_group_batch_size(
            mut self,
            scheduling_controls_max_work_group_batch_size: u32,
        ) -> Self {
            self.scheduling_controls_max_work_group_batch_size =
                scheduling_controls_max_work_group_batch_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDispatchParametersARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DispatchParametersARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub work_group_batch_size: u32,
        pub max_queued_work_group_batches: u32,
        pub max_warps_per_shader_core: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DispatchParametersARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DispatchParametersARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("work_group_batch_size", &self.work_group_batch_size)
                .field(
                    "max_queued_work_group_batches",
                    &self.max_queued_work_group_batches,
                )
                .field("max_warps_per_shader_core", &self.max_warps_per_shader_core)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DispatchParametersARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DISPATCH_PARAMETERS_ARM;
    }

    impl Default for DispatchParametersARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                work_group_batch_size: Default::default(),
                max_queued_work_group_batches: Default::default(),
                max_warps_per_shader_core: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DispatchParametersARM<'a> {
        #[inline]
        pub fn work_group_batch_size(mut self, work_group_batch_size: u32) -> Self {
            self.work_group_batch_size = work_group_batch_size;
            self
        }

        #[inline]
        pub fn max_queued_work_group_batches(mut self, max_queued_work_group_batches: u32) -> Self {
            self.max_queued_work_group_batches = max_queued_work_group_batches;
            self
        }

        #[inline]
        pub fn max_warps_per_shader_core(mut self, max_warps_per_shader_core: u32) -> Self {
            self.max_warps_per_shader_core = max_warps_per_shader_core;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSchedulingControlsFlagsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PhysicalDeviceSchedulingControlsFlagsARM(Flags64);
    vk_bitflags_wrapped!(
        PhysicalDeviceSchedulingControlsFlagsARM,
        Flags64,
        PhysicalDeviceSchedulingControlsFlagBitsARM
    );

    impl fmt::Debug for PhysicalDeviceSchedulingControlsFlagsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags64, &str)] = &[
                (
                    PhysicalDeviceSchedulingControlsFlagBitsARM::SHADER_CORE_COUNT_ARM.0,
                    "SHADER_CORE_COUNT_ARM",
                ),
                (
                    PhysicalDeviceSchedulingControlsFlagBitsARM::DISPATCH_PARAMETERS_ARM.0,
                    "DISPATCH_PARAMETERS_ARM",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSchedulingControlsFlagBitsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PhysicalDeviceSchedulingControlsFlagBitsARM(u64);

    impl PhysicalDeviceSchedulingControlsFlagBitsARM {
        pub const SHADER_CORE_COUNT_ARM: Self = Self(1 << 0);
        pub const DISPATCH_PARAMETERS_ARM: Self = Self(1 << 1);
    }

    impl fmt::Debug for PhysicalDeviceSchedulingControlsFlagBitsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SHADER_CORE_COUNT_ARM => Some("SHADER_CORE_COUNT_ARM"),
                Self::DISPATCH_PARAMETERS_ARM => Some("DISPATCH_PARAMETERS_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDispatchParametersARM.html>
    pub type PFN_vkCmdSetDispatchParametersARM = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_dispatch_parameters: *const DispatchParametersARM<'_>,
    );
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
    pub type VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM =
        PhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM<'static>;
    pub type VkDispatchParametersARM = DispatchParametersARM<'static>;
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
    impl PhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DispatchParametersARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDispatchParametersARM {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    cmd_set_dispatch_parameters: PFN_vkCmdSetDispatchParametersARM,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_dispatch_parameters: transmute(
                    load(c"vkCmdSetDispatchParametersARM").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDispatchParametersARM.html>
    #[inline]
    pub unsafe fn cmd_set_dispatch_parameters(
        &self,
        command_buffer: CommandBuffer,
        dispatch_parameters: &DispatchParametersARM<'_>,
    ) {
        unsafe { (self.cmd_set_dispatch_parameters)(command_buffer, dispatch_parameters) }
    }
}
