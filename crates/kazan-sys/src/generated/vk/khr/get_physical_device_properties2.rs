#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PhysicalDeviceFeatures2KHR<'a> = PhysicalDeviceFeatures2<'a>;
pub type PhysicalDeviceProperties2KHR<'a> = PhysicalDeviceProperties2<'a>;
pub type FormatProperties2KHR<'a> = FormatProperties2<'a>;
pub type ImageFormatProperties2KHR<'a> = ImageFormatProperties2<'a>;
pub type PhysicalDeviceImageFormatInfo2KHR<'a> = PhysicalDeviceImageFormatInfo2<'a>;
pub type QueueFamilyProperties2KHR<'a> = QueueFamilyProperties2<'a>;
pub type PhysicalDeviceMemoryProperties2KHR<'a> = PhysicalDeviceMemoryProperties2<'a>;
pub type SparseImageFormatProperties2KHR<'a> = SparseImageFormatProperties2<'a>;
pub type PhysicalDeviceSparseImageFormatInfo2KHR<'a> = PhysicalDeviceSparseImageFormatInfo2<'a>;
pub type PFN_vkGetPhysicalDeviceFeatures2KHR = PFN_vkGetPhysicalDeviceFeatures2;
pub type PFN_vkGetPhysicalDeviceProperties2KHR = PFN_vkGetPhysicalDeviceProperties2;
pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR = PFN_vkGetPhysicalDeviceFormatProperties2;
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR =
    PFN_vkGetPhysicalDeviceImageFormatProperties2;
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR =
    PFN_vkGetPhysicalDeviceQueueFamilyProperties2;
pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = PFN_vkGetPhysicalDeviceMemoryProperties2;
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR =
    PFN_vkGetPhysicalDeviceSparseImageFormatProperties2;
