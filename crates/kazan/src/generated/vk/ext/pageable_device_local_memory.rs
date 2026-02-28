#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pageable_device_local_memory: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT,
                p_next: core::ptr::null_mut(),
                pageable_device_local_memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'a> {
        pub fn pageable_device_local_memory(
            mut self,
            pageable_device_local_memory: Bool32,
        ) -> Self {
            self.pageable_device_local_memory = pageable_device_local_memory;
            self
        }
    }
    pub type PFN_vkSetDeviceMemoryPriorityEXT =
        unsafe extern "system" fn(device: Device, memory: DeviceMemory, priority: f32);
}
pub struct DeviceFn {
    set_device_memory_priority_ext: PFN_vkSetDeviceMemoryPriorityEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                set_device_memory_priority_ext: transmute(
                    load(c"vkSetDeviceMemoryPriorityEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn set_device_memory_priority_ext(
        &self,
        device: Device,
        memory: DeviceMemory,
        priority: f32,
    ) {
        unsafe { (self.set_device_memory_priority_ext)(device, memory, priority) }
    }
}
