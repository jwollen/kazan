#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_memory_budget";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMemoryBudgetPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMemoryBudgetPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub heap_budget: [DeviceSize; MAX_MEMORY_HEAPS as usize],
        pub heap_usage: [DeviceSize; MAX_MEMORY_HEAPS as usize],
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMemoryBudgetPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMemoryBudgetPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("heap_budget", &self.heap_budget)
                .field("heap_usage", &self.heap_usage)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMemoryBudgetPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceMemoryProperties2<'a>>
        for PhysicalDeviceMemoryBudgetPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceMemoryBudgetPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                heap_budget: [Default::default(); _],
                heap_usage: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMemoryBudgetPropertiesEXT<'a> {
        pub fn heap_budget(mut self, heap_budget: [DeviceSize; MAX_MEMORY_HEAPS as usize]) -> Self {
            self.heap_budget = heap_budget;
            self
        }

        pub fn heap_usage(mut self, heap_usage: [DeviceSize; MAX_MEMORY_HEAPS as usize]) -> Self {
            self.heap_usage = heap_usage;
            self
        }
    }
}
