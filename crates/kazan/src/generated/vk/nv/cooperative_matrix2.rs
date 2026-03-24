//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_cooperative_matrix2.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_cooperative_matrix2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCooperativeMatrix2FeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCooperativeMatrix2FeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cooperative_matrix_workgroup_scope: Bool32,
        pub cooperative_matrix_flexible_dimensions: Bool32,
        pub cooperative_matrix_reductions: Bool32,
        pub cooperative_matrix_conversions: Bool32,
        pub cooperative_matrix_per_element_operations: Bool32,
        pub cooperative_matrix_tensor_addressing: Bool32,
        pub cooperative_matrix_block_loads: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCooperativeMatrix2FeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCooperativeMatrix2FeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "cooperative_matrix_workgroup_scope",
                    &self.cooperative_matrix_workgroup_scope,
                )
                .field(
                    "cooperative_matrix_flexible_dimensions",
                    &self.cooperative_matrix_flexible_dimensions,
                )
                .field(
                    "cooperative_matrix_reductions",
                    &self.cooperative_matrix_reductions,
                )
                .field(
                    "cooperative_matrix_conversions",
                    &self.cooperative_matrix_conversions,
                )
                .field(
                    "cooperative_matrix_per_element_operations",
                    &self.cooperative_matrix_per_element_operations,
                )
                .field(
                    "cooperative_matrix_tensor_addressing",
                    &self.cooperative_matrix_tensor_addressing,
                )
                .field(
                    "cooperative_matrix_block_loads",
                    &self.cooperative_matrix_block_loads,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCooperativeMatrix2FeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceCooperativeMatrix2FeaturesNV<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceCooperativeMatrix2FeaturesNV<'_> {}

    impl Default for PhysicalDeviceCooperativeMatrix2FeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                cooperative_matrix_workgroup_scope: Default::default(),
                cooperative_matrix_flexible_dimensions: Default::default(),
                cooperative_matrix_reductions: Default::default(),
                cooperative_matrix_conversions: Default::default(),
                cooperative_matrix_per_element_operations: Default::default(),
                cooperative_matrix_tensor_addressing: Default::default(),
                cooperative_matrix_block_loads: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCooperativeMatrix2FeaturesNV<'a> {
        #[inline]
        pub fn cooperative_matrix_workgroup_scope(
            mut self,
            cooperative_matrix_workgroup_scope: bool,
        ) -> Self {
            self.cooperative_matrix_workgroup_scope = cooperative_matrix_workgroup_scope.into();
            self
        }

        #[inline]
        pub fn cooperative_matrix_flexible_dimensions(
            mut self,
            cooperative_matrix_flexible_dimensions: bool,
        ) -> Self {
            self.cooperative_matrix_flexible_dimensions =
                cooperative_matrix_flexible_dimensions.into();
            self
        }

        #[inline]
        pub fn cooperative_matrix_reductions(
            mut self,
            cooperative_matrix_reductions: bool,
        ) -> Self {
            self.cooperative_matrix_reductions = cooperative_matrix_reductions.into();
            self
        }

        #[inline]
        pub fn cooperative_matrix_conversions(
            mut self,
            cooperative_matrix_conversions: bool,
        ) -> Self {
            self.cooperative_matrix_conversions = cooperative_matrix_conversions.into();
            self
        }

        #[inline]
        pub fn cooperative_matrix_per_element_operations(
            mut self,
            cooperative_matrix_per_element_operations: bool,
        ) -> Self {
            self.cooperative_matrix_per_element_operations =
                cooperative_matrix_per_element_operations.into();
            self
        }

        #[inline]
        pub fn cooperative_matrix_tensor_addressing(
            mut self,
            cooperative_matrix_tensor_addressing: bool,
        ) -> Self {
            self.cooperative_matrix_tensor_addressing = cooperative_matrix_tensor_addressing.into();
            self
        }

        #[inline]
        pub fn cooperative_matrix_block_loads(
            mut self,
            cooperative_matrix_block_loads: bool,
        ) -> Self {
            self.cooperative_matrix_block_loads = cooperative_matrix_block_loads.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCooperativeMatrix2PropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCooperativeMatrix2PropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cooperative_matrix_workgroup_scope_max_workgroup_size: u32,
        pub cooperative_matrix_flexible_dimensions_max_dimension: u32,
        pub cooperative_matrix_workgroup_scope_reserved_shared_memory: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCooperativeMatrix2PropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCooperativeMatrix2PropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "cooperative_matrix_workgroup_scope_max_workgroup_size",
                    &self.cooperative_matrix_workgroup_scope_max_workgroup_size,
                )
                .field(
                    "cooperative_matrix_flexible_dimensions_max_dimension",
                    &self.cooperative_matrix_flexible_dimensions_max_dimension,
                )
                .field(
                    "cooperative_matrix_workgroup_scope_reserved_shared_memory",
                    &self.cooperative_matrix_workgroup_scope_reserved_shared_memory,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCooperativeMatrix2PropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceCooperativeMatrix2PropertiesNV<'_>
    {
    }

    impl Default for PhysicalDeviceCooperativeMatrix2PropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                cooperative_matrix_workgroup_scope_max_workgroup_size: Default::default(),
                cooperative_matrix_flexible_dimensions_max_dimension: Default::default(),
                cooperative_matrix_workgroup_scope_reserved_shared_memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCooperativeMatrix2PropertiesNV<'a> {
        #[inline]
        pub fn cooperative_matrix_workgroup_scope_max_workgroup_size(
            mut self,
            cooperative_matrix_workgroup_scope_max_workgroup_size: u32,
        ) -> Self {
            self.cooperative_matrix_workgroup_scope_max_workgroup_size =
                cooperative_matrix_workgroup_scope_max_workgroup_size;
            self
        }

        #[inline]
        pub fn cooperative_matrix_flexible_dimensions_max_dimension(
            mut self,
            cooperative_matrix_flexible_dimensions_max_dimension: u32,
        ) -> Self {
            self.cooperative_matrix_flexible_dimensions_max_dimension =
                cooperative_matrix_flexible_dimensions_max_dimension;
            self
        }

        #[inline]
        pub fn cooperative_matrix_workgroup_scope_reserved_shared_memory(
            mut self,
            cooperative_matrix_workgroup_scope_reserved_shared_memory: u32,
        ) -> Self {
            self.cooperative_matrix_workgroup_scope_reserved_shared_memory =
                cooperative_matrix_workgroup_scope_reserved_shared_memory;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCooperativeMatrixFlexibleDimensionsPropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CooperativeMatrixFlexibleDimensionsPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub m_granularity: u32,
        pub n_granularity: u32,
        pub k_granularity: u32,
        pub a_type: ComponentTypeKHR,
        pub b_type: ComponentTypeKHR,
        pub c_type: ComponentTypeKHR,
        pub result_type: ComponentTypeKHR,
        pub saturating_accumulation: Bool32,
        pub scope: ScopeKHR,
        pub workgroup_invocations: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CooperativeMatrixFlexibleDimensionsPropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CooperativeMatrixFlexibleDimensionsPropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("m_granularity", &self.m_granularity)
                .field("n_granularity", &self.n_granularity)
                .field("k_granularity", &self.k_granularity)
                .field("a_type", &self.a_type)
                .field("b_type", &self.b_type)
                .field("c_type", &self.c_type)
                .field("result_type", &self.result_type)
                .field("saturating_accumulation", &self.saturating_accumulation)
                .field("scope", &self.scope)
                .field("workgroup_invocations", &self.workgroup_invocations)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CooperativeMatrixFlexibleDimensionsPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV;
    }

    impl Default for CooperativeMatrixFlexibleDimensionsPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                m_granularity: Default::default(),
                n_granularity: Default::default(),
                k_granularity: Default::default(),
                a_type: Default::default(),
                b_type: Default::default(),
                c_type: Default::default(),
                result_type: Default::default(),
                saturating_accumulation: Default::default(),
                scope: Default::default(),
                workgroup_invocations: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CooperativeMatrixFlexibleDimensionsPropertiesNV<'a> {
        #[inline]
        pub fn m_granularity(mut self, m_granularity: u32) -> Self {
            self.m_granularity = m_granularity;
            self
        }

        #[inline]
        pub fn n_granularity(mut self, n_granularity: u32) -> Self {
            self.n_granularity = n_granularity;
            self
        }

        #[inline]
        pub fn k_granularity(mut self, k_granularity: u32) -> Self {
            self.k_granularity = k_granularity;
            self
        }

        #[inline]
        pub fn a_type(mut self, a_type: ComponentTypeKHR) -> Self {
            self.a_type = a_type;
            self
        }

        #[inline]
        pub fn b_type(mut self, b_type: ComponentTypeKHR) -> Self {
            self.b_type = b_type;
            self
        }

        #[inline]
        pub fn c_type(mut self, c_type: ComponentTypeKHR) -> Self {
            self.c_type = c_type;
            self
        }

        #[inline]
        pub fn result_type(mut self, result_type: ComponentTypeKHR) -> Self {
            self.result_type = result_type;
            self
        }

        #[inline]
        pub fn saturating_accumulation(mut self, saturating_accumulation: bool) -> Self {
            self.saturating_accumulation = saturating_accumulation.into();
            self
        }

        #[inline]
        pub fn scope(mut self, scope: ScopeKHR) -> Self {
            self.scope = scope;
            self
        }

        #[inline]
        pub fn workgroup_invocations(mut self, workgroup_invocations: u32) -> Self {
            self.workgroup_invocations = workgroup_invocations;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV.html>
    pub type PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut CooperativeMatrixFlexibleDimensionsPropertiesNV<'_>,
        ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceCooperativeMatrix2FeaturesNV =
        PhysicalDeviceCooperativeMatrix2FeaturesNV<'static>;
    pub type VkPhysicalDeviceCooperativeMatrix2PropertiesNV =
        PhysicalDeviceCooperativeMatrix2PropertiesNV<'static>;
    pub type VkCooperativeMatrixFlexibleDimensionsPropertiesNV =
        CooperativeMatrixFlexibleDimensionsPropertiesNV<'static>;
    impl PhysicalDeviceCooperativeMatrix2FeaturesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceCooperativeMatrix2FeaturesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceCooperativeMatrix2PropertiesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceCooperativeMatrix2PropertiesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CooperativeMatrixFlexibleDimensionsPropertiesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkCooperativeMatrixFlexibleDimensionsPropertiesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    get_physical_device_cooperative_matrix_flexible_dimensions_properties:
        PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_cooperative_matrix_flexible_dimensions_properties: transmute(
                    load(c"vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV.html>
    #[inline]
    pub unsafe fn get_physical_device_cooperative_matrix_flexible_dimensions_properties<'a>(
        &self,
        physical_device: PhysicalDevice,
        mut properties: impl EnumerateInto<CooperativeMatrixFlexibleDimensionsPropertiesNV<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |property_count, properties| {
                let result = (self
                    .get_physical_device_cooperative_matrix_flexible_dimensions_properties)(
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
