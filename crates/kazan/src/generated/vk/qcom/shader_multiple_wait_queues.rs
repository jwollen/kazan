//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_QCOM_shader_multiple_wait_queues.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_QCOM_shader_multiple_wait_queues";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_multiple_wait_queues: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "shader_multiple_wait_queues",
                    &self.shader_multiple_wait_queues,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_FEATURES_QCOM;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'_>
    {
    }

    impl Default for PhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_multiple_wait_queues: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'a> {
        #[inline]
        pub fn shader_multiple_wait_queues(mut self, shader_multiple_wait_queues: bool) -> Self {
            self.shader_multiple_wait_queues = shader_multiple_wait_queues.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_shader_wait_queues: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_shader_wait_queues", &self.max_shader_wait_queues)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_PROPERTIES_QCOM;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM<'_>
    {
    }

    impl Default for PhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_shader_wait_queues: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM<'a> {
        #[inline]
        pub fn max_shader_wait_queues(mut self, max_shader_wait_queues: u32) -> Self {
            self.max_shader_wait_queues = max_shader_wait_queues;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM =
        PhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'static>;
    pub type VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM =
        PhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM<'static>;
    impl PhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
}
