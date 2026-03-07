//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_depth_stencil_resolve.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_depth_stencil_resolve";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDepthStencilResolvePropertiesKHR.html>
    pub type PhysicalDeviceDepthStencilResolvePropertiesKHR<'a> =
        PhysicalDeviceDepthStencilResolveProperties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubpassDescriptionDepthStencilResolveKHR.html>
    pub type SubpassDescriptionDepthStencilResolveKHR<'a> =
        SubpassDescriptionDepthStencilResolve<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkResolveModeFlagsKHR.html>
    pub type ResolveModeFlagsKHR = ResolveModeFlags;
}
