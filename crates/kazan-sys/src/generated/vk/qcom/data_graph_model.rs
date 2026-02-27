#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub const DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM: u32 = 3;
#[repr(C)]
#[derive(Copy, Clone)]
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
    pub fn header_size(mut self, header_size: u32) -> Self {
        self.header_size = header_size;
        self
    }
    pub fn header_version(mut self, header_version: PipelineCacheHeaderVersion) -> Self {
        self.header_version = header_version;
        self
    }
    pub fn cache_type(mut self, cache_type: DataGraphModelCacheTypeQCOM) -> Self {
        self.cache_type = cache_type;
        self
    }
    pub fn cache_version(mut self, cache_version: u32) -> Self {
        self.cache_version = cache_version;
        self
    }
    pub fn toolchain_version(
        mut self,
        toolchain_version: [u32; DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM as usize],
    ) -> Self {
        self.toolchain_version = toolchain_version;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelineBuiltinModelCreateInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_operation: *const PhysicalDeviceDataGraphOperationSupportARM,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DataGraphPipelineBuiltinModelCreateInfoQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM,
            p_next: core::ptr::null(),
            p_operation: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DataGraphPipelineBuiltinModelCreateInfoQCOM<'a> {
    pub fn operation(mut self, operation: &'a PhysicalDeviceDataGraphOperationSupportARM) -> Self {
        self.p_operation = operation;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDataGraphModelFeaturesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub data_graph_model: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDataGraphModelFeaturesQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM,
            p_next: core::ptr::null_mut(),
            data_graph_model: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceDataGraphModelFeaturesQCOM<'a> {
    pub fn data_graph_model(mut self, data_graph_model: Bool32) -> Self {
        self.data_graph_model = data_graph_model;
        self
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphModelCacheTypeQCOM(i32);
impl DataGraphModelCacheTypeQCOM {
    pub const GENERIC_BINARY_QCOM: Self = Self(0);
}
