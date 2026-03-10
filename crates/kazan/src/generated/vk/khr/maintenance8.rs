//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_maintenance8.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_maintenance8";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance8FeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMaintenance8FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub maintenance8: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMaintenance8FeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMaintenance8FeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("maintenance8", &self.maintenance8)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance8FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceMaintenance8FeaturesKHR<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceMaintenance8FeaturesKHR<'_> {}

    impl Default for PhysicalDeviceMaintenance8FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                maintenance8: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMaintenance8FeaturesKHR<'a> {
        #[inline]
        pub fn maintenance8(mut self, maintenance8: bool) -> Self {
            self.maintenance8 = maintenance8.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryBarrierAccessFlags3KHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryBarrierAccessFlags3KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_access_mask3: AccessFlags3KHR,
        pub dst_access_mask3: AccessFlags3KHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryBarrierAccessFlags3KHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryBarrierAccessFlags3KHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_access_mask3", &self.src_access_mask3)
                .field("dst_access_mask3", &self.dst_access_mask3)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryBarrierAccessFlags3KHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_BARRIER_ACCESS_FLAGS_3_KHR;
    }

    unsafe impl Extends<SubpassDependency2<'_>> for MemoryBarrierAccessFlags3KHR<'_> {}
    unsafe impl Extends<BufferMemoryBarrier2<'_>> for MemoryBarrierAccessFlags3KHR<'_> {}
    unsafe impl Extends<ImageMemoryBarrier2<'_>> for MemoryBarrierAccessFlags3KHR<'_> {}

    impl Default for MemoryBarrierAccessFlags3KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_access_mask3: Default::default(),
                dst_access_mask3: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryBarrierAccessFlags3KHR<'a> {
        #[inline]
        pub fn src_access_mask3(mut self, src_access_mask3: AccessFlags3KHR) -> Self {
            self.src_access_mask3 = src_access_mask3;
            self
        }

        #[inline]
        pub fn dst_access_mask3(mut self, dst_access_mask3: AccessFlags3KHR) -> Self {
            self.dst_access_mask3 = dst_access_mask3;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccessFlags3KHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct AccessFlags3KHR(Flags64);
    vk_bitflags_wrapped!(AccessFlags3KHR, Flags64, AccessFlagBits3KHR);

    impl AccessFlags3KHR {
        pub const NONE: Self = Self(0);
    }

    impl fmt::Debug for AccessFlags3KHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags64, &str)] = &[];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccessFlagBits3KHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct AccessFlagBits3KHR(u64);

    impl AccessFlagBits3KHR {}

    impl fmt::Debug for AccessFlagBits3KHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
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

    pub type VkPhysicalDeviceMaintenance8FeaturesKHR =
        PhysicalDeviceMaintenance8FeaturesKHR<'static>;
    pub type VkMemoryBarrierAccessFlags3KHR = MemoryBarrierAccessFlags3KHR<'static>;
    pub type VkAccessFlags3KHR = AccessFlags3KHR;
    pub type VkAccessFlagBits3KHR = AccessFlagBits3KHR;
    impl PhysicalDeviceMaintenance8FeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceMaintenance8FeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MemoryBarrierAccessFlags3KHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMemoryBarrierAccessFlags3KHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
