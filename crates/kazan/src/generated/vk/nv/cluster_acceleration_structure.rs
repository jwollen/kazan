#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_cluster_acceleration_structure";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceClusterAccelerationStructureFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceClusterAccelerationStructureFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cluster_acceleration_structure: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceClusterAccelerationStructureFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceClusterAccelerationStructureFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "cluster_acceleration_structure",
                    &self.cluster_acceleration_structure,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceClusterAccelerationStructureFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceClusterAccelerationStructureFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceClusterAccelerationStructureFeaturesNV<'a>
    {
    }

    impl Default for PhysicalDeviceClusterAccelerationStructureFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                cluster_acceleration_structure: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceClusterAccelerationStructureFeaturesNV<'a> {
        pub fn cluster_acceleration_structure(
            mut self,
            cluster_acceleration_structure: bool,
        ) -> Self {
            self.cluster_acceleration_structure = cluster_acceleration_structure.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceClusterAccelerationStructurePropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceClusterAccelerationStructurePropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_vertices_per_cluster: u32,
        pub max_triangles_per_cluster: u32,
        pub cluster_scratch_byte_alignment: u32,
        pub cluster_byte_alignment: u32,
        pub cluster_template_byte_alignment: u32,
        pub cluster_bottom_level_byte_alignment: u32,
        pub cluster_template_bounds_byte_alignment: u32,
        pub max_cluster_geometry_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceClusterAccelerationStructurePropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceClusterAccelerationStructurePropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_vertices_per_cluster", &self.max_vertices_per_cluster)
                .field("max_triangles_per_cluster", &self.max_triangles_per_cluster)
                .field(
                    "cluster_scratch_byte_alignment",
                    &self.cluster_scratch_byte_alignment,
                )
                .field("cluster_byte_alignment", &self.cluster_byte_alignment)
                .field(
                    "cluster_template_byte_alignment",
                    &self.cluster_template_byte_alignment,
                )
                .field(
                    "cluster_bottom_level_byte_alignment",
                    &self.cluster_bottom_level_byte_alignment,
                )
                .field(
                    "cluster_template_bounds_byte_alignment",
                    &self.cluster_template_bounds_byte_alignment,
                )
                .field(
                    "max_cluster_geometry_index",
                    &self.max_cluster_geometry_index,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceClusterAccelerationStructurePropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceClusterAccelerationStructurePropertiesNV<'a>
    {
    }

    impl Default for PhysicalDeviceClusterAccelerationStructurePropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_vertices_per_cluster: Default::default(),
                max_triangles_per_cluster: Default::default(),
                cluster_scratch_byte_alignment: Default::default(),
                cluster_byte_alignment: Default::default(),
                cluster_template_byte_alignment: Default::default(),
                cluster_bottom_level_byte_alignment: Default::default(),
                cluster_template_bounds_byte_alignment: Default::default(),
                max_cluster_geometry_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceClusterAccelerationStructurePropertiesNV<'a> {
        pub fn max_vertices_per_cluster(mut self, max_vertices_per_cluster: u32) -> Self {
            self.max_vertices_per_cluster = max_vertices_per_cluster;
            self
        }

        pub fn max_triangles_per_cluster(mut self, max_triangles_per_cluster: u32) -> Self {
            self.max_triangles_per_cluster = max_triangles_per_cluster;
            self
        }

        pub fn cluster_scratch_byte_alignment(
            mut self,
            cluster_scratch_byte_alignment: u32,
        ) -> Self {
            self.cluster_scratch_byte_alignment = cluster_scratch_byte_alignment;
            self
        }

        pub fn cluster_byte_alignment(mut self, cluster_byte_alignment: u32) -> Self {
            self.cluster_byte_alignment = cluster_byte_alignment;
            self
        }

        pub fn cluster_template_byte_alignment(
            mut self,
            cluster_template_byte_alignment: u32,
        ) -> Self {
            self.cluster_template_byte_alignment = cluster_template_byte_alignment;
            self
        }

        pub fn cluster_bottom_level_byte_alignment(
            mut self,
            cluster_bottom_level_byte_alignment: u32,
        ) -> Self {
            self.cluster_bottom_level_byte_alignment = cluster_bottom_level_byte_alignment;
            self
        }

        pub fn cluster_template_bounds_byte_alignment(
            mut self,
            cluster_template_bounds_byte_alignment: u32,
        ) -> Self {
            self.cluster_template_bounds_byte_alignment = cluster_template_bounds_byte_alignment;
            self
        }

        pub fn max_cluster_geometry_index(mut self, max_cluster_geometry_index: u32) -> Self {
            self.max_cluster_geometry_index = max_cluster_geometry_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkStridedDeviceAddressNV.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct StridedDeviceAddressNV {
        pub start_address: DeviceAddress,
        pub stride_in_bytes: DeviceSize,
    }

    impl StridedDeviceAddressNV {
        pub fn start_address(mut self, start_address: DeviceAddress) -> Self {
            self.start_address = start_address;
            self
        }

        pub fn stride_in_bytes(mut self, stride_in_bytes: DeviceSize) -> Self {
            self.stride_in_bytes = stride_in_bytes;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRayTracingPipelineClusterAccelerationStructureCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RayTracingPipelineClusterAccelerationStructureCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub allow_cluster_acceleration_structure: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for RayTracingPipelineClusterAccelerationStructureCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RayTracingPipelineClusterAccelerationStructureCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "allow_cluster_acceleration_structure",
                    &self.allow_cluster_acceleration_structure,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for RayTracingPipelineClusterAccelerationStructureCreateInfoNV<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV;
    }

    unsafe impl<'a> Extends<RayTracingPipelineCreateInfoKHR<'a>>
        for RayTracingPipelineClusterAccelerationStructureCreateInfoNV<'a>
    {
    }

    impl Default for RayTracingPipelineClusterAccelerationStructureCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                allow_cluster_acceleration_structure: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RayTracingPipelineClusterAccelerationStructureCreateInfoNV<'a> {
        pub fn allow_cluster_acceleration_structure(
            mut self,
            allow_cluster_acceleration_structure: bool,
        ) -> Self {
            self.allow_cluster_acceleration_structure = allow_cluster_acceleration_structure.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct ClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV {
        pub geometry_index: u32,
        pub reserved: u32,
        pub geometry_flags: u32,
    }

    impl ClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV {
        pub fn geometry_index(mut self, geometry_index: u32) -> Self {
            self.geometry_index = geometry_index;
            self
        }

        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }

        pub fn geometry_flags(mut self, geometry_flags: u32) -> Self {
            self.geometry_flags = geometry_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureMoveObjectsInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct ClusterAccelerationStructureMoveObjectsInfoNV {
        pub src_acceleration_structure: DeviceAddress,
    }

    impl ClusterAccelerationStructureMoveObjectsInfoNV {
        pub fn src_acceleration_structure(
            mut self,
            src_acceleration_structure: DeviceAddress,
        ) -> Self {
            self.src_acceleration_structure = src_acceleration_structure;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureBuildClustersBottomLevelInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct ClusterAccelerationStructureBuildClustersBottomLevelInfoNV {
        pub cluster_references_count: u32,
        pub cluster_references_stride: u32,
        pub cluster_references: DeviceAddress,
    }

    impl ClusterAccelerationStructureBuildClustersBottomLevelInfoNV {
        pub fn cluster_references_count(mut self, cluster_references_count: u32) -> Self {
            self.cluster_references_count = cluster_references_count;
            self
        }

        pub fn cluster_references_stride(mut self, cluster_references_stride: u32) -> Self {
            self.cluster_references_stride = cluster_references_stride;
            self
        }

        pub fn cluster_references(mut self, cluster_references: DeviceAddress) -> Self {
            self.cluster_references = cluster_references;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureGetTemplateIndicesInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct ClusterAccelerationStructureGetTemplateIndicesInfoNV {
        pub cluster_template_address: DeviceAddress,
    }

    impl ClusterAccelerationStructureGetTemplateIndicesInfoNV {
        pub fn cluster_template_address(mut self, cluster_template_address: DeviceAddress) -> Self {
            self.cluster_template_address = cluster_template_address;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureBuildTriangleClusterInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct ClusterAccelerationStructureBuildTriangleClusterInfoNV {
        pub cluster_id: u32,
        pub cluster_flags: ClusterAccelerationStructureClusterFlagsNV,
        pub triangle_count: u32,
        pub vertex_count: u32,
        pub position_truncate_bit_count: u32,
        pub index_type: u32,
        pub opacity_micromap_index_type: u32,
        pub base_geometry_index_and_geometry_flags:
            ClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV,
        pub index_buffer_stride: u16,
        pub vertex_buffer_stride: u16,
        pub geometry_index_and_flags_buffer_stride: u16,
        pub opacity_micromap_index_buffer_stride: u16,
        pub index_buffer: DeviceAddress,
        pub vertex_buffer: DeviceAddress,
        pub geometry_index_and_flags_buffer: DeviceAddress,
        pub opacity_micromap_array: DeviceAddress,
        pub opacity_micromap_index_buffer: DeviceAddress,
    }

    impl ClusterAccelerationStructureBuildTriangleClusterInfoNV {
        pub fn cluster_id(mut self, cluster_id: u32) -> Self {
            self.cluster_id = cluster_id;
            self
        }

        pub fn cluster_flags(
            mut self,
            cluster_flags: ClusterAccelerationStructureClusterFlagsNV,
        ) -> Self {
            self.cluster_flags = cluster_flags;
            self
        }

        pub fn triangle_count(mut self, triangle_count: u32) -> Self {
            self.triangle_count = triangle_count;
            self
        }

        pub fn vertex_count(mut self, vertex_count: u32) -> Self {
            self.vertex_count = vertex_count;
            self
        }

        pub fn position_truncate_bit_count(mut self, position_truncate_bit_count: u32) -> Self {
            self.position_truncate_bit_count = position_truncate_bit_count;
            self
        }

        pub fn index_type(mut self, index_type: u32) -> Self {
            self.index_type = index_type;
            self
        }

        pub fn opacity_micromap_index_type(mut self, opacity_micromap_index_type: u32) -> Self {
            self.opacity_micromap_index_type = opacity_micromap_index_type;
            self
        }

        pub fn base_geometry_index_and_geometry_flags(
            mut self,
            base_geometry_index_and_geometry_flags: ClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV,
        ) -> Self {
            self.base_geometry_index_and_geometry_flags = base_geometry_index_and_geometry_flags;
            self
        }

        pub fn index_buffer_stride(mut self, index_buffer_stride: u16) -> Self {
            self.index_buffer_stride = index_buffer_stride;
            self
        }

        pub fn vertex_buffer_stride(mut self, vertex_buffer_stride: u16) -> Self {
            self.vertex_buffer_stride = vertex_buffer_stride;
            self
        }

        pub fn geometry_index_and_flags_buffer_stride(
            mut self,
            geometry_index_and_flags_buffer_stride: u16,
        ) -> Self {
            self.geometry_index_and_flags_buffer_stride = geometry_index_and_flags_buffer_stride;
            self
        }

        pub fn opacity_micromap_index_buffer_stride(
            mut self,
            opacity_micromap_index_buffer_stride: u16,
        ) -> Self {
            self.opacity_micromap_index_buffer_stride = opacity_micromap_index_buffer_stride;
            self
        }

        pub fn index_buffer(mut self, index_buffer: DeviceAddress) -> Self {
            self.index_buffer = index_buffer;
            self
        }

        pub fn vertex_buffer(mut self, vertex_buffer: DeviceAddress) -> Self {
            self.vertex_buffer = vertex_buffer;
            self
        }

        pub fn geometry_index_and_flags_buffer(
            mut self,
            geometry_index_and_flags_buffer: DeviceAddress,
        ) -> Self {
            self.geometry_index_and_flags_buffer = geometry_index_and_flags_buffer;
            self
        }

        pub fn opacity_micromap_array(mut self, opacity_micromap_array: DeviceAddress) -> Self {
            self.opacity_micromap_array = opacity_micromap_array;
            self
        }

        pub fn opacity_micromap_index_buffer(
            mut self,
            opacity_micromap_index_buffer: DeviceAddress,
        ) -> Self {
            self.opacity_micromap_index_buffer = opacity_micromap_index_buffer;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureBuildTriangleClusterTemplateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct ClusterAccelerationStructureBuildTriangleClusterTemplateInfoNV {
        pub cluster_id: u32,
        pub cluster_flags: ClusterAccelerationStructureClusterFlagsNV,
        pub triangle_count: u32,
        pub vertex_count: u32,
        pub position_truncate_bit_count: u32,
        pub index_type: u32,
        pub opacity_micromap_index_type: u32,
        pub base_geometry_index_and_geometry_flags:
            ClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV,
        pub index_buffer_stride: u16,
        pub vertex_buffer_stride: u16,
        pub geometry_index_and_flags_buffer_stride: u16,
        pub opacity_micromap_index_buffer_stride: u16,
        pub index_buffer: DeviceAddress,
        pub vertex_buffer: DeviceAddress,
        pub geometry_index_and_flags_buffer: DeviceAddress,
        pub opacity_micromap_array: DeviceAddress,
        pub opacity_micromap_index_buffer: DeviceAddress,
        pub instantiation_bounding_box_limit: DeviceAddress,
    }

    impl ClusterAccelerationStructureBuildTriangleClusterTemplateInfoNV {
        pub fn cluster_id(mut self, cluster_id: u32) -> Self {
            self.cluster_id = cluster_id;
            self
        }

        pub fn cluster_flags(
            mut self,
            cluster_flags: ClusterAccelerationStructureClusterFlagsNV,
        ) -> Self {
            self.cluster_flags = cluster_flags;
            self
        }

        pub fn triangle_count(mut self, triangle_count: u32) -> Self {
            self.triangle_count = triangle_count;
            self
        }

        pub fn vertex_count(mut self, vertex_count: u32) -> Self {
            self.vertex_count = vertex_count;
            self
        }

        pub fn position_truncate_bit_count(mut self, position_truncate_bit_count: u32) -> Self {
            self.position_truncate_bit_count = position_truncate_bit_count;
            self
        }

        pub fn index_type(mut self, index_type: u32) -> Self {
            self.index_type = index_type;
            self
        }

        pub fn opacity_micromap_index_type(mut self, opacity_micromap_index_type: u32) -> Self {
            self.opacity_micromap_index_type = opacity_micromap_index_type;
            self
        }

        pub fn base_geometry_index_and_geometry_flags(
            mut self,
            base_geometry_index_and_geometry_flags: ClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV,
        ) -> Self {
            self.base_geometry_index_and_geometry_flags = base_geometry_index_and_geometry_flags;
            self
        }

        pub fn index_buffer_stride(mut self, index_buffer_stride: u16) -> Self {
            self.index_buffer_stride = index_buffer_stride;
            self
        }

        pub fn vertex_buffer_stride(mut self, vertex_buffer_stride: u16) -> Self {
            self.vertex_buffer_stride = vertex_buffer_stride;
            self
        }

        pub fn geometry_index_and_flags_buffer_stride(
            mut self,
            geometry_index_and_flags_buffer_stride: u16,
        ) -> Self {
            self.geometry_index_and_flags_buffer_stride = geometry_index_and_flags_buffer_stride;
            self
        }

        pub fn opacity_micromap_index_buffer_stride(
            mut self,
            opacity_micromap_index_buffer_stride: u16,
        ) -> Self {
            self.opacity_micromap_index_buffer_stride = opacity_micromap_index_buffer_stride;
            self
        }

        pub fn index_buffer(mut self, index_buffer: DeviceAddress) -> Self {
            self.index_buffer = index_buffer;
            self
        }

        pub fn vertex_buffer(mut self, vertex_buffer: DeviceAddress) -> Self {
            self.vertex_buffer = vertex_buffer;
            self
        }

        pub fn geometry_index_and_flags_buffer(
            mut self,
            geometry_index_and_flags_buffer: DeviceAddress,
        ) -> Self {
            self.geometry_index_and_flags_buffer = geometry_index_and_flags_buffer;
            self
        }

        pub fn opacity_micromap_array(mut self, opacity_micromap_array: DeviceAddress) -> Self {
            self.opacity_micromap_array = opacity_micromap_array;
            self
        }

        pub fn opacity_micromap_index_buffer(
            mut self,
            opacity_micromap_index_buffer: DeviceAddress,
        ) -> Self {
            self.opacity_micromap_index_buffer = opacity_micromap_index_buffer;
            self
        }

        pub fn instantiation_bounding_box_limit(
            mut self,
            instantiation_bounding_box_limit: DeviceAddress,
        ) -> Self {
            self.instantiation_bounding_box_limit = instantiation_bounding_box_limit;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureInstantiateClusterInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct ClusterAccelerationStructureInstantiateClusterInfoNV {
        pub cluster_id_offset: u32,
        pub geometry_index_offset: u32,
        pub reserved: u32,
        pub cluster_template_address: DeviceAddress,
        pub vertex_buffer: StridedDeviceAddressNV,
    }

    impl ClusterAccelerationStructureInstantiateClusterInfoNV {
        pub fn cluster_id_offset(mut self, cluster_id_offset: u32) -> Self {
            self.cluster_id_offset = cluster_id_offset;
            self
        }

        pub fn geometry_index_offset(mut self, geometry_index_offset: u32) -> Self {
            self.geometry_index_offset = geometry_index_offset;
            self
        }

        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }

        pub fn cluster_template_address(mut self, cluster_template_address: DeviceAddress) -> Self {
            self.cluster_template_address = cluster_template_address;
            self
        }

        pub fn vertex_buffer(mut self, vertex_buffer: StridedDeviceAddressNV) -> Self {
            self.vertex_buffer = vertex_buffer;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureClustersBottomLevelInputNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ClusterAccelerationStructureClustersBottomLevelInputNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_total_cluster_count: u32,
        pub max_cluster_count_per_acceleration_structure: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for ClusterAccelerationStructureClustersBottomLevelInputNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ClusterAccelerationStructureClustersBottomLevelInputNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_total_cluster_count", &self.max_total_cluster_count)
                .field(
                    "max_cluster_count_per_acceleration_structure",
                    &self.max_cluster_count_per_acceleration_structure,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ClusterAccelerationStructureClustersBottomLevelInputNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV;
    }

    impl Default for ClusterAccelerationStructureClustersBottomLevelInputNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_total_cluster_count: Default::default(),
                max_cluster_count_per_acceleration_structure: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ClusterAccelerationStructureClustersBottomLevelInputNV<'a> {
        pub fn max_total_cluster_count(mut self, max_total_cluster_count: u32) -> Self {
            self.max_total_cluster_count = max_total_cluster_count;
            self
        }

        pub fn max_cluster_count_per_acceleration_structure(
            mut self,
            max_cluster_count_per_acceleration_structure: u32,
        ) -> Self {
            self.max_cluster_count_per_acceleration_structure =
                max_cluster_count_per_acceleration_structure;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureTriangleClusterInputNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ClusterAccelerationStructureTriangleClusterInputNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub vertex_format: Format,
        pub max_geometry_index_value: u32,
        pub max_cluster_unique_geometry_count: u32,
        pub max_cluster_triangle_count: u32,
        pub max_cluster_vertex_count: u32,
        pub max_total_triangle_count: u32,
        pub max_total_vertex_count: u32,
        pub min_position_truncate_bit_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for ClusterAccelerationStructureTriangleClusterInputNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ClusterAccelerationStructureTriangleClusterInputNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("vertex_format", &self.vertex_format)
                .field("max_geometry_index_value", &self.max_geometry_index_value)
                .field(
                    "max_cluster_unique_geometry_count",
                    &self.max_cluster_unique_geometry_count,
                )
                .field(
                    "max_cluster_triangle_count",
                    &self.max_cluster_triangle_count,
                )
                .field("max_cluster_vertex_count", &self.max_cluster_vertex_count)
                .field("max_total_triangle_count", &self.max_total_triangle_count)
                .field("max_total_vertex_count", &self.max_total_vertex_count)
                .field(
                    "min_position_truncate_bit_count",
                    &self.min_position_truncate_bit_count,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ClusterAccelerationStructureTriangleClusterInputNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV;
    }

    impl Default for ClusterAccelerationStructureTriangleClusterInputNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                vertex_format: Default::default(),
                max_geometry_index_value: Default::default(),
                max_cluster_unique_geometry_count: Default::default(),
                max_cluster_triangle_count: Default::default(),
                max_cluster_vertex_count: Default::default(),
                max_total_triangle_count: Default::default(),
                max_total_vertex_count: Default::default(),
                min_position_truncate_bit_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ClusterAccelerationStructureTriangleClusterInputNV<'a> {
        pub fn vertex_format(mut self, vertex_format: Format) -> Self {
            self.vertex_format = vertex_format;
            self
        }

        pub fn max_geometry_index_value(mut self, max_geometry_index_value: u32) -> Self {
            self.max_geometry_index_value = max_geometry_index_value;
            self
        }

        pub fn max_cluster_unique_geometry_count(
            mut self,
            max_cluster_unique_geometry_count: u32,
        ) -> Self {
            self.max_cluster_unique_geometry_count = max_cluster_unique_geometry_count;
            self
        }

        pub fn max_cluster_triangle_count(mut self, max_cluster_triangle_count: u32) -> Self {
            self.max_cluster_triangle_count = max_cluster_triangle_count;
            self
        }

        pub fn max_cluster_vertex_count(mut self, max_cluster_vertex_count: u32) -> Self {
            self.max_cluster_vertex_count = max_cluster_vertex_count;
            self
        }

        pub fn max_total_triangle_count(mut self, max_total_triangle_count: u32) -> Self {
            self.max_total_triangle_count = max_total_triangle_count;
            self
        }

        pub fn max_total_vertex_count(mut self, max_total_vertex_count: u32) -> Self {
            self.max_total_vertex_count = max_total_vertex_count;
            self
        }

        pub fn min_position_truncate_bit_count(
            mut self,
            min_position_truncate_bit_count: u32,
        ) -> Self {
            self.min_position_truncate_bit_count = min_position_truncate_bit_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureMoveObjectsInputNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ClusterAccelerationStructureMoveObjectsInputNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ty: ClusterAccelerationStructureTypeNV,
        pub no_move_overlap: Bool32,
        pub max_moved_bytes: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for ClusterAccelerationStructureMoveObjectsInputNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ClusterAccelerationStructureMoveObjectsInputNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("ty", &self.ty)
                .field("no_move_overlap", &self.no_move_overlap)
                .field("max_moved_bytes", &self.max_moved_bytes)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ClusterAccelerationStructureMoveObjectsInputNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV;
    }

    impl Default for ClusterAccelerationStructureMoveObjectsInputNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                ty: Default::default(),
                no_move_overlap: Default::default(),
                max_moved_bytes: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ClusterAccelerationStructureMoveObjectsInputNV<'a> {
        pub fn ty(mut self, ty: ClusterAccelerationStructureTypeNV) -> Self {
            self.ty = ty;
            self
        }

        pub fn no_move_overlap(mut self, no_move_overlap: bool) -> Self {
            self.no_move_overlap = no_move_overlap.into();
            self
        }

        pub fn max_moved_bytes(mut self, max_moved_bytes: DeviceSize) -> Self {
            self.max_moved_bytes = max_moved_bytes;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureInputInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ClusterAccelerationStructureInputInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_acceleration_structure_count: u32,
        pub flags: BuildAccelerationStructureFlagsKHR,
        pub op_type: ClusterAccelerationStructureOpTypeNV,
        pub op_mode: ClusterAccelerationStructureOpModeNV,
        pub op_input: ClusterAccelerationStructureOpInputNV<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for ClusterAccelerationStructureInputInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ClusterAccelerationStructureInputInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_acceleration_structure_count",
                    &self.max_acceleration_structure_count,
                )
                .field("flags", &self.flags)
                .field("op_type", &self.op_type)
                .field("op_mode", &self.op_mode)
                .field("op_input", &self.op_input)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ClusterAccelerationStructureInputInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV;
    }

    impl Default for ClusterAccelerationStructureInputInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_acceleration_structure_count: Default::default(),
                flags: Default::default(),
                op_type: Default::default(),
                op_mode: Default::default(),
                op_input: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ClusterAccelerationStructureInputInfoNV<'a> {
        pub fn max_acceleration_structure_count(
            mut self,
            max_acceleration_structure_count: u32,
        ) -> Self {
            self.max_acceleration_structure_count = max_acceleration_structure_count;
            self
        }

        pub fn flags(mut self, flags: BuildAccelerationStructureFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        pub fn op_type(mut self, op_type: ClusterAccelerationStructureOpTypeNV) -> Self {
            self.op_type = op_type;
            self
        }

        pub fn op_mode(mut self, op_mode: ClusterAccelerationStructureOpModeNV) -> Self {
            self.op_mode = op_mode;
            self
        }

        pub fn op_input(mut self, op_input: ClusterAccelerationStructureOpInputNV<'a>) -> Self {
            self.op_input = op_input;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureCommandsInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ClusterAccelerationStructureCommandsInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub input: ClusterAccelerationStructureInputInfoNV<'a>,
        pub dst_implicit_data: DeviceAddress,
        pub scratch_data: DeviceAddress,
        pub dst_addresses_array: StridedDeviceAddressRegionKHR,
        pub dst_sizes_array: StridedDeviceAddressRegionKHR,
        pub src_infos_array: StridedDeviceAddressRegionKHR,
        pub src_infos_count: DeviceAddress,
        pub address_resolution_flags: ClusterAccelerationStructureAddressResolutionFlagsNV,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for ClusterAccelerationStructureCommandsInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ClusterAccelerationStructureCommandsInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("input", &self.input)
                .field("dst_implicit_data", &self.dst_implicit_data)
                .field("scratch_data", &self.scratch_data)
                .field("dst_addresses_array", &self.dst_addresses_array)
                .field("dst_sizes_array", &self.dst_sizes_array)
                .field("src_infos_array", &self.src_infos_array)
                .field("src_infos_count", &self.src_infos_count)
                .field("address_resolution_flags", &self.address_resolution_flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ClusterAccelerationStructureCommandsInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV;
    }

    impl Default for ClusterAccelerationStructureCommandsInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                input: Default::default(),
                dst_implicit_data: Default::default(),
                scratch_data: Default::default(),
                dst_addresses_array: Default::default(),
                dst_sizes_array: Default::default(),
                src_infos_array: Default::default(),
                src_infos_count: Default::default(),
                address_resolution_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ClusterAccelerationStructureCommandsInfoNV<'a> {
        pub fn input(mut self, input: ClusterAccelerationStructureInputInfoNV<'a>) -> Self {
            self.input = input;
            self
        }

        pub fn dst_implicit_data(mut self, dst_implicit_data: DeviceAddress) -> Self {
            self.dst_implicit_data = dst_implicit_data;
            self
        }

        pub fn scratch_data(mut self, scratch_data: DeviceAddress) -> Self {
            self.scratch_data = scratch_data;
            self
        }

        pub fn dst_addresses_array(
            mut self,
            dst_addresses_array: StridedDeviceAddressRegionKHR,
        ) -> Self {
            self.dst_addresses_array = dst_addresses_array;
            self
        }

        pub fn dst_sizes_array(mut self, dst_sizes_array: StridedDeviceAddressRegionKHR) -> Self {
            self.dst_sizes_array = dst_sizes_array;
            self
        }

        pub fn src_infos_array(mut self, src_infos_array: StridedDeviceAddressRegionKHR) -> Self {
            self.src_infos_array = src_infos_array;
            self
        }

        pub fn src_infos_count(mut self, src_infos_count: DeviceAddress) -> Self {
            self.src_infos_count = src_infos_count;
            self
        }

        pub fn address_resolution_flags(
            mut self,
            address_resolution_flags: ClusterAccelerationStructureAddressResolutionFlagsNV,
        ) -> Self {
            self.address_resolution_flags = address_resolution_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureOpInputNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union ClusterAccelerationStructureOpInputNV<'a> {
        pub p_clusters_bottom_level:
            *mut ClusterAccelerationStructureClustersBottomLevelInputNV<'a>,
        pub p_triangle_clusters: *mut ClusterAccelerationStructureTriangleClusterInputNV<'a>,
        pub p_move_objects: *mut ClusterAccelerationStructureMoveObjectsInputNV<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for ClusterAccelerationStructureOpInputNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ClusterAccelerationStructureOpInputNV")
                .finish()
        }
    }

    impl Default for ClusterAccelerationStructureOpInputNV<'_> {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureTypeNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ClusterAccelerationStructureTypeNV(i32);

    impl ClusterAccelerationStructureTypeNV {
        pub const CLUSTERS_BOTTOM_LEVEL_NV: Self = Self(0);
        pub const TRIANGLE_CLUSTER_NV: Self = Self(1);
        pub const TRIANGLE_CLUSTER_TEMPLATE_NV: Self = Self(2);
    }

    impl fmt::Debug for ClusterAccelerationStructureTypeNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::CLUSTERS_BOTTOM_LEVEL_NV => Some("CLUSTERS_BOTTOM_LEVEL_NV"),
                Self::TRIANGLE_CLUSTER_NV => Some("TRIANGLE_CLUSTER_NV"),
                Self::TRIANGLE_CLUSTER_TEMPLATE_NV => Some("TRIANGLE_CLUSTER_TEMPLATE_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureOpTypeNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ClusterAccelerationStructureOpTypeNV(i32);

    impl ClusterAccelerationStructureOpTypeNV {
        pub const MOVE_OBJECTS_NV: Self = Self(0);
        pub const BUILD_CLUSTERS_BOTTOM_LEVEL_NV: Self = Self(1);
        pub const BUILD_TRIANGLE_CLUSTER_NV: Self = Self(2);
        pub const BUILD_TRIANGLE_CLUSTER_TEMPLATE_NV: Self = Self(3);
        pub const INSTANTIATE_TRIANGLE_CLUSTER_NV: Self = Self(4);
        pub const GET_CLUSTER_TEMPLATE_INDICES_NV: Self = Self(5);
    }

    impl fmt::Debug for ClusterAccelerationStructureOpTypeNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::MOVE_OBJECTS_NV => Some("MOVE_OBJECTS_NV"),
                Self::BUILD_CLUSTERS_BOTTOM_LEVEL_NV => Some("BUILD_CLUSTERS_BOTTOM_LEVEL_NV"),
                Self::BUILD_TRIANGLE_CLUSTER_NV => Some("BUILD_TRIANGLE_CLUSTER_NV"),
                Self::BUILD_TRIANGLE_CLUSTER_TEMPLATE_NV => {
                    Some("BUILD_TRIANGLE_CLUSTER_TEMPLATE_NV")
                }
                Self::INSTANTIATE_TRIANGLE_CLUSTER_NV => Some("INSTANTIATE_TRIANGLE_CLUSTER_NV"),
                Self::GET_CLUSTER_TEMPLATE_INDICES_NV => Some("GET_CLUSTER_TEMPLATE_INDICES_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureOpModeNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ClusterAccelerationStructureOpModeNV(i32);

    impl ClusterAccelerationStructureOpModeNV {
        pub const IMPLICIT_DESTINATIONS_NV: Self = Self(0);
        pub const EXPLICIT_DESTINATIONS_NV: Self = Self(1);
        pub const COMPUTE_SIZES_NV: Self = Self(2);
    }

    impl fmt::Debug for ClusterAccelerationStructureOpModeNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::IMPLICIT_DESTINATIONS_NV => Some("IMPLICIT_DESTINATIONS_NV"),
                Self::EXPLICIT_DESTINATIONS_NV => Some("EXPLICIT_DESTINATIONS_NV"),
                Self::COMPUTE_SIZES_NV => Some("COMPUTE_SIZES_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureGeometryFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ClusterAccelerationStructureGeometryFlagsNV(Flags);
    vk_bitflags_wrapped!(ClusterAccelerationStructureGeometryFlagsNV, Flags);

    impl ClusterAccelerationStructureGeometryFlagsNV {
        pub const CULL_DISABLE_NV: Self =
            Self(ClusterAccelerationStructureGeometryFlagBitsNV::CULL_DISABLE_NV.0);
        pub const NO_DUPLICATE_ANYHIT_INVOCATION_NV: Self = Self(
            ClusterAccelerationStructureGeometryFlagBitsNV::NO_DUPLICATE_ANYHIT_INVOCATION_NV.0,
        );
        pub const OPAQUE_NV: Self =
            Self(ClusterAccelerationStructureGeometryFlagBitsNV::OPAQUE_NV.0);
    }

    impl fmt::Debug for ClusterAccelerationStructureGeometryFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    ClusterAccelerationStructureGeometryFlagsNV::CULL_DISABLE_NV.0,
                    "CULL_DISABLE_NV",
                ),
                (
                    ClusterAccelerationStructureGeometryFlagsNV::NO_DUPLICATE_ANYHIT_INVOCATION_NV
                        .0,
                    "NO_DUPLICATE_ANYHIT_INVOCATION_NV",
                ),
                (
                    ClusterAccelerationStructureGeometryFlagsNV::OPAQUE_NV.0,
                    "OPAQUE_NV",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureGeometryFlagBitsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ClusterAccelerationStructureGeometryFlagBitsNV(u32);

    impl ClusterAccelerationStructureGeometryFlagBitsNV {
        pub const CULL_DISABLE_NV: Self = Self(1 << 0);
        pub const NO_DUPLICATE_ANYHIT_INVOCATION_NV: Self = Self(1 << 1);
        pub const OPAQUE_NV: Self = Self(1 << 2);
    }

    impl fmt::Debug for ClusterAccelerationStructureGeometryFlagBitsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::CULL_DISABLE_NV => Some("CULL_DISABLE_NV"),
                Self::NO_DUPLICATE_ANYHIT_INVOCATION_NV => {
                    Some("NO_DUPLICATE_ANYHIT_INVOCATION_NV")
                }
                Self::OPAQUE_NV => Some("OPAQUE_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureClusterFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ClusterAccelerationStructureClusterFlagsNV(Flags);
    vk_bitflags_wrapped!(ClusterAccelerationStructureClusterFlagsNV, Flags);

    impl ClusterAccelerationStructureClusterFlagsNV {
        pub const ALLOW_DISABLE_OPACITY_MICROMAPS_NV: Self = Self(
            ClusterAccelerationStructureClusterFlagBitsNV::ALLOW_DISABLE_OPACITY_MICROMAPS_NV.0,
        );
    }

    impl fmt::Debug for ClusterAccelerationStructureClusterFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[(
                ClusterAccelerationStructureClusterFlagsNV::ALLOW_DISABLE_OPACITY_MICROMAPS_NV.0,
                "ALLOW_DISABLE_OPACITY_MICROMAPS_NV",
            )];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureClusterFlagBitsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ClusterAccelerationStructureClusterFlagBitsNV(u32);

    impl ClusterAccelerationStructureClusterFlagBitsNV {
        pub const ALLOW_DISABLE_OPACITY_MICROMAPS_NV: Self = Self(1 << 0);
    }

    impl fmt::Debug for ClusterAccelerationStructureClusterFlagBitsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ALLOW_DISABLE_OPACITY_MICROMAPS_NV => {
                    Some("ALLOW_DISABLE_OPACITY_MICROMAPS_NV")
                }
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureAddressResolutionFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ClusterAccelerationStructureAddressResolutionFlagsNV(Flags);
    vk_bitflags_wrapped!(ClusterAccelerationStructureAddressResolutionFlagsNV, Flags);

    impl ClusterAccelerationStructureAddressResolutionFlagsNV {
        pub const INDIRECTED_DST_IMPLICIT_DATA_NV: Self = Self(ClusterAccelerationStructureAddressResolutionFlagBitsNV::INDIRECTED_DST_IMPLICIT_DATA_NV.0);
        pub const INDIRECTED_SCRATCH_DATA_NV: Self = Self(
            ClusterAccelerationStructureAddressResolutionFlagBitsNV::INDIRECTED_SCRATCH_DATA_NV.0,
        );
        pub const INDIRECTED_DST_ADDRESS_ARRAY_NV: Self = Self(ClusterAccelerationStructureAddressResolutionFlagBitsNV::INDIRECTED_DST_ADDRESS_ARRAY_NV.0);
        pub const INDIRECTED_DST_SIZES_ARRAY_NV: Self = Self(
            ClusterAccelerationStructureAddressResolutionFlagBitsNV::INDIRECTED_DST_SIZES_ARRAY_NV
                .0,
        );
        pub const INDIRECTED_SRC_INFOS_ARRAY_NV: Self = Self(
            ClusterAccelerationStructureAddressResolutionFlagBitsNV::INDIRECTED_SRC_INFOS_ARRAY_NV
                .0,
        );
        pub const INDIRECTED_SRC_INFOS_COUNT_NV: Self = Self(
            ClusterAccelerationStructureAddressResolutionFlagBitsNV::INDIRECTED_SRC_INFOS_COUNT_NV
                .0,
        );
        pub const NONE: Self = Self(0);
    }

    impl fmt::Debug for ClusterAccelerationStructureAddressResolutionFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (ClusterAccelerationStructureAddressResolutionFlagsNV::INDIRECTED_DST_IMPLICIT_DATA_NV.0, "INDIRECTED_DST_IMPLICIT_DATA_NV"),
                (ClusterAccelerationStructureAddressResolutionFlagsNV::INDIRECTED_SCRATCH_DATA_NV.0, "INDIRECTED_SCRATCH_DATA_NV"),
                (ClusterAccelerationStructureAddressResolutionFlagsNV::INDIRECTED_DST_ADDRESS_ARRAY_NV.0, "INDIRECTED_DST_ADDRESS_ARRAY_NV"),
                (ClusterAccelerationStructureAddressResolutionFlagsNV::INDIRECTED_DST_SIZES_ARRAY_NV.0, "INDIRECTED_DST_SIZES_ARRAY_NV"),
                (ClusterAccelerationStructureAddressResolutionFlagsNV::INDIRECTED_SRC_INFOS_ARRAY_NV.0, "INDIRECTED_SRC_INFOS_ARRAY_NV"),
                (ClusterAccelerationStructureAddressResolutionFlagsNV::INDIRECTED_SRC_INFOS_COUNT_NV.0, "INDIRECTED_SRC_INFOS_COUNT_NV"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureAddressResolutionFlagBitsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ClusterAccelerationStructureAddressResolutionFlagBitsNV(u32);

    impl ClusterAccelerationStructureAddressResolutionFlagBitsNV {
        pub const INDIRECTED_DST_IMPLICIT_DATA_NV: Self = Self(1 << 0);
        pub const INDIRECTED_SCRATCH_DATA_NV: Self = Self(1 << 1);
        pub const INDIRECTED_DST_ADDRESS_ARRAY_NV: Self = Self(1 << 2);
        pub const INDIRECTED_DST_SIZES_ARRAY_NV: Self = Self(1 << 3);
        pub const INDIRECTED_SRC_INFOS_ARRAY_NV: Self = Self(1 << 4);
        pub const INDIRECTED_SRC_INFOS_COUNT_NV: Self = Self(1 << 5);
    }

    impl fmt::Debug for ClusterAccelerationStructureAddressResolutionFlagBitsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::INDIRECTED_DST_IMPLICIT_DATA_NV => Some("INDIRECTED_DST_IMPLICIT_DATA_NV"),
                Self::INDIRECTED_SCRATCH_DATA_NV => Some("INDIRECTED_SCRATCH_DATA_NV"),
                Self::INDIRECTED_DST_ADDRESS_ARRAY_NV => Some("INDIRECTED_DST_ADDRESS_ARRAY_NV"),
                Self::INDIRECTED_DST_SIZES_ARRAY_NV => Some("INDIRECTED_DST_SIZES_ARRAY_NV"),
                Self::INDIRECTED_SRC_INFOS_ARRAY_NV => Some("INDIRECTED_SRC_INFOS_ARRAY_NV"),
                Self::INDIRECTED_SRC_INFOS_COUNT_NV => Some("INDIRECTED_SRC_INFOS_COUNT_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureIndexFormatFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ClusterAccelerationStructureIndexFormatFlagsNV(Flags);
    vk_bitflags_wrapped!(ClusterAccelerationStructureIndexFormatFlagsNV, Flags);

    impl ClusterAccelerationStructureIndexFormatFlagsNV {
        pub const _8BIT_NV: Self =
            Self(ClusterAccelerationStructureIndexFormatFlagBitsNV::_8BIT_NV.0);
        pub const _16BIT_NV: Self =
            Self(ClusterAccelerationStructureIndexFormatFlagBitsNV::_16BIT_NV.0);
        pub const _32BIT_NV: Self =
            Self(ClusterAccelerationStructureIndexFormatFlagBitsNV::_32BIT_NV.0);
    }

    impl fmt::Debug for ClusterAccelerationStructureIndexFormatFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    ClusterAccelerationStructureIndexFormatFlagsNV::_8BIT_NV.0,
                    "_8BIT_NV",
                ),
                (
                    ClusterAccelerationStructureIndexFormatFlagsNV::_16BIT_NV.0,
                    "_16BIT_NV",
                ),
                (
                    ClusterAccelerationStructureIndexFormatFlagsNV::_32BIT_NV.0,
                    "_32BIT_NV",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkClusterAccelerationStructureIndexFormatFlagBitsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ClusterAccelerationStructureIndexFormatFlagBitsNV(u32);

    impl ClusterAccelerationStructureIndexFormatFlagBitsNV {
        pub const _8BIT_NV: Self = Self(1 << 0);
        pub const _16BIT_NV: Self = Self(1 << 1);
        pub const _32BIT_NV: Self = Self(1 << 2);
    }

    impl fmt::Debug for ClusterAccelerationStructureIndexFormatFlagBitsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_8BIT_NV => Some("_8BIT_NV"),
                Self::_16BIT_NV => Some("_16BIT_NV"),
                Self::_32BIT_NV => Some("_32BIT_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetClusterAccelerationStructureBuildSizesNV.html>
    pub type PFN_vkGetClusterAccelerationStructureBuildSizesNV = unsafe extern "system" fn(
        device: Device,
        p_info: *const ClusterAccelerationStructureInputInfoNV<'_>,
        p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBuildClusterAccelerationStructureIndirectNV.html>
    pub type PFN_vkCmdBuildClusterAccelerationStructureIndirectNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_command_infos: *const ClusterAccelerationStructureCommandsInfoNV<'_>,
    );
}

pub struct DeviceFn {
    get_cluster_acceleration_structure_build_sizes_nv:
        PFN_vkGetClusterAccelerationStructureBuildSizesNV,
    cmd_build_cluster_acceleration_structure_indirect_nv:
        PFN_vkCmdBuildClusterAccelerationStructureIndirectNV,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_cluster_acceleration_structure_build_sizes_nv: transmute(
                    load(c"vkGetClusterAccelerationStructureBuildSizesNV")
                        .ok_or(MissingEntryPointError)?,
                ),
                cmd_build_cluster_acceleration_structure_indirect_nv: transmute(
                    load(c"vkCmdBuildClusterAccelerationStructureIndirectNV")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetClusterAccelerationStructureBuildSizesNV.html>
    pub unsafe fn get_cluster_acceleration_structure_build_sizes_nv(
        &self,
        device: Device,
        info: &ClusterAccelerationStructureInputInfoNV<'_>,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ) {
        unsafe { (self.get_cluster_acceleration_structure_build_sizes_nv)(device, info, size_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBuildClusterAccelerationStructureIndirectNV.html>
    pub unsafe fn cmd_build_cluster_acceleration_structure_indirect_nv(
        &self,
        command_buffer: CommandBuffer,
        command_infos: &ClusterAccelerationStructureCommandsInfoNV<'_>,
    ) {
        unsafe {
            (self.cmd_build_cluster_acceleration_structure_indirect_nv)(
                command_buffer,
                command_infos,
            )
        }
    }
}
