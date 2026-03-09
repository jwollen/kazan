//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_compute_shader_derivatives.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_compute_shader_derivatives";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceComputeShaderDerivativesFeaturesNV.html>
    pub type PhysicalDeviceComputeShaderDerivativesFeaturesNV<'a> =
        PhysicalDeviceComputeShaderDerivativesFeaturesKHR<'a>;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceComputeShaderDerivativesFeaturesNV =
        PhysicalDeviceComputeShaderDerivativesFeaturesNV<'static>;
}
