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
    pub type PhysicalDeviceRobustness2FeaturesEXT<'a> = PhysicalDeviceRobustness2FeaturesKHR<'a>;
    pub type PhysicalDeviceRobustness2PropertiesEXT<'a> =
        PhysicalDeviceRobustness2PropertiesKHR<'a>;
}
