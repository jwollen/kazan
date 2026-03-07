//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_external_fence_fd.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_external_fence_fd";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportFenceFdInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImportFenceFdInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub fence: Fence,
        pub flags: FenceImportFlags,
        pub handle_type: ExternalFenceHandleTypeFlagBits,
        pub fd: c_int,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImportFenceFdInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImportFenceFdInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("fence", &self.fence)
                .field("flags", &self.flags)
                .field("handle_type", &self.handle_type)
                .field("fd", &self.fd)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImportFenceFdInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_FENCE_FD_INFO_KHR;
    }

    impl Default for ImportFenceFdInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                fence: Default::default(),
                flags: Default::default(),
                handle_type: Default::default(),
                fd: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportFenceFdInfoKHR<'a> {
        #[inline]
        pub fn fence(mut self, fence: Fence) -> Self {
            self.fence = fence;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: FenceImportFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn handle_type(mut self, handle_type: ExternalFenceHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }

        #[inline]
        pub fn fd(mut self, fd: c_int) -> Self {
            self.fd = fd;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFenceGetFdInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct FenceGetFdInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub fence: Fence,
        pub handle_type: ExternalFenceHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for FenceGetFdInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("FenceGetFdInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("fence", &self.fence)
                .field("handle_type", &self.handle_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for FenceGetFdInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::FENCE_GET_FD_INFO_KHR;
    }

    impl Default for FenceGetFdInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                fence: Default::default(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> FenceGetFdInfoKHR<'a> {
        #[inline]
        pub fn fence(mut self, fence: Fence) -> Self {
            self.fence = fence;
            self
        }

        #[inline]
        pub fn handle_type(mut self, handle_type: ExternalFenceHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetFenceFdKHR.html>
    pub type PFN_vkGetFenceFdKHR = unsafe extern "system" fn(
        device: Device,
        p_get_fd_info: *const FenceGetFdInfoKHR<'_>,
        p_fd: *mut c_int,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkImportFenceFdKHR.html>
    pub type PFN_vkImportFenceFdKHR = unsafe extern "system" fn(
        device: Device,
        p_import_fence_fd_info: *const ImportFenceFdInfoKHR<'_>,
    ) -> vk::Result;
}

pub struct DeviceFn {
    import_fence_fd_khr: PFN_vkImportFenceFdKHR,
    get_fence_fd_khr: PFN_vkGetFenceFdKHR,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                import_fence_fd_khr: transmute(
                    load(c"vkImportFenceFdKHR").ok_or(MissingEntryPointError)?,
                ),
                get_fence_fd_khr: transmute(
                    load(c"vkGetFenceFdKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkImportFenceFdKHR.html>
    #[inline]
    pub unsafe fn import_fence_fd_khr(
        &self,
        device: Device,
        import_fence_fd_info: &ImportFenceFdInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.import_fence_fd_khr)(device, import_fence_fd_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetFenceFdKHR.html>
    #[inline]
    pub unsafe fn get_fence_fd_khr(
        &self,
        device: Device,
        get_fd_info: &FenceGetFdInfoKHR<'_>,
    ) -> crate::Result<c_int> {
        unsafe {
            let mut fd = core::mem::MaybeUninit::uninit();
            let result = (self.get_fence_fd_khr)(device, get_fd_info, fd.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(fd.assume_init()),
                err => Err(err),
            }
        }
    }
}
