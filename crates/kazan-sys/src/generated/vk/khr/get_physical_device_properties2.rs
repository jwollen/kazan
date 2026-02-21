#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type PhysicalDeviceFeatures2KHR = PhysicalDeviceFeatures2;
pub type PhysicalDeviceProperties2KHR = PhysicalDeviceProperties2;
pub type FormatProperties2KHR = FormatProperties2;
pub type ImageFormatProperties2KHR = ImageFormatProperties2;
pub type PhysicalDeviceImageFormatInfo2KHR = PhysicalDeviceImageFormatInfo2;
pub type QueueFamilyProperties2KHR = QueueFamilyProperties2;
pub type PhysicalDeviceMemoryProperties2KHR = PhysicalDeviceMemoryProperties2;
pub type SparseImageFormatProperties2KHR = SparseImageFormatProperties2;
pub type PhysicalDeviceSparseImageFormatInfo2KHR = PhysicalDeviceSparseImageFormatInfo2;
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
