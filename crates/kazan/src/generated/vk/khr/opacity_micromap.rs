//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_opacity_micromap.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_opacity_micromap";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureGeometryMicromapDataKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureGeometryMicromapDataKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub usage_counts_count: u32,
        pub p_usage_counts: *const MicromapUsageKHR,
        pub pp_usage_counts: *const *const MicromapUsageKHR,
        pub data: DeviceAddress,
        pub triangle_array: DeviceAddress,
        pub triangle_array_stride: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureGeometryMicromapDataKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureGeometryMicromapDataKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("usage_counts_count", &self.usage_counts_count)
                .field("p_usage_counts", &self.p_usage_counts)
                .field("pp_usage_counts", &self.pp_usage_counts)
                .field("data", &self.data)
                .field("triangle_array", &self.triangle_array)
                .field("triangle_array_stride", &self.triangle_array_stride)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureGeometryMicromapDataKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_GEOMETRY_MICROMAP_DATA_KHR;
    }

    unsafe impl Extends<AccelerationStructureGeometryKHR<'_>>
        for AccelerationStructureGeometryMicromapDataKHR<'_>
    {
    }

    impl Default for AccelerationStructureGeometryMicromapDataKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                usage_counts_count: Default::default(),
                p_usage_counts: ptr::null(),
                pp_usage_counts: ptr::null(),
                data: Default::default(),
                triangle_array: Default::default(),
                triangle_array_stride: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureGeometryMicromapDataKHR<'a> {
        #[inline]
        pub fn usage_counts(
            mut self,
            usage_counts: Option<&'a [MicromapUsageKHR]>,
            usage_counts_ptrs: Option<&'a [&'a MicromapUsageKHR]>,
        ) -> Self {
            self.usage_counts_count = None
                .or_else(|| usage_counts.as_deref().map(|s| s.len()))
                .or_else(|| usage_counts_ptrs.as_deref().map(|s| s.len()))
                .unwrap_or(0)
                .try_into()
                .unwrap();
            if let Some(s) = &usage_counts_ptrs {
                assert_eq!(s.len(), self.usage_counts_count as usize);
            }
            self.p_usage_counts = usage_counts.map_or(ptr::null(), |s| s.as_ptr() as _);
            self.pp_usage_counts = usage_counts_ptrs.map_or(ptr::null(), |s| s.as_ptr() as _);
            self
        }

        #[inline]
        pub fn data(mut self, data: DeviceAddress) -> Self {
            self.data = data;
            self
        }

        #[inline]
        pub fn triangle_array(mut self, triangle_array: DeviceAddress) -> Self {
            self.triangle_array = triangle_array;
            self
        }

        #[inline]
        pub fn triangle_array_stride(mut self, triangle_array_stride: DeviceSize) -> Self {
            self.triangle_array_stride = triangle_array_stride;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMicromapUsageKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct MicromapUsageKHR {
        pub count: u32,
        pub subdivision_level: u32,
        pub format: OpacityMicromapFormatKHR,
    }

    impl MicromapUsageKHR {
        #[inline]
        pub fn count(mut self, count: u32) -> Self {
            self.count = count;
            self
        }

        #[inline]
        pub fn subdivision_level(mut self, subdivision_level: u32) -> Self {
            self.subdivision_level = subdivision_level;
            self
        }

        #[inline]
        pub fn format(mut self, format: OpacityMicromapFormatKHR) -> Self {
            self.format = format;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMicromapTriangleKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct MicromapTriangleKHR {
        pub data_offset: u32,
        pub subdivision_level: u16,
        pub format: u16,
    }

    impl MicromapTriangleKHR {
        #[inline]
        pub fn data_offset(mut self, data_offset: u32) -> Self {
            self.data_offset = data_offset;
            self
        }

        #[inline]
        pub fn subdivision_level(mut self, subdivision_level: u16) -> Self {
            self.subdivision_level = subdivision_level;
            self
        }

        #[inline]
        pub fn format(mut self, format: u16) -> Self {
            self.format = format;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceOpacityMicromapFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceOpacityMicromapFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub micromap: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceOpacityMicromapFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceOpacityMicromapFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("micromap", &self.micromap)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceOpacityMicromapFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceOpacityMicromapFeaturesKHR<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceOpacityMicromapFeaturesKHR<'_> {}

    impl Default for PhysicalDeviceOpacityMicromapFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                micromap: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceOpacityMicromapFeaturesKHR<'a> {
        #[inline]
        pub fn micromap(mut self, micromap: bool) -> Self {
            self.micromap = micromap.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceOpacityMicromapPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceOpacityMicromapPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_opacity2_state_subdivision_level: u32,
        pub max_opacity4_state_subdivision_level: u32,
        pub max_opacity_lossy4_state_subdivision_level: u32,
        pub max_micromap_triangles: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceOpacityMicromapPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceOpacityMicromapPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_opacity2_state_subdivision_level",
                    &self.max_opacity2_state_subdivision_level,
                )
                .field(
                    "max_opacity4_state_subdivision_level",
                    &self.max_opacity4_state_subdivision_level,
                )
                .field(
                    "max_opacity_lossy4_state_subdivision_level",
                    &self.max_opacity_lossy4_state_subdivision_level,
                )
                .field("max_micromap_triangles", &self.max_micromap_triangles)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceOpacityMicromapPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceOpacityMicromapPropertiesKHR<'_>
    {
    }

    impl Default for PhysicalDeviceOpacityMicromapPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_opacity2_state_subdivision_level: Default::default(),
                max_opacity4_state_subdivision_level: Default::default(),
                max_opacity_lossy4_state_subdivision_level: Default::default(),
                max_micromap_triangles: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceOpacityMicromapPropertiesKHR<'a> {
        #[inline]
        pub fn max_opacity2_state_subdivision_level(
            mut self,
            max_opacity2_state_subdivision_level: u32,
        ) -> Self {
            self.max_opacity2_state_subdivision_level = max_opacity2_state_subdivision_level;
            self
        }

        #[inline]
        pub fn max_opacity4_state_subdivision_level(
            mut self,
            max_opacity4_state_subdivision_level: u32,
        ) -> Self {
            self.max_opacity4_state_subdivision_level = max_opacity4_state_subdivision_level;
            self
        }

        #[inline]
        pub fn max_opacity_lossy4_state_subdivision_level(
            mut self,
            max_opacity_lossy4_state_subdivision_level: u32,
        ) -> Self {
            self.max_opacity_lossy4_state_subdivision_level =
                max_opacity_lossy4_state_subdivision_level;
            self
        }

        #[inline]
        pub fn max_micromap_triangles(mut self, max_micromap_triangles: u64) -> Self {
            self.max_micromap_triangles = max_micromap_triangles;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureTrianglesOpacityMicromapKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureTrianglesOpacityMicromapKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub index_type: IndexType,
        pub index_buffer: DeviceAddress,
        pub index_stride: DeviceSize,
        pub base_triangle: u32,
        pub micromap: AccelerationStructureKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureTrianglesOpacityMicromapKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureTrianglesOpacityMicromapKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("index_type", &self.index_type)
                .field("index_buffer", &self.index_buffer)
                .field("index_stride", &self.index_stride)
                .field("base_triangle", &self.base_triangle)
                .field("micromap", &self.micromap)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureTrianglesOpacityMicromapKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_KHR;
    }

    unsafe impl Extends<AccelerationStructureGeometryTrianglesDataKHR<'_>>
        for AccelerationStructureTrianglesOpacityMicromapKHR<'_>
    {
    }
    #[cfg(feature = "provisional")]
    unsafe impl Extends<AccelerationStructureDenseGeometryFormatTrianglesDataAMDX<'_>>
        for AccelerationStructureTrianglesOpacityMicromapKHR<'_>
    {
    }

    impl Default for AccelerationStructureTrianglesOpacityMicromapKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                index_type: Default::default(),
                index_buffer: Default::default(),
                index_stride: Default::default(),
                base_triangle: Default::default(),
                micromap: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureTrianglesOpacityMicromapKHR<'a> {
        #[inline]
        pub fn index_type(mut self, index_type: IndexType) -> Self {
            self.index_type = index_type;
            self
        }

        #[inline]
        pub fn index_buffer(mut self, index_buffer: DeviceAddress) -> Self {
            self.index_buffer = index_buffer;
            self
        }

        #[inline]
        pub fn index_stride(mut self, index_stride: DeviceSize) -> Self {
            self.index_stride = index_stride;
            self
        }

        #[inline]
        pub fn base_triangle(mut self, base_triangle: u32) -> Self {
            self.base_triangle = base_triangle;
            self
        }

        #[inline]
        pub fn micromap(mut self, micromap: AccelerationStructureKHR) -> Self {
            self.micromap = micromap;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpacityMicromapFormatKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OpacityMicromapFormatKHR(i32);

    impl OpacityMicromapFormatKHR {
        pub const _2_STATE_KHR: Self = Self(1);
        pub const _4_STATE_KHR: Self = Self(2);

        // VK_EXT_opacity_micromap
        pub const _2_STATE_EXT: Self = Self::_2_STATE_KHR;
        pub const _4_STATE_EXT: Self = Self::_4_STATE_KHR;
    }

    impl fmt::Debug for OpacityMicromapFormatKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_2_STATE_KHR => Some("_2_STATE_KHR"),
                Self::_4_STATE_KHR => Some("_4_STATE_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpacityMicromapSpecialIndexKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OpacityMicromapSpecialIndexKHR(i32);

    impl OpacityMicromapSpecialIndexKHR {
        pub const FULLY_TRANSPARENT_KHR: Self = Self(-1);
        pub const FULLY_OPAQUE_KHR: Self = Self(-2);
        pub const FULLY_UNKNOWN_TRANSPARENT_KHR: Self = Self(-3);
        pub const FULLY_UNKNOWN_OPAQUE_KHR: Self = Self(-4);

        // VK_EXT_opacity_micromap
        pub const FULLY_TRANSPARENT_EXT: Self = Self::FULLY_TRANSPARENT_KHR;
        pub const FULLY_OPAQUE_EXT: Self = Self::FULLY_OPAQUE_KHR;
        pub const FULLY_UNKNOWN_TRANSPARENT_EXT: Self = Self::FULLY_UNKNOWN_TRANSPARENT_KHR;
        pub const FULLY_UNKNOWN_OPAQUE_EXT: Self = Self::FULLY_UNKNOWN_OPAQUE_KHR;

        // VK_NV_cluster_acceleration_structure
        pub const CLUSTER_GEOMETRY_DISABLE_OPACITY_MICROMAP_NV: Self = Self(-5);
    }

    impl fmt::Debug for OpacityMicromapSpecialIndexKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::FULLY_TRANSPARENT_KHR => Some("FULLY_TRANSPARENT_KHR"),
                Self::FULLY_OPAQUE_KHR => Some("FULLY_OPAQUE_KHR"),
                Self::FULLY_UNKNOWN_TRANSPARENT_KHR => Some("FULLY_UNKNOWN_TRANSPARENT_KHR"),
                Self::FULLY_UNKNOWN_OPAQUE_KHR => Some("FULLY_UNKNOWN_OPAQUE_KHR"),
                Self::CLUSTER_GEOMETRY_DISABLE_OPACITY_MICROMAP_NV => {
                    Some("CLUSTER_GEOMETRY_DISABLE_OPACITY_MICROMAP_NV")
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureSerializedBlockTypeKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccelerationStructureSerializedBlockTypeKHR(i32);

    impl AccelerationStructureSerializedBlockTypeKHR {
        pub const OPACITY_MICROMAP_KHR: Self = Self(0);
    }

    impl fmt::Debug for AccelerationStructureSerializedBlockTypeKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::OPACITY_MICROMAP_KHR => Some("OPACITY_MICROMAP_KHR"),
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

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkAccelerationStructureGeometryMicromapDataKHR =
        AccelerationStructureGeometryMicromapDataKHR<'static>;
    pub type VkMicromapUsageKHR = MicromapUsageKHR;
    pub type VkMicromapTriangleKHR = MicromapTriangleKHR;
    pub type VkPhysicalDeviceOpacityMicromapFeaturesKHR =
        PhysicalDeviceOpacityMicromapFeaturesKHR<'static>;
    pub type VkPhysicalDeviceOpacityMicromapPropertiesKHR =
        PhysicalDeviceOpacityMicromapPropertiesKHR<'static>;
    pub type VkAccelerationStructureTrianglesOpacityMicromapKHR =
        AccelerationStructureTrianglesOpacityMicromapKHR<'static>;
    pub type VkOpacityMicromapFormatKHR = OpacityMicromapFormatKHR;
    pub type VkOpacityMicromapSpecialIndexKHR = OpacityMicromapSpecialIndexKHR;
    pub type VkAccelerationStructureSerializedBlockTypeKHR =
        AccelerationStructureSerializedBlockTypeKHR;
    impl AccelerationStructureGeometryMicromapDataKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkAccelerationStructureGeometryMicromapDataKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceOpacityMicromapFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceOpacityMicromapFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceOpacityMicromapPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceOpacityMicromapPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AccelerationStructureTrianglesOpacityMicromapKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkAccelerationStructureTrianglesOpacityMicromapKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
