#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct MultiDrawInfoEXT {
        pub first_vertex: u32,
        pub vertex_count: u32,
    }
    impl MultiDrawInfoEXT {
        pub fn first_vertex(mut self, first_vertex: u32) -> Self {
            self.first_vertex = first_vertex;
            self
        }
        pub fn vertex_count(mut self, vertex_count: u32) -> Self {
            self.vertex_count = vertex_count;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct MultiDrawIndexedInfoEXT {
        pub first_index: u32,
        pub index_count: u32,
        pub vertex_offset: i32,
    }
    impl MultiDrawIndexedInfoEXT {
        pub fn first_index(mut self, first_index: u32) -> Self {
            self.first_index = first_index;
            self
        }
        pub fn index_count(mut self, index_count: u32) -> Self {
            self.index_count = index_count;
            self
        }
        pub fn vertex_offset(mut self, vertex_offset: i32) -> Self {
            self.vertex_offset = vertex_offset;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMultiDrawPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_multi_draw_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMultiDrawPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceMultiDrawPropertiesEXT<'a>
    {
    }
    impl Default for PhysicalDeviceMultiDrawPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_multi_draw_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMultiDrawPropertiesEXT<'a> {
        pub fn max_multi_draw_count(mut self, max_multi_draw_count: u32) -> Self {
            self.max_multi_draw_count = max_multi_draw_count;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMultiDrawFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub multi_draw: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMultiDrawFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceMultiDrawFeaturesEXT<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceMultiDrawFeaturesEXT<'a> {}
    impl Default for PhysicalDeviceMultiDrawFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                multi_draw: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMultiDrawFeaturesEXT<'a> {
        pub fn multi_draw(mut self, multi_draw: bool) -> Self {
            self.multi_draw = multi_draw.into();
            self
        }
    }
    pub type PFN_vkCmdDrawMultiEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        draw_count: u32,
        p_vertex_info: *const MultiDrawInfoEXT,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    );
    pub type PFN_vkCmdDrawMultiIndexedEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        draw_count: u32,
        p_index_info: *const MultiDrawIndexedInfoEXT,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        p_vertex_offset: *const i32,
    );
}
pub struct DeviceFn {
    cmd_draw_multi_ext: PFN_vkCmdDrawMultiEXT,
    cmd_draw_multi_indexed_ext: PFN_vkCmdDrawMultiIndexedEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_draw_multi_ext: transmute(
                    load(c"vkCmdDrawMultiEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_draw_multi_indexed_ext: transmute(
                    load(c"vkCmdDrawMultiIndexedEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_draw_multi_ext(
        &self,
        command_buffer: CommandBuffer,
        vertex_info: &[MultiDrawInfoEXT],
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_multi_ext)(
                command_buffer,
                vertex_info.len().try_into().unwrap(),
                vertex_info.as_ptr() as _,
                instance_count,
                first_instance,
                stride,
            )
        }
    }
    pub unsafe fn cmd_draw_multi_indexed_ext(
        &self,
        command_buffer: CommandBuffer,
        index_info: &[MultiDrawIndexedInfoEXT],
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        vertex_offset: Option<&i32>,
    ) {
        unsafe {
            (self.cmd_draw_multi_indexed_ext)(
                command_buffer,
                index_info.len().try_into().unwrap(),
                index_info.as_ptr() as _,
                instance_count,
                first_instance,
                stride,
                vertex_offset.to_raw_ptr(),
            )
        }
    }
}
