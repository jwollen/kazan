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
    pub type PFN_vkCmdSetCullModeEXT = PFN_vkCmdSetCullMode;
    pub type PFN_vkCmdSetFrontFaceEXT = PFN_vkCmdSetFrontFace;
    pub type PFN_vkCmdSetPrimitiveTopologyEXT = PFN_vkCmdSetPrimitiveTopology;
    pub type PFN_vkCmdSetViewportWithCountEXT = PFN_vkCmdSetViewportWithCount;
    pub type PFN_vkCmdSetScissorWithCountEXT = PFN_vkCmdSetScissorWithCount;
    pub type PFN_vkCmdBindVertexBuffers2EXT = PFN_vkCmdBindVertexBuffers2;
    pub type PFN_vkCmdSetDepthTestEnableEXT = PFN_vkCmdSetDepthTestEnable;
    pub type PFN_vkCmdSetDepthWriteEnableEXT = PFN_vkCmdSetDepthWriteEnable;
    pub type PFN_vkCmdSetDepthCompareOpEXT = PFN_vkCmdSetDepthCompareOp;
    pub type PFN_vkCmdSetDepthBoundsTestEnableEXT = PFN_vkCmdSetDepthBoundsTestEnable;
    pub type PFN_vkCmdSetStencilTestEnableEXT = PFN_vkCmdSetStencilTestEnable;
    pub type PFN_vkCmdSetStencilOpEXT = PFN_vkCmdSetStencilOp;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub extended_dynamic_state: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceExtendedDynamicStateFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT,
                p_next: core::ptr::null_mut(),
                extended_dynamic_state: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceExtendedDynamicStateFeaturesEXT<'a> {
        pub fn extended_dynamic_state(mut self, extended_dynamic_state: Bool32) -> Self {
            self.extended_dynamic_state = extended_dynamic_state;
            self
        }
    }
}
pub struct DeviceFn {
    cmd_set_cull_mode_ext: PFN_vkCmdSetCullMode,
    cmd_set_front_face_ext: PFN_vkCmdSetFrontFace,
    cmd_set_primitive_topology_ext: PFN_vkCmdSetPrimitiveTopology,
    cmd_set_viewport_with_count_ext: PFN_vkCmdSetViewportWithCount,
    cmd_set_scissor_with_count_ext: PFN_vkCmdSetScissorWithCount,
    cmd_bind_vertex_buffers2_ext: PFN_vkCmdBindVertexBuffers2,
    cmd_set_depth_test_enable_ext: PFN_vkCmdSetDepthTestEnable,
    cmd_set_depth_write_enable_ext: PFN_vkCmdSetDepthWriteEnable,
    cmd_set_depth_compare_op_ext: PFN_vkCmdSetDepthCompareOp,
    cmd_set_depth_bounds_test_enable_ext: PFN_vkCmdSetDepthBoundsTestEnable,
    cmd_set_stencil_test_enable_ext: PFN_vkCmdSetStencilTestEnable,
    cmd_set_stencil_op_ext: PFN_vkCmdSetStencilOp,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_cull_mode_ext: transmute(load(c"vkCmdSetCullModeEXT").ok_or(LoadingError)?),
                cmd_set_front_face_ext: transmute(
                    load(c"vkCmdSetFrontFaceEXT").ok_or(LoadingError)?,
                ),
                cmd_set_primitive_topology_ext: transmute(
                    load(c"vkCmdSetPrimitiveTopologyEXT").ok_or(LoadingError)?,
                ),
                cmd_set_viewport_with_count_ext: transmute(
                    load(c"vkCmdSetViewportWithCountEXT").ok_or(LoadingError)?,
                ),
                cmd_set_scissor_with_count_ext: transmute(
                    load(c"vkCmdSetScissorWithCountEXT").ok_or(LoadingError)?,
                ),
                cmd_bind_vertex_buffers2_ext: transmute(
                    load(c"vkCmdBindVertexBuffers2EXT").ok_or(LoadingError)?,
                ),
                cmd_set_depth_test_enable_ext: transmute(
                    load(c"vkCmdSetDepthTestEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_depth_write_enable_ext: transmute(
                    load(c"vkCmdSetDepthWriteEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_depth_compare_op_ext: transmute(
                    load(c"vkCmdSetDepthCompareOpEXT").ok_or(LoadingError)?,
                ),
                cmd_set_depth_bounds_test_enable_ext: transmute(
                    load(c"vkCmdSetDepthBoundsTestEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_stencil_test_enable_ext: transmute(
                    load(c"vkCmdSetStencilTestEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_stencil_op_ext: transmute(
                    load(c"vkCmdSetStencilOpEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_cull_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        cull_mode: CullModeFlags,
    ) {
        unsafe { (self.cmd_set_cull_mode_ext)(command_buffer, cull_mode) }
    }
    pub unsafe fn cmd_set_front_face_ext(
        &self,
        command_buffer: CommandBuffer,
        front_face: FrontFace,
    ) {
        unsafe { (self.cmd_set_front_face_ext)(command_buffer, front_face) }
    }
    pub unsafe fn cmd_set_primitive_topology_ext(
        &self,
        command_buffer: CommandBuffer,
        primitive_topology: PrimitiveTopology,
    ) {
        unsafe { (self.cmd_set_primitive_topology_ext)(command_buffer, primitive_topology) }
    }
    pub unsafe fn cmd_set_viewport_with_count_ext(
        &self,
        command_buffer: CommandBuffer,
        viewports: &[Viewport],
    ) {
        unsafe {
            (self.cmd_set_viewport_with_count_ext)(
                command_buffer,
                viewports.len().try_into().unwrap(),
                viewports.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_scissor_with_count_ext(
        &self,
        command_buffer: CommandBuffer,
        scissors: &[Rect2D],
    ) {
        unsafe {
            (self.cmd_set_scissor_with_count_ext)(
                command_buffer,
                scissors.len().try_into().unwrap(),
                scissors.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_bind_vertex_buffers2_ext(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
        sizes: Option<&[DeviceSize]>,
        strides: Option<&[DeviceSize]>,
    ) {
        unsafe {
            (self.cmd_bind_vertex_buffers2_ext)(
                command_buffer,
                first_binding,
                buffers.len().try_into().unwrap(),
                buffers.as_ptr() as _,
                offsets.as_ptr() as _,
                sizes.to_raw_ptr(),
                strides.to_raw_ptr(),
            )
        }
    }
    pub unsafe fn cmd_set_depth_test_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_test_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_test_enable_ext)(command_buffer, depth_test_enable) }
    }
    pub unsafe fn cmd_set_depth_write_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_write_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_write_enable_ext)(command_buffer, depth_write_enable) }
    }
    pub unsafe fn cmd_set_depth_compare_op_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_compare_op: CompareOp,
    ) {
        unsafe { (self.cmd_set_depth_compare_op_ext)(command_buffer, depth_compare_op) }
    }
    pub unsafe fn cmd_set_depth_bounds_test_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_bounds_test_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_depth_bounds_test_enable_ext)(command_buffer, depth_bounds_test_enable)
        }
    }
    pub unsafe fn cmd_set_stencil_test_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        stencil_test_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_stencil_test_enable_ext)(command_buffer, stencil_test_enable) }
    }
    pub unsafe fn cmd_set_stencil_op_ext(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) {
        unsafe {
            (self.cmd_set_stencil_op_ext)(
                command_buffer,
                face_mask,
                fail_op,
                pass_op,
                depth_fail_op,
                compare_op,
            )
        }
    }
}
