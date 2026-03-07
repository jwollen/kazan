#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_map_memory_placed";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMapMemoryPlacedFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMapMemoryPlacedFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_map_placed: Bool32,
        pub memory_map_range_placed: Bool32,
        pub memory_unmap_reserve: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMapMemoryPlacedFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMapMemoryPlacedFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory_map_placed", &self.memory_map_placed)
                .field("memory_map_range_placed", &self.memory_map_range_placed)
                .field("memory_unmap_reserve", &self.memory_unmap_reserve)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMapMemoryPlacedFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceMapMemoryPlacedFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceMapMemoryPlacedFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceMapMemoryPlacedFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                memory_map_placed: Default::default(),
                memory_map_range_placed: Default::default(),
                memory_unmap_reserve: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMapMemoryPlacedFeaturesEXT<'a> {
        #[inline]
        pub fn memory_map_placed(mut self, memory_map_placed: bool) -> Self {
            self.memory_map_placed = memory_map_placed.into();
            self
        }

        #[inline]
        pub fn memory_map_range_placed(mut self, memory_map_range_placed: bool) -> Self {
            self.memory_map_range_placed = memory_map_range_placed.into();
            self
        }

        #[inline]
        pub fn memory_unmap_reserve(mut self, memory_unmap_reserve: bool) -> Self {
            self.memory_unmap_reserve = memory_unmap_reserve.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMapMemoryPlacedPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMapMemoryPlacedPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_placed_memory_map_alignment: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMapMemoryPlacedPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMapMemoryPlacedPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "min_placed_memory_map_alignment",
                    &self.min_placed_memory_map_alignment,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMapMemoryPlacedPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MAP_MEMORY_PLACED_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceMapMemoryPlacedPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceMapMemoryPlacedPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                min_placed_memory_map_alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMapMemoryPlacedPropertiesEXT<'a> {
        #[inline]
        pub fn min_placed_memory_map_alignment(
            mut self,
            min_placed_memory_map_alignment: DeviceSize,
        ) -> Self {
            self.min_placed_memory_map_alignment = min_placed_memory_map_alignment;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryMapPlacedInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryMapPlacedInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_placed_address: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryMapPlacedInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryMapPlacedInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_placed_address", &self.p_placed_address)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryMapPlacedInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_MAP_PLACED_INFO_EXT;
    }

    unsafe impl<'a> Extends<MemoryMapInfo<'a>> for MemoryMapPlacedInfoEXT<'a> {}

    impl Default for MemoryMapPlacedInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_placed_address: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryMapPlacedInfoEXT<'a> {
        #[inline]
        pub fn placed_address(mut self, placed_address: *mut c_void) -> Self {
            self.p_placed_address = placed_address;
            self
        }
    }
}
