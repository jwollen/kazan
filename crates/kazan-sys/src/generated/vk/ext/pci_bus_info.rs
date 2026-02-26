#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
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
impl Default for PhysicalDevicePCIBusInfoPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            pci_domain: Default::default(),
            pci_bus: Default::default(),
            pci_device: Default::default(),
            pci_function: Default::default(),
            _marker: PhantomData,
        }
    }
}
