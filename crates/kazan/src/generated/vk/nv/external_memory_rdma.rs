//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_external_memory_rdma.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_external_memory_rdma";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    pub type RemoteAddressNV = c_void;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExternalMemoryRDMAFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceExternalMemoryRDMAFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub external_memory_rdma: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceExternalMemoryRDMAFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceExternalMemoryRDMAFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("external_memory_rdma", &self.external_memory_rdma)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExternalMemoryRDMAFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceExternalMemoryRDMAFeaturesNV<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceExternalMemoryRDMAFeaturesNV<'_> {}

    impl Default for PhysicalDeviceExternalMemoryRDMAFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                external_memory_rdma: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceExternalMemoryRDMAFeaturesNV<'a> {
        #[inline]
        pub fn external_memory_rdma(mut self, external_memory_rdma: bool) -> Self {
            self.external_memory_rdma = external_memory_rdma.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryGetRemoteAddressInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryGetRemoteAddressInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryGetRemoteAddressInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryGetRemoteAddressInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory", &self.memory)
                .field("handle_type", &self.handle_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryGetRemoteAddressInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_GET_REMOTE_ADDRESS_INFO_NV;
    }

    impl Default for MemoryGetRemoteAddressInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                memory: Default::default(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryGetRemoteAddressInfoNV<'a> {
        #[inline]
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }

        #[inline]
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryRemoteAddressNV.html>
    pub type PFN_vkGetMemoryRemoteAddressNV = unsafe extern "system" fn(
        device: Device,
        p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV<'_>,
        p_address: *mut RemoteAddressNV,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkRemoteAddressNV = RemoteAddressNV;
    pub type VkPhysicalDeviceExternalMemoryRDMAFeaturesNV =
        PhysicalDeviceExternalMemoryRDMAFeaturesNV<'static>;
    pub type VkMemoryGetRemoteAddressInfoNV = MemoryGetRemoteAddressInfoNV<'static>;
    impl PhysicalDeviceExternalMemoryRDMAFeaturesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceExternalMemoryRDMAFeaturesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MemoryGetRemoteAddressInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMemoryGetRemoteAddressInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_memory_remote_address: PFN_vkGetMemoryRemoteAddressNV,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_memory_remote_address: transmute(
                    load(c"vkGetMemoryRemoteAddressNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryRemoteAddressNV.html>
    #[inline]
    pub unsafe fn get_memory_remote_address(
        &self,
        device: Device,
        memory_get_remote_address_info: &MemoryGetRemoteAddressInfoNV<'_>,
    ) -> crate::Result<RemoteAddressNV> {
        unsafe {
            let mut address = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_remote_address)(
                device,
                memory_get_remote_address_info,
                address.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(address.assume_init()),
                err => Err(err),
            }
        }
    }
}
