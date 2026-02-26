#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceCooperativeVectorFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_vector: Bool32,
    pub cooperative_vector_training: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct CooperativeVectorPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub input_type: ComponentTypeKHR,
    pub input_interpretation: ComponentTypeKHR,
    pub matrix_interpretation: ComponentTypeKHR,
    pub bias_interpretation: ComponentTypeKHR,
    pub result_type: ComponentTypeKHR,
    pub transpose: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceCooperativeVectorPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_vector_supported_stages: ShaderStageFlags,
    pub cooperative_vector_training_float16_accumulation: Bool32,
    pub cooperative_vector_training_float32_accumulation: Bool32,
    pub max_cooperative_vector_components: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ConvertCooperativeVectorMatrixInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_size: usize,
    pub src_data: DeviceOrHostAddressConstKHR<'a>,
    pub p_dst_size: *mut usize,
    pub dst_data: DeviceOrHostAddressKHR<'a>,
    pub src_component_type: ComponentTypeKHR,
    pub dst_component_type: ComponentTypeKHR,
    pub num_rows: u32,
    pub num_columns: u32,
    pub src_layout: CooperativeVectorMatrixLayoutNV,
    pub src_stride: usize,
    pub dst_layout: CooperativeVectorMatrixLayoutNV,
    pub dst_stride: usize,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentTypeKHR(i32);
impl ComponentTypeKHR {
    pub const FLOAT16_KHR: Self = Self(0);
    pub const FLOAT32_KHR: Self = Self(1);
    pub const FLOAT64_KHR: Self = Self(2);
    pub const SINT8_KHR: Self = Self(3);
    pub const SINT16_KHR: Self = Self(4);
    pub const SINT32_KHR: Self = Self(5);
    pub const SINT64_KHR: Self = Self(6);
    pub const UINT8_KHR: Self = Self(7);
    pub const UINT16_KHR: Self = Self(8);
    pub const UINT32_KHR: Self = Self(9);
    pub const UINT64_KHR: Self = Self(10);
    pub const BFLOAT16_KHR: Self = Self(1000141000);
    pub const FLOAT8_E4M3_EXT: Self = Self(1000491002);
    pub const FLOAT8_E5M2_EXT: Self = Self(1000491003);
    pub const SINT8_PACKED_NV: Self = Self(1000491000);
    pub const UINT8_PACKED_NV: Self = Self(1000491001);
    pub const FLOAT16_NV: Self = Self::FLOAT16_KHR;
    pub const FLOAT32_NV: Self = Self::FLOAT32_KHR;
    pub const FLOAT64_NV: Self = Self::FLOAT64_KHR;
    pub const FLOAT_E4M3_NV: Self = Self::FLOAT8_E4M3_EXT;
    pub const FLOAT_E5M2_NV: Self = Self::FLOAT8_E5M2_EXT;
    pub const SINT16_NV: Self = Self::SINT16_KHR;
    pub const SINT32_NV: Self = Self::SINT32_KHR;
    pub const SINT64_NV: Self = Self::SINT64_KHR;
    pub const SINT8_NV: Self = Self::SINT8_KHR;
    pub const UINT16_NV: Self = Self::UINT16_KHR;
    pub const UINT32_NV: Self = Self::UINT32_KHR;
    pub const UINT64_NV: Self = Self::UINT64_KHR;
    pub const UINT8_NV: Self = Self::UINT8_KHR;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CooperativeVectorMatrixLayoutNV(i32);
impl CooperativeVectorMatrixLayoutNV {
    pub const ROW_MAJOR_NV: Self = Self(0);
    pub const COLUMN_MAJOR_NV: Self = Self(1);
    pub const INFERENCING_OPTIMAL_NV: Self = Self(2);
    pub const TRAINING_OPTIMAL_NV: Self = Self(3);
}
pub type PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut CooperativeVectorPropertiesNV<'_>,
)
    -> Result;
pub type PFN_vkConvertCooperativeVectorMatrixNV = unsafe extern "system" fn(
    device: Device,
    p_info: *const ConvertCooperativeVectorMatrixInfoNV<'_>,
) -> Result;
pub type PFN_vkCmdConvertCooperativeVectorMatrixNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    p_infos: *const ConvertCooperativeVectorMatrixInfoNV<'_>,
);
