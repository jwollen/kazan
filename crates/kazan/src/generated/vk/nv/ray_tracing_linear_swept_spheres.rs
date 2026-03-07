//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_ray_tracing_linear_swept_spheres.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_ray_tracing_linear_swept_spheres";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureGeometryLinearSweptSpheresDataNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureGeometryLinearSweptSpheresDataNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub vertex_format: Format,
        pub vertex_data: DeviceOrHostAddressConstKHR<'a>,
        pub vertex_stride: DeviceSize,
        pub radius_format: Format,
        pub radius_data: DeviceOrHostAddressConstKHR<'a>,
        pub radius_stride: DeviceSize,
        pub index_type: IndexType,
        pub index_data: DeviceOrHostAddressConstKHR<'a>,
        pub index_stride: DeviceSize,
        pub indexing_mode: RayTracingLssIndexingModeNV,
        pub end_caps_mode: RayTracingLssPrimitiveEndCapsModeNV,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureGeometryLinearSweptSpheresDataNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureGeometryLinearSweptSpheresDataNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("vertex_format", &self.vertex_format)
                .field("vertex_data", &self.vertex_data)
                .field("vertex_stride", &self.vertex_stride)
                .field("radius_format", &self.radius_format)
                .field("radius_data", &self.radius_data)
                .field("radius_stride", &self.radius_stride)
                .field("index_type", &self.index_type)
                .field("index_data", &self.index_data)
                .field("index_stride", &self.index_stride)
                .field("indexing_mode", &self.indexing_mode)
                .field("end_caps_mode", &self.end_caps_mode)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureGeometryLinearSweptSpheresDataNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV;
    }

    unsafe impl<'a> Extends<AccelerationStructureGeometryKHR<'a>>
        for AccelerationStructureGeometryLinearSweptSpheresDataNV<'a>
    {
    }

    impl Default for AccelerationStructureGeometryLinearSweptSpheresDataNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                vertex_format: Default::default(),
                vertex_data: Default::default(),
                vertex_stride: Default::default(),
                radius_format: Default::default(),
                radius_data: Default::default(),
                radius_stride: Default::default(),
                index_type: Default::default(),
                index_data: Default::default(),
                index_stride: Default::default(),
                indexing_mode: Default::default(),
                end_caps_mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureGeometryLinearSweptSpheresDataNV<'a> {
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
        pub fn radius_format(mut self, radius_format: Format) -> Self {
            self.radius_format = radius_format;
            self
        }

        #[inline]
        pub fn radius_data(mut self, radius_data: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.radius_data = radius_data;
            self
        }

        #[inline]
        pub fn radius_stride(mut self, radius_stride: DeviceSize) -> Self {
            self.radius_stride = radius_stride;
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
        pub fn index_stride(mut self, index_stride: DeviceSize) -> Self {
            self.index_stride = index_stride;
            self
        }

        #[inline]
        pub fn indexing_mode(mut self, indexing_mode: RayTracingLssIndexingModeNV) -> Self {
            self.indexing_mode = indexing_mode;
            self
        }

        #[inline]
        pub fn end_caps_mode(mut self, end_caps_mode: RayTracingLssPrimitiveEndCapsModeNV) -> Self {
            self.end_caps_mode = end_caps_mode;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureGeometrySpheresDataNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureGeometrySpheresDataNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub vertex_format: Format,
        pub vertex_data: DeviceOrHostAddressConstKHR<'a>,
        pub vertex_stride: DeviceSize,
        pub radius_format: Format,
        pub radius_data: DeviceOrHostAddressConstKHR<'a>,
        pub radius_stride: DeviceSize,
        pub index_type: IndexType,
        pub index_data: DeviceOrHostAddressConstKHR<'a>,
        pub index_stride: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureGeometrySpheresDataNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureGeometrySpheresDataNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("vertex_format", &self.vertex_format)
                .field("vertex_data", &self.vertex_data)
                .field("vertex_stride", &self.vertex_stride)
                .field("radius_format", &self.radius_format)
                .field("radius_data", &self.radius_data)
                .field("radius_stride", &self.radius_stride)
                .field("index_type", &self.index_type)
                .field("index_data", &self.index_data)
                .field("index_stride", &self.index_stride)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureGeometrySpheresDataNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV;
    }

    unsafe impl<'a> Extends<AccelerationStructureGeometryKHR<'a>>
        for AccelerationStructureGeometrySpheresDataNV<'a>
    {
    }

    impl Default for AccelerationStructureGeometrySpheresDataNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                vertex_format: Default::default(),
                vertex_data: Default::default(),
                vertex_stride: Default::default(),
                radius_format: Default::default(),
                radius_data: Default::default(),
                radius_stride: Default::default(),
                index_type: Default::default(),
                index_data: Default::default(),
                index_stride: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureGeometrySpheresDataNV<'a> {
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
        pub fn radius_format(mut self, radius_format: Format) -> Self {
            self.radius_format = radius_format;
            self
        }

        #[inline]
        pub fn radius_data(mut self, radius_data: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.radius_data = radius_data;
            self
        }

        #[inline]
        pub fn radius_stride(mut self, radius_stride: DeviceSize) -> Self {
            self.radius_stride = radius_stride;
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
        pub fn index_stride(mut self, index_stride: DeviceSize) -> Self {
            self.index_stride = index_stride;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub spheres: Bool32,
        pub linear_swept_spheres: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("spheres", &self.spheres)
                .field("linear_swept_spheres", &self.linear_swept_spheres)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'a>
    {
    }

    impl Default for PhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                spheres: Default::default(),
                linear_swept_spheres: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'a> {
        #[inline]
        pub fn spheres(mut self, spheres: bool) -> Self {
            self.spheres = spheres.into();
            self
        }

        #[inline]
        pub fn linear_swept_spheres(mut self, linear_swept_spheres: bool) -> Self {
            self.linear_swept_spheres = linear_swept_spheres.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRayTracingLssIndexingModeNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RayTracingLssIndexingModeNV(i32);

    impl RayTracingLssIndexingModeNV {
        pub const LIST_NV: Self = Self(0);
        pub const SUCCESSIVE_NV: Self = Self(1);
    }

    impl fmt::Debug for RayTracingLssIndexingModeNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::LIST_NV => Some("LIST_NV"),
                Self::SUCCESSIVE_NV => Some("SUCCESSIVE_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRayTracingLssPrimitiveEndCapsModeNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RayTracingLssPrimitiveEndCapsModeNV(i32);

    impl RayTracingLssPrimitiveEndCapsModeNV {
        pub const NONE_NV: Self = Self(0);
        pub const CHAINED_NV: Self = Self(1);
    }

    impl fmt::Debug for RayTracingLssPrimitiveEndCapsModeNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::NONE_NV => Some("NONE_NV"),
                Self::CHAINED_NV => Some("CHAINED_NV"),
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
