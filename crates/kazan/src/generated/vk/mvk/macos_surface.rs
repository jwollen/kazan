//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_MVK_macos_surface.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_MVK_macos_surface";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMacOSSurfaceCreateInfoMVK.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MacOSSurfaceCreateInfoMVK<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: MacOSSurfaceCreateFlagsMVK,
        pub p_view: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MacOSSurfaceCreateInfoMVK<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MacOSSurfaceCreateInfoMVK")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("p_view", &self.p_view)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MacOSSurfaceCreateInfoMVK<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MACOS_SURFACE_CREATE_INFO_MVK;
    }

    impl Default for MacOSSurfaceCreateInfoMVK<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                p_view: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MacOSSurfaceCreateInfoMVK<'a> {
        #[inline]
        pub fn flags(mut self, flags: MacOSSurfaceCreateFlagsMVK) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn view(mut self, view: *const c_void) -> Self {
            self.p_view = view;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMacOSSurfaceCreateFlagsMVK.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MacOSSurfaceCreateFlagsMVK(Flags);
    vk_bitflags_wrapped!(MacOSSurfaceCreateFlagsMVK, Flags);

    impl fmt::Debug for MacOSSurfaceCreateFlagsMVK {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateMacOSSurfaceMVK.html>
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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateMacOSSurfaceMVK.html>
    #[inline]
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
