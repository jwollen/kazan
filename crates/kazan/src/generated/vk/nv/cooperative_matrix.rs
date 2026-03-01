#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    pub type ScopeNV = ScopeKHR;
    pub type ComponentTypeNV = ComponentTypeKHR;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceCooperativeMatrixFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cooperative_matrix: Bool32,
        pub cooperative_matrix_robust_buffer_access: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCooperativeMatrixFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceCooperativeMatrixFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceCooperativeMatrixFeaturesNV<'a> {}
    impl Default for PhysicalDeviceCooperativeMatrixFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                cooperative_matrix: Default::default(),
                cooperative_matrix_robust_buffer_access: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceCooperativeMatrixFeaturesNV<'a> {
        pub fn cooperative_matrix(mut self, cooperative_matrix: Bool32) -> Self {
            self.cooperative_matrix = cooperative_matrix;
            self
        }
        pub fn cooperative_matrix_robust_buffer_access(
            mut self,
            cooperative_matrix_robust_buffer_access: Bool32,
        ) -> Self {
            self.cooperative_matrix_robust_buffer_access = cooperative_matrix_robust_buffer_access;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceCooperativeMatrixPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cooperative_matrix_supported_stages: ShaderStageFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCooperativeMatrixPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceCooperativeMatrixPropertiesNV<'a>
    {
    }
    impl Default for PhysicalDeviceCooperativeMatrixPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                cooperative_matrix_supported_stages: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceCooperativeMatrixPropertiesNV<'a> {
        pub fn cooperative_matrix_supported_stages(
            mut self,
            cooperative_matrix_supported_stages: ShaderStageFlags,
        ) -> Self {
            self.cooperative_matrix_supported_stages = cooperative_matrix_supported_stages;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CooperativeMatrixPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub m_size: u32,
        pub n_size: u32,
        pub k_size: u32,
        pub a_type: ComponentTypeNV,
        pub b_type: ComponentTypeNV,
        pub c_type: ComponentTypeNV,
        pub d_type: ComponentTypeNV,
        pub scope: ScopeNV,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CooperativeMatrixPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COOPERATIVE_MATRIX_PROPERTIES_NV;
    }
    impl Default for CooperativeMatrixPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                m_size: Default::default(),
                n_size: Default::default(),
                k_size: Default::default(),
                a_type: Default::default(),
                b_type: Default::default(),
                c_type: Default::default(),
                d_type: Default::default(),
                scope: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CooperativeMatrixPropertiesNV<'a> {
        pub fn m_size(mut self, m_size: u32) -> Self {
            self.m_size = m_size;
            self
        }
        pub fn n_size(mut self, n_size: u32) -> Self {
            self.n_size = n_size;
            self
        }
        pub fn k_size(mut self, k_size: u32) -> Self {
            self.k_size = k_size;
            self
        }
        pub fn a_type(mut self, a_type: ComponentTypeNV) -> Self {
            self.a_type = a_type;
            self
        }
        pub fn b_type(mut self, b_type: ComponentTypeNV) -> Self {
            self.b_type = b_type;
            self
        }
        pub fn c_type(mut self, c_type: ComponentTypeNV) -> Self {
            self.c_type = c_type;
            self
        }
        pub fn d_type(mut self, d_type: ComponentTypeNV) -> Self {
            self.d_type = d_type;
            self
        }
        pub fn scope(mut self, scope: ScopeNV) -> Self {
            self.scope = scope;
            self
        }
    }
    pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut CooperativeMatrixPropertiesNV<'_>,
        ) -> vk::Result;
}
pub struct InstanceFn {
    get_physical_device_cooperative_matrix_properties_nv:
        PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_cooperative_matrix_properties_nv: transmute(
                    load(c"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_cooperative_matrix_properties_nv<'a>(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<CooperativeMatrixPropertiesNV<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result = (self.get_physical_device_cooperative_matrix_properties_nv)(
                    physical_device,
                    property_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
}
