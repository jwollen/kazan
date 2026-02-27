#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PhysicalDeviceGlobalPriorityQueryFeaturesEXT<'a> =
    PhysicalDeviceGlobalPriorityQueryFeatures<'a>;
pub type QueueFamilyGlobalPriorityPropertiesEXT<'a> = QueueFamilyGlobalPriorityProperties<'a>;
