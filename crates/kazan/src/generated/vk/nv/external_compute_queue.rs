//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_external_compute_queue.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_external_compute_queue";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    define_handle!(
        ExternalComputeQueueNV,
        EXTERNAL_COMPUTE_QUEUE_NV,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalComputeQueueNV.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalComputeQueueDeviceCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExternalComputeQueueDeviceCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub reserved_external_queues: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExternalComputeQueueDeviceCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExternalComputeQueueDeviceCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("reserved_external_queues", &self.reserved_external_queues)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExternalComputeQueueDeviceCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV;
    }

    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for ExternalComputeQueueDeviceCreateInfoNV<'a> {}

    impl Default for ExternalComputeQueueDeviceCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                reserved_external_queues: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExternalComputeQueueDeviceCreateInfoNV<'a> {
        #[inline]
        pub fn reserved_external_queues(mut self, reserved_external_queues: u32) -> Self {
            self.reserved_external_queues = reserved_external_queues;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalComputeQueueCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExternalComputeQueueCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub preferred_queue: Queue,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExternalComputeQueueCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExternalComputeQueueCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("preferred_queue", &self.preferred_queue)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExternalComputeQueueCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV;
    }

    impl Default for ExternalComputeQueueCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                preferred_queue: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExternalComputeQueueCreateInfoNV<'a> {
        #[inline]
        pub fn preferred_queue(mut self, preferred_queue: Queue) -> Self {
            self.preferred_queue = preferred_queue;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalComputeQueueDataParamsNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExternalComputeQueueDataParamsNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub device_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExternalComputeQueueDataParamsNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExternalComputeQueueDataParamsNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("device_index", &self.device_index)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExternalComputeQueueDataParamsNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV;
    }

    impl Default for ExternalComputeQueueDataParamsNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                device_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExternalComputeQueueDataParamsNV<'a> {
        #[inline]
        pub fn device_index(mut self, device_index: u32) -> Self {
            self.device_index = device_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExternalComputeQueuePropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceExternalComputeQueuePropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub external_data_size: u32,
        pub max_external_queues: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceExternalComputeQueuePropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceExternalComputeQueuePropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("external_data_size", &self.external_data_size)
                .field("max_external_queues", &self.max_external_queues)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExternalComputeQueuePropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceExternalComputeQueuePropertiesNV<'a>
    {
    }

    impl Default for PhysicalDeviceExternalComputeQueuePropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                external_data_size: Default::default(),
                max_external_queues: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceExternalComputeQueuePropertiesNV<'a> {
        #[inline]
        pub fn external_data_size(mut self, external_data_size: u32) -> Self {
            self.external_data_size = external_data_size;
            self
        }

        #[inline]
        pub fn max_external_queues(mut self, max_external_queues: u32) -> Self {
            self.max_external_queues = max_external_queues;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateExternalComputeQueueNV.html>
    pub type PFN_vkCreateExternalComputeQueueNV = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ExternalComputeQueueCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_external_queue: *mut ExternalComputeQueueNV,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyExternalComputeQueueNV.html>
    pub type PFN_vkDestroyExternalComputeQueueNV = unsafe extern "system" fn(
        device: Device,
        external_queue: ExternalComputeQueueNV,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetExternalComputeQueueDataNV.html>
    pub type PFN_vkGetExternalComputeQueueDataNV = unsafe extern "system" fn(
        external_queue: ExternalComputeQueueNV,
        params: *mut ExternalComputeQueueDataParamsNV<'_>,
        p_data: *mut c_void,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkExternalComputeQueueNV = ExternalComputeQueueNV;
    pub type VkExternalComputeQueueDeviceCreateInfoNV =
        ExternalComputeQueueDeviceCreateInfoNV<'static>;
    pub type VkExternalComputeQueueCreateInfoNV = ExternalComputeQueueCreateInfoNV<'static>;
    pub type VkExternalComputeQueueDataParamsNV = ExternalComputeQueueDataParamsNV<'static>;
    pub type VkPhysicalDeviceExternalComputeQueuePropertiesNV =
        PhysicalDeviceExternalComputeQueuePropertiesNV<'static>;
    impl ExternalComputeQueueDeviceCreateInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExternalComputeQueueDeviceCreateInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExternalComputeQueueCreateInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExternalComputeQueueCreateInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExternalComputeQueueDataParamsNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExternalComputeQueueDataParamsNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceExternalComputeQueuePropertiesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceExternalComputeQueuePropertiesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    create_external_compute_queue_nv: PFN_vkCreateExternalComputeQueueNV,
    destroy_external_compute_queue_nv: PFN_vkDestroyExternalComputeQueueNV,
    get_external_compute_queue_data_nv: PFN_vkGetExternalComputeQueueDataNV,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_external_compute_queue_nv: transmute(
                    load(c"vkCreateExternalComputeQueueNV").ok_or(MissingEntryPointError)?,
                ),
                destroy_external_compute_queue_nv: transmute(
                    load(c"vkDestroyExternalComputeQueueNV").ok_or(MissingEntryPointError)?,
                ),
                get_external_compute_queue_data_nv: transmute(
                    load(c"vkGetExternalComputeQueueDataNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateExternalComputeQueueNV.html>
    #[inline]
    pub unsafe fn create_external_compute_queue_nv(
        &self,
        device: Device,
        create_info: &ExternalComputeQueueCreateInfoNV<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<ExternalComputeQueueNV> {
        unsafe {
            let mut external_queue = core::mem::MaybeUninit::uninit();
            let result = (self.create_external_compute_queue_nv)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                external_queue.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(external_queue.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyExternalComputeQueueNV.html>
    #[inline]
    pub unsafe fn destroy_external_compute_queue_nv(
        &self,
        device: Device,
        external_queue: ExternalComputeQueueNV,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_external_compute_queue_nv)(device, external_queue, allocator.to_raw_ptr())
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetExternalComputeQueueDataNV.html>
    #[inline]
    pub unsafe fn get_external_compute_queue_data_nv(
        &self,
        external_queue: ExternalComputeQueueNV,
        params: &mut ExternalComputeQueueDataParamsNV<'_>,
        data: *mut c_void,
    ) {
        unsafe { (self.get_external_compute_queue_data_nv)(external_queue, params, data) }
    }
}
