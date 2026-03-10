//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_QCOM_tile_memory_heap.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_QCOM_tile_memory_heap";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTileMemoryBindInfoQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct TileMemoryBindInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for TileMemoryBindInfoQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TileMemoryBindInfoQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory", &self.memory)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TileMemoryBindInfoQCOM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TILE_MEMORY_BIND_INFO_QCOM;
    }

    unsafe impl Extends<CommandBufferInheritanceInfo<'_>> for TileMemoryBindInfoQCOM<'_> {}

    impl Default for TileMemoryBindInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TileMemoryBindInfoQCOM<'a> {
        #[inline]
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceTileMemoryHeapFeaturesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceTileMemoryHeapFeaturesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub tile_memory_heap: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceTileMemoryHeapFeaturesQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceTileMemoryHeapFeaturesQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("tile_memory_heap", &self.tile_memory_heap)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTileMemoryHeapFeaturesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceTileMemoryHeapFeaturesQCOM<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceTileMemoryHeapFeaturesQCOM<'_> {}

    impl Default for PhysicalDeviceTileMemoryHeapFeaturesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                tile_memory_heap: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceTileMemoryHeapFeaturesQCOM<'a> {
        #[inline]
        pub fn tile_memory_heap(mut self, tile_memory_heap: bool) -> Self {
            self.tile_memory_heap = tile_memory_heap.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceTileMemoryHeapPropertiesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceTileMemoryHeapPropertiesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub queue_submit_boundary: Bool32,
        pub tile_buffer_transfers: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceTileMemoryHeapPropertiesQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceTileMemoryHeapPropertiesQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("queue_submit_boundary", &self.queue_submit_boundary)
                .field("tile_buffer_transfers", &self.tile_buffer_transfers)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTileMemoryHeapPropertiesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceTileMemoryHeapPropertiesQCOM<'_>
    {
    }

    impl Default for PhysicalDeviceTileMemoryHeapPropertiesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                queue_submit_boundary: Default::default(),
                tile_buffer_transfers: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceTileMemoryHeapPropertiesQCOM<'a> {
        #[inline]
        pub fn queue_submit_boundary(mut self, queue_submit_boundary: bool) -> Self {
            self.queue_submit_boundary = queue_submit_boundary.into();
            self
        }

        #[inline]
        pub fn tile_buffer_transfers(mut self, tile_buffer_transfers: bool) -> Self {
            self.tile_buffer_transfers = tile_buffer_transfers.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTileMemorySizeInfoQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct TileMemorySizeInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for TileMemorySizeInfoQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TileMemorySizeInfoQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("size", &self.size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TileMemorySizeInfoQCOM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TILE_MEMORY_SIZE_INFO_QCOM;
    }

    unsafe impl Extends<RenderPassCreateInfo<'_>> for TileMemorySizeInfoQCOM<'_> {}
    unsafe impl Extends<RenderPassCreateInfo2<'_>> for TileMemorySizeInfoQCOM<'_> {}
    unsafe impl Extends<RenderingInfo<'_>> for TileMemorySizeInfoQCOM<'_> {}

    impl Default for TileMemorySizeInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TileMemorySizeInfoQCOM<'a> {
        #[inline]
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTileMemoryRequirementsQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct TileMemoryRequirementsQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub size: DeviceSize,
        pub alignment: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for TileMemoryRequirementsQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TileMemoryRequirementsQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("size", &self.size)
                .field("alignment", &self.alignment)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TileMemoryRequirementsQCOM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TILE_MEMORY_REQUIREMENTS_QCOM;
    }

    unsafe impl Extends<MemoryRequirements2<'_>> for TileMemoryRequirementsQCOM<'_> {}

    impl Default for TileMemoryRequirementsQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                size: Default::default(),
                alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TileMemoryRequirementsQCOM<'a> {
        #[inline]
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }

        #[inline]
        pub fn alignment(mut self, alignment: DeviceSize) -> Self {
            self.alignment = alignment;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindTileMemoryQCOM.html>
    pub type PFN_vkCmdBindTileMemoryQCOM = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_tile_memory_bind_info: *const TileMemoryBindInfoQCOM<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkTileMemoryBindInfoQCOM = TileMemoryBindInfoQCOM<'static>;
    pub type VkPhysicalDeviceTileMemoryHeapFeaturesQCOM =
        PhysicalDeviceTileMemoryHeapFeaturesQCOM<'static>;
    pub type VkPhysicalDeviceTileMemoryHeapPropertiesQCOM =
        PhysicalDeviceTileMemoryHeapPropertiesQCOM<'static>;
    pub type VkTileMemorySizeInfoQCOM = TileMemorySizeInfoQCOM<'static>;
    pub type VkTileMemoryRequirementsQCOM = TileMemoryRequirementsQCOM<'static>;
    impl TileMemoryBindInfoQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkTileMemoryBindInfoQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceTileMemoryHeapFeaturesQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceTileMemoryHeapFeaturesQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceTileMemoryHeapPropertiesQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceTileMemoryHeapPropertiesQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl TileMemorySizeInfoQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkTileMemorySizeInfoQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl TileMemoryRequirementsQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkTileMemoryRequirementsQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    cmd_bind_tile_memory_qcom: PFN_vkCmdBindTileMemoryQCOM,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_bind_tile_memory_qcom: transmute(
                    load(c"vkCmdBindTileMemoryQCOM").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindTileMemoryQCOM.html>
    #[inline]
    pub unsafe fn cmd_bind_tile_memory_qcom(
        &self,
        command_buffer: CommandBuffer,
        tile_memory_bind_info: Option<&TileMemoryBindInfoQCOM<'_>>,
    ) {
        unsafe {
            (self.cmd_bind_tile_memory_qcom)(command_buffer, tile_memory_bind_info.to_raw_ptr())
        }
    }
}
