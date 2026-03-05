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
    pub struct IOSSurfaceCreateInfoMVK<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: IOSSurfaceCreateFlagsMVK,
        pub p_view: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for IOSSurfaceCreateInfoMVK<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IOS_SURFACE_CREATE_INFO_MVK;
    }
    impl Default for IOSSurfaceCreateInfoMVK<'_> {
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
    impl<'a> IOSSurfaceCreateInfoMVK<'a> {
        pub fn flags(mut self, flags: IOSSurfaceCreateFlagsMVK) -> Self {
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
    pub struct IOSSurfaceCreateFlagsMVK(Flags);
    vk_bitflags_wrapped!(IOSSurfaceCreateFlagsMVK, Flags);
    impl IOSSurfaceCreateFlagsMVK {}
    pub type PFN_vkCreateIOSSurfaceMVK = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const IOSSurfaceCreateInfoMVK<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
}
pub struct InstanceFn {
    create_ios_surface_mvk: PFN_vkCreateIOSSurfaceMVK,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_ios_surface_mvk: transmute(
                    load(c"vkCreateIOSSurfaceMVK").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn create_ios_surface_mvk(
        &self,
        instance: Instance,
        create_info: &IOSSurfaceCreateInfoMVK<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_ios_surface_mvk)(
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
