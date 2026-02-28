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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub vertex_input_dynamic_state: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT,
                p_next: core::ptr::null_mut(),
                vertex_input_dynamic_state: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'a> {
        pub fn vertex_input_dynamic_state(mut self, vertex_input_dynamic_state: Bool32) -> Self {
            self.vertex_input_dynamic_state = vertex_input_dynamic_state;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VertexInputBindingDescription2EXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub binding: u32,
        pub stride: u32,
        pub input_rate: VertexInputRate,
        pub divisor: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for VertexInputBindingDescription2EXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT,
                p_next: core::ptr::null_mut(),
                binding: Default::default(),
                stride: Default::default(),
                input_rate: Default::default(),
                divisor: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VertexInputBindingDescription2EXT<'a> {
        pub fn binding(mut self, binding: u32) -> Self {
            self.binding = binding;
            self
        }
        pub fn stride(mut self, stride: u32) -> Self {
            self.stride = stride;
            self
        }
        pub fn input_rate(mut self, input_rate: VertexInputRate) -> Self {
            self.input_rate = input_rate;
            self
        }
        pub fn divisor(mut self, divisor: u32) -> Self {
            self.divisor = divisor;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VertexInputAttributeDescription2EXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub location: u32,
        pub binding: u32,
        pub format: Format,
        pub offset: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for VertexInputAttributeDescription2EXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT,
                p_next: core::ptr::null_mut(),
                location: Default::default(),
                binding: Default::default(),
                format: Default::default(),
                offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VertexInputAttributeDescription2EXT<'a> {
        pub fn location(mut self, location: u32) -> Self {
            self.location = location;
            self
        }
        pub fn binding(mut self, binding: u32) -> Self {
            self.binding = binding;
            self
        }
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
        pub fn offset(mut self, offset: u32) -> Self {
            self.offset = offset;
            self
        }
    }
    pub type PFN_vkCmdSetVertexInputEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        vertex_binding_description_count: u32,
        p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT<'_>,
        vertex_attribute_description_count: u32,
        p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT<'_>,
    );
}
pub struct DeviceFn {
    cmd_set_vertex_input_ext: PFN_vkCmdSetVertexInputEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_vertex_input_ext: transmute(
                    load(c"vkCmdSetVertexInputEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
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
