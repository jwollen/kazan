#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCooperativeVectorFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_vector: Bool32,
    pub cooperative_vector_training: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCooperativeVectorFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            cooperative_vector: Default::default(),
            cooperative_vector_training: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceCooperativeVectorFeaturesNV<'a> {
    pub fn cooperative_vector(mut self, cooperative_vector: Bool32) -> Self {
        self.cooperative_vector = cooperative_vector;
        self
    }
    pub fn cooperative_vector_training(mut self, cooperative_vector_training: Bool32) -> Self {
        self.cooperative_vector_training = cooperative_vector_training;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for CooperativeVectorPropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COOPERATIVE_VECTOR_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            input_type: Default::default(),
            input_interpretation: Default::default(),
            matrix_interpretation: Default::default(),
            bias_interpretation: Default::default(),
            result_type: Default::default(),
            transpose: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> CooperativeVectorPropertiesNV<'a> {
    pub fn input_type(mut self, input_type: ComponentTypeKHR) -> Self {
        self.input_type = input_type;
        self
    }
    pub fn input_interpretation(mut self, input_interpretation: ComponentTypeKHR) -> Self {
        self.input_interpretation = input_interpretation;
        self
    }
    pub fn matrix_interpretation(mut self, matrix_interpretation: ComponentTypeKHR) -> Self {
        self.matrix_interpretation = matrix_interpretation;
        self
    }
    pub fn bias_interpretation(mut self, bias_interpretation: ComponentTypeKHR) -> Self {
        self.bias_interpretation = bias_interpretation;
        self
    }
    pub fn result_type(mut self, result_type: ComponentTypeKHR) -> Self {
        self.result_type = result_type;
        self
    }
    pub fn transpose(mut self, transpose: Bool32) -> Self {
        self.transpose = transpose;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCooperativeVectorPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cooperative_vector_supported_stages: ShaderStageFlags,
    pub cooperative_vector_training_float16_accumulation: Bool32,
    pub cooperative_vector_training_float32_accumulation: Bool32,
    pub max_cooperative_vector_components: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCooperativeVectorPropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            cooperative_vector_supported_stages: Default::default(),
            cooperative_vector_training_float16_accumulation: Default::default(),
            cooperative_vector_training_float32_accumulation: Default::default(),
            max_cooperative_vector_components: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceCooperativeVectorPropertiesNV<'a> {
    pub fn cooperative_vector_supported_stages(
        mut self,
        cooperative_vector_supported_stages: ShaderStageFlags,
    ) -> Self {
        self.cooperative_vector_supported_stages = cooperative_vector_supported_stages;
        self
    }
    pub fn cooperative_vector_training_float16_accumulation(
        mut self,
        cooperative_vector_training_float16_accumulation: Bool32,
    ) -> Self {
        self.cooperative_vector_training_float16_accumulation =
            cooperative_vector_training_float16_accumulation;
        self
    }
    pub fn cooperative_vector_training_float32_accumulation(
        mut self,
        cooperative_vector_training_float32_accumulation: Bool32,
    ) -> Self {
        self.cooperative_vector_training_float32_accumulation =
            cooperative_vector_training_float32_accumulation;
        self
    }
    pub fn max_cooperative_vector_components(
        mut self,
        max_cooperative_vector_components: u32,
    ) -> Self {
        self.max_cooperative_vector_components = max_cooperative_vector_components;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for ConvertCooperativeVectorMatrixInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV,
            p_next: core::ptr::null(),
            src_size: Default::default(),
            src_data: Default::default(),
            p_dst_size: core::ptr::null_mut(),
            dst_data: Default::default(),
            src_component_type: Default::default(),
            dst_component_type: Default::default(),
            num_rows: Default::default(),
            num_columns: Default::default(),
            src_layout: Default::default(),
            src_stride: Default::default(),
            dst_layout: Default::default(),
            dst_stride: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ConvertCooperativeVectorMatrixInfoNV<'a> {
    pub fn src_size(mut self, src_size: usize) -> Self {
        self.src_size = src_size;
        self
    }
    pub fn src_data(mut self, src_data: DeviceOrHostAddressConstKHR<'a>) -> Self {
        self.src_data = src_data;
        self
    }
    pub fn dst_size(mut self, dst_size: &'a mut usize) -> Self {
        self.p_dst_size = dst_size;
        self
    }
    pub fn dst_data(mut self, dst_data: DeviceOrHostAddressKHR<'a>) -> Self {
        self.dst_data = dst_data;
        self
    }
    pub fn src_component_type(mut self, src_component_type: ComponentTypeKHR) -> Self {
        self.src_component_type = src_component_type;
        self
    }
    pub fn dst_component_type(mut self, dst_component_type: ComponentTypeKHR) -> Self {
        self.dst_component_type = dst_component_type;
        self
    }
    pub fn num_rows(mut self, num_rows: u32) -> Self {
        self.num_rows = num_rows;
        self
    }
    pub fn num_columns(mut self, num_columns: u32) -> Self {
        self.num_columns = num_columns;
        self
    }
    pub fn src_layout(mut self, src_layout: CooperativeVectorMatrixLayoutNV) -> Self {
        self.src_layout = src_layout;
        self
    }
    pub fn src_stride(mut self, src_stride: usize) -> Self {
        self.src_stride = src_stride;
        self
    }
    pub fn dst_layout(mut self, dst_layout: CooperativeVectorMatrixLayoutNV) -> Self {
        self.dst_layout = dst_layout;
        self
    }
    pub fn dst_stride(mut self, dst_stride: usize) -> Self {
        self.dst_stride = dst_stride;
        self
    }
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
