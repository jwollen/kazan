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
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct AccelerationStructureKHR(u64);
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct WriteDescriptorSetAccelerationStructureKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub acceleration_structure_count: u32,
        pub p_acceleration_structures: *const AccelerationStructureKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for WriteDescriptorSetAccelerationStructureKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR;
    }
    unsafe impl<'a> Extends<WriteDescriptorSet<'a>> for WriteDescriptorSetAccelerationStructureKHR<'a> {}
    impl Default for WriteDescriptorSetAccelerationStructureKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                acceleration_structure_count: Default::default(),
                p_acceleration_structures: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> WriteDescriptorSetAccelerationStructureKHR<'a> {
        pub fn acceleration_structures(
            mut self,
            acceleration_structures: &'a [AccelerationStructureKHR],
        ) -> Self {
            self.acceleration_structure_count = acceleration_structures.len().try_into().unwrap();
            self.p_acceleration_structures = acceleration_structures.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceAccelerationStructureFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceAccelerationStructureFeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceAccelerationStructureFeaturesKHR<'a>
    {
    }
    impl Default for PhysicalDeviceAccelerationStructureFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
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
        pub fn acceleration_structure(mut self, acceleration_structure: Bool32) -> Self {
            self.acceleration_structure = acceleration_structure;
            self
        }
        pub fn acceleration_structure_capture_replay(
            mut self,
            acceleration_structure_capture_replay: Bool32,
        ) -> Self {
            self.acceleration_structure_capture_replay = acceleration_structure_capture_replay;
            self
        }
        pub fn acceleration_structure_indirect_build(
            mut self,
            acceleration_structure_indirect_build: Bool32,
        ) -> Self {
            self.acceleration_structure_indirect_build = acceleration_structure_indirect_build;
            self
        }
        pub fn acceleration_structure_host_commands(
            mut self,
            acceleration_structure_host_commands: Bool32,
        ) -> Self {
            self.acceleration_structure_host_commands = acceleration_structure_host_commands;
            self
        }
        pub fn descriptor_binding_acceleration_structure_update_after_bind(
            mut self,
            descriptor_binding_acceleration_structure_update_after_bind: Bool32,
        ) -> Self {
            self.descriptor_binding_acceleration_structure_update_after_bind =
                descriptor_binding_acceleration_structure_update_after_bind;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceAccelerationStructurePropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceAccelerationStructurePropertiesKHR<'a>
    {
    }
    impl Default for PhysicalDeviceAccelerationStructurePropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
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
        pub fn max_geometry_count(mut self, max_geometry_count: u64) -> Self {
            self.max_geometry_count = max_geometry_count;
            self
        }
        pub fn max_instance_count(mut self, max_instance_count: u64) -> Self {
            self.max_instance_count = max_instance_count;
            self
        }
        pub fn max_primitive_count(mut self, max_primitive_count: u64) -> Self {
            self.max_primitive_count = max_primitive_count;
            self
        }
        pub fn max_per_stage_descriptor_acceleration_structures(
            mut self,
            max_per_stage_descriptor_acceleration_structures: u32,
        ) -> Self {
            self.max_per_stage_descriptor_acceleration_structures =
                max_per_stage_descriptor_acceleration_structures;
            self
        }
        pub fn max_per_stage_descriptor_update_after_bind_acceleration_structures(
            mut self,
            max_per_stage_descriptor_update_after_bind_acceleration_structures: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_acceleration_structures =
                max_per_stage_descriptor_update_after_bind_acceleration_structures;
            self
        }
        pub fn max_descriptor_set_acceleration_structures(
            mut self,
            max_descriptor_set_acceleration_structures: u32,
        ) -> Self {
            self.max_descriptor_set_acceleration_structures =
                max_descriptor_set_acceleration_structures;
            self
        }
        pub fn max_descriptor_set_update_after_bind_acceleration_structures(
            mut self,
            max_descriptor_set_update_after_bind_acceleration_structures: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_acceleration_structures =
                max_descriptor_set_update_after_bind_acceleration_structures;
            self
        }
        pub fn min_acceleration_structure_scratch_offset_alignment(
            mut self,
            min_acceleration_structure_scratch_offset_alignment: u32,
        ) -> Self {
            self.min_acceleration_structure_scratch_offset_alignment =
                min_acceleration_structure_scratch_offset_alignment;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureGeometryTrianglesDataKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR;
    }
    impl Default for AccelerationStructureGeometryTrianglesDataKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
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
        pub fn vertex_format(mut self, vertex_format: Format) -> Self {
            self.vertex_format = vertex_format;
            self
        }
        pub fn vertex_data(mut self, vertex_data: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.vertex_data = vertex_data;
            self
        }
        pub fn vertex_stride(mut self, vertex_stride: DeviceSize) -> Self {
            self.vertex_stride = vertex_stride;
            self
        }
        pub fn max_vertex(mut self, max_vertex: u32) -> Self {
            self.max_vertex = max_vertex;
            self
        }
        pub fn index_type(mut self, index_type: IndexType) -> Self {
            self.index_type = index_type;
            self
        }
        pub fn index_data(mut self, index_data: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.index_data = index_data;
            self
        }
        pub fn transform_data(mut self, transform_data: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.transform_data = transform_data;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AccelerationStructureGeometryAabbsDataKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub data: DeviceOrHostAddressConstKHR<'a>,
        pub stride: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureGeometryAabbsDataKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR;
    }
    impl Default for AccelerationStructureGeometryAabbsDataKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                data: Default::default(),
                stride: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> AccelerationStructureGeometryAabbsDataKHR<'a> {
        pub fn data(mut self, data: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.data = data;
            self
        }
        pub fn stride(mut self, stride: DeviceSize) -> Self {
            self.stride = stride;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AccelerationStructureGeometryInstancesDataKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub array_of_pointers: Bool32,
        pub data: DeviceOrHostAddressConstKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureGeometryInstancesDataKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR;
    }
    impl Default for AccelerationStructureGeometryInstancesDataKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                array_of_pointers: Default::default(),
                data: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> AccelerationStructureGeometryInstancesDataKHR<'a> {
        pub fn array_of_pointers(mut self, array_of_pointers: Bool32) -> Self {
            self.array_of_pointers = array_of_pointers;
            self
        }
        pub fn data(mut self, data: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.data = data;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AccelerationStructureGeometryKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub geometry_type: GeometryTypeKHR,
        pub geometry: AccelerationStructureGeometryDataKHR<'a>,
        pub flags: GeometryFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureGeometryKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ACCELERATION_STRUCTURE_GEOMETRY_KHR;
    }
    impl Default for AccelerationStructureGeometryKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                geometry_type: Default::default(),
                geometry: Default::default(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> AccelerationStructureGeometryKHR<'a> {
        pub fn geometry_type(mut self, geometry_type: GeometryTypeKHR) -> Self {
            self.geometry_type = geometry_type;
            self
        }
        pub fn geometry(mut self, geometry: AccelerationStructureGeometryDataKHR<'a>) -> Self {
            self.geometry = geometry;
            self
        }
        pub fn flags(mut self, flags: GeometryFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureBuildGeometryInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR;
    }
    impl Default for AccelerationStructureBuildGeometryInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                ty: Default::default(),
                flags: Default::default(),
                mode: Default::default(),
                src_acceleration_structure: Default::default(),
                dst_acceleration_structure: Default::default(),
                geometry_count: Default::default(),
                p_geometries: core::ptr::null(),
                pp_geometries: core::ptr::null(),
                scratch_data: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> AccelerationStructureBuildGeometryInfoKHR<'a> {
        pub fn ty(mut self, ty: AccelerationStructureTypeKHR) -> Self {
            self.ty = ty;
            self
        }
        pub fn flags(mut self, flags: BuildAccelerationStructureFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn mode(mut self, mode: BuildAccelerationStructureModeKHR) -> Self {
            self.mode = mode;
            self
        }
        pub fn src_acceleration_structure(
            mut self,
            src_acceleration_structure: AccelerationStructureKHR,
        ) -> Self {
            self.src_acceleration_structure = src_acceleration_structure;
            self
        }
        pub fn dst_acceleration_structure(
            mut self,
            dst_acceleration_structure: AccelerationStructureKHR,
        ) -> Self {
            self.dst_acceleration_structure = dst_acceleration_structure;
            self
        }
        pub fn geometries(
            mut self,
            geometries: &'a [AccelerationStructureGeometryKHR<'a>],
        ) -> Self {
            self.geometry_count = geometries.len().try_into().unwrap();
            self.p_geometries = geometries.as_ptr();
            self
        }
        pub fn geometries_ptrs(
            mut self,
            geometries_ptrs: &'a [&'a AccelerationStructureGeometryKHR<'a>],
        ) -> Self {
            self.geometry_count = geometries_ptrs.len().try_into().unwrap();
            self.pp_geometries = geometries_ptrs.as_ptr() as _;
            self
        }
        pub fn scratch_data(mut self, scratch_data: DeviceOrHostAddressKHR<'a>) -> Self {
            self.scratch_data = scratch_data;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct AccelerationStructureBuildRangeInfoKHR {
        pub primitive_count: u32,
        pub primitive_offset: u32,
        pub first_vertex: u32,
        pub transform_offset: u32,
    }
    impl AccelerationStructureBuildRangeInfoKHR {
        pub fn primitive_count(mut self, primitive_count: u32) -> Self {
            self.primitive_count = primitive_count;
            self
        }
        pub fn primitive_offset(mut self, primitive_offset: u32) -> Self {
            self.primitive_offset = primitive_offset;
            self
        }
        pub fn first_vertex(mut self, first_vertex: u32) -> Self {
            self.first_vertex = first_vertex;
            self
        }
        pub fn transform_offset(mut self, transform_offset: u32) -> Self {
            self.transform_offset = transform_offset;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ACCELERATION_STRUCTURE_CREATE_INFO_KHR;
    }
    impl Default for AccelerationStructureCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
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
        pub fn create_flags(mut self, create_flags: AccelerationStructureCreateFlagsKHR) -> Self {
            self.create_flags = create_flags;
            self
        }
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
        pub fn offset(mut self, offset: DeviceSize) -> Self {
            self.offset = offset;
            self
        }
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
        pub fn ty(mut self, ty: AccelerationStructureTypeKHR) -> Self {
            self.ty = ty;
            self
        }
        pub fn device_address(mut self, device_address: DeviceAddress) -> Self {
            self.device_address = device_address;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct AabbPositionsKHR {
        pub min_x: f32,
        pub min_y: f32,
        pub min_z: f32,
        pub max_x: f32,
        pub max_y: f32,
        pub max_z: f32,
    }
    impl AabbPositionsKHR {
        pub fn min_x(mut self, min_x: f32) -> Self {
            self.min_x = min_x;
            self
        }
        pub fn min_y(mut self, min_y: f32) -> Self {
            self.min_y = min_y;
            self
        }
        pub fn min_z(mut self, min_z: f32) -> Self {
            self.min_z = min_z;
            self
        }
        pub fn max_x(mut self, max_x: f32) -> Self {
            self.max_x = max_x;
            self
        }
        pub fn max_y(mut self, max_y: f32) -> Self {
            self.max_y = max_y;
            self
        }
        pub fn max_z(mut self, max_z: f32) -> Self {
            self.max_z = max_z;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
        pub fn matrix(mut self, matrix: [[f32; 4]; 3]) -> Self {
            self.matrix = matrix;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AccelerationStructureInstanceKHR {
        pub transform: TransformMatrixKHR,
        pub instance_custom_index: u32,
        pub mask: u32,
        pub instance_shader_binding_table_record_offset: u32,
        pub flags: GeometryInstanceFlagsKHR,
        pub acceleration_structure_reference: u64,
    }
    impl Default for AccelerationStructureInstanceKHR {
        fn default() -> Self {
            Self {
                transform: Default::default(),
                instance_custom_index: Default::default(),
                mask: Default::default(),
                instance_shader_binding_table_record_offset: Default::default(),
                flags: Default::default(),
                acceleration_structure_reference: Default::default(),
            }
        }
    }
    impl AccelerationStructureInstanceKHR {
        pub fn transform(mut self, transform: TransformMatrixKHR) -> Self {
            self.transform = transform;
            self
        }
        pub fn instance_custom_index(mut self, instance_custom_index: u32) -> Self {
            self.instance_custom_index = instance_custom_index;
            self
        }
        pub fn mask(mut self, mask: u32) -> Self {
            self.mask = mask;
            self
        }
        pub fn instance_shader_binding_table_record_offset(
            mut self,
            instance_shader_binding_table_record_offset: u32,
        ) -> Self {
            self.instance_shader_binding_table_record_offset =
                instance_shader_binding_table_record_offset;
            self
        }
        pub fn flags(mut self, flags: GeometryInstanceFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn acceleration_structure_reference(
            mut self,
            acceleration_structure_reference: u64,
        ) -> Self {
            self.acceleration_structure_reference = acceleration_structure_reference;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AccelerationStructureDeviceAddressInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub acceleration_structure: AccelerationStructureKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureDeviceAddressInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR;
    }
    impl Default for AccelerationStructureDeviceAddressInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                acceleration_structure: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> AccelerationStructureDeviceAddressInfoKHR<'a> {
        pub fn acceleration_structure(
            mut self,
            acceleration_structure: AccelerationStructureKHR,
        ) -> Self {
            self.acceleration_structure = acceleration_structure;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AccelerationStructureVersionInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_version_data: *const u8,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureVersionInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_VERSION_INFO_KHR;
    }
    impl Default for AccelerationStructureVersionInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_version_data: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> AccelerationStructureVersionInfoKHR<'a> {}
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CopyAccelerationStructureInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src: AccelerationStructureKHR,
        pub dst: AccelerationStructureKHR,
        pub mode: CopyAccelerationStructureModeKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CopyAccelerationStructureInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_ACCELERATION_STRUCTURE_INFO_KHR;
    }
    impl Default for CopyAccelerationStructureInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src: Default::default(),
                dst: Default::default(),
                mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CopyAccelerationStructureInfoKHR<'a> {
        pub fn src(mut self, src: AccelerationStructureKHR) -> Self {
            self.src = src;
            self
        }
        pub fn dst(mut self, dst: AccelerationStructureKHR) -> Self {
            self.dst = dst;
            self
        }
        pub fn mode(mut self, mode: CopyAccelerationStructureModeKHR) -> Self {
            self.mode = mode;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CopyAccelerationStructureToMemoryInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src: AccelerationStructureKHR,
        pub dst: DeviceOrHostAddressKHR<'a>,
        pub mode: CopyAccelerationStructureModeKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CopyAccelerationStructureToMemoryInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR;
    }
    impl Default for CopyAccelerationStructureToMemoryInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src: Default::default(),
                dst: Default::default(),
                mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CopyAccelerationStructureToMemoryInfoKHR<'a> {
        pub fn src(mut self, src: AccelerationStructureKHR) -> Self {
            self.src = src;
            self
        }
        pub fn dst(mut self, dst: DeviceOrHostAddressKHR<'a>) -> Self {
            self.dst = dst;
            self
        }
        pub fn mode(mut self, mode: CopyAccelerationStructureModeKHR) -> Self {
            self.mode = mode;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CopyMemoryToAccelerationStructureInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src: DeviceOrHostAddressConstKHR<'a>,
        pub dst: AccelerationStructureKHR,
        pub mode: CopyAccelerationStructureModeKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CopyMemoryToAccelerationStructureInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR;
    }
    impl Default for CopyMemoryToAccelerationStructureInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src: Default::default(),
                dst: Default::default(),
                mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CopyMemoryToAccelerationStructureInfoKHR<'a> {
        pub fn src(mut self, src: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.src = src;
            self
        }
        pub fn dst(mut self, dst: AccelerationStructureKHR) -> Self {
            self.dst = dst;
            self
        }
        pub fn mode(mut self, mode: CopyAccelerationStructureModeKHR) -> Self {
            self.mode = mode;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AccelerationStructureBuildSizesInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub acceleration_structure_size: DeviceSize,
        pub update_scratch_size: DeviceSize,
        pub build_scratch_size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureBuildSizesInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR;
    }
    impl Default for AccelerationStructureBuildSizesInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                acceleration_structure_size: Default::default(),
                update_scratch_size: Default::default(),
                build_scratch_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> AccelerationStructureBuildSizesInfoKHR<'a> {
        pub fn acceleration_structure_size(
            mut self,
            acceleration_structure_size: DeviceSize,
        ) -> Self {
            self.acceleration_structure_size = acceleration_structure_size;
            self
        }
        pub fn update_scratch_size(mut self, update_scratch_size: DeviceSize) -> Self {
            self.update_scratch_size = update_scratch_size;
            self
        }
        pub fn build_scratch_size(mut self, build_scratch_size: DeviceSize) -> Self {
            self.build_scratch_size = build_scratch_size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union DeviceOrHostAddressKHR<'a> {
        pub device_address: DeviceAddress,
        pub host_address: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for DeviceOrHostAddressKHR<'_> {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union DeviceOrHostAddressConstKHR<'a> {
        pub device_address: DeviceAddress,
        pub host_address: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for DeviceOrHostAddressConstKHR<'_> {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union AccelerationStructureGeometryDataKHR<'a> {
        pub triangles: AccelerationStructureGeometryTrianglesDataKHR<'a>,
        pub aabbs: AccelerationStructureGeometryAabbsDataKHR<'a>,
        pub instances: AccelerationStructureGeometryInstancesDataKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for AccelerationStructureGeometryDataKHR<'_> {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CopyAccelerationStructureModeKHR(i32);
    impl CopyAccelerationStructureModeKHR {
        pub const CLONE_KHR: Self = Self(0);
        pub const COMPACT_KHR: Self = Self(1);
        pub const DESERIALIZE_KHR: Self = Self(3);
        pub const SERIALIZE_KHR: Self = Self(2);
        pub const CLONE_NV: Self = Self::CLONE_KHR;
        pub const COMPACT_NV: Self = Self::COMPACT_KHR;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BuildAccelerationStructureModeKHR(i32);
    impl BuildAccelerationStructureModeKHR {
        pub const BUILD_KHR: Self = Self(0);
        pub const UPDATE_KHR: Self = Self(1);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccelerationStructureTypeKHR(i32);
    impl AccelerationStructureTypeKHR {
        pub const TOP_LEVEL_KHR: Self = Self(0);
        pub const BOTTOM_LEVEL_KHR: Self = Self(1);
        pub const GENERIC_KHR: Self = Self(2);
        pub const BOTTOM_LEVEL_NV: Self = Self::BOTTOM_LEVEL_KHR;
        pub const TOP_LEVEL_NV: Self = Self::TOP_LEVEL_KHR;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GeometryTypeKHR(i32);
    impl GeometryTypeKHR {
        pub const TRIANGLES_KHR: Self = Self(0);
        pub const AABBS_KHR: Self = Self(1);
        pub const INSTANCES_KHR: Self = Self(2);
        pub const DENSE_GEOMETRY_FORMAT_TRIANGLES_AMDX: Self = Self(1000478000);
        pub const LINEAR_SWEPT_SPHERES_NV: Self = Self(1000429005);
        pub const SPHERES_NV: Self = Self(1000429004);
        pub const AABBS_NV: Self = Self::AABBS_KHR;
        pub const TRIANGLES_NV: Self = Self::TRIANGLES_KHR;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccelerationStructureBuildTypeKHR(i32);
    impl AccelerationStructureBuildTypeKHR {
        pub const HOST_KHR: Self = Self(0);
        pub const DEVICE_KHR: Self = Self(1);
        pub const HOST_OR_DEVICE_KHR: Self = Self(2);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccelerationStructureCompatibilityKHR(i32);
    impl AccelerationStructureCompatibilityKHR {
        pub const COMPATIBLE_KHR: Self = Self(0);
        pub const INCOMPATIBLE_KHR: Self = Self(1);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct GeometryFlagsKHR(Flags);
    impl GeometryFlagsKHR {
        pub const OPAQUE_KHR: Self = Self(GeometryFlagBitsKHR::OPAQUE_KHR.0);
        pub const NO_DUPLICATE_ANY_HIT_INVOCATION_KHR: Self =
            Self(GeometryFlagBitsKHR::NO_DUPLICATE_ANY_HIT_INVOCATION_KHR.0);
        pub const NO_DUPLICATE_ANY_HIT_INVOCATION_NV: Self =
            Self::NO_DUPLICATE_ANY_HIT_INVOCATION_KHR;
        pub const OPAQUE_NV: Self = Self::OPAQUE_KHR;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct GeometryFlagBitsKHR(u32);
    impl GeometryFlagBitsKHR {
        pub const OPAQUE_KHR: Self = Self(1 << 0);
        pub const NO_DUPLICATE_ANY_HIT_INVOCATION_KHR: Self = Self(1 << 1);
        pub const NO_DUPLICATE_ANY_HIT_INVOCATION_NV: Self =
            Self::NO_DUPLICATE_ANY_HIT_INVOCATION_KHR;
        pub const OPAQUE_NV: Self = Self::OPAQUE_KHR;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct GeometryInstanceFlagsKHR(Flags);
    impl GeometryInstanceFlagsKHR {
        pub const TRIANGLE_FACING_CULL_DISABLE_KHR: Self =
            Self(GeometryInstanceFlagBitsKHR::TRIANGLE_FACING_CULL_DISABLE_KHR.0);
        pub const TRIANGLE_FLIP_FACING_KHR: Self =
            Self(GeometryInstanceFlagBitsKHR::TRIANGLE_FLIP_FACING_KHR.0);
        pub const FORCE_OPAQUE_KHR: Self = Self(GeometryInstanceFlagBitsKHR::FORCE_OPAQUE_KHR.0);
        pub const FORCE_NO_OPAQUE_KHR: Self =
            Self(GeometryInstanceFlagBitsKHR::FORCE_NO_OPAQUE_KHR.0);
        pub const FORCE_OPACITY_MICROMAP_2_STATE_EXT: Self =
            Self(GeometryInstanceFlagBitsKHR::FORCE_OPACITY_MICROMAP_2_STATE_EXT.0);
        pub const DISABLE_OPACITY_MICROMAPS_EXT: Self =
            Self(GeometryInstanceFlagBitsKHR::DISABLE_OPACITY_MICROMAPS_EXT.0);
        pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR: Self = Self::TRIANGLE_FLIP_FACING_KHR;
        pub const FORCE_NO_OPAQUE_NV: Self = Self::FORCE_NO_OPAQUE_KHR;
        pub const FORCE_OPAQUE_NV: Self = Self::FORCE_OPAQUE_KHR;
        pub const TRIANGLE_CULL_DISABLE_NV: Self = Self::TRIANGLE_FACING_CULL_DISABLE_KHR;
        pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_NV: Self =
            Self::TRIANGLE_FRONT_COUNTERCLOCKWISE_KHR;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct GeometryInstanceFlagBitsKHR(u32);
    impl GeometryInstanceFlagBitsKHR {
        pub const TRIANGLE_FACING_CULL_DISABLE_KHR: Self = Self(1 << 0);
        pub const TRIANGLE_FLIP_FACING_KHR: Self = Self(1 << 1);
        pub const FORCE_OPAQUE_KHR: Self = Self(1 << 2);
        pub const FORCE_NO_OPAQUE_KHR: Self = Self(1 << 3);
        pub const FORCE_OPACITY_MICROMAP_2_STATE_EXT: Self = Self(1 << 4);
        pub const DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(1 << 5);
        pub const FORCE_NO_OPAQUE_NV: Self = Self::FORCE_NO_OPAQUE_KHR;
        pub const FORCE_OPAQUE_NV: Self = Self::FORCE_OPAQUE_KHR;
        pub const TRIANGLE_CULL_DISABLE_NV: Self = Self::TRIANGLE_FACING_CULL_DISABLE_KHR;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct BuildAccelerationStructureFlagsKHR(Flags);
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
        pub const MOTION_NV: Self = Self(BuildAccelerationStructureFlagBitsKHR::MOTION_NV.0);
        pub const ALLOW_OPACITY_MICROMAP_UPDATE_EXT: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::ALLOW_OPACITY_MICROMAP_UPDATE_EXT.0);
        pub const ALLOW_DISABLE_OPACITY_MICROMAPS_EXT: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::ALLOW_DISABLE_OPACITY_MICROMAPS_EXT.0);
        pub const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT.0);
        pub const ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV.0);
        pub const ALLOW_DATA_ACCESS_KHR: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::ALLOW_DATA_ACCESS_KHR.0);
        pub const ALLOW_CLUSTER_OPACITY_MICROMAPS_NV: Self =
            Self(BuildAccelerationStructureFlagBitsKHR::ALLOW_CLUSTER_OPACITY_MICROMAPS_NV.0);
        pub const ALLOW_COMPACTION_NV: Self = Self::ALLOW_COMPACTION_KHR;
        pub const ALLOW_UPDATE_NV: Self = Self::ALLOW_UPDATE_KHR;
        pub const LOW_MEMORY_NV: Self = Self::LOW_MEMORY_KHR;
        pub const PREFER_FAST_BUILD_NV: Self = Self::PREFER_FAST_BUILD_KHR;
        pub const PREFER_FAST_TRACE_NV: Self = Self::PREFER_FAST_TRACE_KHR;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct BuildAccelerationStructureFlagBitsKHR(u32);
    impl BuildAccelerationStructureFlagBitsKHR {
        pub const ALLOW_UPDATE_KHR: Self = Self(1 << 0);
        pub const ALLOW_COMPACTION_KHR: Self = Self(1 << 1);
        pub const PREFER_FAST_TRACE_KHR: Self = Self(1 << 2);
        pub const PREFER_FAST_BUILD_KHR: Self = Self(1 << 3);
        pub const LOW_MEMORY_KHR: Self = Self(1 << 4);
        pub const MOTION_NV: Self = Self(1 << 5);
        pub const ALLOW_OPACITY_MICROMAP_UPDATE_EXT: Self = Self(1 << 6);
        pub const ALLOW_DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(1 << 7);
        pub const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT: Self = Self(1 << 8);
        pub const ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV: Self = Self(1 << 9);
        pub const ALLOW_DATA_ACCESS_KHR: Self = Self(1 << 11);
        pub const ALLOW_CLUSTER_OPACITY_MICROMAPS_NV: Self = Self(1 << 12);
        pub const ALLOW_COMPACTION_NV: Self = Self::ALLOW_COMPACTION_KHR;
        pub const ALLOW_UPDATE_NV: Self = Self::ALLOW_UPDATE_KHR;
        pub const LOW_MEMORY_NV: Self = Self::LOW_MEMORY_KHR;
        pub const PREFER_FAST_BUILD_NV: Self = Self::PREFER_FAST_BUILD_KHR;
        pub const PREFER_FAST_TRACE_NV: Self = Self::PREFER_FAST_TRACE_KHR;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct AccelerationStructureCreateFlagsKHR(Flags);
    impl AccelerationStructureCreateFlagsKHR {
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self =
            Self(AccelerationStructureCreateFlagBitsKHR::DEVICE_ADDRESS_CAPTURE_REPLAY_KHR.0);
        pub const MOTION_NV: Self = Self(AccelerationStructureCreateFlagBitsKHR::MOTION_NV.0);
        pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self =
            Self(AccelerationStructureCreateFlagBitsKHR::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct AccelerationStructureCreateFlagBitsKHR(u32);
    impl AccelerationStructureCreateFlagBitsKHR {
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self(1 << 0);
        pub const MOTION_NV: Self = Self(1 << 2);
        pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(1 << 3);
    }
    pub type PFN_vkDestroyAccelerationStructureKHR = unsafe extern "system" fn(
        device: Device,
        acceleration_structure: AccelerationStructureKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkCmdCopyAccelerationStructureKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const CopyAccelerationStructureInfoKHR<'_>,
    );
    pub type PFN_vkCopyAccelerationStructureKHR = unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyAccelerationStructureInfoKHR<'_>,
    ) -> vk::Result;
    pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
    );
    pub type PFN_vkCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
    )
        -> vk::Result;
    pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
    );
    pub type PFN_vkCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
    )
        -> vk::Result;
    pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const AccelerationStructureKHR,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    );
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
    pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR = unsafe extern "system" fn(
        device: Device,
        p_version_info: *const AccelerationStructureVersionInfoKHR<'_>,
        p_compatibility: *mut AccelerationStructureCompatibilityKHR,
    );
    pub type PFN_vkCreateAccelerationStructureKHR = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const AccelerationStructureCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_acceleration_structure: *mut AccelerationStructureKHR,
    ) -> vk::Result;
    pub type PFN_vkCmdBuildAccelerationStructuresKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
    );
    pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        p_indirect_device_addresses: *const DeviceAddress,
        p_indirect_strides: *const u32,
        pp_max_primitive_counts: *const *const u32,
    );
    pub type PFN_vkBuildAccelerationStructuresKHR = unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
    ) -> vk::Result;
    pub type PFN_vkGetAccelerationStructureDeviceAddressKHR =
        unsafe extern "system" fn(
            device: Device,
            p_info: *const AccelerationStructureDeviceAddressInfoKHR<'_>,
        ) -> DeviceAddress;
    pub type PFN_vkGetAccelerationStructureBuildSizesKHR = unsafe extern "system" fn(
        device: Device,
        build_type: AccelerationStructureBuildTypeKHR,
        p_build_info: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        p_max_primitive_counts: *const u32,
        p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
    );
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
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_acceleration_structure_khr: transmute(
                    load(c"vkCreateAccelerationStructureKHR").ok_or(LoadingError)?,
                ),
                destroy_acceleration_structure_khr: transmute(
                    load(c"vkDestroyAccelerationStructureKHR").ok_or(LoadingError)?,
                ),
                cmd_build_acceleration_structures_khr: transmute(
                    load(c"vkCmdBuildAccelerationStructuresKHR").ok_or(LoadingError)?,
                ),
                cmd_build_acceleration_structures_indirect_khr: transmute(
                    load(c"vkCmdBuildAccelerationStructuresIndirectKHR").ok_or(LoadingError)?,
                ),
                build_acceleration_structures_khr: transmute(
                    load(c"vkBuildAccelerationStructuresKHR").ok_or(LoadingError)?,
                ),
                copy_acceleration_structure_khr: transmute(
                    load(c"vkCopyAccelerationStructureKHR").ok_or(LoadingError)?,
                ),
                copy_acceleration_structure_to_memory_khr: transmute(
                    load(c"vkCopyAccelerationStructureToMemoryKHR").ok_or(LoadingError)?,
                ),
                copy_memory_to_acceleration_structure_khr: transmute(
                    load(c"vkCopyMemoryToAccelerationStructureKHR").ok_or(LoadingError)?,
                ),
                write_acceleration_structures_properties_khr: transmute(
                    load(c"vkWriteAccelerationStructuresPropertiesKHR").ok_or(LoadingError)?,
                ),
                cmd_copy_acceleration_structure_khr: transmute(
                    load(c"vkCmdCopyAccelerationStructureKHR").ok_or(LoadingError)?,
                ),
                cmd_copy_acceleration_structure_to_memory_khr: transmute(
                    load(c"vkCmdCopyAccelerationStructureToMemoryKHR").ok_or(LoadingError)?,
                ),
                cmd_copy_memory_to_acceleration_structure_khr: transmute(
                    load(c"vkCmdCopyMemoryToAccelerationStructureKHR").ok_or(LoadingError)?,
                ),
                get_acceleration_structure_device_address_khr: transmute(
                    load(c"vkGetAccelerationStructureDeviceAddressKHR").ok_or(LoadingError)?,
                ),
                cmd_write_acceleration_structures_properties_khr: transmute(
                    load(c"vkCmdWriteAccelerationStructuresPropertiesKHR").ok_or(LoadingError)?,
                ),
                get_device_acceleration_structure_compatibility_khr: transmute(
                    load(c"vkGetDeviceAccelerationStructureCompatibilityKHR")
                        .ok_or(LoadingError)?,
                ),
                get_acceleration_structure_build_sizes_khr: transmute(
                    load(c"vkGetAccelerationStructureBuildSizesKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
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
                VkResult::OPERATION_DEFERRED_KHR => Ok(()),
                VkResult::OPERATION_NOT_DEFERRED_KHR => Ok(()),
                err => Err(err),
            }
        }
    }
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
                VkResult::OPERATION_DEFERRED_KHR => Ok(()),
                VkResult::OPERATION_NOT_DEFERRED_KHR => Ok(()),
                err => Err(err),
            }
        }
    }
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
                VkResult::OPERATION_DEFERRED_KHR => Ok(()),
                VkResult::OPERATION_NOT_DEFERRED_KHR => Ok(()),
                err => Err(err),
            }
        }
    }
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
                VkResult::OPERATION_DEFERRED_KHR => Ok(()),
                VkResult::OPERATION_NOT_DEFERRED_KHR => Ok(()),
                err => Err(err),
            }
        }
    }
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
    pub unsafe fn cmd_copy_acceleration_structure_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyAccelerationStructureInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_copy_acceleration_structure_khr)(command_buffer, info) }
    }
    pub unsafe fn cmd_copy_acceleration_structure_to_memory_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyAccelerationStructureToMemoryInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_copy_acceleration_structure_to_memory_khr)(command_buffer, info) }
    }
    pub unsafe fn cmd_copy_memory_to_acceleration_structure_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyMemoryToAccelerationStructureInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_copy_memory_to_acceleration_structure_khr)(command_buffer, info) }
    }
    pub unsafe fn get_acceleration_structure_device_address_khr(
        &self,
        device: Device,
        info: &AccelerationStructureDeviceAddressInfoKHR<'_>,
    ) -> DeviceAddress {
        unsafe { (self.get_acceleration_structure_device_address_khr)(device, info) }
    }
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
    pub unsafe fn get_acceleration_structure_build_sizes_khr(
        &self,
        device: Device,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: &AccelerationStructureBuildGeometryInfoKHR<'_>,
        max_primitive_counts: Option<&[u32]>,
    ) -> AccelerationStructureBuildSizesInfoKHR<'_> {
        unsafe {
            let mut size_info = core::mem::MaybeUninit::uninit();
            (self.get_acceleration_structure_build_sizes_khr)(
                device,
                build_type,
                build_info,
                max_primitive_counts.to_raw_ptr(),
                size_info.as_mut_ptr(),
            );
            size_info.assume_init()
        }
    }
}
