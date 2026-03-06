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
    pub type OHNativeWindow = *const c_void;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SurfaceCreateInfoOHOS<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: SurfaceCreateFlagsOHOS,
        pub window: *mut OHNativeWindow,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SurfaceCreateInfoOHOS<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SURFACE_CREATE_INFO_OHOS;
    }
    impl Default for SurfaceCreateInfoOHOS<'_> {
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
    impl<'a> SurfaceCreateInfoOHOS<'a> {
        pub fn flags(mut self, flags: SurfaceCreateFlagsOHOS) -> Self {
            self.flags = flags;
            self
        }
        pub fn window(mut self, window: &'a mut OHNativeWindow) -> Self {
            self.window = window;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SurfaceCreateFlagsOHOS(Flags);
    vk_bitflags_wrapped!(SurfaceCreateFlagsOHOS, Flags);
    impl fmt::Debug for SurfaceCreateFlagsOHOS {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }
    pub type PFN_vkCreateSurfaceOHOS = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const SurfaceCreateInfoOHOS<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
}
pub struct InstanceFn {
    create_surface_ohos: PFN_vkCreateSurfaceOHOS,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_surface_ohos: transmute(
                    load(c"vkCreateSurfaceOHOS").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn create_surface_ohos(
        &self,
        instance: Instance,
        create_info: &SurfaceCreateInfoOHOS<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_surface_ohos)(
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
