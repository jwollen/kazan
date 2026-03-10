//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_vertex_input_dynamic_state.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_vertex_input_dynamic_state";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub vertex_input_dynamic_state: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVertexInputDynamicStateFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "vertex_input_dynamic_state",
                    &self.vertex_input_dynamic_state,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                vertex_input_dynamic_state: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'a> {
        #[inline]
        pub fn vertex_input_dynamic_state(mut self, vertex_input_dynamic_state: bool) -> Self {
            self.vertex_input_dynamic_state = vertex_input_dynamic_state.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVertexInputBindingDescription2EXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VertexInputBindingDescription2EXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub binding: u32,
        pub stride: u32,
        pub input_rate: VertexInputRate,
        pub divisor: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VertexInputBindingDescription2EXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VertexInputBindingDescription2EXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("binding", &self.binding)
                .field("stride", &self.stride)
                .field("input_rate", &self.input_rate)
                .field("divisor", &self.divisor)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VertexInputBindingDescription2EXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT;
    }

    impl Default for VertexInputBindingDescription2EXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                binding: Default::default(),
                stride: Default::default(),
                input_rate: Default::default(),
                divisor: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VertexInputBindingDescription2EXT<'a> {
        #[inline]
        pub fn binding(mut self, binding: u32) -> Self {
            self.binding = binding;
            self
        }

        #[inline]
        pub fn stride(mut self, stride: u32) -> Self {
            self.stride = stride;
            self
        }

        #[inline]
        pub fn input_rate(mut self, input_rate: VertexInputRate) -> Self {
            self.input_rate = input_rate;
            self
        }

        #[inline]
        pub fn divisor(mut self, divisor: u32) -> Self {
            self.divisor = divisor;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVertexInputAttributeDescription2EXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VertexInputAttributeDescription2EXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub location: u32,
        pub binding: u32,
        pub format: Format,
        pub offset: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VertexInputAttributeDescription2EXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VertexInputAttributeDescription2EXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("location", &self.location)
                .field("binding", &self.binding)
                .field("format", &self.format)
                .field("offset", &self.offset)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VertexInputAttributeDescription2EXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT;
    }

    impl Default for VertexInputAttributeDescription2EXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                location: Default::default(),
                binding: Default::default(),
                format: Default::default(),
                offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VertexInputAttributeDescription2EXT<'a> {
        #[inline]
        pub fn location(mut self, location: u32) -> Self {
            self.location = location;
            self
        }

        #[inline]
        pub fn binding(mut self, binding: u32) -> Self {
            self.binding = binding;
            self
        }

        #[inline]
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }

        #[inline]
        pub fn offset(mut self, offset: u32) -> Self {
            self.offset = offset;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetVertexInputEXT.html>
    pub type PFN_vkCmdSetVertexInputEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        vertex_binding_description_count: u32,
        p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT<'_>,
        vertex_attribute_description_count: u32,
        p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT =
        PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'static>;
    pub type VkVertexInputBindingDescription2EXT = VertexInputBindingDescription2EXT<'static>;
    pub type VkVertexInputAttributeDescription2EXT = VertexInputAttributeDescription2EXT<'static>;
    impl PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VertexInputBindingDescription2EXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVertexInputBindingDescription2EXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VertexInputAttributeDescription2EXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVertexInputAttributeDescription2EXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    cmd_set_vertex_input_ext: PFN_vkCmdSetVertexInputEXT,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_vertex_input_ext: transmute(
                    load(c"vkCmdSetVertexInputEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetVertexInputEXT.html>
    #[inline]
    pub unsafe fn cmd_set_vertex_input_ext(
        &self,
        command_buffer: CommandBuffer,
        vertex_binding_descriptions: &[VertexInputBindingDescription2EXT<'_>],
        vertex_attribute_descriptions: &[VertexInputAttributeDescription2EXT<'_>],
    ) {
        unsafe {
            (self.cmd_set_vertex_input_ext)(
                command_buffer,
                vertex_binding_descriptions.len().try_into().unwrap(),
                vertex_binding_descriptions.as_ptr() as _,
                vertex_attribute_descriptions.len().try_into().unwrap(),
                vertex_attribute_descriptions.as_ptr() as _,
            )
        }
    }
}
