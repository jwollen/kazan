//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_partitioned_acceleration_structure.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_partitioned_acceleration_structure";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    pub const PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV: u32 = !0;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePartitionedAccelerationStructureFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub partitioned_acceleration_structure: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePartitionedAccelerationStructureFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePartitionedAccelerationStructureFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "partitioned_acceleration_structure",
                    &self.partitioned_acceleration_structure,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDevicePartitionedAccelerationStructureFeaturesNV<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePartitionedAccelerationStructureFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDevicePartitionedAccelerationStructureFeaturesNV<'a>
    {
    }

    impl Default for PhysicalDevicePartitionedAccelerationStructureFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                partitioned_acceleration_structure: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePartitionedAccelerationStructureFeaturesNV<'a> {
        #[inline]
        pub fn partitioned_acceleration_structure(
            mut self,
            partitioned_acceleration_structure: bool,
        ) -> Self {
            self.partitioned_acceleration_structure = partitioned_acceleration_structure.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePartitionedAccelerationStructurePropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_partition_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePartitionedAccelerationStructurePropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePartitionedAccelerationStructurePropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_partition_count", &self.max_partition_count)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDevicePartitionedAccelerationStructurePropertiesNV<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDevicePartitionedAccelerationStructurePropertiesNV<'a>
    {
    }

    impl Default for PhysicalDevicePartitionedAccelerationStructurePropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_partition_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePartitionedAccelerationStructurePropertiesNV<'a> {
        #[inline]
        pub fn max_partition_count(mut self, max_partition_count: u32) -> Self {
            self.max_partition_count = max_partition_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBuildPartitionedAccelerationStructureIndirectCommandNV.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct BuildPartitionedAccelerationStructureIndirectCommandNV {
        pub op_type: PartitionedAccelerationStructureOpTypeNV,
        pub arg_count: u32,
        pub arg_data: StridedDeviceAddressNV,
    }

    impl BuildPartitionedAccelerationStructureIndirectCommandNV {
        #[inline]
        pub fn op_type(mut self, op_type: PartitionedAccelerationStructureOpTypeNV) -> Self {
            self.op_type = op_type;
            self
        }

        #[inline]
        pub fn arg_count(mut self, arg_count: u32) -> Self {
            self.arg_count = arg_count;
            self
        }

        #[inline]
        pub fn arg_data(mut self, arg_data: StridedDeviceAddressNV) -> Self {
            self.arg_data = arg_data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPartitionedAccelerationStructureFlagsNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PartitionedAccelerationStructureFlagsNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub enable_partition_translation: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PartitionedAccelerationStructureFlagsNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PartitionedAccelerationStructureFlagsNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "enable_partition_translation",
                    &self.enable_partition_translation,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PartitionedAccelerationStructureFlagsNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV;
    }

    unsafe impl<'a> Extends<PartitionedAccelerationStructureInstancesInputNV<'a>>
        for PartitionedAccelerationStructureFlagsNV<'a>
    {
    }

    impl Default for PartitionedAccelerationStructureFlagsNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                enable_partition_translation: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PartitionedAccelerationStructureFlagsNV<'a> {
        #[inline]
        pub fn enable_partition_translation(mut self, enable_partition_translation: bool) -> Self {
            self.enable_partition_translation = enable_partition_translation.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPartitionedAccelerationStructureWriteInstanceDataNV.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
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
        #[inline]
        pub fn transform(mut self, transform: TransformMatrixKHR) -> Self {
            self.transform = transform;
            self
        }

        #[inline]
        pub fn explicit_aabb(mut self, explicit_aabb: [f32; 6]) -> Self {
            self.explicit_aabb = explicit_aabb;
            self
        }

        #[inline]
        pub fn instance_id(mut self, instance_id: u32) -> Self {
            self.instance_id = instance_id;
            self
        }

        #[inline]
        pub fn instance_mask(mut self, instance_mask: u32) -> Self {
            self.instance_mask = instance_mask;
            self
        }

        #[inline]
        pub fn instance_contribution_to_hit_group_index(
            mut self,
            instance_contribution_to_hit_group_index: u32,
        ) -> Self {
            self.instance_contribution_to_hit_group_index =
                instance_contribution_to_hit_group_index;
            self
        }

        #[inline]
        pub fn instance_flags(
            mut self,
            instance_flags: PartitionedAccelerationStructureInstanceFlagsNV,
        ) -> Self {
            self.instance_flags = instance_flags;
            self
        }

        #[inline]
        pub fn instance_index(mut self, instance_index: u32) -> Self {
            self.instance_index = instance_index;
            self
        }

        #[inline]
        pub fn partition_index(mut self, partition_index: u32) -> Self {
            self.partition_index = partition_index;
            self
        }

        #[inline]
        pub fn acceleration_structure(mut self, acceleration_structure: DeviceAddress) -> Self {
            self.acceleration_structure = acceleration_structure;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPartitionedAccelerationStructureUpdateInstanceDataNV.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct PartitionedAccelerationStructureUpdateInstanceDataNV {
        pub instance_index: u32,
        pub instance_contribution_to_hit_group_index: u32,
        pub acceleration_structure: DeviceAddress,
    }

    impl PartitionedAccelerationStructureUpdateInstanceDataNV {
        #[inline]
        pub fn instance_index(mut self, instance_index: u32) -> Self {
            self.instance_index = instance_index;
            self
        }

        #[inline]
        pub fn instance_contribution_to_hit_group_index(
            mut self,
            instance_contribution_to_hit_group_index: u32,
        ) -> Self {
            self.instance_contribution_to_hit_group_index =
                instance_contribution_to_hit_group_index;
            self
        }

        #[inline]
        pub fn acceleration_structure(mut self, acceleration_structure: DeviceAddress) -> Self {
            self.acceleration_structure = acceleration_structure;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPartitionedAccelerationStructureWritePartitionTranslationDataNV.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
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
        #[inline]
        pub fn partition_index(mut self, partition_index: u32) -> Self {
            self.partition_index = partition_index;
            self
        }

        #[inline]
        pub fn partition_translation(mut self, partition_translation: [f32; 3]) -> Self {
            self.partition_translation = partition_translation;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkWriteDescriptorSetPartitionedAccelerationStructureNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct WriteDescriptorSetPartitionedAccelerationStructureNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub acceleration_structure_count: u32,
        pub p_acceleration_structures: *const DeviceAddress,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for WriteDescriptorSetPartitionedAccelerationStructureNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("WriteDescriptorSetPartitionedAccelerationStructureNV")
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

    unsafe impl<'a> TaggedStructure<'a> for WriteDescriptorSetPartitionedAccelerationStructureNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV;
    }

    unsafe impl<'a> Extends<WriteDescriptorSet<'a>>
        for WriteDescriptorSetPartitionedAccelerationStructureNV<'a>
    {
    }

    impl Default for WriteDescriptorSetPartitionedAccelerationStructureNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                acceleration_structure_count: Default::default(),
                p_acceleration_structures: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> WriteDescriptorSetPartitionedAccelerationStructureNV<'a> {
        #[inline]
        pub fn acceleration_structures(
            mut self,
            acceleration_structures: &'a [DeviceAddress],
        ) -> Self {
            self.acceleration_structure_count = acceleration_structures.len().try_into().unwrap();
            self.p_acceleration_structures = acceleration_structures.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPartitionedAccelerationStructureInstancesInputNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for PartitionedAccelerationStructureInstancesInputNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PartitionedAccelerationStructureInstancesInputNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("instance_count", &self.instance_count)
                .field(
                    "max_instance_per_partition_count",
                    &self.max_instance_per_partition_count,
                )
                .field("partition_count", &self.partition_count)
                .field(
                    "max_instance_in_global_partition_count",
                    &self.max_instance_in_global_partition_count,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PartitionedAccelerationStructureInstancesInputNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV;
    }

    impl Default for PartitionedAccelerationStructureInstancesInputNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
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
        #[inline]
        pub fn flags(mut self, flags: BuildAccelerationStructureFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn instance_count(mut self, instance_count: u32) -> Self {
            self.instance_count = instance_count;
            self
        }

        #[inline]
        pub fn max_instance_per_partition_count(
            mut self,
            max_instance_per_partition_count: u32,
        ) -> Self {
            self.max_instance_per_partition_count = max_instance_per_partition_count;
            self
        }

        #[inline]
        pub fn partition_count(mut self, partition_count: u32) -> Self {
            self.partition_count = partition_count;
            self
        }

        #[inline]
        pub fn max_instance_in_global_partition_count(
            mut self,
            max_instance_in_global_partition_count: u32,
        ) -> Self {
            self.max_instance_in_global_partition_count = max_instance_in_global_partition_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBuildPartitionedAccelerationStructureInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for BuildPartitionedAccelerationStructureInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BuildPartitionedAccelerationStructureInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("input", &self.input)
                .field(
                    "src_acceleration_structure_data",
                    &self.src_acceleration_structure_data,
                )
                .field(
                    "dst_acceleration_structure_data",
                    &self.dst_acceleration_structure_data,
                )
                .field("scratch_data", &self.scratch_data)
                .field("src_infos", &self.src_infos)
                .field("src_infos_count", &self.src_infos_count)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BuildPartitionedAccelerationStructureInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV;
    }

    impl Default for BuildPartitionedAccelerationStructureInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
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
        #[inline]
        pub fn input(
            mut self,
            input: PartitionedAccelerationStructureInstancesInputNV<'a>,
        ) -> Self {
            self.input = input;
            self
        }

        #[inline]
        pub fn src_acceleration_structure_data(
            mut self,
            src_acceleration_structure_data: DeviceAddress,
        ) -> Self {
            self.src_acceleration_structure_data = src_acceleration_structure_data;
            self
        }

        #[inline]
        pub fn dst_acceleration_structure_data(
            mut self,
            dst_acceleration_structure_data: DeviceAddress,
        ) -> Self {
            self.dst_acceleration_structure_data = dst_acceleration_structure_data;
            self
        }

        #[inline]
        pub fn scratch_data(mut self, scratch_data: DeviceAddress) -> Self {
            self.scratch_data = scratch_data;
            self
        }

        #[inline]
        pub fn src_infos(mut self, src_infos: DeviceAddress) -> Self {
            self.src_infos = src_infos;
            self
        }

        #[inline]
        pub fn src_infos_count(mut self, src_infos_count: DeviceAddress) -> Self {
            self.src_infos_count = src_infos_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPartitionedAccelerationStructureOpTypeNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PartitionedAccelerationStructureOpTypeNV(i32);

    impl PartitionedAccelerationStructureOpTypeNV {
        pub const WRITE_INSTANCE_NV: Self = Self(0);
        pub const UPDATE_INSTANCE_NV: Self = Self(1);
        pub const WRITE_PARTITION_TRANSLATION_NV: Self = Self(2);
    }

    impl fmt::Debug for PartitionedAccelerationStructureOpTypeNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::WRITE_INSTANCE_NV => Some("WRITE_INSTANCE_NV"),
                Self::UPDATE_INSTANCE_NV => Some("UPDATE_INSTANCE_NV"),
                Self::WRITE_PARTITION_TRANSLATION_NV => Some("WRITE_PARTITION_TRANSLATION_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPartitionedAccelerationStructureInstanceFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PartitionedAccelerationStructureInstanceFlagsNV(Flags);
    vk_bitflags_wrapped!(PartitionedAccelerationStructureInstanceFlagsNV, Flags);

    impl PartitionedAccelerationStructureInstanceFlagsNV {
        pub const FLAG_TRIANGLE_FACING_CULL_DISABLE_NV: Self = Self(PartitionedAccelerationStructureInstanceFlagBitsNV::FLAG_TRIANGLE_FACING_CULL_DISABLE_NV.0);
        pub const FLAG_TRIANGLE_FLIP_FACING_NV: Self = Self(
            PartitionedAccelerationStructureInstanceFlagBitsNV::FLAG_TRIANGLE_FLIP_FACING_NV.0,
        );
        pub const FLAG_FORCE_OPAQUE_NV: Self =
            Self(PartitionedAccelerationStructureInstanceFlagBitsNV::FLAG_FORCE_OPAQUE_NV.0);
        pub const FLAG_FORCE_NO_OPAQUE_NV: Self =
            Self(PartitionedAccelerationStructureInstanceFlagBitsNV::FLAG_FORCE_NO_OPAQUE_NV.0);
        pub const FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV: Self = Self(PartitionedAccelerationStructureInstanceFlagBitsNV::FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV.0);
    }

    impl fmt::Debug for PartitionedAccelerationStructureInstanceFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (PartitionedAccelerationStructureInstanceFlagsNV::FLAG_TRIANGLE_FACING_CULL_DISABLE_NV.0, "FLAG_TRIANGLE_FACING_CULL_DISABLE_NV"),
                (PartitionedAccelerationStructureInstanceFlagsNV::FLAG_TRIANGLE_FLIP_FACING_NV.0, "FLAG_TRIANGLE_FLIP_FACING_NV"),
                (PartitionedAccelerationStructureInstanceFlagsNV::FLAG_FORCE_OPAQUE_NV.0, "FLAG_FORCE_OPAQUE_NV"),
                (PartitionedAccelerationStructureInstanceFlagsNV::FLAG_FORCE_NO_OPAQUE_NV.0, "FLAG_FORCE_NO_OPAQUE_NV"),
                (PartitionedAccelerationStructureInstanceFlagsNV::FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV.0, "FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPartitionedAccelerationStructureInstanceFlagBitsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PartitionedAccelerationStructureInstanceFlagBitsNV(u32);

    impl PartitionedAccelerationStructureInstanceFlagBitsNV {
        pub const FLAG_TRIANGLE_FACING_CULL_DISABLE_NV: Self = Self(1 << 0);
        pub const FLAG_TRIANGLE_FLIP_FACING_NV: Self = Self(1 << 1);
        pub const FLAG_FORCE_OPAQUE_NV: Self = Self(1 << 2);
        pub const FLAG_FORCE_NO_OPAQUE_NV: Self = Self(1 << 3);
        pub const FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV: Self = Self(1 << 4);
    }

    impl fmt::Debug for PartitionedAccelerationStructureInstanceFlagBitsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::FLAG_TRIANGLE_FACING_CULL_DISABLE_NV => {
                    Some("FLAG_TRIANGLE_FACING_CULL_DISABLE_NV")
                }
                Self::FLAG_TRIANGLE_FLIP_FACING_NV => Some("FLAG_TRIANGLE_FLIP_FACING_NV"),
                Self::FLAG_FORCE_OPAQUE_NV => Some("FLAG_FORCE_OPAQUE_NV"),
                Self::FLAG_FORCE_NO_OPAQUE_NV => Some("FLAG_FORCE_NO_OPAQUE_NV"),
                Self::FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV => {
                    Some("FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV")
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPartitionedAccelerationStructuresBuildSizesNV.html>
    pub type PFN_vkGetPartitionedAccelerationStructuresBuildSizesNV = unsafe extern "system" fn(
        device: Device,
        p_info: *const PartitionedAccelerationStructureInstancesInputNV<'_>,
        p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBuildPartitionedAccelerationStructuresNV.html>
    pub type PFN_vkCmdBuildPartitionedAccelerationStructuresNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_build_info: *const BuildPartitionedAccelerationStructureInfoNV<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV =
        PhysicalDevicePartitionedAccelerationStructureFeaturesNV<'static>;
    pub type VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV =
        PhysicalDevicePartitionedAccelerationStructurePropertiesNV<'static>;
    pub type VkBuildPartitionedAccelerationStructureIndirectCommandNV =
        BuildPartitionedAccelerationStructureIndirectCommandNV;
    pub type VkPartitionedAccelerationStructureFlagsNV =
        PartitionedAccelerationStructureFlagsNV<'static>;
    pub type VkPartitionedAccelerationStructureWriteInstanceDataNV =
        PartitionedAccelerationStructureWriteInstanceDataNV;
    pub type VkPartitionedAccelerationStructureUpdateInstanceDataNV =
        PartitionedAccelerationStructureUpdateInstanceDataNV;
    pub type VkPartitionedAccelerationStructureWritePartitionTranslationDataNV =
        PartitionedAccelerationStructureWritePartitionTranslationDataNV;
    pub type VkWriteDescriptorSetPartitionedAccelerationStructureNV =
        WriteDescriptorSetPartitionedAccelerationStructureNV<'static>;
    pub type VkPartitionedAccelerationStructureInstancesInputNV =
        PartitionedAccelerationStructureInstancesInputNV<'static>;
    pub type VkBuildPartitionedAccelerationStructureInfoNV =
        BuildPartitionedAccelerationStructureInfoNV<'static>;
    pub type VkPartitionedAccelerationStructureOpTypeNV = PartitionedAccelerationStructureOpTypeNV;
    pub type VkPartitionedAccelerationStructureInstanceFlagsNV =
        PartitionedAccelerationStructureInstanceFlagsNV;
    pub type VkPartitionedAccelerationStructureInstanceFlagBitsNV =
        PartitionedAccelerationStructureInstanceFlagBitsNV;
    impl PhysicalDevicePartitionedAccelerationStructureFeaturesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDevicePartitionedAccelerationStructurePropertiesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PartitionedAccelerationStructureFlagsNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPartitionedAccelerationStructureFlagsNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl WriteDescriptorSetPartitionedAccelerationStructureNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkWriteDescriptorSetPartitionedAccelerationStructureNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PartitionedAccelerationStructureInstancesInputNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPartitionedAccelerationStructureInstancesInputNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BuildPartitionedAccelerationStructureInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkBuildPartitionedAccelerationStructureInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_partitioned_acceleration_structures_build_sizes_nv:
        PFN_vkGetPartitionedAccelerationStructuresBuildSizesNV,
    cmd_build_partitioned_acceleration_structures_nv:
        PFN_vkCmdBuildPartitionedAccelerationStructuresNV,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_partitioned_acceleration_structures_build_sizes_nv: transmute(
                    load(c"vkGetPartitionedAccelerationStructuresBuildSizesNV")
                        .ok_or(MissingEntryPointError)?,
                ),
                cmd_build_partitioned_acceleration_structures_nv: transmute(
                    load(c"vkCmdBuildPartitionedAccelerationStructuresNV")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPartitionedAccelerationStructuresBuildSizesNV.html>
    #[inline]
    pub unsafe fn get_partitioned_acceleration_structures_build_sizes_nv(
        &self,
        device: Device,
        info: &PartitionedAccelerationStructureInstancesInputNV<'_>,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ) {
        unsafe {
            (self.get_partitioned_acceleration_structures_build_sizes_nv)(device, info, size_info)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBuildPartitionedAccelerationStructuresNV.html>
    #[inline]
    pub unsafe fn cmd_build_partitioned_acceleration_structures_nv(
        &self,
        command_buffer: CommandBuffer,
        build_info: &BuildPartitionedAccelerationStructureInfoNV<'_>,
    ) {
        unsafe {
            (self.cmd_build_partitioned_acceleration_structures_nv)(command_buffer, build_info)
        }
    }
}
