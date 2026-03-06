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
    pub struct ImagePipeSurfaceCreateInfoFUCHSIA<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: ImagePipeSurfaceCreateFlagsFUCHSIA,
        pub image_pipe_handle: zx_handle_t,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImagePipeSurfaceCreateInfoFUCHSIA<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA;
    }
    impl Default for ImagePipeSurfaceCreateInfoFUCHSIA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                image_pipe_handle: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImagePipeSurfaceCreateInfoFUCHSIA<'a> {
        pub fn flags(mut self, flags: ImagePipeSurfaceCreateFlagsFUCHSIA) -> Self {
            self.flags = flags;
            self
        }
        pub fn image_pipe_handle(mut self, image_pipe_handle: zx_handle_t) -> Self {
            self.image_pipe_handle = image_pipe_handle;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ImagePipeSurfaceCreateFlagsFUCHSIA(Flags);
    vk_bitflags_wrapped!(ImagePipeSurfaceCreateFlagsFUCHSIA, Flags);
    pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
}
pub struct InstanceFn {
    create_image_pipe_surface_fuchsia: PFN_vkCreateImagePipeSurfaceFUCHSIA,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_image_pipe_surface_fuchsia: transmute(
                    load(c"vkCreateImagePipeSurfaceFUCHSIA").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn create_image_pipe_surface_fuchsia(
        &self,
        instance: Instance,
        create_info: &ImagePipeSurfaceCreateInfoFUCHSIA<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_image_pipe_surface_fuchsia)(
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
