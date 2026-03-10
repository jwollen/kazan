//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_cooperative_matrix.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_cooperative_matrix";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCooperativeMatrixFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCooperativeMatrixFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cooperative_matrix: Bool32,
        pub cooperative_matrix_robust_buffer_access: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCooperativeMatrixFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCooperativeMatrixFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("cooperative_matrix", &self.cooperative_matrix)
                .field(
                    "cooperative_matrix_robust_buffer_access",
                    &self.cooperative_matrix_robust_buffer_access,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCooperativeMatrixFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceCooperativeMatrixFeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceCooperativeMatrixFeaturesKHR<'a> {}

    impl Default for PhysicalDeviceCooperativeMatrixFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                cooperative_matrix: Default::default(),
                cooperative_matrix_robust_buffer_access: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCooperativeMatrixFeaturesKHR<'a> {
        #[inline]
        pub fn cooperative_matrix(mut self, cooperative_matrix: bool) -> Self {
            self.cooperative_matrix = cooperative_matrix.into();
            self
        }

        #[inline]
        pub fn cooperative_matrix_robust_buffer_access(
            mut self,
            cooperative_matrix_robust_buffer_access: bool,
        ) -> Self {
            self.cooperative_matrix_robust_buffer_access =
                cooperative_matrix_robust_buffer_access.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCooperativeMatrixPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for CooperativeMatrixPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CooperativeMatrixPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("m_size", &self.m_size)
                .field("n_size", &self.n_size)
                .field("k_size", &self.k_size)
                .field("a_type", &self.a_type)
                .field("b_type", &self.b_type)
                .field("c_type", &self.c_type)
                .field("result_type", &self.result_type)
                .field("saturating_accumulation", &self.saturating_accumulation)
                .field("scope", &self.scope)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CooperativeMatrixPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COOPERATIVE_MATRIX_PROPERTIES_KHR;
    }

    impl Default for CooperativeMatrixPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
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
        #[inline]
        pub fn m_size(mut self, m_size: u32) -> Self {
            self.m_size = m_size;
            self
        }

        #[inline]
        pub fn n_size(mut self, n_size: u32) -> Self {
            self.n_size = n_size;
            self
        }

        #[inline]
        pub fn k_size(mut self, k_size: u32) -> Self {
            self.k_size = k_size;
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
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCooperativeMatrixPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCooperativeMatrixPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cooperative_matrix_supported_stages: ShaderStageFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCooperativeMatrixPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCooperativeMatrixPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "cooperative_matrix_supported_stages",
                    &self.cooperative_matrix_supported_stages,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCooperativeMatrixPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceCooperativeMatrixPropertiesKHR<'a>
    {
    }

    impl Default for PhysicalDeviceCooperativeMatrixPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                cooperative_matrix_supported_stages: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCooperativeMatrixPropertiesKHR<'a> {
        #[inline]
        pub fn cooperative_matrix_supported_stages(
            mut self,
            cooperative_matrix_supported_stages: ShaderStageFlags,
        ) -> Self {
            self.cooperative_matrix_supported_stages = cooperative_matrix_supported_stages;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkScopeKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ScopeKHR(i32);

    impl ScopeKHR {
        pub const DEVICE_KHR: Self = Self(1);
        pub const WORKGROUP_KHR: Self = Self(2);
        pub const SUBGROUP_KHR: Self = Self(3);
        pub const QUEUE_FAMILY_KHR: Self = Self(5);

        // VK_NV_cooperative_matrix
        pub const DEVICE_NV: Self = Self::DEVICE_KHR;
        pub const WORKGROUP_NV: Self = Self::WORKGROUP_KHR;
        pub const SUBGROUP_NV: Self = Self::SUBGROUP_KHR;
        pub const QUEUE_FAMILY_NV: Self = Self::QUEUE_FAMILY_KHR;
    }

    impl fmt::Debug for ScopeKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEVICE_KHR => Some("DEVICE_KHR"),
                Self::WORKGROUP_KHR => Some("WORKGROUP_KHR"),
                Self::SUBGROUP_KHR => Some("SUBGROUP_KHR"),
                Self::QUEUE_FAMILY_KHR => Some("QUEUE_FAMILY_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkComponentTypeKHR.html>
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

        // VK_EXT_shader_float8
        pub const FLOAT8_E4M3_EXT: Self = Self(1000491002);
        pub const FLOAT8_E5M2_EXT: Self = Self(1000491003);

        // VK_KHR_shader_bfloat16
        pub const BFLOAT16_KHR: Self = Self(1000141000);

        // VK_NV_cooperative_matrix
        pub const FLOAT16_NV: Self = Self::FLOAT16_KHR;
        pub const FLOAT32_NV: Self = Self::FLOAT32_KHR;
        pub const FLOAT64_NV: Self = Self::FLOAT64_KHR;
        pub const SINT8_NV: Self = Self::SINT8_KHR;
        pub const SINT16_NV: Self = Self::SINT16_KHR;
        pub const SINT32_NV: Self = Self::SINT32_KHR;
        pub const SINT64_NV: Self = Self::SINT64_KHR;
        pub const UINT8_NV: Self = Self::UINT8_KHR;
        pub const UINT16_NV: Self = Self::UINT16_KHR;
        pub const UINT32_NV: Self = Self::UINT32_KHR;
        pub const UINT64_NV: Self = Self::UINT64_KHR;

        // VK_NV_cooperative_vector
        pub const SINT8_PACKED_NV: Self = Self(1000491000);
        pub const UINT8_PACKED_NV: Self = Self(1000491001);
        pub const FLOAT_E4M3_NV: Self = Self::FLOAT8_E4M3_EXT;
        pub const FLOAT_E5M2_NV: Self = Self::FLOAT8_E5M2_EXT;
    }

    impl fmt::Debug for ComponentTypeKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::FLOAT16_KHR => Some("FLOAT16_KHR"),
                Self::FLOAT32_KHR => Some("FLOAT32_KHR"),
                Self::FLOAT64_KHR => Some("FLOAT64_KHR"),
                Self::SINT8_KHR => Some("SINT8_KHR"),
                Self::SINT16_KHR => Some("SINT16_KHR"),
                Self::SINT32_KHR => Some("SINT32_KHR"),
                Self::SINT64_KHR => Some("SINT64_KHR"),
                Self::UINT8_KHR => Some("UINT8_KHR"),
                Self::UINT16_KHR => Some("UINT16_KHR"),
                Self::UINT32_KHR => Some("UINT32_KHR"),
                Self::UINT64_KHR => Some("UINT64_KHR"),
                Self::FLOAT8_E4M3_EXT => Some("FLOAT8_E4M3_EXT"),
                Self::FLOAT8_E5M2_EXT => Some("FLOAT8_E5M2_EXT"),
                Self::BFLOAT16_KHR => Some("BFLOAT16_KHR"),
                Self::SINT8_PACKED_NV => Some("SINT8_PACKED_NV"),
                Self::UINT8_PACKED_NV => Some("UINT8_PACKED_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html>
    pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut CooperativeMatrixPropertiesKHR<'_>,
        ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceCooperativeMatrixFeaturesKHR =
        PhysicalDeviceCooperativeMatrixFeaturesKHR<'static>;
    pub type VkCooperativeMatrixPropertiesKHR = CooperativeMatrixPropertiesKHR<'static>;
    pub type VkPhysicalDeviceCooperativeMatrixPropertiesKHR =
        PhysicalDeviceCooperativeMatrixPropertiesKHR<'static>;
    pub type VkScopeKHR = ScopeKHR;
    pub type VkComponentTypeKHR = ComponentTypeKHR;
    impl PhysicalDeviceCooperativeMatrixFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceCooperativeMatrixFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CooperativeMatrixPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCooperativeMatrixPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceCooperativeMatrixPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceCooperativeMatrixPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    get_physical_device_cooperative_matrix_properties_khr:
        PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_cooperative_matrix_properties_khr: transmute(
                    load(c"vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_cooperative_matrix_properties_khr<'a>(
        &self,
        physical_device: PhysicalDevice,
        mut properties: impl ExtendUninit<CooperativeMatrixPropertiesKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |property_count, properties| {
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
