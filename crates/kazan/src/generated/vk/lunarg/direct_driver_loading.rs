//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_LUNARG_direct_driver_loading.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_LUNARG_direct_driver_loading";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDirectDriverLoadingInfoLUNARG.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DirectDriverLoadingInfoLUNARG<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: DirectDriverLoadingFlagsLUNARG,
        pub pfn_get_instance_proc_addr: Option<PFN_vkGetInstanceProcAddrLUNARG>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DirectDriverLoadingInfoLUNARG<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DirectDriverLoadingInfoLUNARG")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field(
                    "pfn_get_instance_proc_addr",
                    &self.pfn_get_instance_proc_addr.map(|f| f as *const ()),
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DirectDriverLoadingInfoLUNARG<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DIRECT_DRIVER_LOADING_INFO_LUNARG;
    }

    impl Default for DirectDriverLoadingInfoLUNARG<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                flags: Default::default(),
                pfn_get_instance_proc_addr: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DirectDriverLoadingInfoLUNARG<'a> {
        #[inline]
        pub fn flags(mut self, flags: DirectDriverLoadingFlagsLUNARG) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn pfn_get_instance_proc_addr(
            mut self,
            pfn_get_instance_proc_addr: PFN_vkGetInstanceProcAddrLUNARG,
        ) -> Self {
            self.pfn_get_instance_proc_addr = Some(pfn_get_instance_proc_addr);
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDirectDriverLoadingListLUNARG.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DirectDriverLoadingListLUNARG<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub mode: DirectDriverLoadingModeLUNARG,
        pub driver_count: u32,
        pub p_drivers: *const DirectDriverLoadingInfoLUNARG<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DirectDriverLoadingListLUNARG<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DirectDriverLoadingListLUNARG")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("mode", &self.mode)
                .field("driver_count", &self.driver_count)
                .field("p_drivers", &self.p_drivers)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DirectDriverLoadingListLUNARG<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DIRECT_DRIVER_LOADING_LIST_LUNARG;
    }

    unsafe impl Extends<InstanceCreateInfo<'_>> for DirectDriverLoadingListLUNARG<'_> {}

    impl Default for DirectDriverLoadingListLUNARG<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                mode: Default::default(),
                driver_count: Default::default(),
                p_drivers: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DirectDriverLoadingListLUNARG<'a> {
        #[inline]
        pub fn mode(mut self, mode: DirectDriverLoadingModeLUNARG) -> Self {
            self.mode = mode;
            self
        }

        #[inline]
        pub fn drivers(mut self, drivers: &'a [DirectDriverLoadingInfoLUNARG<'_>]) -> Self {
            self.driver_count = drivers.len().try_into().unwrap();
            self.p_drivers = drivers.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDirectDriverLoadingModeLUNARG.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DirectDriverLoadingModeLUNARG(i32);

    impl DirectDriverLoadingModeLUNARG {
        pub const EXCLUSIVE_LUNARG: Self = Self(0);
        pub const INCLUSIVE_LUNARG: Self = Self(1);
    }

    impl fmt::Debug for DirectDriverLoadingModeLUNARG {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::EXCLUSIVE_LUNARG => Some("EXCLUSIVE_LUNARG"),
                Self::INCLUSIVE_LUNARG => Some("INCLUSIVE_LUNARG"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDirectDriverLoadingFlagsLUNARG.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DirectDriverLoadingFlagsLUNARG(Flags);
    vk_bitflags_wrapped!(DirectDriverLoadingFlagsLUNARG, Flags);

    impl fmt::Debug for DirectDriverLoadingFlagsLUNARG {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/PFN_vkGetInstanceProcAddrLUNARG.html>
    pub type PFN_vkGetInstanceProcAddrLUNARG =
        unsafe extern "system" fn(instance: Instance, p_name: *const c_char) -> PFN_vkVoidFunction;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkDirectDriverLoadingInfoLUNARG = DirectDriverLoadingInfoLUNARG<'static>;
    pub type VkDirectDriverLoadingListLUNARG = DirectDriverLoadingListLUNARG<'static>;
    pub type VkDirectDriverLoadingModeLUNARG = DirectDriverLoadingModeLUNARG;
    pub type VkDirectDriverLoadingFlagsLUNARG = DirectDriverLoadingFlagsLUNARG;
    impl DirectDriverLoadingInfoLUNARG<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDirectDriverLoadingInfoLUNARG {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DirectDriverLoadingListLUNARG<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDirectDriverLoadingListLUNARG {
            unsafe { core::mem::transmute(self) }
        }
    }
}
