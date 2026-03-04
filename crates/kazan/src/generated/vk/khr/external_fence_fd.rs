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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImportFenceFdInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub fence: Fence,
        pub flags: FenceImportFlags,
        pub handle_type: ExternalFenceHandleTypeFlagBits,
        pub fd: c_int,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImportFenceFdInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_FENCE_FD_INFO_KHR;
    }
    impl Default for ImportFenceFdInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                fence: Default::default(),
                flags: Default::default(),
                handle_type: Default::default(),
                fd: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImportFenceFdInfoKHR<'a> {
        pub fn fence(mut self, fence: Fence) -> Self {
            self.fence = fence;
            self
        }
        pub fn flags(mut self, flags: FenceImportFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn handle_type(mut self, handle_type: ExternalFenceHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
        pub fn fd(mut self, fd: c_int) -> Self {
            self.fd = fd;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct FenceGetFdInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub fence: Fence,
        pub handle_type: ExternalFenceHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for FenceGetFdInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::FENCE_GET_FD_INFO_KHR;
    }
    impl Default for FenceGetFdInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                fence: Default::default(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> FenceGetFdInfoKHR<'a> {
        pub fn fence(mut self, fence: Fence) -> Self {
            self.fence = fence;
            self
        }
        pub fn handle_type(mut self, handle_type: ExternalFenceHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }
    pub type PFN_vkGetFenceFdKHR = unsafe extern "system" fn(
        device: Device,
        p_get_fd_info: *const FenceGetFdInfoKHR<'_>,
        p_fd: *mut c_int,
    ) -> vk::Result;
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
