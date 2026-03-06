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
    pub struct DirectDriverLoadingInfoLUNARG<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: DirectDriverLoadingFlagsLUNARG,
        pub pfn_get_instance_proc_addr: Option<PFN_vkGetInstanceProcAddrLUNARG>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DirectDriverLoadingInfoLUNARG<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DIRECT_DRIVER_LOADING_INFO_LUNARG;
    }
    impl Default for DirectDriverLoadingInfoLUNARG<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                flags: Default::default(),
                pfn_get_instance_proc_addr: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DirectDriverLoadingInfoLUNARG<'a> {
        pub fn flags(mut self, flags: DirectDriverLoadingFlagsLUNARG) -> Self {
            self.flags = flags;
            self
        }
        pub fn pfn_get_instance_proc_addr(
            mut self,
            pfn_get_instance_proc_addr: PFN_vkGetInstanceProcAddrLUNARG,
        ) -> Self {
            self.pfn_get_instance_proc_addr = Some(pfn_get_instance_proc_addr);
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DirectDriverLoadingListLUNARG<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub mode: DirectDriverLoadingModeLUNARG,
        pub driver_count: u32,
        pub p_drivers: *const DirectDriverLoadingInfoLUNARG<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DirectDriverLoadingListLUNARG<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DIRECT_DRIVER_LOADING_LIST_LUNARG;
    }
    unsafe impl<'a> Extends<InstanceCreateInfo<'a>> for DirectDriverLoadingListLUNARG<'a> {}
    impl Default for DirectDriverLoadingListLUNARG<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                mode: Default::default(),
                driver_count: Default::default(),
                p_drivers: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DirectDriverLoadingListLUNARG<'a> {
        pub fn mode(mut self, mode: DirectDriverLoadingModeLUNARG) -> Self {
            self.mode = mode;
            self
        }
        pub fn drivers(mut self, drivers: &'a [DirectDriverLoadingInfoLUNARG<'a>]) -> Self {
            self.driver_count = drivers.len().try_into().unwrap();
            self.p_drivers = drivers.as_ptr();
            self
        }
    }
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
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DirectDriverLoadingFlagsLUNARG(Flags);
    vk_bitflags_wrapped!(DirectDriverLoadingFlagsLUNARG, Flags);
    pub type PFN_vkGetInstanceProcAddrLUNARG =
        unsafe extern "system" fn(instance: Instance, p_name: *const c_char) -> PFN_vkVoidFunction;
}
