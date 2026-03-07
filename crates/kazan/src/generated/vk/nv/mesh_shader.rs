#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_mesh_shader";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMeshShaderFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMeshShaderFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub task_shader: Bool32,
        pub mesh_shader: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMeshShaderFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMeshShaderFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("task_shader", &self.task_shader)
                .field("mesh_shader", &self.mesh_shader)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMeshShaderFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceMeshShaderFeaturesNV<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceMeshShaderFeaturesNV<'a> {}

    impl Default for PhysicalDeviceMeshShaderFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                task_shader: Default::default(),
                mesh_shader: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMeshShaderFeaturesNV<'a> {
        pub fn task_shader(mut self, task_shader: bool) -> Self {
            self.task_shader = task_shader.into();
            self
        }

        pub fn mesh_shader(mut self, mesh_shader: bool) -> Self {
            self.mesh_shader = mesh_shader.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMeshShaderPropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMeshShaderPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_draw_mesh_tasks_count: u32,
        pub max_task_work_group_invocations: u32,
        pub max_task_work_group_size: [u32; 3],
        pub max_task_total_memory_size: u32,
        pub max_task_output_count: u32,
        pub max_mesh_work_group_invocations: u32,
        pub max_mesh_work_group_size: [u32; 3],
        pub max_mesh_total_memory_size: u32,
        pub max_mesh_output_vertices: u32,
        pub max_mesh_output_primitives: u32,
        pub max_mesh_multiview_view_count: u32,
        pub mesh_output_per_vertex_granularity: u32,
        pub mesh_output_per_primitive_granularity: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMeshShaderPropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMeshShaderPropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_draw_mesh_tasks_count", &self.max_draw_mesh_tasks_count)
                .field(
                    "max_task_work_group_invocations",
                    &self.max_task_work_group_invocations,
                )
                .field("max_task_work_group_size", &self.max_task_work_group_size)
                .field(
                    "max_task_total_memory_size",
                    &self.max_task_total_memory_size,
                )
                .field("max_task_output_count", &self.max_task_output_count)
                .field(
                    "max_mesh_work_group_invocations",
                    &self.max_mesh_work_group_invocations,
                )
                .field("max_mesh_work_group_size", &self.max_mesh_work_group_size)
                .field(
                    "max_mesh_total_memory_size",
                    &self.max_mesh_total_memory_size,
                )
                .field("max_mesh_output_vertices", &self.max_mesh_output_vertices)
                .field(
                    "max_mesh_output_primitives",
                    &self.max_mesh_output_primitives,
                )
                .field(
                    "max_mesh_multiview_view_count",
                    &self.max_mesh_multiview_view_count,
                )
                .field(
                    "mesh_output_per_vertex_granularity",
                    &self.mesh_output_per_vertex_granularity,
                )
                .field(
                    "mesh_output_per_primitive_granularity",
                    &self.mesh_output_per_primitive_granularity,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMeshShaderPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceMeshShaderPropertiesNV<'a>
    {
    }

    impl Default for PhysicalDeviceMeshShaderPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_draw_mesh_tasks_count: Default::default(),
                max_task_work_group_invocations: Default::default(),
                max_task_work_group_size: [Default::default(); _],
                max_task_total_memory_size: Default::default(),
                max_task_output_count: Default::default(),
                max_mesh_work_group_invocations: Default::default(),
                max_mesh_work_group_size: [Default::default(); _],
                max_mesh_total_memory_size: Default::default(),
                max_mesh_output_vertices: Default::default(),
                max_mesh_output_primitives: Default::default(),
                max_mesh_multiview_view_count: Default::default(),
                mesh_output_per_vertex_granularity: Default::default(),
                mesh_output_per_primitive_granularity: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMeshShaderPropertiesNV<'a> {
        pub fn max_draw_mesh_tasks_count(mut self, max_draw_mesh_tasks_count: u32) -> Self {
            self.max_draw_mesh_tasks_count = max_draw_mesh_tasks_count;
            self
        }

        pub fn max_task_work_group_invocations(
            mut self,
            max_task_work_group_invocations: u32,
        ) -> Self {
            self.max_task_work_group_invocations = max_task_work_group_invocations;
            self
        }

        pub fn max_task_work_group_size(mut self, max_task_work_group_size: [u32; 3]) -> Self {
            self.max_task_work_group_size = max_task_work_group_size;
            self
        }

        pub fn max_task_total_memory_size(mut self, max_task_total_memory_size: u32) -> Self {
            self.max_task_total_memory_size = max_task_total_memory_size;
            self
        }

        pub fn max_task_output_count(mut self, max_task_output_count: u32) -> Self {
            self.max_task_output_count = max_task_output_count;
            self
        }

        pub fn max_mesh_work_group_invocations(
            mut self,
            max_mesh_work_group_invocations: u32,
        ) -> Self {
            self.max_mesh_work_group_invocations = max_mesh_work_group_invocations;
            self
        }

        pub fn max_mesh_work_group_size(mut self, max_mesh_work_group_size: [u32; 3]) -> Self {
            self.max_mesh_work_group_size = max_mesh_work_group_size;
            self
        }

        pub fn max_mesh_total_memory_size(mut self, max_mesh_total_memory_size: u32) -> Self {
            self.max_mesh_total_memory_size = max_mesh_total_memory_size;
            self
        }

        pub fn max_mesh_output_vertices(mut self, max_mesh_output_vertices: u32) -> Self {
            self.max_mesh_output_vertices = max_mesh_output_vertices;
            self
        }

        pub fn max_mesh_output_primitives(mut self, max_mesh_output_primitives: u32) -> Self {
            self.max_mesh_output_primitives = max_mesh_output_primitives;
            self
        }

        pub fn max_mesh_multiview_view_count(mut self, max_mesh_multiview_view_count: u32) -> Self {
            self.max_mesh_multiview_view_count = max_mesh_multiview_view_count;
            self
        }

        pub fn mesh_output_per_vertex_granularity(
            mut self,
            mesh_output_per_vertex_granularity: u32,
        ) -> Self {
            self.mesh_output_per_vertex_granularity = mesh_output_per_vertex_granularity;
            self
        }

        pub fn mesh_output_per_primitive_granularity(
            mut self,
            mesh_output_per_primitive_granularity: u32,
        ) -> Self {
            self.mesh_output_per_primitive_granularity = mesh_output_per_primitive_granularity;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDrawMeshTasksIndirectCommandNV.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    pub struct DrawMeshTasksIndirectCommandNV {
        pub task_count: u32,
        pub first_task: u32,
    }

    impl DrawMeshTasksIndirectCommandNV {
        pub fn task_count(mut self, task_count: u32) -> Self {
            self.task_count = task_count;
            self
        }

        pub fn first_task(mut self, first_task: u32) -> Self {
            self.first_task = first_task;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawMeshTasksNV.html>
    pub type PFN_vkCmdDrawMeshTasksNV =
        unsafe extern "system" fn(command_buffer: CommandBuffer, task_count: u32, first_task: u32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawMeshTasksIndirectNV.html>
    pub type PFN_vkCmdDrawMeshTasksIndirectNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawMeshTasksIndirectCountNV.html>
    pub type PFN_vkCmdDrawMeshTasksIndirectCountNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    );
}

pub struct DeviceFn {
    cmd_draw_mesh_tasks_nv: PFN_vkCmdDrawMeshTasksNV,
    cmd_draw_mesh_tasks_indirect_nv: PFN_vkCmdDrawMeshTasksIndirectNV,
    cmd_draw_mesh_tasks_indirect_count_nv: Option<PFN_vkCmdDrawMeshTasksIndirectCountNV>,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_draw_mesh_tasks_nv: transmute(
                    load(c"vkCmdDrawMeshTasksNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_draw_mesh_tasks_indirect_nv: transmute(
                    load(c"vkCmdDrawMeshTasksIndirectNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_draw_mesh_tasks_indirect_count_nv: transmute(load(
                    c"vkCmdDrawMeshTasksIndirectCountNV",
                )),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawMeshTasksNV.html>
    pub unsafe fn cmd_draw_mesh_tasks_nv(
        &self,
        command_buffer: CommandBuffer,
        task_count: u32,
        first_task: u32,
    ) {
        unsafe { (self.cmd_draw_mesh_tasks_nv)(command_buffer, task_count, first_task) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawMeshTasksIndirectNV.html>
    pub unsafe fn cmd_draw_mesh_tasks_indirect_nv(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_mesh_tasks_indirect_nv)(
                command_buffer,
                buffer,
                offset,
                draw_count,
                stride,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawMeshTasksIndirectCountNV.html>
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_nv(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_mesh_tasks_indirect_count_nv.unwrap())(
                command_buffer,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        }
    }
}
