//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_QCOM_queue_perf_hint.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_QCOM_queue_perf_hint";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerfHintInfoQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PerfHintInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ty: PerfHintTypeQCOM,
        pub scale: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PerfHintInfoQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PerfHintInfoQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("ty", &self.ty)
                .field("scale", &self.scale)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PerfHintInfoQCOM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PERF_HINT_INFO_QCOM;
    }

    impl Default for PerfHintInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                ty: Default::default(),
                scale: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PerfHintInfoQCOM<'a> {
        #[inline]
        pub fn ty(mut self, ty: PerfHintTypeQCOM) -> Self {
            self.ty = ty;
            self
        }

        #[inline]
        pub fn scale(mut self, scale: u32) -> Self {
            self.scale = scale;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceQueuePerfHintFeaturesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceQueuePerfHintFeaturesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub queue_perf_hint: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceQueuePerfHintFeaturesQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceQueuePerfHintFeaturesQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("queue_perf_hint", &self.queue_perf_hint)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceQueuePerfHintFeaturesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_QUEUE_PERF_HINT_FEATURES_QCOM;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceQueuePerfHintFeaturesQCOM<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceQueuePerfHintFeaturesQCOM<'_> {}

    impl Default for PhysicalDeviceQueuePerfHintFeaturesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                queue_perf_hint: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceQueuePerfHintFeaturesQCOM<'a> {
        #[inline]
        pub fn queue_perf_hint(mut self, queue_perf_hint: bool) -> Self {
            self.queue_perf_hint = queue_perf_hint.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceQueuePerfHintPropertiesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceQueuePerfHintPropertiesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub supported_queues: QueueFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceQueuePerfHintPropertiesQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceQueuePerfHintPropertiesQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("supported_queues", &self.supported_queues)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceQueuePerfHintPropertiesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_QUEUE_PERF_HINT_PROPERTIES_QCOM;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceQueuePerfHintPropertiesQCOM<'_>
    {
    }

    impl Default for PhysicalDeviceQueuePerfHintPropertiesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                supported_queues: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceQueuePerfHintPropertiesQCOM<'a> {
        #[inline]
        pub fn supported_queues(mut self, supported_queues: QueueFlags) -> Self {
            self.supported_queues = supported_queues;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerfHintTypeQCOM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PerfHintTypeQCOM(i32);

    impl PerfHintTypeQCOM {
        pub const DEFAULT_QCOM: Self = Self(0);
        pub const FREQUENCY_MIN_QCOM: Self = Self(1);
        pub const FREQUENCY_MAX_QCOM: Self = Self(2);
        pub const FREQUENCY_SCALED_QCOM: Self = Self(3);
    }

    impl fmt::Debug for PerfHintTypeQCOM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEFAULT_QCOM => Some("DEFAULT_QCOM"),
                Self::FREQUENCY_MIN_QCOM => Some("FREQUENCY_MIN_QCOM"),
                Self::FREQUENCY_MAX_QCOM => Some("FREQUENCY_MAX_QCOM"),
                Self::FREQUENCY_SCALED_QCOM => Some("FREQUENCY_SCALED_QCOM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueSetPerfHintQCOM.html>
    pub type PFN_vkQueueSetPerfHintQCOM = unsafe extern "system" fn(
        queue: Queue,
        p_perf_hint_info: *const PerfHintInfoQCOM<'_>,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPerfHintInfoQCOM = PerfHintInfoQCOM<'static>;
    pub type VkPhysicalDeviceQueuePerfHintFeaturesQCOM =
        PhysicalDeviceQueuePerfHintFeaturesQCOM<'static>;
    pub type VkPhysicalDeviceQueuePerfHintPropertiesQCOM =
        PhysicalDeviceQueuePerfHintPropertiesQCOM<'static>;
    pub type VkPerfHintTypeQCOM = PerfHintTypeQCOM;
    impl PerfHintInfoQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPerfHintInfoQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceQueuePerfHintFeaturesQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceQueuePerfHintFeaturesQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceQueuePerfHintPropertiesQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceQueuePerfHintPropertiesQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    queue_set_perf_hint: PFN_vkQueueSetPerfHintQCOM,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                queue_set_perf_hint: transmute(
                    load(c"vkQueueSetPerfHintQCOM").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueSetPerfHintQCOM.html>
    #[inline]
    pub unsafe fn queue_set_perf_hint(
        &self,
        queue: Queue,
        perf_hint_info: &PerfHintInfoQCOM<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.queue_set_perf_hint)(queue, perf_hint_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
