//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_GOOGLE_hdr_smpte_st_209450_metadata.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_GOOGLE_hdr_smpte_st_209450_metadata";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkHdrSmpteSt209450GOOGLE.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct HdrSmpteSt209450GOOGLE<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub metadata_size: usize,
        pub p_metadata: *const c_void,
        pub power_preference: HdrSmpteSt209450PowerPreferenceGOOGLE,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for HdrSmpteSt209450GOOGLE<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("HdrSmpteSt209450GOOGLE")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("metadata_size", &self.metadata_size)
                .field("p_metadata", &self.p_metadata)
                .field("power_preference", &self.power_preference)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for HdrSmpteSt209450GOOGLE<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::HDR_SMPTE_ST_209450_GOOGLE;
    }

    unsafe impl Extends<HdrMetadataEXT<'_>> for HdrSmpteSt209450GOOGLE<'_> {}

    impl Default for HdrSmpteSt209450GOOGLE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                metadata_size: Default::default(),
                p_metadata: ptr::null(),
                power_preference: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> HdrSmpteSt209450GOOGLE<'a> {
        #[inline]
        pub fn metadata(mut self, metadata: &'a [u8]) -> Self {
            self.metadata_size = metadata.len().try_into().unwrap();
            self.p_metadata = metadata.as_ptr() as _;
            self
        }

        #[inline]
        pub fn power_preference(
            mut self,
            power_preference: HdrSmpteSt209450PowerPreferenceGOOGLE,
        ) -> Self {
            self.power_preference = power_preference;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceCapabilitiesHdrSmpteSt209450GOOGLE.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SurfaceCapabilitiesHdrSmpteSt209450GOOGLE<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub hdr_smpte_st209450_supported: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SurfaceCapabilitiesHdrSmpteSt209450GOOGLE<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SurfaceCapabilitiesHdrSmpteSt209450GOOGLE")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "hdr_smpte_st209450_supported",
                    &self.hdr_smpte_st209450_supported,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SurfaceCapabilitiesHdrSmpteSt209450GOOGLE<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SURFACE_CAPABILITIES_HDR_SMPTE_ST_209450_GOOGLE;
    }

    unsafe impl Extends<SurfaceCapabilities2KHR<'_>> for SurfaceCapabilitiesHdrSmpteSt209450GOOGLE<'_> {}

    impl Default for SurfaceCapabilitiesHdrSmpteSt209450GOOGLE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                hdr_smpte_st209450_supported: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SurfaceCapabilitiesHdrSmpteSt209450GOOGLE<'a> {
        #[inline]
        pub fn hdr_smpte_st209450_supported(mut self, hdr_smpte_st209450_supported: bool) -> Self {
            self.hdr_smpte_st209450_supported = hdr_smpte_st209450_supported.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceHdrSmpteSt209450FeaturesGOOGLE.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceHdrSmpteSt209450FeaturesGOOGLE<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub hdr_smpte_st209450: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceHdrSmpteSt209450FeaturesGOOGLE<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceHdrSmpteSt209450FeaturesGOOGLE")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("hdr_smpte_st209450", &self.hdr_smpte_st209450)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceHdrSmpteSt209450FeaturesGOOGLE<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_HDR_SMPTE_ST_209450_FEATURES_GOOGLE;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceHdrSmpteSt209450FeaturesGOOGLE<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceHdrSmpteSt209450FeaturesGOOGLE<'_> {}

    impl Default for PhysicalDeviceHdrSmpteSt209450FeaturesGOOGLE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                hdr_smpte_st209450: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceHdrSmpteSt209450FeaturesGOOGLE<'a> {
        #[inline]
        pub fn hdr_smpte_st209450(mut self, hdr_smpte_st209450: bool) -> Self {
            self.hdr_smpte_st209450 = hdr_smpte_st209450.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkHdrSmpteSt209450PowerPreferenceGOOGLE.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct HdrSmpteSt209450PowerPreferenceGOOGLE(i32);

    impl HdrSmpteSt209450PowerPreferenceGOOGLE {
        #[inline]
        pub const fn from_raw(x: i32) -> Self {
            Self(x)
        }
        #[inline]
        pub const fn as_raw(self) -> i32 {
            self.0
        }

        pub const COLOR_ACCURACY_GOOGLE: Self = Self(0);
        pub const LOW_POWER_GOOGLE: Self = Self(1);
    }

    impl fmt::Debug for HdrSmpteSt209450PowerPreferenceGOOGLE {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::COLOR_ACCURACY_GOOGLE => Some("COLOR_ACCURACY_GOOGLE"),
                Self::LOW_POWER_GOOGLE => Some("LOW_POWER_GOOGLE"),
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

    pub type VkHdrSmpteSt209450GOOGLE = HdrSmpteSt209450GOOGLE<'static>;
    pub type VkSurfaceCapabilitiesHdrSmpteSt209450GOOGLE =
        SurfaceCapabilitiesHdrSmpteSt209450GOOGLE<'static>;
    pub type VkPhysicalDeviceHdrSmpteSt209450FeaturesGOOGLE =
        PhysicalDeviceHdrSmpteSt209450FeaturesGOOGLE<'static>;
    pub type VkHdrSmpteSt209450PowerPreferenceGOOGLE = HdrSmpteSt209450PowerPreferenceGOOGLE;
    impl HdrSmpteSt209450GOOGLE<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkHdrSmpteSt209450GOOGLE {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SurfaceCapabilitiesHdrSmpteSt209450GOOGLE<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSurfaceCapabilitiesHdrSmpteSt209450GOOGLE {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceHdrSmpteSt209450FeaturesGOOGLE<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceHdrSmpteSt209450FeaturesGOOGLE {
            unsafe { core::mem::transmute(self) }
        }
    }
}
