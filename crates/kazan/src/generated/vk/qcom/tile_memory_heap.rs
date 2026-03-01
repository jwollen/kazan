#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct TileMemoryBindInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for TileMemoryBindInfoQCOM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TILE_MEMORY_BIND_INFO_QCOM;
    }
    unsafe impl<'a> Extends<CommandBufferInheritanceInfo<'a>> for TileMemoryBindInfoQCOM<'a> {}
    impl Default for TileMemoryBindInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> TileMemoryBindInfoQCOM<'a> {
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceTileMemoryHeapFeaturesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub tile_memory_heap: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTileMemoryHeapFeaturesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceTileMemoryHeapFeaturesQCOM<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceTileMemoryHeapFeaturesQCOM<'a> {}
    impl Default for PhysicalDeviceTileMemoryHeapFeaturesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                tile_memory_heap: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceTileMemoryHeapFeaturesQCOM<'a> {
        pub fn tile_memory_heap(mut self, tile_memory_heap: Bool32) -> Self {
            self.tile_memory_heap = tile_memory_heap;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceTileMemoryHeapPropertiesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub queue_submit_boundary: Bool32,
        pub tile_buffer_transfers: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTileMemoryHeapPropertiesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceTileMemoryHeapPropertiesQCOM<'a>
    {
    }
    impl Default for PhysicalDeviceTileMemoryHeapPropertiesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                queue_submit_boundary: Default::default(),
                tile_buffer_transfers: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceTileMemoryHeapPropertiesQCOM<'a> {
        pub fn queue_submit_boundary(mut self, queue_submit_boundary: Bool32) -> Self {
            self.queue_submit_boundary = queue_submit_boundary;
            self
        }
        pub fn tile_buffer_transfers(mut self, tile_buffer_transfers: Bool32) -> Self {
            self.tile_buffer_transfers = tile_buffer_transfers;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct TileMemorySizeInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for TileMemorySizeInfoQCOM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TILE_MEMORY_SIZE_INFO_QCOM;
    }
    unsafe impl<'a> Extends<RenderPassCreateInfo<'a>> for TileMemorySizeInfoQCOM<'a> {}
    unsafe impl<'a> Extends<RenderPassCreateInfo2<'a>> for TileMemorySizeInfoQCOM<'a> {}
    unsafe impl<'a> Extends<RenderingInfo<'a>> for TileMemorySizeInfoQCOM<'a> {}
    impl Default for TileMemorySizeInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> TileMemorySizeInfoQCOM<'a> {
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct TileMemoryRequirementsQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub size: DeviceSize,
        pub alignment: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for TileMemoryRequirementsQCOM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TILE_MEMORY_REQUIREMENTS_QCOM;
    }
    unsafe impl<'a> Extends<MemoryRequirements2<'a>> for TileMemoryRequirementsQCOM<'a> {}
    impl Default for TileMemoryRequirementsQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                size: Default::default(),
                alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> TileMemoryRequirementsQCOM<'a> {
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
        pub fn alignment(mut self, alignment: DeviceSize) -> Self {
            self.alignment = alignment;
            self
        }
    }
    pub type PFN_vkCmdBindTileMemoryQCOM = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_tile_memory_bind_info: *const TileMemoryBindInfoQCOM<'_>,
    );
}
pub struct DeviceFn {
    cmd_bind_tile_memory_qcom: PFN_vkCmdBindTileMemoryQCOM,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_bind_tile_memory_qcom: transmute(
                    load(c"vkCmdBindTileMemoryQCOM").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
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
