#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance8FeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMaintenance8FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub maintenance8: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance8FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceMaintenance8FeaturesKHR<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceMaintenance8FeaturesKHR<'a> {}

    impl Default for PhysicalDeviceMaintenance8FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                maintenance8: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMaintenance8FeaturesKHR<'a> {
        pub fn maintenance8(mut self, maintenance8: bool) -> Self {
            self.maintenance8 = maintenance8.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryBarrierAccessFlags3KHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryBarrierAccessFlags3KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_access_mask3: AccessFlags3KHR,
        pub dst_access_mask3: AccessFlags3KHR,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryBarrierAccessFlags3KHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_BARRIER_ACCESS_FLAGS_3_KHR;
    }

    unsafe impl<'a> Extends<SubpassDependency2<'a>> for MemoryBarrierAccessFlags3KHR<'a> {}
    unsafe impl<'a> Extends<BufferMemoryBarrier2<'a>> for MemoryBarrierAccessFlags3KHR<'a> {}
    unsafe impl<'a> Extends<ImageMemoryBarrier2<'a>> for MemoryBarrierAccessFlags3KHR<'a> {}

    impl Default for MemoryBarrierAccessFlags3KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_access_mask3: Default::default(),
                dst_access_mask3: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryBarrierAccessFlags3KHR<'a> {
        pub fn src_access_mask3(mut self, src_access_mask3: AccessFlags3KHR) -> Self {
            self.src_access_mask3 = src_access_mask3;
            self
        }

        pub fn dst_access_mask3(mut self, dst_access_mask3: AccessFlags3KHR) -> Self {
            self.dst_access_mask3 = dst_access_mask3;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccessFlags3KHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct AccessFlags3KHR(Flags64);
    vk_bitflags_wrapped!(AccessFlags3KHR, Flags64);

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
}
