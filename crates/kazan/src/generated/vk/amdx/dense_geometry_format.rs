#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_AMDX_dense_geometry_format";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    pub const COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX: u32 = 128;
    pub const COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX: u32 = 128;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDenseGeometryFormatFeaturesAMDX<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub dense_geometry_format: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDenseGeometryFormatFeaturesAMDX<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDenseGeometryFormatFeaturesAMDX")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("dense_geometry_format", &self.dense_geometry_format)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDenseGeometryFormatFeaturesAMDX<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDenseGeometryFormatFeaturesAMDX<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceDenseGeometryFormatFeaturesAMDX<'a>
    {
    }

    impl Default for PhysicalDeviceDenseGeometryFormatFeaturesAMDX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                dense_geometry_format: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDenseGeometryFormatFeaturesAMDX<'a> {
        pub fn dense_geometry_format(mut self, dense_geometry_format: bool) -> Self {
            self.dense_geometry_format = dense_geometry_format.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureDenseGeometryFormatTrianglesDataAMDX.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureDenseGeometryFormatTrianglesDataAMDX<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub compressed_data: DeviceOrHostAddressConstKHR<'a>,
        pub data_size: DeviceSize,
        pub num_triangles: u32,
        pub num_vertices: u32,
        pub max_primitive_index: u32,
        pub max_geometry_index: u32,
        pub format: CompressedTriangleFormatAMDX,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureDenseGeometryFormatTrianglesDataAMDX<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureDenseGeometryFormatTrianglesDataAMDX")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("compressed_data", &self.compressed_data)
                .field("data_size", &self.data_size)
                .field("num_triangles", &self.num_triangles)
                .field("num_vertices", &self.num_vertices)
                .field("max_primitive_index", &self.max_primitive_index)
                .field("max_geometry_index", &self.max_geometry_index)
                .field("format", &self.format)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for AccelerationStructureDenseGeometryFormatTrianglesDataAMDX<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX;
    }

    unsafe impl<'a> Extends<AccelerationStructureGeometryKHR<'a>>
        for AccelerationStructureDenseGeometryFormatTrianglesDataAMDX<'a>
    {
    }

    impl Default for AccelerationStructureDenseGeometryFormatTrianglesDataAMDX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                compressed_data: Default::default(),
                data_size: Default::default(),
                num_triangles: Default::default(),
                num_vertices: Default::default(),
                max_primitive_index: Default::default(),
                max_geometry_index: Default::default(),
                format: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureDenseGeometryFormatTrianglesDataAMDX<'a> {
        pub fn compressed_data(mut self, compressed_data: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.compressed_data = compressed_data;
            self
        }

        pub fn data_size(mut self, data_size: DeviceSize) -> Self {
            self.data_size = data_size;
            self
        }

        pub fn num_triangles(mut self, num_triangles: u32) -> Self {
            self.num_triangles = num_triangles;
            self
        }

        pub fn num_vertices(mut self, num_vertices: u32) -> Self {
            self.num_vertices = num_vertices;
            self
        }

        pub fn max_primitive_index(mut self, max_primitive_index: u32) -> Self {
            self.max_primitive_index = max_primitive_index;
            self
        }

        pub fn max_geometry_index(mut self, max_geometry_index: u32) -> Self {
            self.max_geometry_index = max_geometry_index;
            self
        }

        pub fn format(mut self, format: CompressedTriangleFormatAMDX) -> Self {
            self.format = format;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCompressedTriangleFormatAMDX.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CompressedTriangleFormatAMDX(i32);

    impl CompressedTriangleFormatAMDX {
        pub const DGF1_AMDX: Self = Self(0);
    }

    impl fmt::Debug for CompressedTriangleFormatAMDX {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DGF1_AMDX => Some("DGF1_AMDX"),
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
