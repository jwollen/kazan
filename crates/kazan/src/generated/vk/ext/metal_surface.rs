#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_metal_surface";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    pub type CAMetalLayer = *const c_void;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMetalSurfaceCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MetalSurfaceCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: MetalSurfaceCreateFlagsEXT,
        pub p_layer: *const CAMetalLayer,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MetalSurfaceCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MetalSurfaceCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("p_layer", &self.p_layer)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MetalSurfaceCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::METAL_SURFACE_CREATE_INFO_EXT;
    }

    impl Default for MetalSurfaceCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                p_layer: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MetalSurfaceCreateInfoEXT<'a> {
        pub fn flags(mut self, flags: MetalSurfaceCreateFlagsEXT) -> Self {
            self.flags = flags;
            self
        }

        pub fn layer(mut self, layer: *const CAMetalLayer) -> Self {
            self.p_layer = layer;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMetalSurfaceCreateFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MetalSurfaceCreateFlagsEXT(Flags);
    vk_bitflags_wrapped!(MetalSurfaceCreateFlagsEXT, Flags);

    impl fmt::Debug for MetalSurfaceCreateFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateMetalSurfaceEXT.html>
    pub type PFN_vkCreateMetalSurfaceEXT = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const MetalSurfaceCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
}

pub struct InstanceFn {
    create_metal_surface_ext: PFN_vkCreateMetalSurfaceEXT,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_metal_surface_ext: transmute(
                    load(c"vkCreateMetalSurfaceEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateMetalSurfaceEXT.html>
    pub unsafe fn create_metal_surface_ext(
        &self,
        instance: Instance,
        create_info: &MetalSurfaceCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_metal_surface_ext)(
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
