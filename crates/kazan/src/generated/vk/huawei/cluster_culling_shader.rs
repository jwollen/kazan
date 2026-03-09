//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_HUAWEI_cluster_culling_shader.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_HUAWEI_cluster_culling_shader";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_work_group_count: [u32; 3],
        pub max_work_group_size: [u32; 3],
        pub max_output_cluster_count: u32,
        pub indirect_buffer_offset_alignment: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceClusterCullingShaderPropertiesHUAWEI")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_work_group_count", &self.max_work_group_count)
                .field("max_work_group_size", &self.max_work_group_size)
                .field("max_output_cluster_count", &self.max_output_cluster_count)
                .field(
                    "indirect_buffer_offset_alignment",
                    &self.indirect_buffer_offset_alignment,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'a>
    {
    }

    impl Default for PhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_work_group_count: [Default::default(); _],
                max_work_group_size: [Default::default(); _],
                max_output_cluster_count: Default::default(),
                indirect_buffer_offset_alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'a> {
        #[inline]
        pub fn max_work_group_count(mut self, max_work_group_count: [u32; 3]) -> Self {
            self.max_work_group_count = max_work_group_count;
            self
        }

        #[inline]
        pub fn max_work_group_size(mut self, max_work_group_size: [u32; 3]) -> Self {
            self.max_work_group_size = max_work_group_size;
            self
        }

        #[inline]
        pub fn max_output_cluster_count(mut self, max_output_cluster_count: u32) -> Self {
            self.max_output_cluster_count = max_output_cluster_count;
            self
        }

        #[inline]
        pub fn indirect_buffer_offset_alignment(
            mut self,
            indirect_buffer_offset_alignment: DeviceSize,
        ) -> Self {
            self.indirect_buffer_offset_alignment = indirect_buffer_offset_alignment;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub clusterculling_shader: Bool32,
        pub multiview_cluster_culling_shader: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceClusterCullingShaderFeaturesHUAWEI")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("clusterculling_shader", &self.clusterculling_shader)
                .field(
                    "multiview_cluster_culling_shader",
                    &self.multiview_cluster_culling_shader,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'a>
    {
    }

    impl Default for PhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                clusterculling_shader: Default::default(),
                multiview_cluster_culling_shader: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'a> {
        #[inline]
        pub fn clusterculling_shader(mut self, clusterculling_shader: bool) -> Self {
            self.clusterculling_shader = clusterculling_shader.into();
            self
        }

        #[inline]
        pub fn multiview_cluster_culling_shader(
            mut self,
            multiview_cluster_culling_shader: bool,
        ) -> Self {
            self.multiview_cluster_culling_shader = multiview_cluster_culling_shader.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cluster_shading_rate: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("cluster_shading_rate", &self.cluster_shading_rate)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI;
    }

    unsafe impl<'a> Extends<PhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'a>>
        for PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'a>
    {
    }

    impl Default for PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                cluster_shading_rate: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'a> {
        #[inline]
        pub fn cluster_shading_rate(mut self, cluster_shading_rate: bool) -> Self {
            self.cluster_shading_rate = cluster_shading_rate.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawClusterHUAWEI.html>
    pub type PFN_vkCmdDrawClusterHUAWEI = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawClusterIndirectHUAWEI.html>
    pub type PFN_vkCmdDrawClusterIndirectHUAWEI = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI =
        PhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'static>;
    pub type VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI =
        PhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'static>;
    pub type VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI =
        PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'static>;
    impl PhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    cmd_draw_cluster_huawei: PFN_vkCmdDrawClusterHUAWEI,
    cmd_draw_cluster_indirect_huawei: PFN_vkCmdDrawClusterIndirectHUAWEI,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_draw_cluster_huawei: transmute(
                    load(c"vkCmdDrawClusterHUAWEI").ok_or(MissingEntryPointError)?,
                ),
                cmd_draw_cluster_indirect_huawei: transmute(
                    load(c"vkCmdDrawClusterIndirectHUAWEI").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawClusterHUAWEI.html>
    #[inline]
    pub unsafe fn cmd_draw_cluster_huawei(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe {
            (self.cmd_draw_cluster_huawei)(
                command_buffer,
                group_count_x,
                group_count_y,
                group_count_z,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawClusterIndirectHUAWEI.html>
    #[inline]
    pub unsafe fn cmd_draw_cluster_indirect_huawei(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
    ) {
        unsafe { (self.cmd_draw_cluster_indirect_huawei)(command_buffer, buffer, offset) }
    }
}
