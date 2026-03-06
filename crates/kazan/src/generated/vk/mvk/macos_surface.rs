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
    pub struct MacOSSurfaceCreateInfoMVK<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: MacOSSurfaceCreateFlagsMVK,
        pub p_view: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MacOSSurfaceCreateInfoMVK<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MACOS_SURFACE_CREATE_INFO_MVK;
    }
    impl Default for MacOSSurfaceCreateInfoMVK<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                p_view: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MacOSSurfaceCreateInfoMVK<'a> {
        pub fn flags(mut self, flags: MacOSSurfaceCreateFlagsMVK) -> Self {
            self.flags = flags;
            self
        }
        pub fn view(mut self, view: *const c_void) -> Self {
            self.p_view = view;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MacOSSurfaceCreateFlagsMVK(Flags);
    vk_bitflags_wrapped!(MacOSSurfaceCreateFlagsMVK, Flags);
    pub type PFN_vkCreateMacOSSurfaceMVK = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const MacOSSurfaceCreateInfoMVK<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
}
pub struct InstanceFn {
    create_mac_os_surface_mvk: PFN_vkCreateMacOSSurfaceMVK,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_mac_os_surface_mvk: transmute(
                    load(c"vkCreateMacOSSurfaceMVK").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn create_mac_os_surface_mvk(
        &self,
        instance: Instance,
        create_info: &MacOSSurfaceCreateInfoMVK<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_mac_os_surface_mvk)(
                instance,
                create_info,
                allocator.to_raw_ptr(),
                surface.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(surface.assume_init()),
                err => Err(err),
            }
        }
    }
}
