//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_QCOM_data_graph_model.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_QCOM_data_graph_model";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    pub const DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM: u32 = 3;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCacheHeaderVersionDataGraphQCOM.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineCacheHeaderVersionDataGraphQCOM {
        pub header_size: u32,
        pub header_version: PipelineCacheHeaderVersion,
        pub cache_type: DataGraphModelCacheTypeQCOM,
        pub cache_version: u32,
        pub toolchain_version: [u32; DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM as usize],
    }

    impl Default for PipelineCacheHeaderVersionDataGraphQCOM {
        fn default() -> Self {
            Self {
                header_size: Default::default(),
                header_version: Default::default(),
                cache_type: Default::default(),
                cache_version: Default::default(),
                toolchain_version: [Default::default(); _],
            }
        }
    }

    impl PipelineCacheHeaderVersionDataGraphQCOM {
        #[inline]
        pub fn header_size(mut self, header_size: u32) -> Self {
            self.header_size = header_size;
            self
        }

        #[inline]
        pub fn header_version(mut self, header_version: PipelineCacheHeaderVersion) -> Self {
            self.header_version = header_version;
            self
        }

        #[inline]
        pub fn cache_type(mut self, cache_type: DataGraphModelCacheTypeQCOM) -> Self {
            self.cache_type = cache_type;
            self
        }

        #[inline]
        pub fn cache_version(mut self, cache_version: u32) -> Self {
            self.cache_version = cache_version;
            self
        }

        #[inline]
        pub fn toolchain_version(
            mut self,
            toolchain_version: [u32; DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM as usize],
        ) -> Self {
            self.toolchain_version = toolchain_version;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphPipelineBuiltinModelCreateInfoQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DataGraphPipelineBuiltinModelCreateInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_operation: *const PhysicalDeviceDataGraphOperationSupportARM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DataGraphPipelineBuiltinModelCreateInfoQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DataGraphPipelineBuiltinModelCreateInfoQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_operation", &self.p_operation)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineBuiltinModelCreateInfoQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM;
    }

    unsafe impl<'a> Extends<DataGraphPipelineCreateInfoARM<'a>>
        for DataGraphPipelineBuiltinModelCreateInfoQCOM<'a>
    {
    }

    impl Default for DataGraphPipelineBuiltinModelCreateInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_operation: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DataGraphPipelineBuiltinModelCreateInfoQCOM<'a> {
        #[inline]
        pub fn operation(
            mut self,
            operation: &'a PhysicalDeviceDataGraphOperationSupportARM,
        ) -> Self {
            self.p_operation = operation;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDataGraphModelFeaturesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDataGraphModelFeaturesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub data_graph_model: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDataGraphModelFeaturesQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDataGraphModelFeaturesQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("data_graph_model", &self.data_graph_model)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDataGraphModelFeaturesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDataGraphModelFeaturesQCOM<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceDataGraphModelFeaturesQCOM<'a> {}

    impl Default for PhysicalDeviceDataGraphModelFeaturesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                data_graph_model: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDataGraphModelFeaturesQCOM<'a> {
        #[inline]
        pub fn data_graph_model(mut self, data_graph_model: bool) -> Self {
            self.data_graph_model = data_graph_model.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphModelCacheTypeQCOM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataGraphModelCacheTypeQCOM(i32);

    impl DataGraphModelCacheTypeQCOM {
        pub const GENERIC_BINARY_QCOM: Self = Self(0);
    }

    impl fmt::Debug for DataGraphModelCacheTypeQCOM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::GENERIC_BINARY_QCOM => Some("GENERIC_BINARY_QCOM"),
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
