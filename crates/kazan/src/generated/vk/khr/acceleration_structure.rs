//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_acceleration_structure.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_acceleration_structure";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    handle_nondispatchable!(
        AccelerationStructureKHR,
        ACCELERATION_STRUCTURE_KHR,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureKHR.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkWriteDescriptorSetAccelerationStructureKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct WriteDescriptorSetAccelerationStructureKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub acceleration_structure_count: u32,
        pub p_acceleration_structures: *const AccelerationStructureKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for WriteDescriptorSetAccelerationStructureKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("WriteDescriptorSetAccelerationStructureKHR")
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

    unsafe impl<'a> TaggedStructure<'a> for WriteDescriptorSetAccelerationStructureKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR;
    }

    unsafe impl Extends<WriteDescriptorSet<'_>> for WriteDescriptorSetAccelerationStructureKHR<'_> {}

    impl Default for WriteDescriptorSetAccelerationStructureKHR<'_> {
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

    impl<'a> WriteDescriptorSetAccelerationStructureKHR<'a> {
        #[inline]
        pub fn acceleration_structures(
            mut self,
            acceleration_structures: &'a [AccelerationStructureKHR],
        ) -> Self {
            self.acceleration_structure_count = acceleration_structures.len().try_into().unwrap();
            self.p_acceleration_structures = acceleration_structures.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceAccelerationStructureFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceAccelerationStructureFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub acceleration_structure: Bool32,
        pub acceleration_structure_capture_replay: Bool32,
        pub acceleration_structure_indirect_build: Bool32,
        pub acceleration_structure_host_commands: Bool32,
        pub descriptor_binding_acceleration_structure_update_after_bind: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceAccelerationStructureFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceAccelerationStructureFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("acceleration_structure", &self.acceleration_structure)
                .field(
                    "acceleration_structure_capture_replay",
                    &self.acceleration_structure_capture_replay,
                )
                .field(
                    "acceleration_structure_indirect_build",
                    &self.acceleration_structure_indirect_build,
                )
                .field(
                    "acceleration_structure_host_commands",
                    &self.acceleration_structure_host_commands,
                )
                .field(
                    "descriptor_binding_acceleration_structure_update_after_bind",
                    &self.descriptor_binding_acceleration_structure_update_after_bind,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceAccelerationStructureFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceAccelerationStructureFeaturesKHR<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceAccelerationStructureFeaturesKHR<'_> {}

    impl Default for PhysicalDeviceAccelerationStructureFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                acceleration_structure: Default::default(),
                acceleration_structure_capture_replay: Default::default(),
                acceleration_structure_indirect_build: Default::default(),
                acceleration_structure_host_commands: Default::default(),
                descriptor_binding_acceleration_structure_update_after_bind: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceAccelerationStructureFeaturesKHR<'a> {
        #[inline]
        pub fn acceleration_structure(mut self, acceleration_structure: bool) -> Self {
            self.acceleration_structure = acceleration_structure.into();
            self
        }

        #[inline]
        pub fn acceleration_structure_capture_replay(
            mut self,
            acceleration_structure_capture_replay: bool,
        ) -> Self {
            self.acceleration_structure_capture_replay =
                acceleration_structure_capture_replay.into();
            self
        }

        #[inline]
        pub fn acceleration_structure_indirect_build(
            mut self,
            acceleration_structure_indirect_build: bool,
        ) -> Self {
            self.acceleration_structure_indirect_build =
                acceleration_structure_indirect_build.into();
            self
        }

        #[inline]
        pub fn acceleration_structure_host_commands(
            mut self,
            acceleration_structure_host_commands: bool,
        ) -> Self {
            self.acceleration_structure_host_commands = acceleration_structure_host_commands.into();
            self
        }

        #[inline]
        pub fn descriptor_binding_acceleration_structure_update_after_bind(
            mut self,
            descriptor_binding_acceleration_structure_update_after_bind: bool,
        ) -> Self {
            self.descriptor_binding_acceleration_structure_update_after_bind =
                descriptor_binding_acceleration_structure_update_after_bind.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceAccelerationStructurePropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceAccelerationStructurePropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_geometry_count: u64,
        pub max_instance_count: u64,
        pub max_primitive_count: u64,
        pub max_per_stage_descriptor_acceleration_structures: u32,
        pub max_per_stage_descriptor_update_after_bind_acceleration_structures: u32,
        pub max_descriptor_set_acceleration_structures: u32,
        pub max_descriptor_set_update_after_bind_acceleration_structures: u32,
        pub min_acceleration_structure_scratch_offset_alignment: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceAccelerationStructurePropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceAccelerationStructurePropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_geometry_count", &self.max_geometry_count)
                .field("max_instance_count", &self.max_instance_count)
                .field("max_primitive_count", &self.max_primitive_count)
                .field(
                    "max_per_stage_descriptor_acceleration_structures",
                    &self.max_per_stage_descriptor_acceleration_structures,
                )
                .field(
                    "max_per_stage_descriptor_update_after_bind_acceleration_structures",
                    &self.max_per_stage_descriptor_update_after_bind_acceleration_structures,
                )
                .field(
                    "max_descriptor_set_acceleration_structures",
                    &self.max_descriptor_set_acceleration_structures,
                )
                .field(
                    "max_descriptor_set_update_after_bind_acceleration_structures",
                    &self.max_descriptor_set_update_after_bind_acceleration_structures,
                )
                .field(
                    "min_acceleration_structure_scratch_offset_alignment",
                    &self.min_acceleration_structure_scratch_offset_alignment,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceAccelerationStructurePropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceAccelerationStructurePropertiesKHR<'_>
    {
    }

    impl Default for PhysicalDeviceAccelerationStructurePropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_geometry_count: Default::default(),
                max_instance_count: Default::default(),
                max_primitive_count: Default::default(),
                max_per_stage_descriptor_acceleration_structures: Default::default(),
                max_per_stage_descriptor_update_after_bind_acceleration_structures:
                    Default::default(),
                max_descriptor_set_acceleration_structures: Default::default(),
                max_descriptor_set_update_after_bind_acceleration_structures: Default::default(),
                min_acceleration_structure_scratch_offset_alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceAccelerationStructurePropertiesKHR<'a> {
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
        pub fn max_primitive_count(mut self, max_primitive_count: u64) -> Self {
            self.max_primitive_count = max_primitive_count;
            self
        }

        #[inline]
        pub fn max_per_stage_descriptor_acceleration_structures(
            mut self,
            max_per_stage_descriptor_acceleration_structures: u32,
        ) -> Self {
            self.max_per_stage_descriptor_acceleration_structures =
                max_per_stage_descriptor_acceleration_structures;
            self
        }

        #[inline]
        pub fn max_per_stage_descriptor_update_after_bind_acceleration_structures(
            mut self,
            max_per_stage_descriptor_update_after_bind_acceleration_structures: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_acceleration_structures =
                max_per_stage_descriptor_update_after_bind_acceleration_structures;
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

        #[inline]
        pub fn max_descriptor_set_update_after_bind_acceleration_structures(
            mut self,
            max_descriptor_set_update_after_bind_acceleration_structures: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_acceleration_structures =
                max_descriptor_set_update_after_bind_acceleration_structures;
            self
        }

        #[inline]
        pub fn min_acceleration_structure_scratch_offset_alignment(
            mut self,
            min_acceleration_structure_scratch_offset_alignment: u32,
        ) -> Self {
            self.min_acceleration_structure_scratch_offset_alignment =
                min_acceleration_structure_scratch_offset_alignment;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureGeometryTrianglesDataKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureGeometryTrianglesDataKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub vertex_format: Format,
        pub vertex_data: DeviceOrHostAddressConstKHR<'a>,
        pub vertex_stride: DeviceSize,
        pub max_vertex: u32,
        pub index_type: IndexType,
        pub index_data: DeviceOrHostAddressConstKHR<'a>,
        pub transform_data: DeviceOrHostAddressConstKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureGeometryTrianglesDataKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureGeometryTrianglesDataKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("vertex_format", &self.vertex_format)
                .field("vertex_data", &self.vertex_data)
                .field("vertex_stride", &self.vertex_stride)
                .field("max_vertex", &self.max_vertex)
                .field("index_type", &self.index_type)
                .field("index_data", &self.index_data)
                .field("transform_data", &self.transform_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureGeometryTrianglesDataKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR;
    }

    impl Default for AccelerationStructureGeometryTrianglesDataKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                vertex_format: Default::default(),
                vertex_data: Default::default(),
                vertex_stride: Default::default(),
                max_vertex: Default::default(),
                index_type: Default::default(),
                index_data: Default::default(),
                transform_data: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureGeometryTrianglesDataKHR<'a> {
        #[inline]
        pub fn vertex_format(mut self, vertex_format: Format) -> Self {
            self.vertex_format = vertex_format;
            self
        }

        #[inline]
        pub fn vertex_data(mut self, vertex_data: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.vertex_data = vertex_data;
            self
        }

        #[inline]
        pub fn vertex_stride(mut self, vertex_stride: DeviceSize) -> Self {
            self.vertex_stride = vertex_stride;
            self
        }

        #[inline]
        pub fn max_vertex(mut self, max_vertex: u32) -> Self {
            self.max_vertex = max_vertex;
            self
        }

        #[inline]
        pub fn index_type(mut self, index_type: IndexType) -> Self {
            self.index_type = index_type;
            self
        }

        #[inline]
        pub fn index_data(mut self, index_data: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.index_data = index_data;
            self
        }

        #[inline]
        pub fn transform_data(mut self, transform_data: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.transform_data = transform_data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureGeometryAabbsDataKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureGeometryAabbsDataKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub data: DeviceOrHostAddressConstKHR<'a>,
        pub stride: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureGeometryAabbsDataKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureGeometryAabbsDataKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("data", &self.data)
                .field("stride", &self.stride)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureGeometryAabbsDataKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR;
    }

    impl Default for AccelerationStructureGeometryAabbsDataKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                data: Default::default(),
                stride: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureGeometryAabbsDataKHR<'a> {
        #[inline]
        pub fn data(mut self, data: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.data = data;
            self
        }

        #[inline]
        pub fn stride(mut self, stride: DeviceSize) -> Self {
            self.stride = stride;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureGeometryInstancesDataKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureGeometryInstancesDataKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub array_of_pointers: Bool32,
        pub data: DeviceOrHostAddressConstKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureGeometryInstancesDataKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureGeometryInstancesDataKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("array_of_pointers", &self.array_of_pointers)
                .field("data", &self.data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureGeometryInstancesDataKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR;
    }

    impl Default for AccelerationStructureGeometryInstancesDataKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                array_of_pointers: Default::default(),
                data: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureGeometryInstancesDataKHR<'a> {
        #[inline]
        pub fn array_of_pointers(mut self, array_of_pointers: bool) -> Self {
            self.array_of_pointers = array_of_pointers.into();
            self
        }

        #[inline]
        pub fn data(mut self, data: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.data = data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureGeometryKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureGeometryKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub geometry_type: GeometryTypeKHR,
        pub geometry: AccelerationStructureGeometryDataKHR<'a>,
        pub flags: GeometryFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureGeometryKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureGeometryKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("geometry_type", &self.geometry_type)
                .field("geometry", &self.geometry)
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureGeometryKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ACCELERATION_STRUCTURE_GEOMETRY_KHR;
    }

    impl Default for AccelerationStructureGeometryKHR<'_> {
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

    impl<'a> AccelerationStructureGeometryKHR<'a> {
        #[inline]
        pub fn geometry_type(mut self, geometry_type: GeometryTypeKHR) -> Self {
            self.geometry_type = geometry_type;
            self
        }

        #[inline]
        pub fn geometry(mut self, geometry: AccelerationStructureGeometryDataKHR<'a>) -> Self {
            self.geometry = geometry;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: GeometryFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureBuildGeometryInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureBuildGeometryInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub ty: AccelerationStructureTypeKHR,
        pub flags: BuildAccelerationStructureFlagsKHR,
        pub mode: BuildAccelerationStructureModeKHR,
        pub src_acceleration_structure: AccelerationStructureKHR,
        pub dst_acceleration_structure: AccelerationStructureKHR,
        pub geometry_count: u32,
        pub p_geometries: *const AccelerationStructureGeometryKHR<'a>,
        pub pp_geometries: *const *const AccelerationStructureGeometryKHR<'a>,
        pub scratch_data: DeviceOrHostAddressKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureBuildGeometryInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureBuildGeometryInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("ty", &self.ty)
                .field("flags", &self.flags)
                .field("mode", &self.mode)
                .field(
                    "src_acceleration_structure",
                    &self.src_acceleration_structure,
                )
                .field(
                    "dst_acceleration_structure",
                    &self.dst_acceleration_structure,
                )
                .field("geometry_count", &self.geometry_count)
                .field("p_geometries", &self.p_geometries)
                .field("pp_geometries", &self.pp_geometries)
                .field("scratch_data", &self.scratch_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureBuildGeometryInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR;
    }

    impl Default for AccelerationStructureBuildGeometryInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                ty: Default::default(),
                flags: Default::default(),
                mode: Default::default(),
                src_acceleration_structure: Default::default(),
                dst_acceleration_structure: Default::default(),
                geometry_count: Default::default(),
                p_geometries: ptr::null(),
                pp_geometries: ptr::null(),
                scratch_data: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureBuildGeometryInfoKHR<'a> {
        #[inline]
        pub fn ty(mut self, ty: AccelerationStructureTypeKHR) -> Self {
            self.ty = ty;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: BuildAccelerationStructureFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn mode(mut self, mode: BuildAccelerationStructureModeKHR) -> Self {
            self.mode = mode;
            self
        }

        #[inline]
        pub fn src_acceleration_structure(
            mut self,
            src_acceleration_structure: AccelerationStructureKHR,
        ) -> Self {
            self.src_acceleration_structure = src_acceleration_structure;
            self
        }

        #[inline]
        pub fn dst_acceleration_structure(
            mut self,
            dst_acceleration_structure: AccelerationStructureKHR,
        ) -> Self {
            self.dst_acceleration_structure = dst_acceleration_structure;
            self
        }

        #[inline]
        pub fn geometries(
            mut self,
            geometries: Option<&'a [AccelerationStructureGeometryKHR<'_>]>,
            geometries_ptrs: Option<&'a [&'a AccelerationStructureGeometryKHR<'_>]>,
        ) -> Self {
            self.geometry_count = None
                .or_else(|| geometries.as_deref().map(|s| s.len()))
                .or_else(|| geometries_ptrs.as_deref().map(|s| s.len()))
                .unwrap_or(0)
                .try_into()
                .unwrap();
            if let Some(s) = &geometries_ptrs {
                assert_eq!(s.len(), self.geometry_count as usize);
            }
            self.p_geometries = geometries.map_or(ptr::null(), |s| s.as_ptr() as _);
            self.pp_geometries = geometries_ptrs.map_or(ptr::null(), |s| s.as_ptr() as _);
            self
        }

        #[inline]
        pub fn scratch_data(mut self, scratch_data: DeviceOrHostAddressKHR<'a>) -> Self {
            self.scratch_data = scratch_data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureBuildRangeInfoKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct AccelerationStructureBuildRangeInfoKHR {
        pub primitive_count: u32,
        pub primitive_offset: u32,
        pub first_vertex: u32,
        pub transform_offset: u32,
    }

    impl AccelerationStructureBuildRangeInfoKHR {
        #[inline]
        pub fn primitive_count(mut self, primitive_count: u32) -> Self {
            self.primitive_count = primitive_count;
            self
        }

        #[inline]
        pub fn primitive_offset(mut self, primitive_offset: u32) -> Self {
            self.primitive_offset = primitive_offset;
            self
        }

        #[inline]
        pub fn first_vertex(mut self, first_vertex: u32) -> Self {
            self.first_vertex = first_vertex;
            self
        }

        #[inline]
        pub fn transform_offset(mut self, transform_offset: u32) -> Self {
            self.transform_offset = transform_offset;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub create_flags: AccelerationStructureCreateFlagsKHR,
        pub buffer: Buffer,
        pub offset: DeviceSize,
        pub size: DeviceSize,
        pub ty: AccelerationStructureTypeKHR,
        pub device_address: DeviceAddress,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("create_flags", &self.create_flags)
                .field("buffer", &self.buffer)
                .field("offset", &self.offset)
                .field("size", &self.size)
                .field("ty", &self.ty)
                .field("device_address", &self.device_address)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ACCELERATION_STRUCTURE_CREATE_INFO_KHR;
    }

    impl Default for AccelerationStructureCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                create_flags: Default::default(),
                buffer: Default::default(),
                offset: Default::default(),
                size: Default::default(),
                ty: Default::default(),
                device_address: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureCreateInfoKHR<'a> {
        #[inline]
        pub fn create_flags(mut self, create_flags: AccelerationStructureCreateFlagsKHR) -> Self {
            self.create_flags = create_flags;
            self
        }

        #[inline]
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }

        #[inline]
        pub fn offset(mut self, offset: DeviceSize) -> Self {
            self.offset = offset;
            self
        }

        #[inline]
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }

        #[inline]
        pub fn ty(mut self, ty: AccelerationStructureTypeKHR) -> Self {
            self.ty = ty;
            self
        }

        #[inline]
        pub fn device_address(mut self, device_address: DeviceAddress) -> Self {
            self.device_address = device_address;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAabbPositionsKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct AabbPositionsKHR {
        pub min_x: f32,
        pub min_y: f32,
        pub min_z: f32,
        pub max_x: f32,
        pub max_y: f32,
        pub max_z: f32,
    }

    impl AabbPositionsKHR {
        #[inline]
        pub fn min_x(mut self, min_x: f32) -> Self {
            self.min_x = min_x;
            self
        }

        #[inline]
        pub fn min_y(mut self, min_y: f32) -> Self {
            self.min_y = min_y;
            self
        }

        #[inline]
        pub fn min_z(mut self, min_z: f32) -> Self {
            self.min_z = min_z;
            self
        }

        #[inline]
        pub fn max_x(mut self, max_x: f32) -> Self {
            self.max_x = max_x;
            self
        }

        #[inline]
        pub fn max_y(mut self, max_y: f32) -> Self {
            self.max_y = max_y;
            self
        }

        #[inline]
        pub fn max_z(mut self, max_z: f32) -> Self {
            self.max_z = max_z;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTransformMatrixKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct TransformMatrixKHR {
        pub matrix: [[f32; 4]; 3],
    }

    impl Default for TransformMatrixKHR {
        fn default() -> Self {
            Self {
                matrix: [[Default::default(); _]; _],
            }
        }
    }

    impl TransformMatrixKHR {
        #[inline]
        pub fn matrix(mut self, matrix: [[f32; 4]; 3]) -> Self {
            self.matrix = matrix;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureInstanceKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureInstanceKHR {
        pub transform: TransformMatrixKHR,
        pub _bitfield_0: u32,
        pub _bitfield_1: u32,
        pub acceleration_structure_reference: u64,
    }

    impl Default for AccelerationStructureInstanceKHR {
        fn default() -> Self {
            Self {
                transform: Default::default(),
                _bitfield_0: Default::default(),
                _bitfield_1: Default::default(),
                acceleration_structure_reference: Default::default(),
            }
        }
    }

    impl AccelerationStructureInstanceKHR {
        #[inline]
        pub fn transform(mut self, transform: TransformMatrixKHR) -> Self {
            self.transform = transform;
            self
        }

        #[inline]
        pub fn instance_custom_index(mut self, instance_custom_index: u32) -> Self {
            set_bitfield::<0, 24>(&mut self._bitfield_0, instance_custom_index);
            self
        }

        #[inline]
        pub fn mask(mut self, mask: u32) -> Self {
            set_bitfield::<24, 8>(&mut self._bitfield_0, mask);
            self
        }

        #[inline]
        pub fn instance_shader_binding_table_record_offset(
            mut self,
            instance_shader_binding_table_record_offset: u32,
        ) -> Self {
            set_bitfield::<0, 24>(
                &mut self._bitfield_1,
                instance_shader_binding_table_record_offset,
            );
            self
        }

        #[inline]
        pub fn flags(mut self, flags: GeometryInstanceFlagsKHR) -> Self {
            set_bitfield::<24, 8>(&mut self._bitfield_1, flags.as_raw());
            self
        }

        #[inline]
        pub fn acceleration_structure_reference(
            mut self,
            acceleration_structure_reference: u64,
        ) -> Self {
            self.acceleration_structure_reference = acceleration_structure_reference;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureDeviceAddressInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureDeviceAddressInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub acceleration_structure: AccelerationStructureKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureDeviceAddressInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureDeviceAddressInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("acceleration_structure", &self.acceleration_structure)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureDeviceAddressInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR;
    }

    impl Default for AccelerationStructureDeviceAddressInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                acceleration_structure: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureDeviceAddressInfoKHR<'a> {
        #[inline]
        pub fn acceleration_structure(
            mut self,
            acceleration_structure: AccelerationStructureKHR,
        ) -> Self {
            self.acceleration_structure = acceleration_structure;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureVersionInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureVersionInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_version_data: *const u8,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureVersionInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureVersionInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_version_data", &self.p_version_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureVersionInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_VERSION_INFO_KHR;
    }

    impl Default for AccelerationStructureVersionInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_version_data: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureVersionInfoKHR<'a> {}

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyAccelerationStructureInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CopyAccelerationStructureInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src: AccelerationStructureKHR,
        pub dst: AccelerationStructureKHR,
        pub mode: CopyAccelerationStructureModeKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyAccelerationStructureInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyAccelerationStructureInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src", &self.src)
                .field("dst", &self.dst)
                .field("mode", &self.mode)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyAccelerationStructureInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_ACCELERATION_STRUCTURE_INFO_KHR;
    }

    impl Default for CopyAccelerationStructureInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src: Default::default(),
                dst: Default::default(),
                mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CopyAccelerationStructureInfoKHR<'a> {
        #[inline]
        pub fn src(mut self, src: AccelerationStructureKHR) -> Self {
            self.src = src;
            self
        }

        #[inline]
        pub fn dst(mut self, dst: AccelerationStructureKHR) -> Self {
            self.dst = dst;
            self
        }

        #[inline]
        pub fn mode(mut self, mode: CopyAccelerationStructureModeKHR) -> Self {
            self.mode = mode;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyAccelerationStructureToMemoryInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CopyAccelerationStructureToMemoryInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src: AccelerationStructureKHR,
        pub dst: DeviceOrHostAddressKHR<'a>,
        pub mode: CopyAccelerationStructureModeKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyAccelerationStructureToMemoryInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyAccelerationStructureToMemoryInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src", &self.src)
                .field("dst", &self.dst)
                .field("mode", &self.mode)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyAccelerationStructureToMemoryInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR;
    }

    impl Default for CopyAccelerationStructureToMemoryInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src: Default::default(),
                dst: Default::default(),
                mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CopyAccelerationStructureToMemoryInfoKHR<'a> {
        #[inline]
        pub fn src(mut self, src: AccelerationStructureKHR) -> Self {
            self.src = src;
            self
        }

        #[inline]
        pub fn dst(mut self, dst: DeviceOrHostAddressKHR<'a>) -> Self {
            self.dst = dst;
            self
        }

        #[inline]
        pub fn mode(mut self, mode: CopyAccelerationStructureModeKHR) -> Self {
            self.mode = mode;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyMemoryToAccelerationStructureInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CopyMemoryToAccelerationStructureInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src: DeviceOrHostAddressConstKHR<'a>,
        pub dst: AccelerationStructureKHR,
        pub mode: CopyAccelerationStructureModeKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyMemoryToAccelerationStructureInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyMemoryToAccelerationStructureInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src", &self.src)
                .field("dst", &self.dst)
                .field("mode", &self.mode)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyMemoryToAccelerationStructureInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR;
    }

    impl Default for CopyMemoryToAccelerationStructureInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src: Default::default(),
                dst: Default::default(),
                mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CopyMemoryToAccelerationStructureInfoKHR<'a> {
        #[inline]
        pub fn src(mut self, src: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.src = src;
            self
        }

        #[inline]
        pub fn dst(mut self, dst: AccelerationStructureKHR) -> Self {
            self.dst = dst;
            self
        }

        #[inline]
        pub fn mode(mut self, mode: CopyAccelerationStructureModeKHR) -> Self {
            self.mode = mode;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureBuildSizesInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureBuildSizesInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub acceleration_structure_size: DeviceSize,
        pub update_scratch_size: DeviceSize,
        pub build_scratch_size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureBuildSizesInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureBuildSizesInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "acceleration_structure_size",
                    &self.acceleration_structure_size,
                )
                .field("update_scratch_size", &self.update_scratch_size)
                .field("build_scratch_size", &self.build_scratch_size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureBuildSizesInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR;
    }

    impl Default for AccelerationStructureBuildSizesInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                acceleration_structure_size: Default::default(),
                update_scratch_size: Default::default(),
                build_scratch_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureBuildSizesInfoKHR<'a> {
        #[inline]
        pub fn acceleration_structure_size(
            mut self,
            acceleration_structure_size: DeviceSize,
        ) -> Self {
            self.acceleration_structure_size = acceleration_structure_size;
            self
        }

        #[inline]
        pub fn update_scratch_size(mut self, update_scratch_size: DeviceSize) -> Self {
            self.update_scratch_size = update_scratch_size;
            self
        }

        #[inline]
        pub fn build_scratch_size(mut self, build_scratch_size: DeviceSize) -> Self {
            self.build_scratch_size = build_scratch_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceOrHostAddressKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union DeviceOrHostAddressKHR<'a> {
        pub device_address: DeviceAddress,
        pub host_address: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceOrHostAddressKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceOrHostAddressKHR").finish()
        }
    }

    impl Default for DeviceOrHostAddressKHR<'_> {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceOrHostAddressConstKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union DeviceOrHostAddressConstKHR<'a> {
        pub device_address: DeviceAddress,
        pub host_address: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceOrHostAddressConstKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceOrHostAddressConstKHR").finish()
        }
    }

    impl Default for DeviceOrHostAddressConstKHR<'_> {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureGeometryDataKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union AccelerationStructureGeometryDataKHR<'a> {
        pub triangles: AccelerationStructureGeometryTrianglesDataKHR<'a>,
        pub aabbs: AccelerationStructureGeometryAabbsDataKHR<'a>,
        pub instances: AccelerationStructureGeometryInstancesDataKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureGeometryDataKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureGeometryDataKHR")
                .finish()
        }
    }

    impl Default for AccelerationStructureGeometryDataKHR<'_> {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyAccelerationStructureModeKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CopyAccelerationStructureModeKHR(i32);

    impl CopyAccelerationStructureModeKHR {
        pub const CLONE_KHR: Self = Self(0);
        pub const COMPACT_KHR: Self = Self(1);

        // VK_KHR_acceleration_structure
        pub const SERIALIZE_KHR: Self = Self(2);
        pub const DESERIALIZE_KHR: Self = Self(3);

        // VK_NV_ray_tracing
        pub const CLONE_NV: Self = Self::CLONE_KHR;
        pub const COMPACT_NV: Self = Self::COMPACT_KHR;
    }

    impl fmt::Debug for CopyAccelerationStructureModeKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::CLONE_KHR => Some("CLONE_KHR"),
                Self::COMPACT_KHR => Some("COMPACT_KHR"),
                Self::SERIALIZE_KHR => Some("SERIALIZE_KHR"),
                Self::DESERIALIZE_KHR => Some("DESERIALIZE_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBuildAccelerationStructureModeKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BuildAccelerationStructureModeKHR(i32);

    impl BuildAccelerationStructureModeKHR {
        pub const BUILD_KHR: Self = Self(0);
        pub const UPDATE_KHR: Self = Self(1);
    }

    impl fmt::Debug for BuildAccelerationStructureModeKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::BUILD_KHR => Some("BUILD_KHR"),
                Self::UPDATE_KHR => Some("UPDATE_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureTypeKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccelerationStructureTypeKHR(i32);

    impl AccelerationStructureTypeKHR {
        pub const TOP_LEVEL_KHR: Self = Self(0);
        pub const BOTTOM_LEVEL_KHR: Self = Self(1);
        pub const GENERIC_KHR: Self = Self(2);

        // VK_NV_ray_tracing
        pub const TOP_LEVEL_NV: Self = Self::TOP_LEVEL_KHR;
        pub const BOTTOM_LEVEL_NV: Self = Self::BOTTOM_LEVEL_KHR;
    }

    impl fmt::Debug for AccelerationStructureTypeKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::TOP_LEVEL_KHR => Some("TOP_LEVEL_KHR"),
                Self::BOTTOM_LEVEL_KHR => Some("BOTTOM_LEVEL_KHR"),
                Self::GENERIC_KHR => Some("GENERIC_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeometryTypeKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GeometryTypeKHR(i32);

    impl GeometryTypeKHR {
        pub const TRIANGLES_KHR: Self = Self(0);
        pub const AABBS_KHR: Self = Self(1);
        pub const INSTANCES_KHR: Self = Self(2);

        // VK_AMDX_dense_geometry_format
        #[cfg(feature = "provisional")]
        pub const DENSE_GEOMETRY_FORMAT_TRIANGLES_AMDX: Self = Self(1000478000);

        // VK_NV_ray_tracing
        pub const TRIANGLES_NV: Self = Self::TRIANGLES_KHR;
        pub const AABBS_NV: Self = Self::AABBS_KHR;

        // VK_NV_ray_tracing_linear_swept_spheres
        pub const SPHERES_NV: Self = Self(1000429004);
        pub const LINEAR_SWEPT_SPHERES_NV: Self = Self(1000429005);
    }

    impl fmt::Debug for GeometryTypeKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::TRIANGLES_KHR => Some("TRIANGLES_KHR"),
                Self::AABBS_KHR => Some("AABBS_KHR"),
                Self::INSTANCES_KHR => Some("INSTANCES_KHR"),
                #[cfg(feature = "provisional")]
                Self::DENSE_GEOMETRY_FORMAT_TRIANGLES_AMDX => {
                    Some("DENSE_GEOMETRY_FORMAT_TRIANGLES_AMDX")
                }
                Self::SPHERES_NV => Some("SPHERES_NV"),
                Self::LINEAR_SWEPT_SPHERES_NV => Some("LINEAR_SWEPT_SPHERES_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureBuildTypeKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccelerationStructureBuildTypeKHR(i32);

    impl AccelerationStructureBuildTypeKHR {
        pub const HOST_KHR: Self = Self(0);
        pub const DEVICE_KHR: Self = Self(1);
        pub const HOST_OR_DEVICE_KHR: Self = Self(2);
    }

    impl fmt::Debug for AccelerationStructureBuildTypeKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::HOST_KHR => Some("HOST_KHR"),
                Self::DEVICE_KHR => Some("DEVICE_KHR"),
                Self::HOST_OR_DEVICE_KHR => Some("HOST_OR_DEVICE_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureCompatibilityKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccelerationStructureCompatibilityKHR(i32);

    impl AccelerationStructureCompatibilityKHR {
        pub const COMPATIBLE_KHR: Self = Self(0);
        pub const INCOMPATIBLE_KHR: Self = Self(1);
    }

    impl fmt::Debug for AccelerationStructureCompatibilityKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::COMPATIBLE_KHR => Some("COMPATIBLE_KHR"),
                Self::INCOMPATIBLE_KHR => Some("INCOMPATIBLE_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeometryFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct GeometryFlagsKHR(Flags);
    vk_bitflags_wrapped!(GeometryFlagsKHR, Flags);

    impl GeometryFlagsKHR {
        pub const OPAQUE_KHR: Self = Self(GeometryFlagBitsKHR::OPAQUE_KHR.0);
        pub const NO_DUPLICATE_ANY_HIT_INVOCATION_KHR: Self =
            Self(GeometryFlagBitsKHR::NO_DUPLICATE_ANY_HIT_INVOCATION_KHR.0);

        // VK_NV_ray_tracing
        pub const OPAQUE_NV: Self = Self::OPAQUE_KHR;
        pub const NO_DUPLICATE_ANY_HIT_INVOCATION_NV: Self =
            Self::NO_DUPLICATE_ANY_HIT_INVOCATION_KHR;
    }

    impl fmt::Debug for GeometryFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (GeometryFlagsKHR::OPAQUE_KHR.0, "OPAQUE_KHR"),
                (
                    GeometryFlagsKHR::NO_DUPLICATE_ANY_HIT_INVOCATION_KHR.0,
                    "NO_DUPLICATE_ANY_HIT_INVOCATION_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeometryFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct GeometryFlagBitsKHR(u32);

    impl GeometryFlagBitsKHR {
        pub const OPAQUE_KHR: Self = Self(1 << 0);
        pub const NO_DUPLICATE_ANY_HIT_INVOCATION_KHR: Self = Self(1 << 1);
        // VK_NV_ray_tracing
        pub const OPAQUE_NV: Self = Self::OPAQUE_KHR;
        pub const NO_DUPLICATE_ANY_HIT_INVOCATION_NV: Self =
            Self::NO_DUPLICATE_ANY_HIT_INVOCATION_KHR;
    }

    impl fmt::Debug for GeometryFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::OPAQUE_KHR => Some("OPAQUE_KHR"),
                Self::NO_DUPLICATE_ANY_HIT_INVOCATION_KHR => {
                    Some("NO_DUPLICATE_ANY_HIT_INVOCATION_KHR")
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeometryInstanceFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct GeometryInstanceFlagsKHR(Flags);
    vk_bitflags_wrapped!(GeometryInstanceFlagsKHR, Flags);

    impl GeometryInstanceFlagsKHR {
        pub const TRIANGLE_FACING_CULL_DISABLE_KHR: Self =
            Self(GeometryInstanceFlagBitsKHR::TRIANGLE_FACING_CULL_DISABLE_KHR.0);
        pub const TRIANGLE_FLIP_FACING_KHR: Self =
            Self(GeometryInstanceFlagBitsKHR::TRIANGLE_FLIP_FACING_KHR.0);
        pub const FORCE_OPAQUE_KHR: Self = Self(GeometryInstanceFlagBitsKHR::FORCE_OPAQUE_KHR.0);
        pub const FORCE_NO_OPAQUE_KHR: Self =
            Self(GeometryInstanceFlagBitsKHR::FORCE_NO_OPAQUE_KHR.0);
        pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR: Self = Self::TRIANGLE_FLIP_FACING_KHR;

        // VK_EXT_opacity_micromap
        pub const FORCE_OPACITY_MICROMAP_2_STATE_EXT: Self =
            Self(GeometryInstanceFlagBitsKHR::FORCE_OPACITY_MICROMAP_2_STATE_EXT.0);
        pub const DISABLE_OPACITY_MICROMAPS_EXT: Self =
            Self(GeometryInstanceFlagBitsKHR::DISABLE_OPACITY_MICROMAPS_EXT.0);

        // VK_NV_ray_tracing
        pub const TRIANGLE_CULL_DISABLE_NV: Self = Self::TRIANGLE_FACING_CULL_DISABLE_KHR;
        pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_NV: Self =
            Self::TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR;
        pub const FORCE_OPAQUE_NV: Self = Self::FORCE_OPAQUE_KHR;
        pub const FORCE_NO_OPAQUE_NV: Self = Self::FORCE_NO_OPAQUE_KHR;
    }

    impl fmt::Debug for GeometryInstanceFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    GeometryInstanceFlagsKHR::TRIANGLE_FACING_CULL_DISABLE_KHR.0,
                    "TRIANGLE_FACING_CULL_DISABLE_KHR",
                ),
                (
                    GeometryInstanceFlagsKHR::TRIANGLE_FLIP_FACING_KHR.0,
                    "TRIANGLE_FLIP_FACING_KHR",
                ),
                (
                    GeometryInstanceFlagsKHR::FORCE_OPAQUE_KHR.0,
                    "FORCE_OPAQUE_KHR",
                ),
                (
                    GeometryInstanceFlagsKHR::FORCE_NO_OPAQUE_KHR.0,
                    "FORCE_NO_OPAQUE_KHR",
                ),
                (
                    GeometryInstanceFlagsKHR::FORCE_OPACITY_MICROMAP_2_STATE_EXT.0,
                    "FORCE_OPACITY_MICROMAP_2_STATE_EXT",
                ),
                (
                    GeometryInstanceFlagsKHR::DISABLE_OPACITY_MICROMAPS_EXT.0,
                    "DISABLE_OPACITY_MICROMAPS_EXT",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeometryInstanceFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct GeometryInstanceFlagBitsKHR(u32);

    impl GeometryInstanceFlagBitsKHR {
        pub const TRIANGLE_FACING_CULL_DISABLE_KHR: Self = Self(1 << 0);
        pub const TRIANGLE_FLIP_FACING_KHR: Self = Self(1 << 1);
        pub const FORCE_OPAQUE_KHR: Self = Self(1 << 2);
        pub const FORCE_NO_OPAQUE_KHR: Self = Self(1 << 3);
        // VK_EXT_opacity_micromap
        pub const FORCE_OPACITY_MICROMAP_2_STATE_EXT: Self = Self(1 << 4);
        pub const DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(1 << 5);

        // VK_NV_ray_tracing
        pub const TRIANGLE_CULL_DISABLE_NV: Self = Self::TRIANGLE_FACING_CULL_DISABLE_KHR;
        pub const FORCE_OPAQUE_NV: Self = Self::FORCE_OPAQUE_KHR;
        pub const FORCE_NO_OPAQUE_NV: Self = Self::FORCE_NO_OPAQUE_KHR;
    }

    impl fmt::Debug for GeometryInstanceFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::TRIANGLE_FACING_CULL_DISABLE_KHR => Some("TRIANGLE_FACING_CULL_DISABLE_KHR"),
                Self::TRIANGLE_FLIP_FACING_KHR => Some("TRIANGLE_FLIP_FACING_KHR"),
                Self::FORCE_OPAQUE_KHR => Some("FORCE_OPAQUE_KHR"),
                Self::FORCE_NO_OPAQUE_KHR => Some("FORCE_NO_OPAQUE_KHR"),
                Self::FORCE_OPACITY_MICROMAP_2_STATE_EXT => {
                    Some("FORCE_OPACITY_MICROMAP_2_STATE_EXT")
                }
                Self::DISABLE_OPACITY_MICROMAPS_EXT => Some("DISABLE_OPACITY_MICROMAPS_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBuildAccelerationStructureFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct BuildAccelerationStructureFlagsKHR(Flags);
    vk_bitflags_wrapped!(BuildAccelerationStructureFlagsKHR, Flags);

    impl BuildAccelerationStructureFlagsKHR {
        pub const ALLOW_UPDATE_KHR: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::ALLOW_UPDATE_KHR.0);
        pub const ALLOW_COMPACTION_KHR: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::ALLOW_COMPACTION_KHR.0);
        pub const PREFER_FAST_TRACE_KHR: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::PREFER_FAST_TRACE_KHR.0);
        pub const PREFER_FAST_BUILD_KHR: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::PREFER_FAST_BUILD_KHR.0);
        pub const LOW_MEMORY_KHR: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::LOW_MEMORY_KHR.0);

        // VK_EXT_opacity_micromap
        pub const ALLOW_OPACITY_MICROMAP_UPDATE_EXT: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::ALLOW_OPACITY_MICROMAP_UPDATE_EXT.0);
        pub const ALLOW_DISABLE_OPACITY_MICROMAPS_EXT: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::ALLOW_DISABLE_OPACITY_MICROMAPS_EXT.0);
        pub const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT.0);

        // VK_KHR_ray_tracing_position_fetch
        pub const ALLOW_DATA_ACCESS_KHR: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::ALLOW_DATA_ACCESS_KHR.0);

        // VK_NV_cluster_acceleration_structure
        pub const ALLOW_CLUSTER_OPACITY_MICROMAPS_NV: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::ALLOW_CLUSTER_OPACITY_MICROMAPS_NV.0);

        // VK_NV_displacement_micromap
        #[cfg(feature = "provisional")]
        pub const ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV.0);

        // VK_NV_ray_tracing
        pub const ALLOW_UPDATE_NV: Self = Self::ALLOW_UPDATE_KHR;
        pub const ALLOW_COMPACTION_NV: Self = Self::ALLOW_COMPACTION_KHR;
        pub const PREFER_FAST_TRACE_NV: Self = Self::PREFER_FAST_TRACE_KHR;
        pub const PREFER_FAST_BUILD_NV: Self = Self::PREFER_FAST_BUILD_KHR;
        pub const LOW_MEMORY_NV: Self = Self::LOW_MEMORY_KHR;

        // VK_NV_ray_tracing_motion_blur
        pub const MOTION_NV: Self = Self(BuildAccelerationStructureFlagBitsKHR::MOTION_NV.0);
    }

    impl fmt::Debug for BuildAccelerationStructureFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    BuildAccelerationStructureFlagsKHR::ALLOW_UPDATE_KHR.0,
                    "ALLOW_UPDATE_KHR",
                ),
                (
                    BuildAccelerationStructureFlagsKHR::ALLOW_COMPACTION_KHR.0,
                    "ALLOW_COMPACTION_KHR",
                ),
                (
                    BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE_KHR.0,
                    "PREFER_FAST_TRACE_KHR",
                ),
                (
                    BuildAccelerationStructureFlagsKHR::PREFER_FAST_BUILD_KHR.0,
                    "PREFER_FAST_BUILD_KHR",
                ),
                (
                    BuildAccelerationStructureFlagsKHR::LOW_MEMORY_KHR.0,
                    "LOW_MEMORY_KHR",
                ),
                (
                    BuildAccelerationStructureFlagsKHR::ALLOW_OPACITY_MICROMAP_UPDATE_EXT.0,
                    "ALLOW_OPACITY_MICROMAP_UPDATE_EXT",
                ),
                (
                    BuildAccelerationStructureFlagsKHR::ALLOW_DISABLE_OPACITY_MICROMAPS_EXT.0,
                    "ALLOW_DISABLE_OPACITY_MICROMAPS_EXT",
                ),
                (
                    BuildAccelerationStructureFlagsKHR::ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT.0,
                    "ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT",
                ),
                (
                    BuildAccelerationStructureFlagsKHR::ALLOW_DATA_ACCESS_KHR.0,
                    "ALLOW_DATA_ACCESS_KHR",
                ),
                (
                    BuildAccelerationStructureFlagsKHR::ALLOW_CLUSTER_OPACITY_MICROMAPS_NV.0,
                    "ALLOW_CLUSTER_OPACITY_MICROMAPS_NV",
                ),
                #[cfg(feature = "provisional")]
                (
                    BuildAccelerationStructureFlagsKHR::ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV.0,
                    "ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV",
                ),
                (BuildAccelerationStructureFlagsKHR::MOTION_NV.0, "MOTION_NV"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBuildAccelerationStructureFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct BuildAccelerationStructureFlagBitsKHR(u32);

    impl BuildAccelerationStructureFlagBitsKHR {
        pub const ALLOW_UPDATE_KHR: Self = Self(1 << 0);
        pub const ALLOW_COMPACTION_KHR: Self = Self(1 << 1);
        pub const PREFER_FAST_TRACE_KHR: Self = Self(1 << 2);
        pub const PREFER_FAST_BUILD_KHR: Self = Self(1 << 3);
        pub const LOW_MEMORY_KHR: Self = Self(1 << 4);
        // VK_EXT_opacity_micromap
        pub const ALLOW_OPACITY_MICROMAP_UPDATE_EXT: Self = Self(1 << 6);
        pub const ALLOW_DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(1 << 7);
        pub const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT: Self = Self(1 << 8);

        // VK_KHR_ray_tracing_position_fetch
        pub const ALLOW_DATA_ACCESS_KHR: Self = Self(1 << 11);

        // VK_NV_cluster_acceleration_structure
        pub const ALLOW_CLUSTER_OPACITY_MICROMAPS_NV: Self = Self(1 << 12);

        // VK_NV_displacement_micromap
        #[cfg(feature = "provisional")]
        pub const ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV: Self = Self(1 << 9);

        // VK_NV_ray_tracing
        pub const ALLOW_UPDATE_NV: Self = Self::ALLOW_UPDATE_KHR;
        pub const ALLOW_COMPACTION_NV: Self = Self::ALLOW_COMPACTION_KHR;
        pub const PREFER_FAST_TRACE_NV: Self = Self::PREFER_FAST_TRACE_KHR;
        pub const PREFER_FAST_BUILD_NV: Self = Self::PREFER_FAST_BUILD_KHR;
        pub const LOW_MEMORY_NV: Self = Self::LOW_MEMORY_KHR;

        // VK_NV_ray_tracing_motion_blur
        pub const MOTION_NV: Self = Self(1 << 5);
    }

    impl fmt::Debug for BuildAccelerationStructureFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ALLOW_UPDATE_KHR => Some("ALLOW_UPDATE_KHR"),
                Self::ALLOW_COMPACTION_KHR => Some("ALLOW_COMPACTION_KHR"),
                Self::PREFER_FAST_TRACE_KHR => Some("PREFER_FAST_TRACE_KHR"),
                Self::PREFER_FAST_BUILD_KHR => Some("PREFER_FAST_BUILD_KHR"),
                Self::LOW_MEMORY_KHR => Some("LOW_MEMORY_KHR"),
                Self::ALLOW_OPACITY_MICROMAP_UPDATE_EXT => {
                    Some("ALLOW_OPACITY_MICROMAP_UPDATE_EXT")
                }
                Self::ALLOW_DISABLE_OPACITY_MICROMAPS_EXT => {
                    Some("ALLOW_DISABLE_OPACITY_MICROMAPS_EXT")
                }
                Self::ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT => {
                    Some("ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT")
                }
                Self::ALLOW_DATA_ACCESS_KHR => Some("ALLOW_DATA_ACCESS_KHR"),
                Self::ALLOW_CLUSTER_OPACITY_MICROMAPS_NV => {
                    Some("ALLOW_CLUSTER_OPACITY_MICROMAPS_NV")
                }
                #[cfg(feature = "provisional")]
                Self::ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV => {
                    Some("ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV")
                }
                Self::MOTION_NV => Some("MOTION_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureCreateFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct AccelerationStructureCreateFlagsKHR(Flags);
    vk_bitflags_wrapped!(AccelerationStructureCreateFlagsKHR, Flags);

    impl AccelerationStructureCreateFlagsKHR {
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self =
            Self(AccelerationStructureCreateFlagBitsKHR::DEVICE_ADDRESS_CAPTURE_REPLAY_KHR.0);

        // VK_EXT_descriptor_buffer
        pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self =
            Self(AccelerationStructureCreateFlagBitsKHR::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT.0);

        // VK_NV_ray_tracing_motion_blur
        pub const MOTION_NV: Self = Self(AccelerationStructureCreateFlagBitsKHR::MOTION_NV.0);
    }

    impl fmt::Debug for AccelerationStructureCreateFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    AccelerationStructureCreateFlagsKHR::DEVICE_ADDRESS_CAPTURE_REPLAY_KHR.0,
                    "DEVICE_ADDRESS_CAPTURE_REPLAY_KHR",
                ),
                (
                    AccelerationStructureCreateFlagsKHR::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT.0,
                    "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT",
                ),
                (
                    AccelerationStructureCreateFlagsKHR::MOTION_NV.0,
                    "MOTION_NV",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureCreateFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct AccelerationStructureCreateFlagBitsKHR(u32);

    impl AccelerationStructureCreateFlagBitsKHR {
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self(1 << 0);
        // VK_EXT_descriptor_buffer
        pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(1 << 3);

        // VK_NV_ray_tracing_motion_blur
        pub const MOTION_NV: Self = Self(1 << 2);
    }

    impl fmt::Debug for AccelerationStructureCreateFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEVICE_ADDRESS_CAPTURE_REPLAY_KHR => {
                    Some("DEVICE_ADDRESS_CAPTURE_REPLAY_KHR")
                }
                Self::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT => {
                    Some("DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT")
                }
                Self::MOTION_NV => Some("MOTION_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyAccelerationStructureKHR.html>
    pub type PFN_vkDestroyAccelerationStructureKHR = unsafe extern "system" fn(
        device: Device,
        acceleration_structure: AccelerationStructureKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyAccelerationStructureKHR.html>
    pub type PFN_vkCmdCopyAccelerationStructureKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const CopyAccelerationStructureInfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyAccelerationStructureKHR.html>
    pub type PFN_vkCopyAccelerationStructureKHR = unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyAccelerationStructureInfoKHR<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html>
    pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyAccelerationStructureToMemoryKHR.html>
    pub type PFN_vkCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html>
    pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyMemoryToAccelerationStructureKHR.html>
    pub type PFN_vkCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html>
    pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const AccelerationStructureKHR,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkWriteAccelerationStructuresPropertiesKHR.html>
    pub type PFN_vkWriteAccelerationStructuresPropertiesKHR =
        unsafe extern "system" fn(
            device: Device,
            acceleration_structure_count: u32,
            p_acceleration_structures: *const AccelerationStructureKHR,
            query_type: QueryType,
            data_size: usize,
            p_data: *mut c_void,
            stride: usize,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html>
    pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR = unsafe extern "system" fn(
        device: Device,
        p_version_info: *const AccelerationStructureVersionInfoKHR<'_>,
        p_compatibility: *mut AccelerationStructureCompatibilityKHR,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateAccelerationStructureKHR.html>
    pub type PFN_vkCreateAccelerationStructureKHR = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const AccelerationStructureCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_acceleration_structure: *mut AccelerationStructureKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBuildAccelerationStructuresKHR.html>
    pub type PFN_vkCmdBuildAccelerationStructuresKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBuildAccelerationStructuresIndirectKHR.html>
    pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        p_indirect_device_addresses: *const DeviceAddress,
        p_indirect_strides: *const u32,
        pp_max_primitive_counts: *const *const u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBuildAccelerationStructuresKHR.html>
    pub type PFN_vkBuildAccelerationStructuresKHR = unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetAccelerationStructureDeviceAddressKHR.html>
    pub type PFN_vkGetAccelerationStructureDeviceAddressKHR =
        unsafe extern "system" fn(
            device: Device,
            p_info: *const AccelerationStructureDeviceAddressInfoKHR<'_>,
        ) -> DeviceAddress;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetAccelerationStructureBuildSizesKHR.html>
    pub type PFN_vkGetAccelerationStructureBuildSizesKHR = unsafe extern "system" fn(
        device: Device,
        build_type: AccelerationStructureBuildTypeKHR,
        p_build_info: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        p_max_primitive_counts: *const u32,
        p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkAccelerationStructureKHR = AccelerationStructureKHR;
    pub type VkWriteDescriptorSetAccelerationStructureKHR =
        WriteDescriptorSetAccelerationStructureKHR<'static>;
    pub type VkPhysicalDeviceAccelerationStructureFeaturesKHR =
        PhysicalDeviceAccelerationStructureFeaturesKHR<'static>;
    pub type VkPhysicalDeviceAccelerationStructurePropertiesKHR =
        PhysicalDeviceAccelerationStructurePropertiesKHR<'static>;
    pub type VkAccelerationStructureGeometryTrianglesDataKHR =
        AccelerationStructureGeometryTrianglesDataKHR<'static>;
    pub type VkAccelerationStructureGeometryAabbsDataKHR =
        AccelerationStructureGeometryAabbsDataKHR<'static>;
    pub type VkAccelerationStructureGeometryInstancesDataKHR =
        AccelerationStructureGeometryInstancesDataKHR<'static>;
    pub type VkAccelerationStructureGeometryKHR = AccelerationStructureGeometryKHR<'static>;
    pub type VkAccelerationStructureBuildGeometryInfoKHR =
        AccelerationStructureBuildGeometryInfoKHR<'static>;
    pub type VkAccelerationStructureBuildRangeInfoKHR = AccelerationStructureBuildRangeInfoKHR;
    pub type VkAccelerationStructureCreateInfoKHR = AccelerationStructureCreateInfoKHR<'static>;
    pub type VkAabbPositionsKHR = AabbPositionsKHR;
    pub type VkTransformMatrixKHR = TransformMatrixKHR;
    pub type VkAccelerationStructureInstanceKHR = AccelerationStructureInstanceKHR;
    pub type VkAccelerationStructureDeviceAddressInfoKHR =
        AccelerationStructureDeviceAddressInfoKHR<'static>;
    pub type VkAccelerationStructureVersionInfoKHR = AccelerationStructureVersionInfoKHR<'static>;
    pub type VkCopyAccelerationStructureInfoKHR = CopyAccelerationStructureInfoKHR<'static>;
    pub type VkCopyAccelerationStructureToMemoryInfoKHR =
        CopyAccelerationStructureToMemoryInfoKHR<'static>;
    pub type VkCopyMemoryToAccelerationStructureInfoKHR =
        CopyMemoryToAccelerationStructureInfoKHR<'static>;
    pub type VkAccelerationStructureBuildSizesInfoKHR =
        AccelerationStructureBuildSizesInfoKHR<'static>;
    pub type VkDeviceOrHostAddressKHR = DeviceOrHostAddressKHR<'static>;
    pub type VkDeviceOrHostAddressConstKHR = DeviceOrHostAddressConstKHR<'static>;
    pub type VkAccelerationStructureGeometryDataKHR = AccelerationStructureGeometryDataKHR<'static>;
    pub type VkCopyAccelerationStructureModeKHR = CopyAccelerationStructureModeKHR;
    pub type VkBuildAccelerationStructureModeKHR = BuildAccelerationStructureModeKHR;
    pub type VkAccelerationStructureTypeKHR = AccelerationStructureTypeKHR;
    pub type VkGeometryTypeKHR = GeometryTypeKHR;
    pub type VkAccelerationStructureBuildTypeKHR = AccelerationStructureBuildTypeKHR;
    pub type VkAccelerationStructureCompatibilityKHR = AccelerationStructureCompatibilityKHR;
    pub type VkGeometryFlagsKHR = GeometryFlagsKHR;
    pub type VkGeometryFlagBitsKHR = GeometryFlagBitsKHR;
    pub type VkGeometryInstanceFlagsKHR = GeometryInstanceFlagsKHR;
    pub type VkGeometryInstanceFlagBitsKHR = GeometryInstanceFlagBitsKHR;
    pub type VkBuildAccelerationStructureFlagsKHR = BuildAccelerationStructureFlagsKHR;
    pub type VkBuildAccelerationStructureFlagBitsKHR = BuildAccelerationStructureFlagBitsKHR;
    pub type VkAccelerationStructureCreateFlagsKHR = AccelerationStructureCreateFlagsKHR;
    pub type VkAccelerationStructureCreateFlagBitsKHR = AccelerationStructureCreateFlagBitsKHR;
    impl WriteDescriptorSetAccelerationStructureKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkWriteDescriptorSetAccelerationStructureKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceAccelerationStructureFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceAccelerationStructureFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceAccelerationStructurePropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceAccelerationStructurePropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AccelerationStructureGeometryTrianglesDataKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkAccelerationStructureGeometryTrianglesDataKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AccelerationStructureGeometryAabbsDataKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkAccelerationStructureGeometryAabbsDataKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AccelerationStructureGeometryInstancesDataKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkAccelerationStructureGeometryInstancesDataKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AccelerationStructureGeometryKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkAccelerationStructureGeometryKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AccelerationStructureBuildGeometryInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkAccelerationStructureBuildGeometryInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AccelerationStructureCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkAccelerationStructureCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AccelerationStructureDeviceAddressInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkAccelerationStructureDeviceAddressInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AccelerationStructureVersionInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkAccelerationStructureVersionInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CopyAccelerationStructureInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCopyAccelerationStructureInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CopyAccelerationStructureToMemoryInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCopyAccelerationStructureToMemoryInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CopyMemoryToAccelerationStructureInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCopyMemoryToAccelerationStructureInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AccelerationStructureBuildSizesInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkAccelerationStructureBuildSizesInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceOrHostAddressKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceOrHostAddressKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceOrHostAddressConstKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceOrHostAddressConstKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AccelerationStructureGeometryDataKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkAccelerationStructureGeometryDataKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    create_acceleration_structure_khr: PFN_vkCreateAccelerationStructureKHR,
    destroy_acceleration_structure_khr: PFN_vkDestroyAccelerationStructureKHR,
    cmd_build_acceleration_structures_khr: PFN_vkCmdBuildAccelerationStructuresKHR,
    cmd_build_acceleration_structures_indirect_khr: PFN_vkCmdBuildAccelerationStructuresIndirectKHR,
    build_acceleration_structures_khr: PFN_vkBuildAccelerationStructuresKHR,
    copy_acceleration_structure_khr: PFN_vkCopyAccelerationStructureKHR,
    copy_acceleration_structure_to_memory_khr: PFN_vkCopyAccelerationStructureToMemoryKHR,
    copy_memory_to_acceleration_structure_khr: PFN_vkCopyMemoryToAccelerationStructureKHR,
    write_acceleration_structures_properties_khr: PFN_vkWriteAccelerationStructuresPropertiesKHR,
    cmd_copy_acceleration_structure_khr: PFN_vkCmdCopyAccelerationStructureKHR,
    cmd_copy_acceleration_structure_to_memory_khr: PFN_vkCmdCopyAccelerationStructureToMemoryKHR,
    cmd_copy_memory_to_acceleration_structure_khr: PFN_vkCmdCopyMemoryToAccelerationStructureKHR,
    get_acceleration_structure_device_address_khr: PFN_vkGetAccelerationStructureDeviceAddressKHR,
    cmd_write_acceleration_structures_properties_khr:
        PFN_vkCmdWriteAccelerationStructuresPropertiesKHR,
    get_device_acceleration_structure_compatibility_khr:
        PFN_vkGetDeviceAccelerationStructureCompatibilityKHR,
    get_acceleration_structure_build_sizes_khr: PFN_vkGetAccelerationStructureBuildSizesKHR,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_acceleration_structure_khr: transmute(
                    load(c"vkCreateAccelerationStructureKHR").ok_or(MissingEntryPointError)?,
                ),
                destroy_acceleration_structure_khr: transmute(
                    load(c"vkDestroyAccelerationStructureKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_build_acceleration_structures_khr: transmute(
                    load(c"vkCmdBuildAccelerationStructuresKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_build_acceleration_structures_indirect_khr: transmute(
                    load(c"vkCmdBuildAccelerationStructuresIndirectKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                build_acceleration_structures_khr: transmute(
                    load(c"vkBuildAccelerationStructuresKHR").ok_or(MissingEntryPointError)?,
                ),
                copy_acceleration_structure_khr: transmute(
                    load(c"vkCopyAccelerationStructureKHR").ok_or(MissingEntryPointError)?,
                ),
                copy_acceleration_structure_to_memory_khr: transmute(
                    load(c"vkCopyAccelerationStructureToMemoryKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                copy_memory_to_acceleration_structure_khr: transmute(
                    load(c"vkCopyMemoryToAccelerationStructureKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                write_acceleration_structures_properties_khr: transmute(
                    load(c"vkWriteAccelerationStructuresPropertiesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_acceleration_structure_khr: transmute(
                    load(c"vkCmdCopyAccelerationStructureKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_acceleration_structure_to_memory_khr: transmute(
                    load(c"vkCmdCopyAccelerationStructureToMemoryKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_memory_to_acceleration_structure_khr: transmute(
                    load(c"vkCmdCopyMemoryToAccelerationStructureKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_acceleration_structure_device_address_khr: transmute(
                    load(c"vkGetAccelerationStructureDeviceAddressKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                cmd_write_acceleration_structures_properties_khr: transmute(
                    load(c"vkCmdWriteAccelerationStructuresPropertiesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_device_acceleration_structure_compatibility_khr: transmute(
                    load(c"vkGetDeviceAccelerationStructureCompatibilityKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_acceleration_structure_build_sizes_khr: transmute(
                    load(c"vkGetAccelerationStructureBuildSizesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateAccelerationStructureKHR.html>
    #[inline]
    pub unsafe fn create_acceleration_structure_khr(
        &self,
        device: Device,
        create_info: &AccelerationStructureCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<AccelerationStructureKHR> {
        unsafe {
            let mut acceleration_structure = core::mem::MaybeUninit::uninit();
            let result = (self.create_acceleration_structure_khr)(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyAccelerationStructureKHR.html>
    #[inline]
    pub unsafe fn destroy_acceleration_structure_khr(
        &self,
        device: Device,
        acceleration_structure: AccelerationStructureKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_acceleration_structure_khr)(
                device,
                acceleration_structure,
                allocator.to_raw_ptr(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBuildAccelerationStructuresKHR.html>
    #[inline]
    pub unsafe fn cmd_build_acceleration_structures_khr(
        &self,
        command_buffer: CommandBuffer,
        infos: &[AccelerationStructureBuildGeometryInfoKHR<'_>],
        build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    ) {
        unsafe {
            (self.cmd_build_acceleration_structures_khr)(
                command_buffer,
                infos.len().try_into().unwrap(),
                infos.as_ptr() as _,
                build_range_infos.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBuildAccelerationStructuresIndirectKHR.html>
    #[inline]
    pub unsafe fn cmd_build_acceleration_structures_indirect_khr(
        &self,
        command_buffer: CommandBuffer,
        infos: &[AccelerationStructureBuildGeometryInfoKHR<'_>],
        indirect_device_addresses: &[DeviceAddress],
        indirect_strides: &[u32],
        max_primitive_counts: &[*const u32],
    ) {
        unsafe {
            (self.cmd_build_acceleration_structures_indirect_khr)(
                command_buffer,
                infos.len().try_into().unwrap(),
                infos.as_ptr() as _,
                indirect_device_addresses.as_ptr() as _,
                indirect_strides.as_ptr() as _,
                max_primitive_counts.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBuildAccelerationStructuresKHR.html>
    #[inline]
    pub unsafe fn build_acceleration_structures_khr(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        infos: &[AccelerationStructureBuildGeometryInfoKHR<'_>],
        build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.build_acceleration_structures_khr)(
                device,
                deferred_operation,
                infos.len().try_into().unwrap(),
                infos.as_ptr() as _,
                build_range_infos.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyAccelerationStructureKHR.html>
    #[inline]
    pub unsafe fn copy_acceleration_structure_khr(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info: &CopyAccelerationStructureInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.copy_acceleration_structure_khr)(device, deferred_operation, info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyAccelerationStructureToMemoryKHR.html>
    #[inline]
    pub unsafe fn copy_acceleration_structure_to_memory_khr(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info: &CopyAccelerationStructureToMemoryInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result =
                (self.copy_acceleration_structure_to_memory_khr)(device, deferred_operation, info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyMemoryToAccelerationStructureKHR.html>
    #[inline]
    pub unsafe fn copy_memory_to_acceleration_structure_khr(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMemoryToAccelerationStructureInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result =
                (self.copy_memory_to_acceleration_structure_khr)(device, deferred_operation, info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkWriteAccelerationStructuresPropertiesKHR.html>
    #[inline]
    pub unsafe fn write_acceleration_structures_properties_khr(
        &self,
        device: Device,
        acceleration_structures: &[AccelerationStructureKHR],
        query_type: QueryType,
        data: &mut [u8],
        stride: usize,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.write_acceleration_structures_properties_khr)(
                device,
                acceleration_structures.len().try_into().unwrap(),
                acceleration_structures.as_ptr() as _,
                query_type,
                data.len().try_into().unwrap(),
                data.as_mut_ptr() as _,
                stride,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyAccelerationStructureKHR.html>
    #[inline]
    pub unsafe fn cmd_copy_acceleration_structure_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyAccelerationStructureInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_copy_acceleration_structure_khr)(command_buffer, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html>
    #[inline]
    pub unsafe fn cmd_copy_acceleration_structure_to_memory_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyAccelerationStructureToMemoryInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_copy_acceleration_structure_to_memory_khr)(command_buffer, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html>
    #[inline]
    pub unsafe fn cmd_copy_memory_to_acceleration_structure_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyMemoryToAccelerationStructureInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_copy_memory_to_acceleration_structure_khr)(command_buffer, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetAccelerationStructureDeviceAddressKHR.html>
    #[inline]
    pub unsafe fn get_acceleration_structure_device_address_khr(
        &self,
        device: Device,
        info: &AccelerationStructureDeviceAddressInfoKHR<'_>,
    ) -> DeviceAddress {
        unsafe { (self.get_acceleration_structure_device_address_khr)(device, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html>
    #[inline]
    pub unsafe fn cmd_write_acceleration_structures_properties_khr(
        &self,
        command_buffer: CommandBuffer,
        acceleration_structures: &[AccelerationStructureKHR],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        unsafe {
            (self.cmd_write_acceleration_structures_properties_khr)(
                command_buffer,
                acceleration_structures.len().try_into().unwrap(),
                acceleration_structures.as_ptr() as _,
                query_type,
                query_pool,
                first_query,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html>
    #[inline]
    pub unsafe fn get_device_acceleration_structure_compatibility_khr(
        &self,
        device: Device,
        version_info: &AccelerationStructureVersionInfoKHR<'_>,
    ) -> AccelerationStructureCompatibilityKHR {
        unsafe {
            let mut compatibility = core::mem::MaybeUninit::uninit();
            (self.get_device_acceleration_structure_compatibility_khr)(
                device,
                version_info,
                compatibility.as_mut_ptr(),
            );
            compatibility.assume_init()
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetAccelerationStructureBuildSizesKHR.html>
    #[inline]
    pub unsafe fn get_acceleration_structure_build_sizes_khr(
        &self,
        device: Device,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: &AccelerationStructureBuildGeometryInfoKHR<'_>,
        max_primitive_counts: Option<&[u32]>,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ) {
        unsafe {
            (self.get_acceleration_structure_build_sizes_khr)(
                device,
                build_type,
                build_info,
                max_primitive_counts.to_raw_ptr(),
                size_info,
            )
        }
    }
}
