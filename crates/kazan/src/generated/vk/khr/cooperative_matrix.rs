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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceCooperativeMatrixFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cooperative_matrix: Bool32,
        pub cooperative_matrix_robust_buffer_access: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceCooperativeMatrixFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR,
                p_next: core::ptr::null_mut(),
                cooperative_matrix: Default::default(),
                cooperative_matrix_robust_buffer_access: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceCooperativeMatrixFeaturesKHR<'a> {
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
    pub struct CooperativeMatrixPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub m_size: u32,
        pub n_size: u32,
        pub k_size: u32,
        pub a_type: ComponentTypeKHR,
        pub b_type: ComponentTypeKHR,
        pub c_type: ComponentTypeKHR,
        pub result_type: ComponentTypeKHR,
        pub saturating_accumulation: Bool32,
        pub scope: ScopeKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for CooperativeMatrixPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::COOPERATIVE_MATRIX_PROPERTIES_KHR,
                p_next: core::ptr::null_mut(),
                m_size: Default::default(),
                n_size: Default::default(),
                k_size: Default::default(),
                a_type: Default::default(),
                b_type: Default::default(),
                c_type: Default::default(),
                result_type: Default::default(),
                saturating_accumulation: Default::default(),
                scope: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CooperativeMatrixPropertiesKHR<'a> {
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
        pub fn a_type(mut self, a_type: ComponentTypeKHR) -> Self {
            self.a_type = a_type;
            self
        }
        pub fn b_type(mut self, b_type: ComponentTypeKHR) -> Self {
            self.b_type = b_type;
            self
        }
        pub fn c_type(mut self, c_type: ComponentTypeKHR) -> Self {
            self.c_type = c_type;
            self
        }
        pub fn result_type(mut self, result_type: ComponentTypeKHR) -> Self {
            self.result_type = result_type;
            self
        }
        pub fn saturating_accumulation(mut self, saturating_accumulation: Bool32) -> Self {
            self.saturating_accumulation = saturating_accumulation;
            self
        }
        pub fn scope(mut self, scope: ScopeKHR) -> Self {
            self.scope = scope;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceCooperativeMatrixPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cooperative_matrix_supported_stages: ShaderStageFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceCooperativeMatrixPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR,
                p_next: core::ptr::null_mut(),
                cooperative_matrix_supported_stages: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceCooperativeMatrixPropertiesKHR<'a> {
        pub fn cooperative_matrix_supported_stages(
            mut self,
            cooperative_matrix_supported_stages: ShaderStageFlags,
        ) -> Self {
            self.cooperative_matrix_supported_stages = cooperative_matrix_supported_stages;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ScopeKHR(i32);
    impl ScopeKHR {
        pub const DEVICE_KHR: Self = Self(1);
        pub const WORKGROUP_KHR: Self = Self(2);
        pub const SUBGROUP_KHR: Self = Self(3);
        pub const QUEUE_FAMILY_KHR: Self = Self(5);
        pub const DEVICE_NV: Self = Self::DEVICE_KHR;
        pub const QUEUE_FAMILY_NV: Self = Self::QUEUE_FAMILY_KHR;
        pub const SUBGROUP_NV: Self = Self::SUBGROUP_KHR;
        pub const WORKGROUP_NV: Self = Self::WORKGROUP_KHR;
    }
    pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut CooperativeMatrixPropertiesKHR<'_>,
        ) -> vk::Result;
}
pub struct InstanceFn {
    get_physical_device_cooperative_matrix_properties_khr:
        PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_cooperative_matrix_properties_khr: transmute(
                    load(c"vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_cooperative_matrix_properties_khr<'a>(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<CooperativeMatrixPropertiesKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result = (self.get_physical_device_cooperative_matrix_properties_khr)(
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
