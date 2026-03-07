#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_displacement_micromap";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDisplacementMicromapFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDisplacementMicromapFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub displacement_micromap: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDisplacementMicromapFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDisplacementMicromapFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("displacement_micromap", &self.displacement_micromap)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDisplacementMicromapFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDisplacementMicromapFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceDisplacementMicromapFeaturesNV<'a> {}

    impl Default for PhysicalDeviceDisplacementMicromapFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                displacement_micromap: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDisplacementMicromapFeaturesNV<'a> {
        pub fn displacement_micromap(mut self, displacement_micromap: bool) -> Self {
            self.displacement_micromap = displacement_micromap.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDisplacementMicromapPropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDisplacementMicromapPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_displacement_micromap_subdivision_level: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDisplacementMicromapPropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDisplacementMicromapPropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_displacement_micromap_subdivision_level",
                    &self.max_displacement_micromap_subdivision_level,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDisplacementMicromapPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceDisplacementMicromapPropertiesNV<'a>
    {
    }

    impl Default for PhysicalDeviceDisplacementMicromapPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_displacement_micromap_subdivision_level: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDisplacementMicromapPropertiesNV<'a> {
        pub fn max_displacement_micromap_subdivision_level(
            mut self,
            max_displacement_micromap_subdivision_level: u32,
        ) -> Self {
            self.max_displacement_micromap_subdivision_level =
                max_displacement_micromap_subdivision_level;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureTrianglesDisplacementMicromapNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AccelerationStructureTrianglesDisplacementMicromapNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub displacement_bias_and_scale_format: Format,
        pub displacement_vector_format: Format,
        pub displacement_bias_and_scale_buffer: DeviceOrHostAddressConstKHR<'a>,
        pub displacement_bias_and_scale_stride: DeviceSize,
        pub displacement_vector_buffer: DeviceOrHostAddressConstKHR<'a>,
        pub displacement_vector_stride: DeviceSize,
        pub displaced_micromap_primitive_flags: DeviceOrHostAddressConstKHR<'a>,
        pub displaced_micromap_primitive_flags_stride: DeviceSize,
        pub index_type: IndexType,
        pub index_buffer: DeviceOrHostAddressConstKHR<'a>,
        pub index_stride: DeviceSize,
        pub base_triangle: u32,
        pub usage_counts_count: u32,
        pub p_usage_counts: *const MicromapUsageEXT,
        pub pp_usage_counts: *const *const MicromapUsageEXT,
        pub micromap: MicromapEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureTrianglesDisplacementMicromapNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureTrianglesDisplacementMicromapNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "displacement_bias_and_scale_format",
                    &self.displacement_bias_and_scale_format,
                )
                .field(
                    "displacement_vector_format",
                    &self.displacement_vector_format,
                )
                .field(
                    "displacement_bias_and_scale_buffer",
                    &self.displacement_bias_and_scale_buffer,
                )
                .field(
                    "displacement_bias_and_scale_stride",
                    &self.displacement_bias_and_scale_stride,
                )
                .field(
                    "displacement_vector_buffer",
                    &self.displacement_vector_buffer,
                )
                .field(
                    "displacement_vector_stride",
                    &self.displacement_vector_stride,
                )
                .field(
                    "displaced_micromap_primitive_flags",
                    &self.displaced_micromap_primitive_flags,
                )
                .field(
                    "displaced_micromap_primitive_flags_stride",
                    &self.displaced_micromap_primitive_flags_stride,
                )
                .field("index_type", &self.index_type)
                .field("index_buffer", &self.index_buffer)
                .field("index_stride", &self.index_stride)
                .field("base_triangle", &self.base_triangle)
                .field("usage_counts_count", &self.usage_counts_count)
                .field("p_usage_counts", &self.p_usage_counts)
                .field("pp_usage_counts", &self.pp_usage_counts)
                .field("micromap", &self.micromap)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureTrianglesDisplacementMicromapNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV;
    }

    unsafe impl<'a> Extends<AccelerationStructureGeometryTrianglesDataKHR<'a>>
        for AccelerationStructureTrianglesDisplacementMicromapNV<'a>
    {
    }

    impl Default for AccelerationStructureTrianglesDisplacementMicromapNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                displacement_bias_and_scale_format: Default::default(),
                displacement_vector_format: Default::default(),
                displacement_bias_and_scale_buffer: Default::default(),
                displacement_bias_and_scale_stride: Default::default(),
                displacement_vector_buffer: Default::default(),
                displacement_vector_stride: Default::default(),
                displaced_micromap_primitive_flags: Default::default(),
                displaced_micromap_primitive_flags_stride: Default::default(),
                index_type: Default::default(),
                index_buffer: Default::default(),
                index_stride: Default::default(),
                base_triangle: Default::default(),
                usage_counts_count: Default::default(),
                p_usage_counts: core::ptr::null(),
                pp_usage_counts: core::ptr::null(),
                micromap: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureTrianglesDisplacementMicromapNV<'a> {
        pub fn displacement_bias_and_scale_format(
            mut self,
            displacement_bias_and_scale_format: Format,
        ) -> Self {
            self.displacement_bias_and_scale_format = displacement_bias_and_scale_format;
            self
        }

        pub fn displacement_vector_format(mut self, displacement_vector_format: Format) -> Self {
            self.displacement_vector_format = displacement_vector_format;
            self
        }

        pub fn displacement_bias_and_scale_buffer(
            mut self,
            displacement_bias_and_scale_buffer: DeviceOrHostAddressConstKHR<'a>,
        ) -> Self {
            self.displacement_bias_and_scale_buffer = displacement_bias_and_scale_buffer;
            self
        }

        pub fn displacement_bias_and_scale_stride(
            mut self,
            displacement_bias_and_scale_stride: DeviceSize,
        ) -> Self {
            self.displacement_bias_and_scale_stride = displacement_bias_and_scale_stride;
            self
        }

        pub fn displacement_vector_buffer(
            mut self,
            displacement_vector_buffer: DeviceOrHostAddressConstKHR<'a>,
        ) -> Self {
            self.displacement_vector_buffer = displacement_vector_buffer;
            self
        }

        pub fn displacement_vector_stride(
            mut self,
            displacement_vector_stride: DeviceSize,
        ) -> Self {
            self.displacement_vector_stride = displacement_vector_stride;
            self
        }

        pub fn displaced_micromap_primitive_flags(
            mut self,
            displaced_micromap_primitive_flags: DeviceOrHostAddressConstKHR<'a>,
        ) -> Self {
            self.displaced_micromap_primitive_flags = displaced_micromap_primitive_flags;
            self
        }

        pub fn displaced_micromap_primitive_flags_stride(
            mut self,
            displaced_micromap_primitive_flags_stride: DeviceSize,
        ) -> Self {
            self.displaced_micromap_primitive_flags_stride =
                displaced_micromap_primitive_flags_stride;
            self
        }

        pub fn index_type(mut self, index_type: IndexType) -> Self {
            self.index_type = index_type;
            self
        }

        pub fn index_buffer(mut self, index_buffer: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.index_buffer = index_buffer;
            self
        }

        pub fn index_stride(mut self, index_stride: DeviceSize) -> Self {
            self.index_stride = index_stride;
            self
        }

        pub fn base_triangle(mut self, base_triangle: u32) -> Self {
            self.base_triangle = base_triangle;
            self
        }

        pub fn usage_counts(mut self, usage_counts: &'a [MicromapUsageEXT]) -> Self {
            self.usage_counts_count = usage_counts.len().try_into().unwrap();
            self.p_usage_counts = usage_counts.as_ptr();
            self
        }

        pub fn usage_counts_ptrs(mut self, usage_counts_ptrs: &'a [&'a MicromapUsageEXT]) -> Self {
            self.usage_counts_count = usage_counts_ptrs.len().try_into().unwrap();
            self.pp_usage_counts = usage_counts_ptrs.as_ptr() as _;
            self
        }

        pub fn micromap(mut self, micromap: MicromapEXT) -> Self {
            self.micromap = micromap;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplacementMicromapFormatNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DisplacementMicromapFormatNV(i32);

    impl DisplacementMicromapFormatNV {
        pub const _64_TRIANGLES_64_BYTES_NV: Self = Self(1);
        pub const _256_TRIANGLES_128_BYTES_NV: Self = Self(2);
        pub const _1024_TRIANGLES_128_BYTES_NV: Self = Self(3);
    }

    impl fmt::Debug for DisplacementMicromapFormatNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_64_TRIANGLES_64_BYTES_NV => Some("_64_TRIANGLES_64_BYTES_NV"),
                Self::_256_TRIANGLES_128_BYTES_NV => Some("_256_TRIANGLES_128_BYTES_NV"),
                Self::_1024_TRIANGLES_128_BYTES_NV => Some("_1024_TRIANGLES_128_BYTES_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
}
