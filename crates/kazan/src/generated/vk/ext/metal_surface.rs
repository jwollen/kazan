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
    pub type CAMetalLayer = *const c_void;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MetalSurfaceCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: MetalSurfaceCreateFlagsEXT,
        pub p_layer: *const CAMetalLayer,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for MetalSurfaceCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::METAL_SURFACE_CREATE_INFO_EXT,
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
    bitflags! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct MetalSurfaceCreateFlagsEXT: Flags {
        }
    }
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
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_metal_surface_ext: transmute(
                    load(c"vkCreateMetalSurfaceEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
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
