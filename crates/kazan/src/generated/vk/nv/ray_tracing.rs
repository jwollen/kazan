//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_ray_tracing.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_ray_tracing";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    handle_nondispatchable!(
        AccelerationStructureNV,
        ACCELERATION_STRUCTURE_NV,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureNV.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyAccelerationStructureModeNV.html>
    pub type CopyAccelerationStructureModeNV = CopyAccelerationStructureModeKHR;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureTypeNV.html>
    pub type AccelerationStructureTypeNV = AccelerationStructureTypeKHR;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeometryTypeNV.html>
    pub type GeometryTypeNV = GeometryTypeKHR;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRayTracingShaderGroupTypeNV.html>
    pub type RayTracingShaderGroupTypeNV = RayTracingShaderGroupTypeKHR;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAabbPositionsNV.html>
    pub type AabbPositionsNV = AabbPositionsKHR;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTransformMatrixNV.html>
    pub type TransformMatrixNV = TransformMatrixKHR;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureInstanceNV.html>
    pub type AccelerationStructureInstanceNV = AccelerationStructureInstanceKHR;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeometryFlagsNV.html>
    pub type GeometryFlagsNV = GeometryFlagsKHR;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeometryInstanceFlagsNV.html>
    pub type GeometryInstanceFlagsNV = GeometryInstanceFlagsKHR;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBuildAccelerationStructureFlagsNV.html>
    pub type BuildAccelerationStructureFlagsNV = BuildAccelerationStructureFlagsKHR;
    pub type PFN_vkGetRayTracingShaderGroupHandlesNV = PFN_vkGetRayTracingShaderGroupHandlesKHR;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRayTracingShaderGroupCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RayTracingShaderGroupCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub ty: RayTracingShaderGroupTypeKHR,
        pub general_shader: u32,
        pub closest_hit_shader: u32,
        pub any_hit_shader: u32,
        pub intersection_shader: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RayTracingShaderGroupCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RayTracingShaderGroupCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("ty", &self.ty)
                .field("general_shader", &self.general_shader)
                .field("closest_hit_shader", &self.closest_hit_shader)
                .field("any_hit_shader", &self.any_hit_shader)
                .field("intersection_shader", &self.intersection_shader)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RayTracingShaderGroupCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV;
    }

    impl Default for RayTracingShaderGroupCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                ty: Default::default(),
                general_shader: Default::default(),
                closest_hit_shader: Default::default(),
                any_hit_shader: Default::default(),
                intersection_shader: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RayTracingShaderGroupCreateInfoNV<'a> {
        #[inline]
        pub fn ty(mut self, ty: RayTracingShaderGroupTypeKHR) -> Self {
            self.ty = ty;
            self
        }

        #[inline]
        pub fn general_shader(mut self, general_shader: u32) -> Self {
            self.general_shader = general_shader;
            self
        }

        #[inline]
        pub fn closest_hit_shader(mut self, closest_hit_shader: u32) -> Self {
            self.closest_hit_shader = closest_hit_shader;
            self
        }

        #[inline]
        pub fn any_hit_shader(mut self, any_hit_shader: u32) -> Self {
            self.any_hit_shader = any_hit_shader;
            self
        }

        #[inline]
        pub fn intersection_shader(mut self, intersection_shader: u32) -> Self {
            self.intersection_shader = intersection_shader;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRayTracingPipelineCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RayTracingPipelineCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineCreateFlags,
        pub stage_count: u32,
        pub p_stages: *const PipelineShaderStageCreateInfo<'a>,
        pub group_count: u32,
        pub p_groups: *const RayTracingShaderGroupCreateInfoNV<'a>,
        pub max_recursion_depth: u32,
        pub layout: PipelineLayout,
        pub base_pipeline_handle: Pipeline,
        pub base_pipeline_index: i32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RayTracingPipelineCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RayTracingPipelineCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("stage_count", &self.stage_count)
                .field("p_stages", &self.p_stages)
                .field("group_count", &self.group_count)
                .field("p_groups", &self.p_groups)
                .field("max_recursion_depth", &self.max_recursion_depth)
                .field("layout", &self.layout)
                .field("base_pipeline_handle", &self.base_pipeline_handle)
                .field("base_pipeline_index", &self.base_pipeline_index)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RayTracingPipelineCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RAY_TRACING_PIPELINE_CREATE_INFO_NV;
    }

    impl Default for RayTracingPipelineCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                stage_count: Default::default(),
                p_stages: ptr::null(),
                group_count: Default::default(),
                p_groups: ptr::null(),
                max_recursion_depth: Default::default(),
                layout: Default::default(),
                base_pipeline_handle: Default::default(),
                base_pipeline_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RayTracingPipelineCreateInfoNV<'a> {
        #[inline]
        pub fn flags(mut self, flags: PipelineCreateFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn stages(mut self, stages: &'a [PipelineShaderStageCreateInfo<'_>]) -> Self {
            self.stage_count = stages.len().try_into().unwrap();
            self.p_stages = stages.as_ptr() as _;
            self
        }

        #[inline]
        pub fn groups(mut self, groups: &'a [RayTracingShaderGroupCreateInfoNV<'_>]) -> Self {
            self.group_count = groups.len().try_into().unwrap();
            self.p_groups = groups.as_ptr() as _;
            self
        }

        #[inline]
        pub fn max_recursion_depth(mut self, max_recursion_depth: u32) -> Self {
            self.max_recursion_depth = max_recursion_depth;
            self
        }

        #[inline]
        pub fn layout(mut self, layout: PipelineLayout) -> Self {
            self.layout = layout;
            self
        }

        #[inline]
        pub fn base_pipeline_handle(mut self, base_pipeline_handle: Pipeline) -> Self {
            self.base_pipeline_handle = base_pipeline_handle;
            self
        }

        #[inline]
        pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
            self.base_pipeline_index = base_pipeline_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeometryTrianglesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct GeometryTrianglesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub vertex_data: Buffer,
        pub vertex_offset: DeviceSize,
        pub vertex_count: u32,
        pub vertex_stride: DeviceSize,
        pub vertex_format: Format,
        pub index_data: Buffer,
        pub index_offset: DeviceSize,
        pub index_count: u32,
        pub index_type: IndexType,
        pub transform_data: Buffer,
        pub transform_offset: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for GeometryTrianglesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("GeometryTrianglesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("vertex_data", &self.vertex_data)
                .field("vertex_offset", &self.vertex_offset)
                .field("vertex_count", &self.vertex_count)
                .field("vertex_stride", &self.vertex_stride)
                .field("vertex_format", &self.vertex_format)
                .field("index_data", &self.index_data)
                .field("index_offset", &self.index_offset)
                .field("index_count", &self.index_count)
                .field("index_type", &self.index_type)
                .field("transform_data", &self.transform_data)
                .field("transform_offset", &self.transform_offset)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for GeometryTrianglesNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::GEOMETRY_TRIANGLES_NV;
    }

    impl Default for GeometryTrianglesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                vertex_data: Default::default(),
                vertex_offset: Default::default(),
                vertex_count: Default::default(),
                vertex_stride: Default::default(),
                vertex_format: Default::default(),
                index_data: Default::default(),
                index_offset: Default::default(),
                index_count: Default::default(),
                index_type: Default::default(),
                transform_data: Default::default(),
                transform_offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> GeometryTrianglesNV<'a> {
        #[inline]
        pub fn vertex_data(mut self, vertex_data: Buffer) -> Self {
            self.vertex_data = vertex_data;
            self
        }

        #[inline]
        pub fn vertex_offset(mut self, vertex_offset: DeviceSize) -> Self {
            self.vertex_offset = vertex_offset;
            self
        }

        #[inline]
        pub fn vertex_count(mut self, vertex_count: u32) -> Self {
            self.vertex_count = vertex_count;
            self
        }

        #[inline]
        pub fn vertex_stride(mut self, vertex_stride: DeviceSize) -> Self {
            self.vertex_stride = vertex_stride;
            self
        }

        #[inline]
        pub fn vertex_format(mut self, vertex_format: Format) -> Self {
            self.vertex_format = vertex_format;
            self
        }

        #[inline]
        pub fn index_data(mut self, index_data: Buffer) -> Self {
            self.index_data = index_data;
            self
        }

        #[inline]
        pub fn index_offset(mut self, index_offset: DeviceSize) -> Self {
            self.index_offset = index_offset;
            self
        }

        #[inline]
        pub fn index_count(mut self, index_count: u32) -> Self {
            self.index_count = index_count;
            self
        }

        #[inline]
        pub fn index_type(mut self, index_type: IndexType) -> Self {
            self.index_type = index_type;
            self
        }

        #[inline]
        pub fn transform_data(mut self, transform_data: Buffer) -> Self {
            self.transform_data = transform_data;
            self
        }

        #[inline]
        pub fn transform_offset(mut self, transform_offset: DeviceSize) -> Self {
            self.transform_offset = transform_offset;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeometryAABBNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct GeometryAABBNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub aabb_data: Buffer,
        pub num_aab_bs: u32,
        pub stride: u32,
        pub offset: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for GeometryAABBNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("GeometryAABBNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("aabb_data", &self.aabb_data)
                .field("num_aab_bs", &self.num_aab_bs)
                .field("stride", &self.stride)
                .field("offset", &self.offset)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for GeometryAABBNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::GEOMETRY_AABB_NV;
    }

    impl Default for GeometryAABBNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                aabb_data: Default::default(),
                num_aab_bs: Default::default(),
                stride: Default::default(),
                offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> GeometryAABBNV<'a> {
        #[inline]
        pub fn aabb_data(mut self, aabb_data: Buffer) -> Self {
            self.aabb_data = aabb_data;
            self
        }

        #[inline]
        pub fn num_aab_bs(mut self, num_aab_bs: u32) -> Self {
            self.num_aab_bs = num_aab_bs;
            self
        }

        #[inline]
        pub fn stride(mut self, stride: u32) -> Self {
            self.stride = stride;
            self
        }

        #[inline]
        pub fn offset(mut self, offset: DeviceSize) -> Self {
            self.offset = offset;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeometryDataNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct GeometryDataNV<'a> {
        pub triangles: GeometryTrianglesNV<'a>,
        pub aabbs: GeometryAABBNV<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for GeometryDataNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("GeometryDataNV")
                .field("triangles", &self.triangles)
                .field("aabbs", &self.aabbs)
                .finish()
        }
    }

    impl Default for GeometryDataNV<'_> {
        fn default() -> Self {
            Self {
                triangles: Default::default(),
                aabbs: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> GeometryDataNV<'a> {
        #[inline]
        pub fn triangles(mut self, triangles: GeometryTrianglesNV<'a>) -> Self {
            self.triangles = triangles;
            self
        }

        #[inline]
        pub fn aabbs(mut self, aabbs: GeometryAABBNV<'a>) -> Self {
            self.aabbs = aabbs;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeometryNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct GeometryNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub geometry_type: GeometryTypeKHR,
        pub geometry: GeometryDataNV<'a>,
        pub flags: GeometryFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for GeometryNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("GeometryNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("geometry_type", &self.geometry_type)
                .field("geometry", &self.geometry)
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for GeometryNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::GEOMETRY_NV;
    }

    impl Default for GeometryNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                geometry_type: Default::default(),
                geometry: Default::default(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> GeometryNV<'a> {
        #[inline]
        pub fn geometry_type(mut self, geometry_type: GeometryTypeKHR) -> Self {
            self.geometry_type = geometry_type;
            self
        }

        #[inline]
        pub fn geometry(mut self, geometry: GeometryDataNV<'a>) -> Self {
            self.geometry = geometry;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: GeometryFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub ty: AccelerationStructureTypeNV,
        pub flags: BuildAccelerationStructureFlagsNV,
        pub instance_count: u32,
        pub geometry_count: u32,
        pub p_geometries: *const GeometryNV<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("ty", &self.ty)
                .field("flags", &self.flags)
                .field("instance_count", &self.instance_count)
                .field("geometry_count", &self.geometry_count)
                .field("p_geometries", &self.p_geometries)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ACCELERATION_STRUCTURE_INFO_NV;
    }

    impl Default for AccelerationStructureInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                ty: Default::default(),
                flags: Default::default(),
                instance_count: Default::default(),
                geometry_count: Default::default(),
                p_geometries: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureInfoNV<'a> {
        #[inline]
        pub fn ty(mut self, ty: AccelerationStructureTypeNV) -> Self {
            self.ty = ty;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: BuildAccelerationStructureFlagsNV) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn instance_count(mut self, instance_count: u32) -> Self {
            self.instance_count = instance_count;
            self
        }

        #[inline]
        pub fn geometries(mut self, geometries: &'a [GeometryNV<'_>]) -> Self {
            self.geometry_count = geometries.len().try_into().unwrap();
            self.p_geometries = geometries.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub compacted_size: DeviceSize,
        pub info: AccelerationStructureInfoNV<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("compacted_size", &self.compacted_size)
                .field("info", &self.info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ACCELERATION_STRUCTURE_CREATE_INFO_NV;
    }

    impl Default for AccelerationStructureCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                compacted_size: Default::default(),
                info: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureCreateInfoNV<'a> {
        #[inline]
        pub fn compacted_size(mut self, compacted_size: DeviceSize) -> Self {
            self.compacted_size = compacted_size;
            self
        }

        #[inline]
        pub fn info(mut self, info: AccelerationStructureInfoNV<'a>) -> Self {
            self.info = info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindAccelerationStructureMemoryInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BindAccelerationStructureMemoryInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub acceleration_structure: AccelerationStructureNV,
        pub memory: DeviceMemory,
        pub memory_offset: DeviceSize,
        pub device_index_count: u32,
        pub p_device_indices: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BindAccelerationStructureMemoryInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BindAccelerationStructureMemoryInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("acceleration_structure", &self.acceleration_structure)
                .field("memory", &self.memory)
                .field("memory_offset", &self.memory_offset)
                .field("device_index_count", &self.device_index_count)
                .field("p_device_indices", &self.p_device_indices)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BindAccelerationStructureMemoryInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV;
    }

    impl Default for BindAccelerationStructureMemoryInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                acceleration_structure: Default::default(),
                memory: Default::default(),
                memory_offset: Default::default(),
                device_index_count: Default::default(),
                p_device_indices: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BindAccelerationStructureMemoryInfoNV<'a> {
        #[inline]
        pub fn acceleration_structure(
            mut self,
            acceleration_structure: AccelerationStructureNV,
        ) -> Self {
            self.acceleration_structure = acceleration_structure;
            self
        }

        #[inline]
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }

        #[inline]
        pub fn memory_offset(mut self, memory_offset: DeviceSize) -> Self {
            self.memory_offset = memory_offset;
            self
        }

        #[inline]
        pub fn device_indices(mut self, device_indices: &'a [u32]) -> Self {
            self.device_index_count = device_indices.len().try_into().unwrap();
            self.p_device_indices = device_indices.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkWriteDescriptorSetAccelerationStructureNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct WriteDescriptorSetAccelerationStructureNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub acceleration_structure_count: u32,
        pub p_acceleration_structures: *const AccelerationStructureNV,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for WriteDescriptorSetAccelerationStructureNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("WriteDescriptorSetAccelerationStructureNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "acceleration_structure_count",
                    &self.acceleration_structure_count,
                )
                .field("p_acceleration_structures", &self.p_acceleration_structures)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for WriteDescriptorSetAccelerationStructureNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV;
    }

    unsafe impl Extends<WriteDescriptorSet<'_>> for WriteDescriptorSetAccelerationStructureNV<'_> {}

    impl Default for WriteDescriptorSetAccelerationStructureNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                acceleration_structure_count: Default::default(),
                p_acceleration_structures: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> WriteDescriptorSetAccelerationStructureNV<'a> {
        #[inline]
        pub fn acceleration_structures(
            mut self,
            acceleration_structures: &'a [AccelerationStructureNV],
        ) -> Self {
            self.acceleration_structure_count = acceleration_structures.len().try_into().unwrap();
            self.p_acceleration_structures = acceleration_structures.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureMemoryRequirementsInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureMemoryRequirementsInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub ty: AccelerationStructureMemoryRequirementsTypeNV,
        pub acceleration_structure: AccelerationStructureNV,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureMemoryRequirementsInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureMemoryRequirementsInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("ty", &self.ty)
                .field("acceleration_structure", &self.acceleration_structure)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureMemoryRequirementsInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV;
    }

    impl Default for AccelerationStructureMemoryRequirementsInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                ty: Default::default(),
                acceleration_structure: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureMemoryRequirementsInfoNV<'a> {
        #[inline]
        pub fn ty(mut self, ty: AccelerationStructureMemoryRequirementsTypeNV) -> Self {
            self.ty = ty;
            self
        }

        #[inline]
        pub fn acceleration_structure(
            mut self,
            acceleration_structure: AccelerationStructureNV,
        ) -> Self {
            self.acceleration_structure = acceleration_structure;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRayTracingPropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceRayTracingPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_group_handle_size: u32,
        pub max_recursion_depth: u32,
        pub max_shader_group_stride: u32,
        pub shader_group_base_alignment: u32,
        pub max_geometry_count: u64,
        pub max_instance_count: u64,
        pub max_triangle_count: u64,
        pub max_descriptor_set_acceleration_structures: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceRayTracingPropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceRayTracingPropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_group_handle_size", &self.shader_group_handle_size)
                .field("max_recursion_depth", &self.max_recursion_depth)
                .field("max_shader_group_stride", &self.max_shader_group_stride)
                .field(
                    "shader_group_base_alignment",
                    &self.shader_group_base_alignment,
                )
                .field("max_geometry_count", &self.max_geometry_count)
                .field("max_instance_count", &self.max_instance_count)
                .field("max_triangle_count", &self.max_triangle_count)
                .field(
                    "max_descriptor_set_acceleration_structures",
                    &self.max_descriptor_set_acceleration_structures,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRayTracingPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>> for PhysicalDeviceRayTracingPropertiesNV<'_> {}

    impl Default for PhysicalDeviceRayTracingPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_group_handle_size: Default::default(),
                max_recursion_depth: Default::default(),
                max_shader_group_stride: Default::default(),
                shader_group_base_alignment: Default::default(),
                max_geometry_count: Default::default(),
                max_instance_count: Default::default(),
                max_triangle_count: Default::default(),
                max_descriptor_set_acceleration_structures: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRayTracingPropertiesNV<'a> {
        #[inline]
        pub fn shader_group_handle_size(mut self, shader_group_handle_size: u32) -> Self {
            self.shader_group_handle_size = shader_group_handle_size;
            self
        }

        #[inline]
        pub fn max_recursion_depth(mut self, max_recursion_depth: u32) -> Self {
            self.max_recursion_depth = max_recursion_depth;
            self
        }

        #[inline]
        pub fn max_shader_group_stride(mut self, max_shader_group_stride: u32) -> Self {
            self.max_shader_group_stride = max_shader_group_stride;
            self
        }

        #[inline]
        pub fn shader_group_base_alignment(mut self, shader_group_base_alignment: u32) -> Self {
            self.shader_group_base_alignment = shader_group_base_alignment;
            self
        }

        #[inline]
        pub fn max_geometry_count(mut self, max_geometry_count: u64) -> Self {
            self.max_geometry_count = max_geometry_count;
            self
        }

        #[inline]
        pub fn max_instance_count(mut self, max_instance_count: u64) -> Self {
            self.max_instance_count = max_instance_count;
            self
        }

        #[inline]
        pub fn max_triangle_count(mut self, max_triangle_count: u64) -> Self {
            self.max_triangle_count = max_triangle_count;
            self
        }

        #[inline]
        pub fn max_descriptor_set_acceleration_structures(
            mut self,
            max_descriptor_set_acceleration_structures: u32,
        ) -> Self {
            self.max_descriptor_set_acceleration_structures =
                max_descriptor_set_acceleration_structures;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureMemoryRequirementsTypeNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccelerationStructureMemoryRequirementsTypeNV(i32);

    impl AccelerationStructureMemoryRequirementsTypeNV {
        pub const OBJECT_NV: Self = Self(0);
        pub const BUILD_SCRATCH_NV: Self = Self(1);
        pub const UPDATE_SCRATCH_NV: Self = Self(2);
    }

    impl fmt::Debug for AccelerationStructureMemoryRequirementsTypeNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::OBJECT_NV => Some("OBJECT_NV"),
                Self::BUILD_SCRATCH_NV => Some("BUILD_SCRATCH_NV"),
                Self::UPDATE_SCRATCH_NV => Some("UPDATE_SCRATCH_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCompileDeferredNV.html>
    pub type PFN_vkCompileDeferredNV =
        unsafe extern "system" fn(device: Device, pipeline: Pipeline, shader: u32) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateAccelerationStructureNV.html>
    pub type PFN_vkCreateAccelerationStructureNV = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const AccelerationStructureCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_acceleration_structure: *mut AccelerationStructureNV,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyAccelerationStructureNV.html>
    pub type PFN_vkDestroyAccelerationStructureNV = unsafe extern "system" fn(
        device: Device,
        acceleration_structure: AccelerationStructureNV,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html>
    pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV = unsafe extern "system" fn(
        device: Device,
        p_info: *const AccelerationStructureMemoryRequirementsInfoNV<'_>,
        p_memory_requirements: *mut MemoryRequirements2KHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBindAccelerationStructureMemoryNV.html>
    pub type PFN_vkBindAccelerationStructureMemoryNV = unsafe extern "system" fn(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindAccelerationStructureMemoryInfoNV<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyAccelerationStructureNV.html>
    pub type PFN_vkCmdCopyAccelerationStructureNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        mode: CopyAccelerationStructureModeKHR,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html>
    pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const AccelerationStructureNV,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBuildAccelerationStructureNV.html>
    pub type PFN_vkCmdBuildAccelerationStructureNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const AccelerationStructureInfoNV<'_>,
        instance_data: Buffer,
        instance_offset: DeviceSize,
        update: Bool32,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        scratch: Buffer,
        scratch_offset: DeviceSize,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdTraceRaysNV.html>
    pub type PFN_vkCmdTraceRaysNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        raygen_shader_binding_table_buffer: Buffer,
        raygen_shader_binding_offset: DeviceSize,
        miss_shader_binding_table_buffer: Buffer,
        miss_shader_binding_offset: DeviceSize,
        miss_shader_binding_stride: DeviceSize,
        hit_shader_binding_table_buffer: Buffer,
        hit_shader_binding_offset: DeviceSize,
        hit_shader_binding_stride: DeviceSize,
        callable_shader_binding_table_buffer: Buffer,
        callable_shader_binding_offset: DeviceSize,
        callable_shader_binding_stride: DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetAccelerationStructureHandleNV.html>
    pub type PFN_vkGetAccelerationStructureHandleNV = unsafe extern "system" fn(
        device: Device,
        acceleration_structure: AccelerationStructureNV,
        data_size: usize,
        p_data: *mut c_void,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateRayTracingPipelinesNV.html>
    pub type PFN_vkCreateRayTracingPipelinesNV = unsafe extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const RayTracingPipelineCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkAccelerationStructureNV = AccelerationStructureNV;
    pub type VkRayTracingShaderGroupCreateInfoNV = RayTracingShaderGroupCreateInfoNV<'static>;
    pub type VkRayTracingPipelineCreateInfoNV = RayTracingPipelineCreateInfoNV<'static>;
    pub type VkGeometryTrianglesNV = GeometryTrianglesNV<'static>;
    pub type VkGeometryAABBNV = GeometryAABBNV<'static>;
    pub type VkGeometryDataNV = GeometryDataNV<'static>;
    pub type VkGeometryNV = GeometryNV<'static>;
    pub type VkAccelerationStructureInfoNV = AccelerationStructureInfoNV<'static>;
    pub type VkAccelerationStructureCreateInfoNV = AccelerationStructureCreateInfoNV<'static>;
    pub type VkBindAccelerationStructureMemoryInfoNV =
        BindAccelerationStructureMemoryInfoNV<'static>;
    pub type VkWriteDescriptorSetAccelerationStructureNV =
        WriteDescriptorSetAccelerationStructureNV<'static>;
    pub type VkAccelerationStructureMemoryRequirementsInfoNV =
        AccelerationStructureMemoryRequirementsInfoNV<'static>;
    pub type VkPhysicalDeviceRayTracingPropertiesNV = PhysicalDeviceRayTracingPropertiesNV<'static>;
    pub type VkAccelerationStructureMemoryRequirementsTypeNV =
        AccelerationStructureMemoryRequirementsTypeNV;
    pub type VkCopyAccelerationStructureModeNV = CopyAccelerationStructureModeNV;
    pub type VkAccelerationStructureTypeNV = AccelerationStructureTypeNV;
    pub type VkGeometryTypeNV = GeometryTypeNV;
    pub type VkRayTracingShaderGroupTypeNV = RayTracingShaderGroupTypeNV;
    pub type VkAabbPositionsNV = AabbPositionsNV;
    pub type VkTransformMatrixNV = TransformMatrixNV;
    pub type VkAccelerationStructureInstanceNV = AccelerationStructureInstanceNV;
    pub type VkGeometryFlagsNV = GeometryFlagsNV;
    pub type VkGeometryInstanceFlagsNV = GeometryInstanceFlagsNV;
    pub type VkBuildAccelerationStructureFlagsNV = BuildAccelerationStructureFlagsNV;
    impl RayTracingShaderGroupCreateInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkRayTracingShaderGroupCreateInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl RayTracingPipelineCreateInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkRayTracingPipelineCreateInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl GeometryTrianglesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkGeometryTrianglesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl GeometryAABBNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkGeometryAABBNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl GeometryDataNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkGeometryDataNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl GeometryNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkGeometryNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AccelerationStructureInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkAccelerationStructureInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AccelerationStructureCreateInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkAccelerationStructureCreateInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BindAccelerationStructureMemoryInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBindAccelerationStructureMemoryInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl WriteDescriptorSetAccelerationStructureNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkWriteDescriptorSetAccelerationStructureNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AccelerationStructureMemoryRequirementsInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkAccelerationStructureMemoryRequirementsInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceRayTracingPropertiesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceRayTracingPropertiesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    create_acceleration_structure: PFN_vkCreateAccelerationStructureNV,
    destroy_acceleration_structure: PFN_vkDestroyAccelerationStructureNV,
    get_acceleration_structure_memory_requirements:
        PFN_vkGetAccelerationStructureMemoryRequirementsNV,
    bind_acceleration_structure_memory: PFN_vkBindAccelerationStructureMemoryNV,
    cmd_build_acceleration_structure: PFN_vkCmdBuildAccelerationStructureNV,
    cmd_copy_acceleration_structure: PFN_vkCmdCopyAccelerationStructureNV,
    cmd_trace_rays: PFN_vkCmdTraceRaysNV,
    create_ray_tracing_pipelines: PFN_vkCreateRayTracingPipelinesNV,
    get_ray_tracing_shader_group_handles: PFN_vkGetRayTracingShaderGroupHandlesKHR,
    get_acceleration_structure_handle: PFN_vkGetAccelerationStructureHandleNV,
    cmd_write_acceleration_structures_properties: PFN_vkCmdWriteAccelerationStructuresPropertiesNV,
    compile_deferred: PFN_vkCompileDeferredNV,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_acceleration_structure: transmute(
                    load(c"vkCreateAccelerationStructureNV").ok_or(MissingEntryPointError)?,
                ),
                destroy_acceleration_structure: transmute(
                    load(c"vkDestroyAccelerationStructureNV").ok_or(MissingEntryPointError)?,
                ),
                get_acceleration_structure_memory_requirements: transmute(
                    load(c"vkGetAccelerationStructureMemoryRequirementsNV")
                        .ok_or(MissingEntryPointError)?,
                ),
                bind_acceleration_structure_memory: transmute(
                    load(c"vkBindAccelerationStructureMemoryNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_build_acceleration_structure: transmute(
                    load(c"vkCmdBuildAccelerationStructureNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_acceleration_structure: transmute(
                    load(c"vkCmdCopyAccelerationStructureNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_trace_rays: transmute(load(c"vkCmdTraceRaysNV").ok_or(MissingEntryPointError)?),
                create_ray_tracing_pipelines: transmute(
                    load(c"vkCreateRayTracingPipelinesNV").ok_or(MissingEntryPointError)?,
                ),
                get_ray_tracing_shader_group_handles: transmute(
                    load(c"vkGetRayTracingShaderGroupHandlesNV").ok_or(MissingEntryPointError)?,
                ),
                get_acceleration_structure_handle: transmute(
                    load(c"vkGetAccelerationStructureHandleNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_write_acceleration_structures_properties: transmute(
                    load(c"vkCmdWriteAccelerationStructuresPropertiesNV")
                        .ok_or(MissingEntryPointError)?,
                ),
                compile_deferred: transmute(
                    load(c"vkCompileDeferredNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateAccelerationStructureNV.html>
    #[inline]
    pub unsafe fn create_acceleration_structure(
        &self,
        device: Device,
        create_info: &AccelerationStructureCreateInfoNV<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<AccelerationStructureNV> {
        unsafe {
            let mut acceleration_structure = core::mem::MaybeUninit::uninit();
            let result = (self.create_acceleration_structure)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                acceleration_structure.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(acceleration_structure.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyAccelerationStructureNV.html>
    #[inline]
    pub unsafe fn destroy_acceleration_structure(
        &self,
        device: Device,
        acceleration_structure: AccelerationStructureNV,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_acceleration_structure)(
                device,
                acceleration_structure,
                allocator.to_raw_ptr(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html>
    #[inline]
    pub unsafe fn get_acceleration_structure_memory_requirements(
        &self,
        device: Device,
        info: &AccelerationStructureMemoryRequirementsInfoNV<'_>,
        memory_requirements: &mut MemoryRequirements2KHR<'_>,
    ) {
        unsafe {
            (self.get_acceleration_structure_memory_requirements)(device, info, memory_requirements)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBindAccelerationStructureMemoryNV.html>
    #[inline]
    pub unsafe fn bind_acceleration_structure_memory(
        &self,
        device: Device,
        bind_infos: &[BindAccelerationStructureMemoryInfoNV<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.bind_acceleration_structure_memory)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBuildAccelerationStructureNV.html>
    #[inline]
    pub unsafe fn cmd_build_acceleration_structure(
        &self,
        command_buffer: CommandBuffer,
        info: &AccelerationStructureInfoNV<'_>,
        instance_data: Buffer,
        instance_offset: DeviceSize,
        update: bool,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        scratch: Buffer,
        scratch_offset: DeviceSize,
    ) {
        unsafe {
            (self.cmd_build_acceleration_structure)(
                command_buffer,
                info,
                instance_data,
                instance_offset,
                update.into(),
                dst,
                src,
                scratch,
                scratch_offset,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyAccelerationStructureNV.html>
    #[inline]
    pub unsafe fn cmd_copy_acceleration_structure(
        &self,
        command_buffer: CommandBuffer,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        mode: CopyAccelerationStructureModeKHR,
    ) {
        unsafe { (self.cmd_copy_acceleration_structure)(command_buffer, dst, src, mode) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdTraceRaysNV.html>
    #[inline]
    pub unsafe fn cmd_trace_rays(
        &self,
        command_buffer: CommandBuffer,
        raygen_shader_binding_table_buffer: Buffer,
        raygen_shader_binding_offset: DeviceSize,
        miss_shader_binding_table_buffer: Buffer,
        miss_shader_binding_offset: DeviceSize,
        miss_shader_binding_stride: DeviceSize,
        hit_shader_binding_table_buffer: Buffer,
        hit_shader_binding_offset: DeviceSize,
        hit_shader_binding_stride: DeviceSize,
        callable_shader_binding_table_buffer: Buffer,
        callable_shader_binding_offset: DeviceSize,
        callable_shader_binding_stride: DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        unsafe {
            (self.cmd_trace_rays)(
                command_buffer,
                raygen_shader_binding_table_buffer,
                raygen_shader_binding_offset,
                miss_shader_binding_table_buffer,
                miss_shader_binding_offset,
                miss_shader_binding_stride,
                hit_shader_binding_table_buffer,
                hit_shader_binding_offset,
                hit_shader_binding_stride,
                callable_shader_binding_table_buffer,
                callable_shader_binding_offset,
                callable_shader_binding_stride,
                width,
                height,
                depth,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateRayTracingPipelinesNV.html>
    #[inline]
    pub unsafe fn create_ray_tracing_pipelines(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_infos: &[RayTracingPipelineCreateInfoNV<'_>],
        allocator: Option<&AllocationCallbacks<'_>>,
        pipelines: &mut [Pipeline],
    ) -> crate::Result<()> {
        unsafe {
            assert_eq!(pipelines.len(), create_infos.len());
            let result = (self.create_ray_tracing_pipelines)(
                device,
                pipeline_cache,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                pipelines.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetRayTracingShaderGroupHandlesNV.html>
    #[inline]
    pub unsafe fn get_ray_tracing_shader_group_handles(
        &self,
        device: Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data: &mut [u8],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_ray_tracing_shader_group_handles)(
                device,
                pipeline,
                first_group,
                group_count,
                data.len().try_into().unwrap(),
                data.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetAccelerationStructureHandleNV.html>
    #[inline]
    pub unsafe fn get_acceleration_structure_handle(
        &self,
        device: Device,
        acceleration_structure: AccelerationStructureNV,
        data: &mut [u8],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_acceleration_structure_handle)(
                device,
                acceleration_structure,
                data.len().try_into().unwrap(),
                data.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html>
    #[inline]
    pub unsafe fn cmd_write_acceleration_structures_properties(
        &self,
        command_buffer: CommandBuffer,
        acceleration_structures: &[AccelerationStructureNV],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        unsafe {
            (self.cmd_write_acceleration_structures_properties)(
                command_buffer,
                acceleration_structures.len().try_into().unwrap(),
                acceleration_structures.as_ptr() as _,
                query_type,
                query_pool,
                first_query,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCompileDeferredNV.html>
    #[inline]
    pub unsafe fn compile_deferred(
        &self,
        device: Device,
        pipeline: Pipeline,
        shader: u32,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.compile_deferred)(device, pipeline, shader);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
