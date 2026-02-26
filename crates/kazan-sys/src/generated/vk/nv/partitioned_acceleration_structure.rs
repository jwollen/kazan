#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
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
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct BuildPartitionedAccelerationStructureIndirectCommandNV {
    pub op_type: PartitionedAccelerationStructureOpTypeNV,
    pub arg_count: u32,
    pub arg_data: StridedDeviceAddressNV,
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
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PartitionedAccelerationStructureUpdateInstanceDataNV {
    pub instance_index: u32,
    pub instance_contribution_to_hit_group_index: u32,
    pub acceleration_structure: DeviceAddress,
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
