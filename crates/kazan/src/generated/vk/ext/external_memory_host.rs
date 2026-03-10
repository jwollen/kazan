//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_external_memory_host.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_external_memory_host";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportMemoryHostPointerInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImportMemoryHostPointerInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub p_host_pointer: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImportMemoryHostPointerInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImportMemoryHostPointerInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("handle_type", &self.handle_type)
                .field("p_host_pointer", &self.p_host_pointer)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImportMemoryHostPointerInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_MEMORY_HOST_POINTER_INFO_EXT;
    }

    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for ImportMemoryHostPointerInfoEXT<'a> {}

    impl Default for ImportMemoryHostPointerInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                handle_type: Default::default(),
                p_host_pointer: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportMemoryHostPointerInfoEXT<'a> {
        #[inline]
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }

        #[inline]
        pub fn host_pointer(mut self, host_pointer: *mut c_void) -> Self {
            self.p_host_pointer = host_pointer;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryHostPointerPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryHostPointerPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_type_bits: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryHostPointerPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryHostPointerPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory_type_bits", &self.memory_type_bits)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryHostPointerPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_HOST_POINTER_PROPERTIES_EXT;
    }

    impl Default for MemoryHostPointerPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                memory_type_bits: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryHostPointerPropertiesEXT<'a> {
        #[inline]
        pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
            self.memory_type_bits = memory_type_bits;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExternalMemoryHostPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_imported_host_pointer_alignment: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceExternalMemoryHostPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceExternalMemoryHostPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "min_imported_host_pointer_alignment",
                    &self.min_imported_host_pointer_alignment,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExternalMemoryHostPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceExternalMemoryHostPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceExternalMemoryHostPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                min_imported_host_pointer_alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceExternalMemoryHostPropertiesEXT<'a> {
        #[inline]
        pub fn min_imported_host_pointer_alignment(
            mut self,
            min_imported_host_pointer_alignment: DeviceSize,
        ) -> Self {
            self.min_imported_host_pointer_alignment = min_imported_host_pointer_alignment;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryHostPointerPropertiesEXT.html>
    pub type PFN_vkGetMemoryHostPointerPropertiesEXT = unsafe extern "system" fn(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        p_host_pointer: *const c_void,
        p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT<'_>,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkImportMemoryHostPointerInfoEXT = ImportMemoryHostPointerInfoEXT<'static>;
    pub type VkMemoryHostPointerPropertiesEXT = MemoryHostPointerPropertiesEXT<'static>;
    pub type VkPhysicalDeviceExternalMemoryHostPropertiesEXT =
        PhysicalDeviceExternalMemoryHostPropertiesEXT<'static>;
    impl ImportMemoryHostPointerInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImportMemoryHostPointerInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MemoryHostPointerPropertiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMemoryHostPointerPropertiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceExternalMemoryHostPropertiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_memory_host_pointer_properties_ext: PFN_vkGetMemoryHostPointerPropertiesEXT,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_memory_host_pointer_properties_ext: transmute(
                    load(c"vkGetMemoryHostPointerPropertiesEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryHostPointerPropertiesEXT.html>
    #[inline]
    pub unsafe fn get_memory_host_pointer_properties_ext(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        host_pointer: *const c_void,
        memory_host_pointer_properties: &mut MemoryHostPointerPropertiesEXT<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_memory_host_pointer_properties_ext)(
                device,
                handle_type,
                host_pointer,
                memory_host_pointer_properties,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
