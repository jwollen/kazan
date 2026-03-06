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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportMemoryZirconHandleInfoFUCHSIA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImportMemoryZirconHandleInfoFUCHSIA<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub handle: zx_handle_t,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImportMemoryZirconHandleInfoFUCHSIA<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA;
    }
    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for ImportMemoryZirconHandleInfoFUCHSIA<'a> {}
    impl Default for ImportMemoryZirconHandleInfoFUCHSIA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                handle_type: Default::default(),
                handle: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImportMemoryZirconHandleInfoFUCHSIA<'a> {
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
        pub fn handle(mut self, handle: zx_handle_t) -> Self {
            self.handle = handle;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryZirconHandlePropertiesFUCHSIA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryZirconHandlePropertiesFUCHSIA<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_type_bits: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryZirconHandlePropertiesFUCHSIA<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA;
    }
    impl Default for MemoryZirconHandlePropertiesFUCHSIA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                memory_type_bits: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryZirconHandlePropertiesFUCHSIA<'a> {
        pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
            self.memory_type_bits = memory_type_bits;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryGetZirconHandleInfoFUCHSIA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryGetZirconHandleInfoFUCHSIA<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryGetZirconHandleInfoFUCHSIA<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA;
    }
    impl Default for MemoryGetZirconHandleInfoFUCHSIA<'_> {
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
    impl<'a> MemoryGetZirconHandleInfoFUCHSIA<'a> {
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryZirconHandleFUCHSIA.html>
    pub type PFN_vkGetMemoryZirconHandleFUCHSIA = unsafe extern "system" fn(
        device: Device,
        p_get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA<'_>,
        p_zircon_handle: *mut zx_handle_t,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryZirconHandlePropertiesFUCHSIA.html>
    pub type PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA = unsafe extern "system" fn(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        zircon_handle: zx_handle_t,
        p_memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA<'_>,
    )
        -> vk::Result;
}
pub struct DeviceFn {
    get_memory_zircon_handle_fuchsia: PFN_vkGetMemoryZirconHandleFUCHSIA,
    get_memory_zircon_handle_properties_fuchsia: PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_memory_zircon_handle_fuchsia: transmute(
                    load(c"vkGetMemoryZirconHandleFUCHSIA").ok_or(MissingEntryPointError)?,
                ),
                get_memory_zircon_handle_properties_fuchsia: transmute(
                    load(c"vkGetMemoryZirconHandlePropertiesFUCHSIA")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryZirconHandleFUCHSIA.html>
    pub unsafe fn get_memory_zircon_handle_fuchsia(
        &self,
        device: Device,
        get_zircon_handle_info: &MemoryGetZirconHandleInfoFUCHSIA<'_>,
    ) -> crate::Result<zx_handle_t> {
        unsafe {
            let mut zircon_handle = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_zircon_handle_fuchsia)(
                device,
                get_zircon_handle_info,
                zircon_handle.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(zircon_handle.assume_init()),
                err => Err(err),
            }
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryZirconHandlePropertiesFUCHSIA.html>
    pub unsafe fn get_memory_zircon_handle_properties_fuchsia(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        zircon_handle: zx_handle_t,
        memory_zircon_handle_properties: &mut MemoryZirconHandlePropertiesFUCHSIA<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_memory_zircon_handle_properties_fuchsia)(
                device,
                handle_type,
                zircon_handle,
                memory_zircon_handle_properties,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
