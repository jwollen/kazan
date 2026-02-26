#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM: u32 = 3;
#[repr(C)]
pub struct PipelineCacheHeaderVersionDataGraphQCOM {
    pub header_size: u32,
    pub header_version: PipelineCacheHeaderVersion,
    pub cache_type: DataGraphModelCacheTypeQCOM,
    pub cache_version: u32,
    pub toolchain_version: [u32; DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM as usize],
}
#[repr(C)]
pub struct DataGraphPipelineBuiltinModelCreateInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_operation: *const PhysicalDeviceDataGraphOperationSupportARM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceDataGraphModelFeaturesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub data_graph_model: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphModelCacheTypeQCOM(i32);
impl DataGraphModelCacheTypeQCOM {
    pub const GENERIC_BINARY_QCOM: Self = Self(0);
}
