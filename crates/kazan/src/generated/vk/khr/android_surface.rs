#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_android_surface";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    pub type ANativeWindow = *const c_void;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAndroidSurfaceCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AndroidSurfaceCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: AndroidSurfaceCreateFlagsKHR,
        pub window: *mut ANativeWindow,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AndroidSurfaceCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AndroidSurfaceCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("window", &self.window)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AndroidSurfaceCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ANDROID_SURFACE_CREATE_INFO_KHR;
    }

    impl Default for AndroidSurfaceCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                window: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AndroidSurfaceCreateInfoKHR<'a> {
        pub fn flags(mut self, flags: AndroidSurfaceCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        pub fn window(mut self, window: *mut ANativeWindow) -> Self {
            self.window = window;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAndroidSurfaceCreateFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct AndroidSurfaceCreateFlagsKHR(Flags);
    vk_bitflags_wrapped!(AndroidSurfaceCreateFlagsKHR, Flags);

    impl fmt::Debug for AndroidSurfaceCreateFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateAndroidSurfaceKHR.html>
    pub type PFN_vkCreateAndroidSurfaceKHR = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const AndroidSurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
}

pub struct InstanceFn {
    create_android_surface_khr: PFN_vkCreateAndroidSurfaceKHR,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_android_surface_khr: transmute(
                    load(c"vkCreateAndroidSurfaceKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateAndroidSurfaceKHR.html>
    pub unsafe fn create_android_surface_khr(
        &self,
        instance: Instance,
        create_info: &AndroidSurfaceCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_android_surface_khr)(
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
