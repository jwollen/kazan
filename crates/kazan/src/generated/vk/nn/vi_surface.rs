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
    pub struct ViSurfaceCreateInfoNN<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: ViSurfaceCreateFlagsNN,
        pub window: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for ViSurfaceCreateInfoNN<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::VI_SURFACE_CREATE_INFO_NN,
                p_next: core::ptr::null(),
                flags: Default::default(),
                window: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ViSurfaceCreateInfoNN<'a> {
        pub fn flags(mut self, flags: ViSurfaceCreateFlagsNN) -> Self {
            self.flags = flags;
            self
        }
        pub fn window(mut self, window: &'a mut c_void) -> Self {
            self.window = window;
            self
        }
    }
    bitflags! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct ViSurfaceCreateFlagsNN: Flags {
        }
    }
    pub type PFN_vkCreateViSurfaceNN = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const ViSurfaceCreateInfoNN<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
}
pub struct InstanceFn {
    create_vi_surface_nn: PFN_vkCreateViSurfaceNN,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_vi_surface_nn: transmute(load(c"vkCreateViSurfaceNN").ok_or(LoadingError)?),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn create_vi_surface_nn(
        &self,
        instance: Instance,
        create_info: &ViSurfaceCreateInfoNN<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_vi_surface_nn)(
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
