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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePCIBusInfoPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePCIBusInfoPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pci_domain: u32,
        pub pci_bus: u32,
        pub pci_device: u32,
        pub pci_function: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePCIBusInfoPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDevicePCIBusInfoPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDevicePCIBusInfoPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                pci_domain: Default::default(),
                pci_bus: Default::default(),
                pci_device: Default::default(),
                pci_function: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePCIBusInfoPropertiesEXT<'a> {
        pub fn pci_domain(mut self, pci_domain: u32) -> Self {
            self.pci_domain = pci_domain;
            self
        }

        pub fn pci_bus(mut self, pci_bus: u32) -> Self {
            self.pci_bus = pci_bus;
            self
        }

        pub fn pci_device(mut self, pci_device: u32) -> Self {
            self.pci_device = pci_device;
            self
        }

        pub fn pci_function(mut self, pci_function: u32) -> Self {
            self.pci_function = pci_function;
            self
        }
    }
}
