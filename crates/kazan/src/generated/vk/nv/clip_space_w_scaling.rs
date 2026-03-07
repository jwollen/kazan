#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_clip_space_w_scaling";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkViewportWScalingNV.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct ViewportWScalingNV {
        pub xcoeff: f32,
        pub ycoeff: f32,
    }

    impl ViewportWScalingNV {
        pub fn xcoeff(mut self, xcoeff: f32) -> Self {
            self.xcoeff = xcoeff;
            self
        }

        pub fn ycoeff(mut self, ycoeff: f32) -> Self {
            self.ycoeff = ycoeff;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineViewportWScalingStateCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineViewportWScalingStateCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub viewport_w_scaling_enable: Bool32,
        pub viewport_count: u32,
        pub p_viewport_w_scalings: *const ViewportWScalingNV,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PipelineViewportWScalingStateCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineViewportWScalingStateCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("viewport_w_scaling_enable", &self.viewport_w_scaling_enable)
                .field("viewport_count", &self.viewport_count)
                .field("p_viewport_w_scalings", &self.p_viewport_w_scalings)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineViewportWScalingStateCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV;
    }

    unsafe impl<'a> Extends<PipelineViewportStateCreateInfo<'a>>
        for PipelineViewportWScalingStateCreateInfoNV<'a>
    {
    }

    impl Default for PipelineViewportWScalingStateCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                viewport_w_scaling_enable: Default::default(),
                viewport_count: Default::default(),
                p_viewport_w_scalings: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineViewportWScalingStateCreateInfoNV<'a> {
        pub fn viewport_w_scaling_enable(mut self, viewport_w_scaling_enable: bool) -> Self {
            self.viewport_w_scaling_enable = viewport_w_scaling_enable.into();
            self
        }

        pub fn viewport_w_scalings(
            mut self,
            viewport_w_scalings: &'a [ViewportWScalingNV],
        ) -> Self {
            self.viewport_count = viewport_w_scalings.len().try_into().unwrap();
            self.p_viewport_w_scalings = viewport_w_scalings.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetViewportWScalingNV.html>
    pub type PFN_vkCmdSetViewportWScalingNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewport_w_scalings: *const ViewportWScalingNV,
    );
}

pub struct DeviceFn {
    cmd_set_viewport_w_scaling_nv: PFN_vkCmdSetViewportWScalingNV,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_viewport_w_scaling_nv: transmute(
                    load(c"vkCmdSetViewportWScalingNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetViewportWScalingNV.html>
    pub unsafe fn cmd_set_viewport_w_scaling_nv(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_w_scalings: &[ViewportWScalingNV],
    ) {
        unsafe {
            (self.cmd_set_viewport_w_scaling_nv)(
                command_buffer,
                first_viewport,
                viewport_w_scalings.len().try_into().unwrap(),
                viewport_w_scalings.as_ptr() as _,
            )
        }
    }
}
