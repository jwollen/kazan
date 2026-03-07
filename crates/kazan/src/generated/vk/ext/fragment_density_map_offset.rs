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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingEndInfoEXT.html>
    pub type RenderingEndInfoEXT<'a> = RenderingEndInfoKHR<'a>;
    pub type PFN_vkCmdEndRendering2EXT = PFN_vkCmdEndRendering2KHR;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFragmentDensityMapOffsetFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub fragment_density_map_offset: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "fragment_density_map_offset",
                    &self.fragment_density_map_offset,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                fragment_density_map_offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'a> {
        pub fn fragment_density_map_offset(mut self, fragment_density_map_offset: bool) -> Self {
            self.fragment_density_map_offset = fragment_density_map_offset.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFragmentDensityMapOffsetPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceFragmentDensityMapOffsetPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub fragment_density_offset_granularity: Extent2D,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceFragmentDensityMapOffsetPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceFragmentDensityMapOffsetPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "fragment_density_offset_granularity",
                    &self.fragment_density_offset_granularity,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentDensityMapOffsetPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceFragmentDensityMapOffsetPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceFragmentDensityMapOffsetPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                fragment_density_offset_granularity: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceFragmentDensityMapOffsetPropertiesEXT<'a> {
        pub fn fragment_density_offset_granularity(
            mut self,
            fragment_density_offset_granularity: Extent2D,
        ) -> Self {
            self.fragment_density_offset_granularity = fragment_density_offset_granularity;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassFragmentDensityMapOffsetEndInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderPassFragmentDensityMapOffsetEndInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub fragment_density_offset_count: u32,
        pub p_fragment_density_offsets: *const Offset2D,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for RenderPassFragmentDensityMapOffsetEndInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderPassFragmentDensityMapOffsetEndInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "fragment_density_offset_count",
                    &self.fragment_density_offset_count,
                )
                .field(
                    "p_fragment_density_offsets",
                    &self.p_fragment_density_offsets,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderPassFragmentDensityMapOffsetEndInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT;
    }

    unsafe impl<'a> Extends<SubpassEndInfo<'a>> for RenderPassFragmentDensityMapOffsetEndInfoEXT<'a> {}
    unsafe impl<'a> Extends<RenderingEndInfoKHR<'a>>
        for RenderPassFragmentDensityMapOffsetEndInfoEXT<'a>
    {
    }

    impl Default for RenderPassFragmentDensityMapOffsetEndInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                fragment_density_offset_count: Default::default(),
                p_fragment_density_offsets: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RenderPassFragmentDensityMapOffsetEndInfoEXT<'a> {
        pub fn fragment_density_offsets(
            mut self,
            fragment_density_offsets: &'a [Offset2D],
        ) -> Self {
            self.fragment_density_offset_count = fragment_density_offsets.len().try_into().unwrap();
            self.p_fragment_density_offsets = fragment_density_offsets.as_ptr();
            self
        }
    }
}

pub struct DeviceFn {
    cmd_end_rendering2_ext: PFN_vkCmdEndRendering2KHR,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_end_rendering2_ext: transmute(
                    load(c"vkCmdEndRendering2EXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndRendering2EXT.html>
    pub unsafe fn cmd_end_rendering2_ext(
        &self,
        command_buffer: CommandBuffer,
        rendering_end_info: Option<&RenderingEndInfoKHR<'_>>,
    ) {
        unsafe { (self.cmd_end_rendering2_ext)(command_buffer, rendering_end_info.to_raw_ptr()) }
    }
}
