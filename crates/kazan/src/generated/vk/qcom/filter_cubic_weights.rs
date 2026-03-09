//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_QCOM_filter_cubic_weights.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_QCOM_filter_cubic_weights";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCubicWeightsFeaturesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCubicWeightsFeaturesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub selectable_cubic_weights: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCubicWeightsFeaturesQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCubicWeightsFeaturesQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("selectable_cubic_weights", &self.selectable_cubic_weights)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCubicWeightsFeaturesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceCubicWeightsFeaturesQCOM<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceCubicWeightsFeaturesQCOM<'a> {}

    impl Default for PhysicalDeviceCubicWeightsFeaturesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                selectable_cubic_weights: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCubicWeightsFeaturesQCOM<'a> {
        #[inline]
        pub fn selectable_cubic_weights(mut self, selectable_cubic_weights: bool) -> Self {
            self.selectable_cubic_weights = selectable_cubic_weights.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerCubicWeightsCreateInfoQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SamplerCubicWeightsCreateInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub cubic_weights: CubicFilterWeightsQCOM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SamplerCubicWeightsCreateInfoQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SamplerCubicWeightsCreateInfoQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("cubic_weights", &self.cubic_weights)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SamplerCubicWeightsCreateInfoQCOM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM;
    }

    unsafe impl<'a> Extends<SamplerCreateInfo<'a>> for SamplerCubicWeightsCreateInfoQCOM<'a> {}

    impl Default for SamplerCubicWeightsCreateInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                cubic_weights: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SamplerCubicWeightsCreateInfoQCOM<'a> {
        #[inline]
        pub fn cubic_weights(mut self, cubic_weights: CubicFilterWeightsQCOM) -> Self {
            self.cubic_weights = cubic_weights;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBlitImageCubicWeightsInfoQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BlitImageCubicWeightsInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub cubic_weights: CubicFilterWeightsQCOM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BlitImageCubicWeightsInfoQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BlitImageCubicWeightsInfoQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("cubic_weights", &self.cubic_weights)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BlitImageCubicWeightsInfoQCOM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM;
    }

    unsafe impl<'a> Extends<BlitImageInfo2<'a>> for BlitImageCubicWeightsInfoQCOM<'a> {}

    impl Default for BlitImageCubicWeightsInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                cubic_weights: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BlitImageCubicWeightsInfoQCOM<'a> {
        #[inline]
        pub fn cubic_weights(mut self, cubic_weights: CubicFilterWeightsQCOM) -> Self {
            self.cubic_weights = cubic_weights;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCubicFilterWeightsQCOM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CubicFilterWeightsQCOM(i32);

    impl CubicFilterWeightsQCOM {
        pub const CATMULL_ROM_QCOM: Self = Self(0);
        pub const ZERO_TANGENT_CARDINAL_QCOM: Self = Self(1);
        pub const B_SPLINE_QCOM: Self = Self(2);
        pub const MITCHELL_NETRAVALI_QCOM: Self = Self(3);
    }

    impl fmt::Debug for CubicFilterWeightsQCOM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::CATMULL_ROM_QCOM => Some("CATMULL_ROM_QCOM"),
                Self::ZERO_TANGENT_CARDINAL_QCOM => Some("ZERO_TANGENT_CARDINAL_QCOM"),
                Self::B_SPLINE_QCOM => Some("B_SPLINE_QCOM"),
                Self::MITCHELL_NETRAVALI_QCOM => Some("MITCHELL_NETRAVALI_QCOM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceCubicWeightsFeaturesQCOM =
        PhysicalDeviceCubicWeightsFeaturesQCOM<'static>;
    pub type VkSamplerCubicWeightsCreateInfoQCOM = SamplerCubicWeightsCreateInfoQCOM<'static>;
    pub type VkBlitImageCubicWeightsInfoQCOM = BlitImageCubicWeightsInfoQCOM<'static>;
    pub type VkCubicFilterWeightsQCOM = CubicFilterWeightsQCOM;
    impl PhysicalDeviceCubicWeightsFeaturesQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceCubicWeightsFeaturesQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SamplerCubicWeightsCreateInfoQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSamplerCubicWeightsCreateInfoQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BlitImageCubicWeightsInfoQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBlitImageCubicWeightsInfoQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
}
