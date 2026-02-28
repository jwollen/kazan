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
    pub struct ImportMemoryHostPointerInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub p_host_pointer: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for ImportMemoryHostPointerInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::IMPORT_MEMORY_HOST_POINTER_INFO_EXT,
                p_next: core::ptr::null(),
                handle_type: Default::default(),
                p_host_pointer: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImportMemoryHostPointerInfoEXT<'a> {
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
        pub fn host_pointer(mut self, host_pointer: &'a mut c_void) -> Self {
            self.p_host_pointer = host_pointer;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryHostPointerPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_type_bits: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for MemoryHostPointerPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::MEMORY_HOST_POINTER_PROPERTIES_EXT,
                p_next: core::ptr::null_mut(),
                memory_type_bits: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryHostPointerPropertiesEXT<'a> {
        pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
            self.memory_type_bits = memory_type_bits;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_imported_host_pointer_alignment: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceExternalMemoryHostPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT,
                p_next: core::ptr::null_mut(),
                min_imported_host_pointer_alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceExternalMemoryHostPropertiesEXT<'a> {
        pub fn min_imported_host_pointer_alignment(
            mut self,
            min_imported_host_pointer_alignment: DeviceSize,
        ) -> Self {
            self.min_imported_host_pointer_alignment = min_imported_host_pointer_alignment;
            self
        }
    }
    pub type PFN_vkGetMemoryHostPointerPropertiesEXT = unsafe extern "system" fn(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        p_host_pointer: *const c_void,
        p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT<'_>,
    ) -> vk::Result;
}
pub struct DeviceFn {
    get_memory_host_pointer_properties_ext: PFN_vkGetMemoryHostPointerPropertiesEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_memory_host_pointer_properties_ext: transmute(
                    load(c"vkGetMemoryHostPointerPropertiesEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_memory_host_pointer_properties_ext(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        host_pointer: &c_void,
    ) -> crate::Result<MemoryHostPointerPropertiesEXT<'_>> {
        unsafe {
            let mut memory_host_pointer_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_host_pointer_properties_ext)(
                device,
                handle_type,
                host_pointer,
                memory_host_pointer_properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(memory_host_pointer_properties.assume_init()),
                err => Err(err),
            }
        }
    }
}
