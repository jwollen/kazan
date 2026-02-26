#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type QueueGlobalPriorityKHR = QueueGlobalPriority;
pub type DeviceQueueGlobalPriorityCreateInfoKHR<'a> = DeviceQueueGlobalPriorityCreateInfo<'a>;
pub type PhysicalDeviceGlobalPriorityQueryFeaturesKHR<'a> =
    PhysicalDeviceGlobalPriorityQueryFeatures<'a>;
pub type QueueFamilyGlobalPriorityPropertiesKHR<'a> = QueueFamilyGlobalPriorityProperties<'a>;
