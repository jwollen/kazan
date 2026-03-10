//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_cooperative_vector.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_cooperative_vector";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCooperativeVectorFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCooperativeVectorFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cooperative_vector: Bool32,
        pub cooperative_vector_training: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCooperativeVectorFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCooperativeVectorFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("cooperative_vector", &self.cooperative_vector)
                .field(
                    "cooperative_vector_training",
                    &self.cooperative_vector_training,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCooperativeVectorFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceCooperativeVectorFeaturesNV<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceCooperativeVectorFeaturesNV<'_> {}

    impl Default for PhysicalDeviceCooperativeVectorFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                cooperative_vector: Default::default(),
                cooperative_vector_training: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCooperativeVectorFeaturesNV<'a> {
        #[inline]
        pub fn cooperative_vector(mut self, cooperative_vector: bool) -> Self {
            self.cooperative_vector = cooperative_vector.into();
            self
        }

        #[inline]
        pub fn cooperative_vector_training(mut self, cooperative_vector_training: bool) -> Self {
            self.cooperative_vector_training = cooperative_vector_training.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCooperativeVectorPropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for CooperativeVectorPropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CooperativeVectorPropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("input_type", &self.input_type)
                .field("input_interpretation", &self.input_interpretation)
                .field("matrix_interpretation", &self.matrix_interpretation)
                .field("bias_interpretation", &self.bias_interpretation)
                .field("result_type", &self.result_type)
                .field("transpose", &self.transpose)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CooperativeVectorPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COOPERATIVE_VECTOR_PROPERTIES_NV;
    }

    impl Default for CooperativeVectorPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
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
        #[inline]
        pub fn input_type(mut self, input_type: ComponentTypeKHR) -> Self {
            self.input_type = input_type;
            self
        }

        #[inline]
        pub fn input_interpretation(mut self, input_interpretation: ComponentTypeKHR) -> Self {
            self.input_interpretation = input_interpretation;
            self
        }

        #[inline]
        pub fn matrix_interpretation(mut self, matrix_interpretation: ComponentTypeKHR) -> Self {
            self.matrix_interpretation = matrix_interpretation;
            self
        }

        #[inline]
        pub fn bias_interpretation(mut self, bias_interpretation: ComponentTypeKHR) -> Self {
            self.bias_interpretation = bias_interpretation;
            self
        }

        #[inline]
        pub fn result_type(mut self, result_type: ComponentTypeKHR) -> Self {
            self.result_type = result_type;
            self
        }

        #[inline]
        pub fn transpose(mut self, transpose: bool) -> Self {
            self.transpose = transpose.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCooperativeVectorPropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCooperativeVectorPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cooperative_vector_supported_stages: ShaderStageFlags,
        pub cooperative_vector_training_float16_accumulation: Bool32,
        pub cooperative_vector_training_float32_accumulation: Bool32,
        pub max_cooperative_vector_components: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCooperativeVectorPropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCooperativeVectorPropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "cooperative_vector_supported_stages",
                    &self.cooperative_vector_supported_stages,
                )
                .field(
                    "cooperative_vector_training_float16_accumulation",
                    &self.cooperative_vector_training_float16_accumulation,
                )
                .field(
                    "cooperative_vector_training_float32_accumulation",
                    &self.cooperative_vector_training_float32_accumulation,
                )
                .field(
                    "max_cooperative_vector_components",
                    &self.max_cooperative_vector_components,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCooperativeVectorPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceCooperativeVectorPropertiesNV<'_>
    {
    }

    impl Default for PhysicalDeviceCooperativeVectorPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                cooperative_vector_supported_stages: Default::default(),
                cooperative_vector_training_float16_accumulation: Default::default(),
                cooperative_vector_training_float32_accumulation: Default::default(),
                max_cooperative_vector_components: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCooperativeVectorPropertiesNV<'a> {
        #[inline]
        pub fn cooperative_vector_supported_stages(
            mut self,
            cooperative_vector_supported_stages: ShaderStageFlags,
        ) -> Self {
            self.cooperative_vector_supported_stages = cooperative_vector_supported_stages;
            self
        }

        #[inline]
        pub fn cooperative_vector_training_float16_accumulation(
            mut self,
            cooperative_vector_training_float16_accumulation: bool,
        ) -> Self {
            self.cooperative_vector_training_float16_accumulation =
                cooperative_vector_training_float16_accumulation.into();
            self
        }

        #[inline]
        pub fn cooperative_vector_training_float32_accumulation(
            mut self,
            cooperative_vector_training_float32_accumulation: bool,
        ) -> Self {
            self.cooperative_vector_training_float32_accumulation =
                cooperative_vector_training_float32_accumulation.into();
            self
        }

        #[inline]
        pub fn max_cooperative_vector_components(
            mut self,
            max_cooperative_vector_components: u32,
        ) -> Self {
            self.max_cooperative_vector_components = max_cooperative_vector_components;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkConvertCooperativeVectorMatrixInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for ConvertCooperativeVectorMatrixInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ConvertCooperativeVectorMatrixInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_size", &self.src_size)
                .field("src_data", &self.src_data)
                .field("p_dst_size", &self.p_dst_size)
                .field("dst_data", &self.dst_data)
                .field("src_component_type", &self.src_component_type)
                .field("dst_component_type", &self.dst_component_type)
                .field("num_rows", &self.num_rows)
                .field("num_columns", &self.num_columns)
                .field("src_layout", &self.src_layout)
                .field("src_stride", &self.src_stride)
                .field("dst_layout", &self.dst_layout)
                .field("dst_stride", &self.dst_stride)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ConvertCooperativeVectorMatrixInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV;
    }

    impl Default for ConvertCooperativeVectorMatrixInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_size: Default::default(),
                src_data: Default::default(),
                p_dst_size: ptr::null_mut(),
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
        #[inline]
        pub fn src_size(mut self, src_size: usize) -> Self {
            self.src_size = src_size;
            self
        }

        #[inline]
        pub fn src_data(mut self, src_data: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.src_data = src_data;
            self
        }

        #[inline]
        pub fn dst_size(mut self, dst_size: &'a mut usize) -> Self {
            self.p_dst_size = dst_size;
            self
        }

        #[inline]
        pub fn dst_data(mut self, dst_data: DeviceOrHostAddressKHR<'a>) -> Self {
            self.dst_data = dst_data;
            self
        }

        #[inline]
        pub fn src_component_type(mut self, src_component_type: ComponentTypeKHR) -> Self {
            self.src_component_type = src_component_type;
            self
        }

        #[inline]
        pub fn dst_component_type(mut self, dst_component_type: ComponentTypeKHR) -> Self {
            self.dst_component_type = dst_component_type;
            self
        }

        #[inline]
        pub fn num_rows(mut self, num_rows: u32) -> Self {
            self.num_rows = num_rows;
            self
        }

        #[inline]
        pub fn num_columns(mut self, num_columns: u32) -> Self {
            self.num_columns = num_columns;
            self
        }

        #[inline]
        pub fn src_layout(mut self, src_layout: CooperativeVectorMatrixLayoutNV) -> Self {
            self.src_layout = src_layout;
            self
        }

        #[inline]
        pub fn src_stride(mut self, src_stride: usize) -> Self {
            self.src_stride = src_stride;
            self
        }

        #[inline]
        pub fn dst_layout(mut self, dst_layout: CooperativeVectorMatrixLayoutNV) -> Self {
            self.dst_layout = dst_layout;
            self
        }

        #[inline]
        pub fn dst_stride(mut self, dst_stride: usize) -> Self {
            self.dst_stride = dst_stride;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCooperativeVectorMatrixLayoutNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CooperativeVectorMatrixLayoutNV(i32);

    impl CooperativeVectorMatrixLayoutNV {
        pub const ROW_MAJOR_NV: Self = Self(0);
        pub const COLUMN_MAJOR_NV: Self = Self(1);
        pub const INFERENCING_OPTIMAL_NV: Self = Self(2);
        pub const TRAINING_OPTIMAL_NV: Self = Self(3);
    }

    impl fmt::Debug for CooperativeVectorMatrixLayoutNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ROW_MAJOR_NV => Some("ROW_MAJOR_NV"),
                Self::COLUMN_MAJOR_NV => Some("COLUMN_MAJOR_NV"),
                Self::INFERENCING_OPTIMAL_NV => Some("INFERENCING_OPTIMAL_NV"),
                Self::TRAINING_OPTIMAL_NV => Some("TRAINING_OPTIMAL_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceCooperativeVectorPropertiesNV.html>
    pub type PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut CooperativeVectorPropertiesNV<'_>,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkConvertCooperativeVectorMatrixNV.html>
    pub type PFN_vkConvertCooperativeVectorMatrixNV = unsafe extern "system" fn(
        device: Device,
        p_info: *const ConvertCooperativeVectorMatrixInfoNV<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdConvertCooperativeVectorMatrixNV.html>
    pub type PFN_vkCmdConvertCooperativeVectorMatrixNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        info_count: u32,
        p_infos: *const ConvertCooperativeVectorMatrixInfoNV<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceCooperativeVectorFeaturesNV =
        PhysicalDeviceCooperativeVectorFeaturesNV<'static>;
    pub type VkCooperativeVectorPropertiesNV = CooperativeVectorPropertiesNV<'static>;
    pub type VkPhysicalDeviceCooperativeVectorPropertiesNV =
        PhysicalDeviceCooperativeVectorPropertiesNV<'static>;
    pub type VkConvertCooperativeVectorMatrixInfoNV = ConvertCooperativeVectorMatrixInfoNV<'static>;
    pub type VkCooperativeVectorMatrixLayoutNV = CooperativeVectorMatrixLayoutNV;
    impl PhysicalDeviceCooperativeVectorFeaturesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceCooperativeVectorFeaturesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CooperativeVectorPropertiesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCooperativeVectorPropertiesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceCooperativeVectorPropertiesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceCooperativeVectorPropertiesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ConvertCooperativeVectorMatrixInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkConvertCooperativeVectorMatrixInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    get_physical_device_cooperative_vector_properties_nv:
        PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_cooperative_vector_properties_nv: transmute(
                    load(c"vkGetPhysicalDeviceCooperativeVectorPropertiesNV")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceCooperativeVectorPropertiesNV.html>
    #[inline]
    pub unsafe fn get_physical_device_cooperative_vector_properties_nv<'a>(
        &self,
        physical_device: PhysicalDevice,
        mut properties: impl ExtendUninit<CooperativeVectorPropertiesNV<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |property_count, properties| {
                let result = (self.get_physical_device_cooperative_vector_properties_nv)(
                    physical_device,
                    property_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let properties_buf = properties.reserve(capacity);
            len = properties_buf.len().try_into().unwrap();
            let result = call(&mut len, properties_buf.as_mut_ptr() as *mut _)?;
            properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}

pub struct DeviceFn {
    convert_cooperative_vector_matrix_nv: PFN_vkConvertCooperativeVectorMatrixNV,
    cmd_convert_cooperative_vector_matrix_nv: PFN_vkCmdConvertCooperativeVectorMatrixNV,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                convert_cooperative_vector_matrix_nv: transmute(
                    load(c"vkConvertCooperativeVectorMatrixNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_convert_cooperative_vector_matrix_nv: transmute(
                    load(c"vkCmdConvertCooperativeVectorMatrixNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkConvertCooperativeVectorMatrixNV.html>
    #[inline]
    pub unsafe fn convert_cooperative_vector_matrix_nv(
        &self,
        device: Device,
        info: &ConvertCooperativeVectorMatrixInfoNV<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.convert_cooperative_vector_matrix_nv)(device, info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdConvertCooperativeVectorMatrixNV.html>
    #[inline]
    pub unsafe fn cmd_convert_cooperative_vector_matrix_nv(
        &self,
        command_buffer: CommandBuffer,
        infos: &[ConvertCooperativeVectorMatrixInfoNV<'_>],
    ) {
        unsafe {
            (self.cmd_convert_cooperative_vector_matrix_nv)(
                command_buffer,
                infos.len().try_into().unwrap(),
                infos.as_ptr() as _,
            )
        }
    }
}
