#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_mesh_shader";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMeshShaderFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMeshShaderFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub task_shader: Bool32,
        pub mesh_shader: Bool32,
        pub multiview_mesh_shader: Bool32,
        pub primitive_fragment_shading_rate_mesh_shader: Bool32,
        pub mesh_shader_queries: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMeshShaderFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMeshShaderFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("task_shader", &self.task_shader)
                .field("mesh_shader", &self.mesh_shader)
                .field("multiview_mesh_shader", &self.multiview_mesh_shader)
                .field(
                    "primitive_fragment_shading_rate_mesh_shader",
                    &self.primitive_fragment_shading_rate_mesh_shader,
                )
                .field("mesh_shader_queries", &self.mesh_shader_queries)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMeshShaderFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceMeshShaderFeaturesEXT<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceMeshShaderFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceMeshShaderFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                task_shader: Default::default(),
                mesh_shader: Default::default(),
                multiview_mesh_shader: Default::default(),
                primitive_fragment_shading_rate_mesh_shader: Default::default(),
                mesh_shader_queries: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMeshShaderFeaturesEXT<'a> {
        pub fn task_shader(mut self, task_shader: bool) -> Self {
            self.task_shader = task_shader.into();
            self
        }

        pub fn mesh_shader(mut self, mesh_shader: bool) -> Self {
            self.mesh_shader = mesh_shader.into();
            self
        }

        pub fn multiview_mesh_shader(mut self, multiview_mesh_shader: bool) -> Self {
            self.multiview_mesh_shader = multiview_mesh_shader.into();
            self
        }

        pub fn primitive_fragment_shading_rate_mesh_shader(
            mut self,
            primitive_fragment_shading_rate_mesh_shader: bool,
        ) -> Self {
            self.primitive_fragment_shading_rate_mesh_shader =
                primitive_fragment_shading_rate_mesh_shader.into();
            self
        }

        pub fn mesh_shader_queries(mut self, mesh_shader_queries: bool) -> Self {
            self.mesh_shader_queries = mesh_shader_queries.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMeshShaderPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMeshShaderPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_task_work_group_total_count: u32,
        pub max_task_work_group_count: [u32; 3],
        pub max_task_work_group_invocations: u32,
        pub max_task_work_group_size: [u32; 3],
        pub max_task_payload_size: u32,
        pub max_task_shared_memory_size: u32,
        pub max_task_payload_and_shared_memory_size: u32,
        pub max_mesh_work_group_total_count: u32,
        pub max_mesh_work_group_count: [u32; 3],
        pub max_mesh_work_group_invocations: u32,
        pub max_mesh_work_group_size: [u32; 3],
        pub max_mesh_shared_memory_size: u32,
        pub max_mesh_payload_and_shared_memory_size: u32,
        pub max_mesh_output_memory_size: u32,
        pub max_mesh_payload_and_output_memory_size: u32,
        pub max_mesh_output_components: u32,
        pub max_mesh_output_vertices: u32,
        pub max_mesh_output_primitives: u32,
        pub max_mesh_output_layers: u32,
        pub max_mesh_multiview_view_count: u32,
        pub mesh_output_per_vertex_granularity: u32,
        pub mesh_output_per_primitive_granularity: u32,
        pub max_preferred_task_work_group_invocations: u32,
        pub max_preferred_mesh_work_group_invocations: u32,
        pub prefers_local_invocation_vertex_output: Bool32,
        pub prefers_local_invocation_primitive_output: Bool32,
        pub prefers_compact_vertex_output: Bool32,
        pub prefers_compact_primitive_output: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMeshShaderPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMeshShaderPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_task_work_group_total_count",
                    &self.max_task_work_group_total_count,
                )
                .field("max_task_work_group_count", &self.max_task_work_group_count)
                .field(
                    "max_task_work_group_invocations",
                    &self.max_task_work_group_invocations,
                )
                .field("max_task_work_group_size", &self.max_task_work_group_size)
                .field("max_task_payload_size", &self.max_task_payload_size)
                .field(
                    "max_task_shared_memory_size",
                    &self.max_task_shared_memory_size,
                )
                .field(
                    "max_task_payload_and_shared_memory_size",
                    &self.max_task_payload_and_shared_memory_size,
                )
                .field(
                    "max_mesh_work_group_total_count",
                    &self.max_mesh_work_group_total_count,
                )
                .field("max_mesh_work_group_count", &self.max_mesh_work_group_count)
                .field(
                    "max_mesh_work_group_invocations",
                    &self.max_mesh_work_group_invocations,
                )
                .field("max_mesh_work_group_size", &self.max_mesh_work_group_size)
                .field(
                    "max_mesh_shared_memory_size",
                    &self.max_mesh_shared_memory_size,
                )
                .field(
                    "max_mesh_payload_and_shared_memory_size",
                    &self.max_mesh_payload_and_shared_memory_size,
                )
                .field(
                    "max_mesh_output_memory_size",
                    &self.max_mesh_output_memory_size,
                )
                .field(
                    "max_mesh_payload_and_output_memory_size",
                    &self.max_mesh_payload_and_output_memory_size,
                )
                .field(
                    "max_mesh_output_components",
                    &self.max_mesh_output_components,
                )
                .field("max_mesh_output_vertices", &self.max_mesh_output_vertices)
                .field(
                    "max_mesh_output_primitives",
                    &self.max_mesh_output_primitives,
                )
                .field("max_mesh_output_layers", &self.max_mesh_output_layers)
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
                .field(
                    "max_preferred_task_work_group_invocations",
                    &self.max_preferred_task_work_group_invocations,
                )
                .field(
                    "max_preferred_mesh_work_group_invocations",
                    &self.max_preferred_mesh_work_group_invocations,
                )
                .field(
                    "prefers_local_invocation_vertex_output",
                    &self.prefers_local_invocation_vertex_output,
                )
                .field(
                    "prefers_local_invocation_primitive_output",
                    &self.prefers_local_invocation_primitive_output,
                )
                .field(
                    "prefers_compact_vertex_output",
                    &self.prefers_compact_vertex_output,
                )
                .field(
                    "prefers_compact_primitive_output",
                    &self.prefers_compact_primitive_output,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMeshShaderPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceMeshShaderPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceMeshShaderPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_task_work_group_total_count: Default::default(),
                max_task_work_group_count: [Default::default(); _],
                max_task_work_group_invocations: Default::default(),
                max_task_work_group_size: [Default::default(); _],
                max_task_payload_size: Default::default(),
                max_task_shared_memory_size: Default::default(),
                max_task_payload_and_shared_memory_size: Default::default(),
                max_mesh_work_group_total_count: Default::default(),
                max_mesh_work_group_count: [Default::default(); _],
                max_mesh_work_group_invocations: Default::default(),
                max_mesh_work_group_size: [Default::default(); _],
                max_mesh_shared_memory_size: Default::default(),
                max_mesh_payload_and_shared_memory_size: Default::default(),
                max_mesh_output_memory_size: Default::default(),
                max_mesh_payload_and_output_memory_size: Default::default(),
                max_mesh_output_components: Default::default(),
                max_mesh_output_vertices: Default::default(),
                max_mesh_output_primitives: Default::default(),
                max_mesh_output_layers: Default::default(),
                max_mesh_multiview_view_count: Default::default(),
                mesh_output_per_vertex_granularity: Default::default(),
                mesh_output_per_primitive_granularity: Default::default(),
                max_preferred_task_work_group_invocations: Default::default(),
                max_preferred_mesh_work_group_invocations: Default::default(),
                prefers_local_invocation_vertex_output: Default::default(),
                prefers_local_invocation_primitive_output: Default::default(),
                prefers_compact_vertex_output: Default::default(),
                prefers_compact_primitive_output: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMeshShaderPropertiesEXT<'a> {
        pub fn max_task_work_group_total_count(
            mut self,
            max_task_work_group_total_count: u32,
        ) -> Self {
            self.max_task_work_group_total_count = max_task_work_group_total_count;
            self
        }

        pub fn max_task_work_group_count(mut self, max_task_work_group_count: [u32; 3]) -> Self {
            self.max_task_work_group_count = max_task_work_group_count;
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

        pub fn max_task_payload_size(mut self, max_task_payload_size: u32) -> Self {
            self.max_task_payload_size = max_task_payload_size;
            self
        }

        pub fn max_task_shared_memory_size(mut self, max_task_shared_memory_size: u32) -> Self {
            self.max_task_shared_memory_size = max_task_shared_memory_size;
            self
        }

        pub fn max_task_payload_and_shared_memory_size(
            mut self,
            max_task_payload_and_shared_memory_size: u32,
        ) -> Self {
            self.max_task_payload_and_shared_memory_size = max_task_payload_and_shared_memory_size;
            self
        }

        pub fn max_mesh_work_group_total_count(
            mut self,
            max_mesh_work_group_total_count: u32,
        ) -> Self {
            self.max_mesh_work_group_total_count = max_mesh_work_group_total_count;
            self
        }

        pub fn max_mesh_work_group_count(mut self, max_mesh_work_group_count: [u32; 3]) -> Self {
            self.max_mesh_work_group_count = max_mesh_work_group_count;
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

        pub fn max_mesh_shared_memory_size(mut self, max_mesh_shared_memory_size: u32) -> Self {
            self.max_mesh_shared_memory_size = max_mesh_shared_memory_size;
            self
        }

        pub fn max_mesh_payload_and_shared_memory_size(
            mut self,
            max_mesh_payload_and_shared_memory_size: u32,
        ) -> Self {
            self.max_mesh_payload_and_shared_memory_size = max_mesh_payload_and_shared_memory_size;
            self
        }

        pub fn max_mesh_output_memory_size(mut self, max_mesh_output_memory_size: u32) -> Self {
            self.max_mesh_output_memory_size = max_mesh_output_memory_size;
            self
        }

        pub fn max_mesh_payload_and_output_memory_size(
            mut self,
            max_mesh_payload_and_output_memory_size: u32,
        ) -> Self {
            self.max_mesh_payload_and_output_memory_size = max_mesh_payload_and_output_memory_size;
            self
        }

        pub fn max_mesh_output_components(mut self, max_mesh_output_components: u32) -> Self {
            self.max_mesh_output_components = max_mesh_output_components;
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

        pub fn max_mesh_output_layers(mut self, max_mesh_output_layers: u32) -> Self {
            self.max_mesh_output_layers = max_mesh_output_layers;
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

        pub fn max_preferred_task_work_group_invocations(
            mut self,
            max_preferred_task_work_group_invocations: u32,
        ) -> Self {
            self.max_preferred_task_work_group_invocations =
                max_preferred_task_work_group_invocations;
            self
        }

        pub fn max_preferred_mesh_work_group_invocations(
            mut self,
            max_preferred_mesh_work_group_invocations: u32,
        ) -> Self {
            self.max_preferred_mesh_work_group_invocations =
                max_preferred_mesh_work_group_invocations;
            self
        }

        pub fn prefers_local_invocation_vertex_output(
            mut self,
            prefers_local_invocation_vertex_output: bool,
        ) -> Self {
            self.prefers_local_invocation_vertex_output =
                prefers_local_invocation_vertex_output.into();
            self
        }

        pub fn prefers_local_invocation_primitive_output(
            mut self,
            prefers_local_invocation_primitive_output: bool,
        ) -> Self {
            self.prefers_local_invocation_primitive_output =
                prefers_local_invocation_primitive_output.into();
            self
        }

        pub fn prefers_compact_vertex_output(
            mut self,
            prefers_compact_vertex_output: bool,
        ) -> Self {
            self.prefers_compact_vertex_output = prefers_compact_vertex_output.into();
            self
        }

        pub fn prefers_compact_primitive_output(
            mut self,
            prefers_compact_primitive_output: bool,
        ) -> Self {
            self.prefers_compact_primitive_output = prefers_compact_primitive_output.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDrawMeshTasksIndirectCommandEXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct DrawMeshTasksIndirectCommandEXT {
        pub group_count_x: u32,
        pub group_count_y: u32,
        pub group_count_z: u32,
    }

    impl DrawMeshTasksIndirectCommandEXT {
        pub fn group_count_x(mut self, group_count_x: u32) -> Self {
            self.group_count_x = group_count_x;
            self
        }

        pub fn group_count_y(mut self, group_count_y: u32) -> Self {
            self.group_count_y = group_count_y;
            self
        }

        pub fn group_count_z(mut self, group_count_z: u32) -> Self {
            self.group_count_z = group_count_z;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawMeshTasksEXT.html>
    pub type PFN_vkCmdDrawMeshTasksEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawMeshTasksIndirectEXT.html>
    pub type PFN_vkCmdDrawMeshTasksIndirectEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawMeshTasksIndirectCountEXT.html>
    pub type PFN_vkCmdDrawMeshTasksIndirectCountEXT = unsafe extern "system" fn(
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
    cmd_draw_mesh_tasks_ext: PFN_vkCmdDrawMeshTasksEXT,
    cmd_draw_mesh_tasks_indirect_ext: PFN_vkCmdDrawMeshTasksIndirectEXT,
    cmd_draw_mesh_tasks_indirect_count_ext: Option<PFN_vkCmdDrawMeshTasksIndirectCountEXT>,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_draw_mesh_tasks_ext: transmute(
                    load(c"vkCmdDrawMeshTasksEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_draw_mesh_tasks_indirect_ext: transmute(
                    load(c"vkCmdDrawMeshTasksIndirectEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_draw_mesh_tasks_indirect_count_ext: transmute(load(
                    c"vkCmdDrawMeshTasksIndirectCountEXT",
                )),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawMeshTasksEXT.html>
    pub unsafe fn cmd_draw_mesh_tasks_ext(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe {
            (self.cmd_draw_mesh_tasks_ext)(
                command_buffer,
                group_count_x,
                group_count_y,
                group_count_z,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawMeshTasksIndirectEXT.html>
    pub unsafe fn cmd_draw_mesh_tasks_indirect_ext(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_mesh_tasks_indirect_ext)(
                command_buffer,
                buffer,
                offset,
                draw_count,
                stride,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawMeshTasksIndirectCountEXT.html>
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_ext(
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
            (self.cmd_draw_mesh_tasks_indirect_count_ext.unwrap())(
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
