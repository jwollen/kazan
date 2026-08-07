//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_cooperative_matrix_maintenance1.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_cooperative_matrix_maintenance1";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCooperativeMatrixInfo2EXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCooperativeMatrixInfo2EXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub scope: ScopeKHR,
        pub invocations: u32,
        pub subgroup_size: u32,
        pub flags: CooperativeMatrixFlagsEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCooperativeMatrixInfo2EXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCooperativeMatrixInfo2EXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("scope", &self.scope)
                .field("invocations", &self.invocations)
                .field("subgroup_size", &self.subgroup_size)
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCooperativeMatrixInfo2EXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_INFO_2_EXT;
    }

    impl Default for PhysicalDeviceCooperativeMatrixInfo2EXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                scope: Default::default(),
                invocations: Default::default(),
                subgroup_size: Default::default(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCooperativeMatrixInfo2EXT<'a> {
        #[inline]
        pub fn scope(mut self, scope: ScopeKHR) -> Self {
            self.scope = scope;
            self
        }

        #[inline]
        pub fn invocations(mut self, invocations: u32) -> Self {
            self.invocations = invocations;
            self
        }

        #[inline]
        pub fn subgroup_size(mut self, subgroup_size: u32) -> Self {
            self.subgroup_size = subgroup_size;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: CooperativeMatrixFlagsEXT) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCooperativeMatrixProperties2EXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CooperativeMatrixProperties2EXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub m_granularity: u32,
        pub n_granularity: u32,
        pub k_granularity: u32,
        pub a_type: ComponentTypeKHR,
        pub b_type: ComponentTypeKHR,
        pub c_type: ComponentTypeKHR,
        pub result_type: ComponentTypeKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CooperativeMatrixProperties2EXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CooperativeMatrixProperties2EXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("m_granularity", &self.m_granularity)
                .field("n_granularity", &self.n_granularity)
                .field("k_granularity", &self.k_granularity)
                .field("a_type", &self.a_type)
                .field("b_type", &self.b_type)
                .field("c_type", &self.c_type)
                .field("result_type", &self.result_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CooperativeMatrixProperties2EXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COOPERATIVE_MATRIX_PROPERTIES_2_EXT;
    }

    impl Default for CooperativeMatrixProperties2EXT<'_> {
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
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CooperativeMatrixProperties2EXT<'a> {
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
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cooperative_matrix_properties2: Bool32,
        pub cooperative_matrix_reductions: Bool32,
        pub cooperative_matrix_conversions: Bool32,
        pub cooperative_matrix_per_element_operations: Bool32,
        pub cooperative_matrix_get_coordinate: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "cooperative_matrix_properties2",
                    &self.cooperative_matrix_properties2,
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
                    "cooperative_matrix_get_coordinate",
                    &self.cooperative_matrix_get_coordinate,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_MAINTENANCE_1_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT<'_>
    {
    }

    impl Default for PhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                cooperative_matrix_properties2: Default::default(),
                cooperative_matrix_reductions: Default::default(),
                cooperative_matrix_conversions: Default::default(),
                cooperative_matrix_per_element_operations: Default::default(),
                cooperative_matrix_get_coordinate: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT<'a> {
        #[inline]
        pub fn cooperative_matrix_properties2(
            mut self,
            cooperative_matrix_properties2: bool,
        ) -> Self {
            self.cooperative_matrix_properties2 = cooperative_matrix_properties2.into();
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
        pub fn cooperative_matrix_get_coordinate(
            mut self,
            cooperative_matrix_get_coordinate: bool,
        ) -> Self {
            self.cooperative_matrix_get_coordinate = cooperative_matrix_get_coordinate.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCooperativeMatrixFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CooperativeMatrixFlagsEXT(Flags);
    vk_bitflags_wrapped!(
        CooperativeMatrixFlagsEXT,
        Flags,
        CooperativeMatrixFlagBitsEXT
    );

    impl fmt::Debug for CooperativeMatrixFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[(
                CooperativeMatrixFlagBitsEXT::SATURATING_ACCUMULATION_EXT.0,
                "SATURATING_ACCUMULATION_EXT",
            )];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCooperativeMatrixFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct CooperativeMatrixFlagBitsEXT(u32);

    impl CooperativeMatrixFlagBitsEXT {
        // VK_EXT_cooperative_matrix_maintenance1
        pub const SATURATING_ACCUMULATION_EXT: Self = Self(1 << 0);
    }

    impl fmt::Debug for CooperativeMatrixFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SATURATING_ACCUMULATION_EXT => Some("SATURATING_ACCUMULATION_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceCooperativeMatrixProperties2EXT.html>
    pub type PFN_vkGetPhysicalDeviceCooperativeMatrixProperties2EXT =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_cooperative_matrix_info: *const PhysicalDeviceCooperativeMatrixInfo2EXT<'_>,
            p_property_count: *mut u32,
            p_properties: *mut CooperativeMatrixProperties2EXT<'_>,
        ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceCooperativeMatrixInfo2EXT =
        PhysicalDeviceCooperativeMatrixInfo2EXT<'static>;
    pub type VkCooperativeMatrixProperties2EXT = CooperativeMatrixProperties2EXT<'static>;
    pub type VkPhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT =
        PhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT<'static>;
    pub type VkCooperativeMatrixFlagsEXT = CooperativeMatrixFlagsEXT;
    pub type VkCooperativeMatrixFlagBitsEXT = CooperativeMatrixFlagBitsEXT;
    impl PhysicalDeviceCooperativeMatrixInfo2EXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceCooperativeMatrixInfo2EXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CooperativeMatrixProperties2EXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCooperativeMatrixProperties2EXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    get_physical_device_cooperative_matrix_properties2:
        PFN_vkGetPhysicalDeviceCooperativeMatrixProperties2EXT,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_cooperative_matrix_properties2: transmute(
                    load(c"vkGetPhysicalDeviceCooperativeMatrixProperties2EXT")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceCooperativeMatrixProperties2EXT.html>
    #[inline]
    pub unsafe fn get_physical_device_cooperative_matrix_properties2<'a>(
        &self,
        physical_device: PhysicalDevice,
        cooperative_matrix_info: &PhysicalDeviceCooperativeMatrixInfo2EXT<'a>,
        mut properties: impl EnumerateInto<CooperativeMatrixProperties2EXT<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |property_count, properties| {
                let result = (self.get_physical_device_cooperative_matrix_properties2)(
                    physical_device,
                    cooperative_matrix_info,
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
