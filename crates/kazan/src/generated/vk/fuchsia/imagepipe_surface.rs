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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImagePipeSurfaceCreateInfoFUCHSIA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImagePipeSurfaceCreateInfoFUCHSIA<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: ImagePipeSurfaceCreateFlagsFUCHSIA,
        pub image_pipe_handle: zx_handle_t,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for ImagePipeSurfaceCreateInfoFUCHSIA<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImagePipeSurfaceCreateInfoFUCHSIA")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("image_pipe_handle", &self.image_pipe_handle)
                .finish()
        }
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImagePipeSurfaceCreateFlagsFUCHSIA.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ImagePipeSurfaceCreateFlagsFUCHSIA(Flags);
    vk_bitflags_wrapped!(ImagePipeSurfaceCreateFlagsFUCHSIA, Flags);

    impl fmt::Debug for ImagePipeSurfaceCreateFlagsFUCHSIA {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateImagePipeSurfaceFUCHSIA.html>
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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateImagePipeSurfaceFUCHSIA.html>
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
