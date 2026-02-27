#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub const PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV: u32 = !0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePartitionedAccelerationStructureFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub partitioned_acceleration_structure: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePartitionedAccelerationStructureFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            partitioned_acceleration_structure: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDevicePartitionedAccelerationStructureFeaturesNV<'a> {
    pub fn partitioned_acceleration_structure(
        mut self,
        partitioned_acceleration_structure: Bool32,
    ) -> Self {
        self.partitioned_acceleration_structure = partitioned_acceleration_structure;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePartitionedAccelerationStructurePropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_partition_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePartitionedAccelerationStructurePropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            max_partition_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDevicePartitionedAccelerationStructurePropertiesNV<'a> {
    pub fn max_partition_count(mut self, max_partition_count: u32) -> Self {
        self.max_partition_count = max_partition_count;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct BuildPartitionedAccelerationStructureIndirectCommandNV {
    pub op_type: PartitionedAccelerationStructureOpTypeNV,
    pub arg_count: u32,
    pub arg_data: StridedDeviceAddressNV,
}
impl BuildPartitionedAccelerationStructureIndirectCommandNV {
    pub fn op_type(mut self, op_type: PartitionedAccelerationStructureOpTypeNV) -> Self {
        self.op_type = op_type;
        self
    }
    pub fn arg_count(mut self, arg_count: u32) -> Self {
        self.arg_count = arg_count;
        self
    }
    pub fn arg_data(mut self, arg_data: StridedDeviceAddressNV) -> Self {
        self.arg_data = arg_data;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PartitionedAccelerationStructureFlagsNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub enable_partition_translation: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PartitionedAccelerationStructureFlagsNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV,
            p_next: core::ptr::null_mut(),
            enable_partition_translation: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PartitionedAccelerationStructureFlagsNV<'a> {
    pub fn enable_partition_translation(mut self, enable_partition_translation: Bool32) -> Self {
        self.enable_partition_translation = enable_partition_translation;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PartitionedAccelerationStructureWriteInstanceDataNV {
    pub transform: TransformMatrixKHR,
    pub explicit_aabb: [f32; 6],
    pub instance_id: u32,
    pub instance_mask: u32,
    pub instance_contribution_to_hit_group_index: u32,
    pub instance_flags: PartitionedAccelerationStructureInstanceFlagsNV,
    pub instance_index: u32,
    pub partition_index: u32,
    pub acceleration_structure: DeviceAddress,
}
impl Default for PartitionedAccelerationStructureWriteInstanceDataNV {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            explicit_aabb: [Default::default(); _],
            instance_id: Default::default(),
            instance_mask: Default::default(),
            instance_contribution_to_hit_group_index: Default::default(),
            instance_flags: Default::default(),
            instance_index: Default::default(),
            partition_index: Default::default(),
            acceleration_structure: Default::default(),
        }
    }
}
impl PartitionedAccelerationStructureWriteInstanceDataNV {
    pub fn transform(mut self, transform: TransformMatrixKHR) -> Self {
        self.transform = transform;
        self
    }
    pub fn explicit_aabb(mut self, explicit_aabb: [f32; 6]) -> Self {
        self.explicit_aabb = explicit_aabb;
        self
    }
    pub fn instance_id(mut self, instance_id: u32) -> Self {
        self.instance_id = instance_id;
        self
    }
    pub fn instance_mask(mut self, instance_mask: u32) -> Self {
        self.instance_mask = instance_mask;
        self
    }
    pub fn instance_contribution_to_hit_group_index(
        mut self,
        instance_contribution_to_hit_group_index: u32,
    ) -> Self {
        self.instance_contribution_to_hit_group_index = instance_contribution_to_hit_group_index;
        self
    }
    pub fn instance_flags(
        mut self,
        instance_flags: PartitionedAccelerationStructureInstanceFlagsNV,
    ) -> Self {
        self.instance_flags = instance_flags;
        self
    }
    pub fn instance_index(mut self, instance_index: u32) -> Self {
        self.instance_index = instance_index;
        self
    }
    pub fn partition_index(mut self, partition_index: u32) -> Self {
        self.partition_index = partition_index;
        self
    }
    pub fn acceleration_structure(mut self, acceleration_structure: DeviceAddress) -> Self {
        self.acceleration_structure = acceleration_structure;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PartitionedAccelerationStructureUpdateInstanceDataNV {
    pub instance_index: u32,
    pub instance_contribution_to_hit_group_index: u32,
    pub acceleration_structure: DeviceAddress,
}
impl PartitionedAccelerationStructureUpdateInstanceDataNV {
    pub fn instance_index(mut self, instance_index: u32) -> Self {
        self.instance_index = instance_index;
        self
    }
    pub fn instance_contribution_to_hit_group_index(
        mut self,
        instance_contribution_to_hit_group_index: u32,
    ) -> Self {
        self.instance_contribution_to_hit_group_index = instance_contribution_to_hit_group_index;
        self
    }
    pub fn acceleration_structure(mut self, acceleration_structure: DeviceAddress) -> Self {
        self.acceleration_structure = acceleration_structure;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PartitionedAccelerationStructureWritePartitionTranslationDataNV {
    pub partition_index: u32,
    pub partition_translation: [f32; 3],
}
impl Default for PartitionedAccelerationStructureWritePartitionTranslationDataNV {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            partition_translation: [Default::default(); _],
        }
    }
}
impl PartitionedAccelerationStructureWritePartitionTranslationDataNV {
    pub fn partition_index(mut self, partition_index: u32) -> Self {
        self.partition_index = partition_index;
        self
    }
    pub fn partition_translation(mut self, partition_translation: [f32; 3]) -> Self {
        self.partition_translation = partition_translation;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteDescriptorSetPartitionedAccelerationStructureNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub acceleration_structure_count: u32,
    pub p_acceleration_structures: *const DeviceAddress,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for WriteDescriptorSetPartitionedAccelerationStructureNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV,
            p_next: core::ptr::null_mut(),
            acceleration_structure_count: Default::default(),
            p_acceleration_structures: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> WriteDescriptorSetPartitionedAccelerationStructureNV<'a> {
    pub fn acceleration_structures(mut self, acceleration_structures: &'a [DeviceAddress]) -> Self {
        self.acceleration_structure_count = acceleration_structures.len().try_into().unwrap();
        self.p_acceleration_structures = acceleration_structures.as_ptr();
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PartitionedAccelerationStructureInstancesInputNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: BuildAccelerationStructureFlagsKHR,
    pub instance_count: u32,
    pub max_instance_per_partition_count: u32,
    pub partition_count: u32,
    pub max_instance_in_global_partition_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PartitionedAccelerationStructureInstancesInputNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV,
            p_next: core::ptr::null_mut(),
            flags: Default::default(),
            instance_count: Default::default(),
            max_instance_per_partition_count: Default::default(),
            partition_count: Default::default(),
            max_instance_in_global_partition_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PartitionedAccelerationStructureInstancesInputNV<'a> {
    pub fn flags(mut self, flags: BuildAccelerationStructureFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    pub fn instance_count(mut self, instance_count: u32) -> Self {
        self.instance_count = instance_count;
        self
    }
    pub fn max_instance_per_partition_count(
        mut self,
        max_instance_per_partition_count: u32,
    ) -> Self {
        self.max_instance_per_partition_count = max_instance_per_partition_count;
        self
    }
    pub fn partition_count(mut self, partition_count: u32) -> Self {
        self.partition_count = partition_count;
        self
    }
    pub fn max_instance_in_global_partition_count(
        mut self,
        max_instance_in_global_partition_count: u32,
    ) -> Self {
        self.max_instance_in_global_partition_count = max_instance_in_global_partition_count;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BuildPartitionedAccelerationStructureInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub input: PartitionedAccelerationStructureInstancesInputNV<'a>,
    pub src_acceleration_structure_data: DeviceAddress,
    pub dst_acceleration_structure_data: DeviceAddress,
    pub scratch_data: DeviceAddress,
    pub src_infos: DeviceAddress,
    pub src_infos_count: DeviceAddress,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BuildPartitionedAccelerationStructureInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV,
            p_next: core::ptr::null_mut(),
            input: Default::default(),
            src_acceleration_structure_data: Default::default(),
            dst_acceleration_structure_data: Default::default(),
            scratch_data: Default::default(),
            src_infos: Default::default(),
            src_infos_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> BuildPartitionedAccelerationStructureInfoNV<'a> {
    pub fn input(mut self, input: PartitionedAccelerationStructureInstancesInputNV<'a>) -> Self {
        self.input = input;
        self
    }
    pub fn src_acceleration_structure_data(
        mut self,
        src_acceleration_structure_data: DeviceAddress,
    ) -> Self {
        self.src_acceleration_structure_data = src_acceleration_structure_data;
        self
    }
    pub fn dst_acceleration_structure_data(
        mut self,
        dst_acceleration_structure_data: DeviceAddress,
    ) -> Self {
        self.dst_acceleration_structure_data = dst_acceleration_structure_data;
        self
    }
    pub fn scratch_data(mut self, scratch_data: DeviceAddress) -> Self {
        self.scratch_data = scratch_data;
        self
    }
    pub fn src_infos(mut self, src_infos: DeviceAddress) -> Self {
        self.src_infos = src_infos;
        self
    }
    pub fn src_infos_count(mut self, src_infos_count: DeviceAddress) -> Self {
        self.src_infos_count = src_infos_count;
        self
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PartitionedAccelerationStructureOpTypeNV(i32);
impl PartitionedAccelerationStructureOpTypeNV {
    pub const WRITE_INSTANCE_NV: Self = Self(0);
    pub const UPDATE_INSTANCE_NV: Self = Self(1);
    pub const WRITE_PARTITION_TRANSLATION_NV: Self = Self(2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PartitionedAccelerationStructureInstanceFlagsNV: Flags {
        const FLAG_TRIANGLE_FACING_CULL_DISABLE_NV = PartitionedAccelerationStructureInstanceFlagBitsNV::FLAG_TRIANGLE_FACING_CULL_DISABLE_NV.0;
        const FLAG_TRIANGLE_FLIP_FACING_NV = PartitionedAccelerationStructureInstanceFlagBitsNV::FLAG_TRIANGLE_FLIP_FACING_NV.0;
        const FLAG_FORCE_OPAQUE_NV = PartitionedAccelerationStructureInstanceFlagBitsNV::FLAG_FORCE_OPAQUE_NV.0;
        const FLAG_FORCE_NO_OPAQUE_NV = PartitionedAccelerationStructureInstanceFlagBitsNV::FLAG_FORCE_NO_OPAQUE_NV.0;
        const FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV = PartitionedAccelerationStructureInstanceFlagBitsNV::FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PartitionedAccelerationStructureInstanceFlagBitsNV(u32);
impl PartitionedAccelerationStructureInstanceFlagBitsNV {
    pub const FLAG_TRIANGLE_FACING_CULL_DISABLE_NV: Self = Self(1 << 0);
    pub const FLAG_TRIANGLE_FLIP_FACING_NV: Self = Self(1 << 1);
    pub const FLAG_FORCE_OPAQUE_NV: Self = Self(1 << 2);
    pub const FLAG_FORCE_NO_OPAQUE_NV: Self = Self(1 << 3);
    pub const FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV: Self = Self(1 << 4);
}
pub type PFN_vkGetPartitionedAccelerationStructuresBuildSizesNV = unsafe extern "system" fn(
    device: Device,
    p_info: *const PartitionedAccelerationStructureInstancesInputNV<'_>,
    p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
);
pub type PFN_vkCmdBuildPartitionedAccelerationStructuresNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_build_info: *const BuildPartitionedAccelerationStructureInfoNV<'_>,
);
