#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type PhysicalDeviceMutableDescriptorTypeFeaturesVALVE<'a> =
    PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'a>;
pub type MutableDescriptorTypeListVALVE<'a> = MutableDescriptorTypeListEXT<'a>;
pub type MutableDescriptorTypeCreateInfoVALVE<'a> = MutableDescriptorTypeCreateInfoEXT<'a>;
