#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    pub type RemoteAddressNV = c_void;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceExternalMemoryRDMAFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub external_memory_rdma: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExternalMemoryRDMAFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceExternalMemoryRDMAFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceExternalMemoryRDMAFeaturesNV<'a> {}
    impl Default for PhysicalDeviceExternalMemoryRDMAFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                external_memory_rdma: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceExternalMemoryRDMAFeaturesNV<'a> {
        pub fn external_memory_rdma(mut self, external_memory_rdma: Bool32) -> Self {
            self.external_memory_rdma = external_memory_rdma;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryGetRemoteAddressInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryGetRemoteAddressInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_GET_REMOTE_ADDRESS_INFO_NV;
    }
    impl Default for MemoryGetRemoteAddressInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                memory: Default::default(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryGetRemoteAddressInfoNV<'a> {
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }
    pub type PFN_vkGetMemoryRemoteAddressNV = unsafe extern "system" fn(
        device: Device,
        p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV<'_>,
        p_address: *mut RemoteAddressNV,
    ) -> vk::Result;
}
pub struct DeviceFn {
    get_memory_remote_address_nv: PFN_vkGetMemoryRemoteAddressNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_memory_remote_address_nv: transmute(
                    load(c"vkGetMemoryRemoteAddressNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_memory_remote_address_nv(
        &self,
        device: Device,
        memory_get_remote_address_info: &MemoryGetRemoteAddressInfoNV<'_>,
    ) -> crate::Result<RemoteAddressNV> {
        unsafe {
            let mut address = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_remote_address_nv)(
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
