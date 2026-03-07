#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_extended_dynamic_state2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    pub type PFN_vkCmdSetRasterizerDiscardEnableEXT = PFN_vkCmdSetRasterizerDiscardEnable;
    pub type PFN_vkCmdSetDepthBiasEnableEXT = PFN_vkCmdSetDepthBiasEnable;
    pub type PFN_vkCmdSetPrimitiveRestartEnableEXT = PFN_vkCmdSetPrimitiveRestartEnable;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceExtendedDynamicState2FeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub extended_dynamic_state2: Bool32,
        pub extended_dynamic_state2_logic_op: Bool32,
        pub extended_dynamic_state2_patch_control_points: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceExtendedDynamicState2FeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceExtendedDynamicState2FeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("extended_dynamic_state2", &self.extended_dynamic_state2)
                .field(
                    "extended_dynamic_state2_logic_op",
                    &self.extended_dynamic_state2_logic_op,
                )
                .field(
                    "extended_dynamic_state2_patch_control_points",
                    &self.extended_dynamic_state2_patch_control_points,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExtendedDynamicState2FeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceExtendedDynamicState2FeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceExtendedDynamicState2FeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceExtendedDynamicState2FeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                extended_dynamic_state2: Default::default(),
                extended_dynamic_state2_logic_op: Default::default(),
                extended_dynamic_state2_patch_control_points: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceExtendedDynamicState2FeaturesEXT<'a> {
        pub fn extended_dynamic_state2(mut self, extended_dynamic_state2: bool) -> Self {
            self.extended_dynamic_state2 = extended_dynamic_state2.into();
            self
        }

        pub fn extended_dynamic_state2_logic_op(
            mut self,
            extended_dynamic_state2_logic_op: bool,
        ) -> Self {
            self.extended_dynamic_state2_logic_op = extended_dynamic_state2_logic_op.into();
            self
        }

        pub fn extended_dynamic_state2_patch_control_points(
            mut self,
            extended_dynamic_state2_patch_control_points: bool,
        ) -> Self {
            self.extended_dynamic_state2_patch_control_points =
                extended_dynamic_state2_patch_control_points.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPatchControlPointsEXT.html>
    pub type PFN_vkCmdSetPatchControlPointsEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer, patch_control_points: u32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetLogicOpEXT.html>
    pub type PFN_vkCmdSetLogicOpEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op: LogicOp);
}

pub struct DeviceFn {
    cmd_set_patch_control_points_ext: PFN_vkCmdSetPatchControlPointsEXT,
    cmd_set_rasterizer_discard_enable_ext: PFN_vkCmdSetRasterizerDiscardEnable,
    cmd_set_depth_bias_enable_ext: PFN_vkCmdSetDepthBiasEnable,
    cmd_set_logic_op_ext: PFN_vkCmdSetLogicOpEXT,
    cmd_set_primitive_restart_enable_ext: PFN_vkCmdSetPrimitiveRestartEnable,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_patch_control_points_ext: transmute(
                    load(c"vkCmdSetPatchControlPointsEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_rasterizer_discard_enable_ext: transmute(
                    load(c"vkCmdSetRasterizerDiscardEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_bias_enable_ext: transmute(
                    load(c"vkCmdSetDepthBiasEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_logic_op_ext: transmute(
                    load(c"vkCmdSetLogicOpEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_primitive_restart_enable_ext: transmute(
                    load(c"vkCmdSetPrimitiveRestartEnableEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPatchControlPointsEXT.html>
    pub unsafe fn cmd_set_patch_control_points_ext(
        &self,
        command_buffer: CommandBuffer,
        patch_control_points: u32,
    ) {
        unsafe { (self.cmd_set_patch_control_points_ext)(command_buffer, patch_control_points) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRasterizerDiscardEnableEXT.html>
    pub unsafe fn cmd_set_rasterizer_discard_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        rasterizer_discard_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_rasterizer_discard_enable_ext)(
                command_buffer,
                rasterizer_discard_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthBiasEnableEXT.html>
    pub unsafe fn cmd_set_depth_bias_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_enable: bool,
    ) {
        unsafe { (self.cmd_set_depth_bias_enable_ext)(command_buffer, depth_bias_enable.into()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetLogicOpEXT.html>
    pub unsafe fn cmd_set_logic_op_ext(&self, command_buffer: CommandBuffer, logic_op: LogicOp) {
        unsafe { (self.cmd_set_logic_op_ext)(command_buffer, logic_op) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPrimitiveRestartEnableEXT.html>
    pub unsafe fn cmd_set_primitive_restart_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        primitive_restart_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_primitive_restart_enable_ext)(
                command_buffer,
                primitive_restart_enable.into(),
            )
        }
    }
}
