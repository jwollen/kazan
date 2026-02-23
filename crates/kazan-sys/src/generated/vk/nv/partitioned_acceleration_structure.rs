#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub const PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV: u32 = !0;
#[repr(C)]
pub struct PhysicalDevicePartitionedAccelerationStructureFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub partitioned_acceleration_structure: Bool32,
}
#[repr(C)]
pub struct PhysicalDevicePartitionedAccelerationStructurePropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_partition_count: u32,
}
#[repr(C)]
pub struct BuildPartitionedAccelerationStructureIndirectCommandNV {
    pub op_type: PartitionedAccelerationStructureOpTypeNV,
    pub arg_count: u32,
    pub arg_data: StridedDeviceAddressNV,
}
#[repr(C)]
pub struct PartitionedAccelerationStructureFlagsNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub enable_partition_translation: Bool32,
}
#[repr(C)]
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
#[repr(C)]
pub struct PartitionedAccelerationStructureUpdateInstanceDataNV {
    pub instance_index: u32,
    pub instance_contribution_to_hit_group_index: u32,
    pub acceleration_structure: DeviceAddress,
}
#[repr(C)]
pub struct PartitionedAccelerationStructureWritePartitionTranslationDataNV {
    pub partition_index: u32,
    pub partition_translation: [f32; 3],
}
#[repr(C)]
pub struct WriteDescriptorSetPartitionedAccelerationStructureNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub acceleration_structure_count: u32,
    pub p_acceleration_structures: *const DeviceAddress,
}
#[repr(C)]
pub struct PartitionedAccelerationStructureInstancesInputNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: BuildAccelerationStructureFlagsKHR,
    pub instance_count: u32,
    pub max_instance_per_partition_count: u32,
    pub partition_count: u32,
    pub max_instance_in_global_partition_count: u32,
}
#[repr(C)]
pub struct BuildPartitionedAccelerationStructureInfoNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub input: PartitionedAccelerationStructureInstancesInputNV,
    pub src_acceleration_structure_data: DeviceAddress,
    pub dst_acceleration_structure_data: DeviceAddress,
    pub scratch_data: DeviceAddress,
    pub src_infos: DeviceAddress,
    pub src_infos_count: DeviceAddress,
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
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PartitionedAccelerationStructureInstanceFlagsNV: Flags {
        const FLAG_TRIANGLE_FACING_CULL_DISABLE_NV = PartitionedAccelerationStructureInstanceFlagBitsNV::FLAG_TRIANGLE_FACING_CULL_DISABLE_NV.0;
        const FLAG_TRIANGLE_FLIP_FACING_NV = PartitionedAccelerationStructureInstanceFlagBitsNV::FLAG_TRIANGLE_FLIP_FACING_NV.0;
        const FLAG_FORCE_OPAQUE_NV = PartitionedAccelerationStructureInstanceFlagBitsNV::FLAG_FORCE_OPAQUE_NV.0;
        const FLAG_FORCE_NO_OPAQUE_NV = PartitionedAccelerationStructureInstanceFlagBitsNV::FLAG_FORCE_NO_OPAQUE_NV.0;
        const FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV = PartitionedAccelerationStructureInstanceFlagBitsNV::FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    p_info: *const PartitionedAccelerationStructureInstancesInputNV,
    p_size_info: *mut AccelerationStructureBuildSizesInfoKHR,
);
pub type PFN_vkCmdBuildPartitionedAccelerationStructuresNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_build_info: *const BuildPartitionedAccelerationStructureInfoNV,
);
