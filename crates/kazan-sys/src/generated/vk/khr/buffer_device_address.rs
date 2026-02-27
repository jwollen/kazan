#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PhysicalDeviceBufferDeviceAddressFeaturesKHR<'a> =
    PhysicalDeviceBufferDeviceAddressFeatures<'a>;
pub type BufferDeviceAddressInfoKHR<'a> = BufferDeviceAddressInfo<'a>;
pub type BufferOpaqueCaptureAddressCreateInfoKHR<'a> = BufferOpaqueCaptureAddressCreateInfo<'a>;
pub type MemoryOpaqueCaptureAddressAllocateInfoKHR<'a> = MemoryOpaqueCaptureAddressAllocateInfo<'a>;
pub type DeviceMemoryOpaqueCaptureAddressInfoKHR<'a> = DeviceMemoryOpaqueCaptureAddressInfo<'a>;
pub type PFN_vkGetBufferOpaqueCaptureAddressKHR = PFN_vkGetBufferOpaqueCaptureAddress;
pub type PFN_vkGetBufferDeviceAddressKHR = PFN_vkGetBufferDeviceAddress;
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR = PFN_vkGetDeviceMemoryOpaqueCaptureAddress;
