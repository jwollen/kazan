#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    pub type QueueGlobalPriorityKHR = QueueGlobalPriority;
    pub type DeviceQueueGlobalPriorityCreateInfoKHR<'a> = DeviceQueueGlobalPriorityCreateInfo<'a>;
    pub type PhysicalDeviceGlobalPriorityQueryFeaturesKHR<'a> =
        PhysicalDeviceGlobalPriorityQueryFeatures<'a>;
    pub type QueueFamilyGlobalPriorityPropertiesKHR<'a> = QueueFamilyGlobalPriorityProperties<'a>;
}
