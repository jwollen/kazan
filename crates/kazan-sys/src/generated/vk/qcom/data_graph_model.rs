#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelineBuiltinModelCreateInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_operation: *const PhysicalDeviceDataGraphOperationSupportARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDataGraphModelFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub data_graph_model: Bool32,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphModelCacheTypeQCOM(i32);
impl DataGraphModelCacheTypeQCOM {
    pub const GENERIC_BINARY_QCOM: Self = Self(0);
}
