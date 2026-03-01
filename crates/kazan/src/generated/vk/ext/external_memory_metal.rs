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
    pub struct ImportMemoryMetalHandleInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub handle: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImportMemoryMetalHandleInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_MEMORY_METAL_HANDLE_INFO_EXT;
    }
    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for ImportMemoryMetalHandleInfoEXT<'a> {}
    impl Default for ImportMemoryMetalHandleInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                handle_type: Default::default(),
                handle: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImportMemoryMetalHandleInfoEXT<'a> {
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
        pub fn handle(mut self, handle: &'a mut c_void) -> Self {
            self.handle = handle;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryMetalHandlePropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_type_bits: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryMetalHandlePropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_METAL_HANDLE_PROPERTIES_EXT;
    }
    impl Default for MemoryMetalHandlePropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                memory_type_bits: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryMetalHandlePropertiesEXT<'a> {
        pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
            self.memory_type_bits = memory_type_bits;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryGetMetalHandleInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryGetMetalHandleInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_GET_METAL_HANDLE_INFO_EXT;
    }
    impl Default for MemoryGetMetalHandleInfoEXT<'_> {
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
    impl<'a> MemoryGetMetalHandleInfoEXT<'a> {
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }
    pub type PFN_vkGetMemoryMetalHandleEXT = unsafe extern "system" fn(
        device: Device,
        p_get_metal_handle_info: *const MemoryGetMetalHandleInfoEXT<'_>,
        p_handle: *mut *mut c_void,
    ) -> vk::Result;
    pub type PFN_vkGetMemoryMetalHandlePropertiesEXT = unsafe extern "system" fn(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        p_handle: *const c_void,
        p_memory_metal_handle_properties: *mut MemoryMetalHandlePropertiesEXT<'_>,
    ) -> vk::Result;
}
pub struct DeviceFn {
    get_memory_metal_handle_ext: PFN_vkGetMemoryMetalHandleEXT,
    get_memory_metal_handle_properties_ext: PFN_vkGetMemoryMetalHandlePropertiesEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_memory_metal_handle_ext: transmute(
                    load(c"vkGetMemoryMetalHandleEXT").ok_or(LoadingError)?,
                ),
                get_memory_metal_handle_properties_ext: transmute(
                    load(c"vkGetMemoryMetalHandlePropertiesEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_memory_metal_handle_ext(
        &self,
        device: Device,
        get_metal_handle_info: &MemoryGetMetalHandleInfoEXT<'_>,
        handle: &mut *mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_memory_metal_handle_ext)(device, get_metal_handle_info, handle);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_memory_metal_handle_properties_ext(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        handle: &c_void,
    ) -> crate::Result<MemoryMetalHandlePropertiesEXT<'_>> {
        unsafe {
            let mut memory_metal_handle_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_metal_handle_properties_ext)(
                device,
                handle_type,
                handle,
                memory_metal_handle_properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(memory_metal_handle_properties.assume_init()),
                err => Err(err),
            }
        }
    }
}
