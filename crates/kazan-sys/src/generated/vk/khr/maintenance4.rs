#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type DeviceBufferMemoryRequirementsKHR = DeviceBufferMemoryRequirements;
pub type DeviceImageMemoryRequirementsKHR = DeviceImageMemoryRequirements;
pub type PhysicalDeviceMaintenance4FeaturesKHR = PhysicalDeviceMaintenance4Features;
pub type PhysicalDeviceMaintenance4PropertiesKHR = PhysicalDeviceMaintenance4Properties;
pub type PFN_vkGetDeviceBufferMemoryRequirementsKHR = PFN_vkGetDeviceBufferMemoryRequirements;
pub type PFN_vkGetDeviceImageMemoryRequirementsKHR = PFN_vkGetDeviceImageMemoryRequirements;
pub type PFN_vkGetDeviceImageSparseMemoryRequirementsKHR =
    PFN_vkGetDeviceImageSparseMemoryRequirements;
