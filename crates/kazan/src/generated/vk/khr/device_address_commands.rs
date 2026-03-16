//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_device_address_commands.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_device_address_commands";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkStridedDeviceAddressRangeKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StridedDeviceAddressRangeKHR {
        pub address: DeviceAddress,
        pub size: DeviceSize,
        pub stride: DeviceSize,
    }

    impl StridedDeviceAddressRangeKHR {
        #[inline]
        pub fn address(mut self, address: DeviceAddress) -> Self {
            self.address = address;
            self
        }

        #[inline]
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }

        #[inline]
        pub fn stride(mut self, stride: DeviceSize) -> Self {
            self.stride = stride;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceAddressRangeKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct DeviceAddressRangeKHR {
        pub address: DeviceAddress,
        pub size: DeviceSize,
    }

    impl DeviceAddressRangeKHR {
        #[inline]
        pub fn address(mut self, address: DeviceAddress) -> Self {
            self.address = address;
            self
        }

        #[inline]
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceMemoryCopyKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceMemoryCopyKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_range: DeviceAddressRangeKHR,
        pub src_flags: AddressCommandFlagsKHR,
        pub dst_range: DeviceAddressRangeKHR,
        pub dst_flags: AddressCommandFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceMemoryCopyKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceMemoryCopyKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_range", &self.src_range)
                .field("src_flags", &self.src_flags)
                .field("dst_range", &self.dst_range)
                .field("dst_flags", &self.dst_flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceMemoryCopyKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_MEMORY_COPY_KHR;
    }

    impl Default for DeviceMemoryCopyKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_range: Default::default(),
                src_flags: Default::default(),
                dst_range: Default::default(),
                dst_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceMemoryCopyKHR<'a> {
        #[inline]
        pub fn src_range(mut self, src_range: DeviceAddressRangeKHR) -> Self {
            self.src_range = src_range;
            self
        }

        #[inline]
        pub fn src_flags(mut self, src_flags: AddressCommandFlagsKHR) -> Self {
            self.src_flags = src_flags;
            self
        }

        #[inline]
        pub fn dst_range(mut self, dst_range: DeviceAddressRangeKHR) -> Self {
            self.dst_range = dst_range;
            self
        }

        #[inline]
        pub fn dst_flags(mut self, dst_flags: AddressCommandFlagsKHR) -> Self {
            self.dst_flags = dst_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyDeviceMemoryInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CopyDeviceMemoryInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub region_count: u32,
        pub p_regions: *const DeviceMemoryCopyKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyDeviceMemoryInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyDeviceMemoryInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("region_count", &self.region_count)
                .field("p_regions", &self.p_regions)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyDeviceMemoryInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_DEVICE_MEMORY_INFO_KHR;
    }

    impl Default for CopyDeviceMemoryInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                region_count: Default::default(),
                p_regions: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CopyDeviceMemoryInfoKHR<'a> {
        #[inline]
        pub fn regions(mut self, regions: &'a [DeviceMemoryCopyKHR<'_>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceMemoryImageCopyKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceMemoryImageCopyKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub address_range: DeviceAddressRangeKHR,
        pub address_flags: AddressCommandFlagsKHR,
        pub address_row_length: u32,
        pub address_image_height: u32,
        pub image_subresource: ImageSubresourceLayers,
        pub image_layout: ImageLayout,
        pub image_offset: Offset3D,
        pub image_extent: Extent3D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceMemoryImageCopyKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceMemoryImageCopyKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("address_range", &self.address_range)
                .field("address_flags", &self.address_flags)
                .field("address_row_length", &self.address_row_length)
                .field("address_image_height", &self.address_image_height)
                .field("image_subresource", &self.image_subresource)
                .field("image_layout", &self.image_layout)
                .field("image_offset", &self.image_offset)
                .field("image_extent", &self.image_extent)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceMemoryImageCopyKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_MEMORY_IMAGE_COPY_KHR;
    }

    impl Default for DeviceMemoryImageCopyKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                address_range: Default::default(),
                address_flags: Default::default(),
                address_row_length: Default::default(),
                address_image_height: Default::default(),
                image_subresource: Default::default(),
                image_layout: Default::default(),
                image_offset: Default::default(),
                image_extent: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceMemoryImageCopyKHR<'a> {
        #[inline]
        pub fn address_range(mut self, address_range: DeviceAddressRangeKHR) -> Self {
            self.address_range = address_range;
            self
        }

        #[inline]
        pub fn address_flags(mut self, address_flags: AddressCommandFlagsKHR) -> Self {
            self.address_flags = address_flags;
            self
        }

        #[inline]
        pub fn address_row_length(mut self, address_row_length: u32) -> Self {
            self.address_row_length = address_row_length;
            self
        }

        #[inline]
        pub fn address_image_height(mut self, address_image_height: u32) -> Self {
            self.address_image_height = address_image_height;
            self
        }

        #[inline]
        pub fn image_subresource(mut self, image_subresource: ImageSubresourceLayers) -> Self {
            self.image_subresource = image_subresource;
            self
        }

        #[inline]
        pub fn image_layout(mut self, image_layout: ImageLayout) -> Self {
            self.image_layout = image_layout;
            self
        }

        #[inline]
        pub fn image_offset(mut self, image_offset: Offset3D) -> Self {
            self.image_offset = image_offset;
            self
        }

        #[inline]
        pub fn image_extent(mut self, image_extent: Extent3D) -> Self {
            self.image_extent = image_extent;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyDeviceMemoryImageInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CopyDeviceMemoryImageInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image: Image,
        pub region_count: u32,
        pub p_regions: *const DeviceMemoryImageCopyKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyDeviceMemoryImageInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyDeviceMemoryImageInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image", &self.image)
                .field("region_count", &self.region_count)
                .field("p_regions", &self.p_regions)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyDeviceMemoryImageInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_DEVICE_MEMORY_IMAGE_INFO_KHR;
    }

    impl Default for CopyDeviceMemoryImageInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                image: Default::default(),
                region_count: Default::default(),
                p_regions: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CopyDeviceMemoryImageInfoKHR<'a> {
        #[inline]
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }

        #[inline]
        pub fn regions(mut self, regions: &'a [DeviceMemoryImageCopyKHR<'_>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryRangeBarriersInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryRangeBarriersInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory_range_barrier_count: u32,
        pub p_memory_range_barriers: *const MemoryRangeBarrierKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryRangeBarriersInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryRangeBarriersInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "memory_range_barrier_count",
                    &self.memory_range_barrier_count,
                )
                .field("p_memory_range_barriers", &self.p_memory_range_barriers)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryRangeBarriersInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_RANGE_BARRIERS_INFO_KHR;
    }

    unsafe impl Extends<DependencyInfo<'_>> for MemoryRangeBarriersInfoKHR<'_> {}

    impl Default for MemoryRangeBarriersInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                memory_range_barrier_count: Default::default(),
                p_memory_range_barriers: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryRangeBarriersInfoKHR<'a> {
        #[inline]
        pub fn memory_range_barriers(
            mut self,
            memory_range_barriers: &'a [MemoryRangeBarrierKHR<'_>],
        ) -> Self {
            self.memory_range_barrier_count = memory_range_barriers.len().try_into().unwrap();
            self.p_memory_range_barriers = memory_range_barriers.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryRangeBarrierKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryRangeBarrierKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_stage_mask: PipelineStageFlags2,
        pub src_access_mask: AccessFlags2,
        pub dst_stage_mask: PipelineStageFlags2,
        pub dst_access_mask: AccessFlags2,
        pub src_queue_family_index: u32,
        pub dst_queue_family_index: u32,
        pub address_range: DeviceAddressRangeKHR,
        pub address_flags: AddressCommandFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryRangeBarrierKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryRangeBarrierKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_stage_mask", &self.src_stage_mask)
                .field("src_access_mask", &self.src_access_mask)
                .field("dst_stage_mask", &self.dst_stage_mask)
                .field("dst_access_mask", &self.dst_access_mask)
                .field("src_queue_family_index", &self.src_queue_family_index)
                .field("dst_queue_family_index", &self.dst_queue_family_index)
                .field("address_range", &self.address_range)
                .field("address_flags", &self.address_flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryRangeBarrierKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_RANGE_BARRIER_KHR;
    }

    impl Default for MemoryRangeBarrierKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_stage_mask: Default::default(),
                src_access_mask: Default::default(),
                dst_stage_mask: Default::default(),
                dst_access_mask: Default::default(),
                src_queue_family_index: Default::default(),
                dst_queue_family_index: Default::default(),
                address_range: Default::default(),
                address_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryRangeBarrierKHR<'a> {
        #[inline]
        pub fn src_stage_mask(mut self, src_stage_mask: PipelineStageFlags2) -> Self {
            self.src_stage_mask = src_stage_mask;
            self
        }

        #[inline]
        pub fn src_access_mask(mut self, src_access_mask: AccessFlags2) -> Self {
            self.src_access_mask = src_access_mask;
            self
        }

        #[inline]
        pub fn dst_stage_mask(mut self, dst_stage_mask: PipelineStageFlags2) -> Self {
            self.dst_stage_mask = dst_stage_mask;
            self
        }

        #[inline]
        pub fn dst_access_mask(mut self, dst_access_mask: AccessFlags2) -> Self {
            self.dst_access_mask = dst_access_mask;
            self
        }

        #[inline]
        pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
            self.src_queue_family_index = src_queue_family_index;
            self
        }

        #[inline]
        pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
            self.dst_queue_family_index = dst_queue_family_index;
            self
        }

        #[inline]
        pub fn address_range(mut self, address_range: DeviceAddressRangeKHR) -> Self {
            self.address_range = address_range;
            self
        }

        #[inline]
        pub fn address_flags(mut self, address_flags: AddressCommandFlagsKHR) -> Self {
            self.address_flags = address_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDeviceAddressCommandsFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDeviceAddressCommandsFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub device_address_commands: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDeviceAddressCommandsFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDeviceAddressCommandsFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("device_address_commands", &self.device_address_commands)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDeviceAddressCommandsFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DEVICE_ADDRESS_COMMANDS_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceDeviceAddressCommandsFeaturesKHR<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceDeviceAddressCommandsFeaturesKHR<'_> {}

    impl Default for PhysicalDeviceDeviceAddressCommandsFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                device_address_commands: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDeviceAddressCommandsFeaturesKHR<'a> {
        #[inline]
        pub fn device_address_commands(mut self, device_address_commands: bool) -> Self {
            self.device_address_commands = device_address_commands.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkConditionalRenderingBeginInfo2EXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ConditionalRenderingBeginInfo2EXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub address_range: DeviceAddressRangeKHR,
        pub address_flags: AddressCommandFlagsKHR,
        pub flags: ConditionalRenderingFlagsEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ConditionalRenderingBeginInfo2EXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ConditionalRenderingBeginInfo2EXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("address_range", &self.address_range)
                .field("address_flags", &self.address_flags)
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ConditionalRenderingBeginInfo2EXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::CONDITIONAL_RENDERING_BEGIN_INFO_2_EXT;
    }

    impl Default for ConditionalRenderingBeginInfo2EXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                address_range: Default::default(),
                address_flags: Default::default(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ConditionalRenderingBeginInfo2EXT<'a> {
        #[inline]
        pub fn address_range(mut self, address_range: DeviceAddressRangeKHR) -> Self {
            self.address_range = address_range;
            self
        }

        #[inline]
        pub fn address_flags(mut self, address_flags: AddressCommandFlagsKHR) -> Self {
            self.address_flags = address_flags;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: ConditionalRenderingFlagsEXT) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureCreateInfo2KHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureCreateInfo2KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub create_flags: AccelerationStructureCreateFlagsKHR,
        pub address_range: DeviceAddressRangeKHR,
        pub address_flags: AddressCommandFlagsKHR,
        pub ty: AccelerationStructureTypeKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureCreateInfo2KHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureCreateInfo2KHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("create_flags", &self.create_flags)
                .field("address_range", &self.address_range)
                .field("address_flags", &self.address_flags)
                .field("ty", &self.ty)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureCreateInfo2KHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_CREATE_INFO_2_KHR;
    }

    impl Default for AccelerationStructureCreateInfo2KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                create_flags: Default::default(),
                address_range: Default::default(),
                address_flags: Default::default(),
                ty: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureCreateInfo2KHR<'a> {
        #[inline]
        pub fn create_flags(mut self, create_flags: AccelerationStructureCreateFlagsKHR) -> Self {
            self.create_flags = create_flags;
            self
        }

        #[inline]
        pub fn address_range(mut self, address_range: DeviceAddressRangeKHR) -> Self {
            self.address_range = address_range;
            self
        }

        #[inline]
        pub fn address_flags(mut self, address_flags: AddressCommandFlagsKHR) -> Self {
            self.address_flags = address_flags;
            self
        }

        #[inline]
        pub fn ty(mut self, ty: AccelerationStructureTypeKHR) -> Self {
            self.ty = ty;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindIndexBuffer3InfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BindIndexBuffer3InfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub address_range: DeviceAddressRangeKHR,
        pub address_flags: AddressCommandFlagsKHR,
        pub index_type: IndexType,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BindIndexBuffer3InfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BindIndexBuffer3InfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("address_range", &self.address_range)
                .field("address_flags", &self.address_flags)
                .field("index_type", &self.index_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BindIndexBuffer3InfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_INDEX_BUFFER_3_INFO_KHR;
    }

    impl Default for BindIndexBuffer3InfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                address_range: Default::default(),
                address_flags: Default::default(),
                index_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BindIndexBuffer3InfoKHR<'a> {
        #[inline]
        pub fn address_range(mut self, address_range: DeviceAddressRangeKHR) -> Self {
            self.address_range = address_range;
            self
        }

        #[inline]
        pub fn address_flags(mut self, address_flags: AddressCommandFlagsKHR) -> Self {
            self.address_flags = address_flags;
            self
        }

        #[inline]
        pub fn index_type(mut self, index_type: IndexType) -> Self {
            self.index_type = index_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindVertexBuffer3InfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BindVertexBuffer3InfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub set_stride: Bool32,
        pub address_range: StridedDeviceAddressRangeKHR,
        pub address_flags: AddressCommandFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BindVertexBuffer3InfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BindVertexBuffer3InfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("set_stride", &self.set_stride)
                .field("address_range", &self.address_range)
                .field("address_flags", &self.address_flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BindVertexBuffer3InfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_VERTEX_BUFFER_3_INFO_KHR;
    }

    impl Default for BindVertexBuffer3InfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                set_stride: Default::default(),
                address_range: Default::default(),
                address_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BindVertexBuffer3InfoKHR<'a> {
        #[inline]
        pub fn set_stride(mut self, set_stride: bool) -> Self {
            self.set_stride = set_stride.into();
            self
        }

        #[inline]
        pub fn address_range(mut self, address_range: StridedDeviceAddressRangeKHR) -> Self {
            self.address_range = address_range;
            self
        }

        #[inline]
        pub fn address_flags(mut self, address_flags: AddressCommandFlagsKHR) -> Self {
            self.address_flags = address_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDrawIndirect2InfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DrawIndirect2InfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub address_range: StridedDeviceAddressRangeKHR,
        pub address_flags: AddressCommandFlagsKHR,
        pub draw_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DrawIndirect2InfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DrawIndirect2InfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("address_range", &self.address_range)
                .field("address_flags", &self.address_flags)
                .field("draw_count", &self.draw_count)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DrawIndirect2InfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DRAW_INDIRECT_2_INFO_KHR;
    }

    impl Default for DrawIndirect2InfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                address_range: Default::default(),
                address_flags: Default::default(),
                draw_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DrawIndirect2InfoKHR<'a> {
        #[inline]
        pub fn address_range(mut self, address_range: StridedDeviceAddressRangeKHR) -> Self {
            self.address_range = address_range;
            self
        }

        #[inline]
        pub fn address_flags(mut self, address_flags: AddressCommandFlagsKHR) -> Self {
            self.address_flags = address_flags;
            self
        }

        #[inline]
        pub fn draw_count(mut self, draw_count: u32) -> Self {
            self.draw_count = draw_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDrawIndirectCount2InfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DrawIndirectCount2InfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub address_range: StridedDeviceAddressRangeKHR,
        pub address_flags: AddressCommandFlagsKHR,
        pub count_address_range: DeviceAddressRangeKHR,
        pub count_address_flags: AddressCommandFlagsKHR,
        pub max_draw_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DrawIndirectCount2InfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DrawIndirectCount2InfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("address_range", &self.address_range)
                .field("address_flags", &self.address_flags)
                .field("count_address_range", &self.count_address_range)
                .field("count_address_flags", &self.count_address_flags)
                .field("max_draw_count", &self.max_draw_count)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DrawIndirectCount2InfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DRAW_INDIRECT_COUNT_2_INFO_KHR;
    }

    impl Default for DrawIndirectCount2InfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                address_range: Default::default(),
                address_flags: Default::default(),
                count_address_range: Default::default(),
                count_address_flags: Default::default(),
                max_draw_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DrawIndirectCount2InfoKHR<'a> {
        #[inline]
        pub fn address_range(mut self, address_range: StridedDeviceAddressRangeKHR) -> Self {
            self.address_range = address_range;
            self
        }

        #[inline]
        pub fn address_flags(mut self, address_flags: AddressCommandFlagsKHR) -> Self {
            self.address_flags = address_flags;
            self
        }

        #[inline]
        pub fn count_address_range(mut self, count_address_range: DeviceAddressRangeKHR) -> Self {
            self.count_address_range = count_address_range;
            self
        }

        #[inline]
        pub fn count_address_flags(mut self, count_address_flags: AddressCommandFlagsKHR) -> Self {
            self.count_address_flags = count_address_flags;
            self
        }

        #[inline]
        pub fn max_draw_count(mut self, max_draw_count: u32) -> Self {
            self.max_draw_count = max_draw_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDispatchIndirect2InfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DispatchIndirect2InfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub address_range: DeviceAddressRangeKHR,
        pub address_flags: AddressCommandFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DispatchIndirect2InfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DispatchIndirect2InfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("address_range", &self.address_range)
                .field("address_flags", &self.address_flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DispatchIndirect2InfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DISPATCH_INDIRECT_2_INFO_KHR;
    }

    impl Default for DispatchIndirect2InfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                address_range: Default::default(),
                address_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DispatchIndirect2InfoKHR<'a> {
        #[inline]
        pub fn address_range(mut self, address_range: DeviceAddressRangeKHR) -> Self {
            self.address_range = address_range;
            self
        }

        #[inline]
        pub fn address_flags(mut self, address_flags: AddressCommandFlagsKHR) -> Self {
            self.address_flags = address_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindTransformFeedbackBuffer2InfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BindTransformFeedbackBuffer2InfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub address_range: DeviceAddressRangeKHR,
        pub address_flags: AddressCommandFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BindTransformFeedbackBuffer2InfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BindTransformFeedbackBuffer2InfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("address_range", &self.address_range)
                .field("address_flags", &self.address_flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BindTransformFeedbackBuffer2InfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::BIND_TRANSFORM_FEEDBACK_BUFFER_2_INFO_EXT;
    }

    impl Default for BindTransformFeedbackBuffer2InfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                address_range: Default::default(),
                address_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BindTransformFeedbackBuffer2InfoEXT<'a> {
        #[inline]
        pub fn address_range(mut self, address_range: DeviceAddressRangeKHR) -> Self {
            self.address_range = address_range;
            self
        }

        #[inline]
        pub fn address_flags(mut self, address_flags: AddressCommandFlagsKHR) -> Self {
            self.address_flags = address_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryMarkerInfoAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryMarkerInfoAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub stage: PipelineStageFlags2KHR,
        pub dst_range: DeviceAddressRangeKHR,
        pub dst_flags: AddressCommandFlagsKHR,
        pub marker: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryMarkerInfoAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryMarkerInfoAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("stage", &self.stage)
                .field("dst_range", &self.dst_range)
                .field("dst_flags", &self.dst_flags)
                .field("marker", &self.marker)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryMarkerInfoAMD<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_MARKER_INFO_AMD;
    }

    impl Default for MemoryMarkerInfoAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                stage: Default::default(),
                dst_range: Default::default(),
                dst_flags: Default::default(),
                marker: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryMarkerInfoAMD<'a> {
        #[inline]
        pub fn stage(mut self, stage: PipelineStageFlags2KHR) -> Self {
            self.stage = stage;
            self
        }

        #[inline]
        pub fn dst_range(mut self, dst_range: DeviceAddressRangeKHR) -> Self {
            self.dst_range = dst_range;
            self
        }

        #[inline]
        pub fn dst_flags(mut self, dst_flags: AddressCommandFlagsKHR) -> Self {
            self.dst_flags = dst_flags;
            self
        }

        #[inline]
        pub fn marker(mut self, marker: u32) -> Self {
            self.marker = marker;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAddressCommandFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct AddressCommandFlagsKHR(Flags);
    vk_bitflags_wrapped!(AddressCommandFlagsKHR, Flags, AddressCommandFlagBitsKHR);

    impl fmt::Debug for AddressCommandFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (AddressCommandFlagBitsKHR::PROTECTED_KHR.0, "PROTECTED_KHR"),
                (
                    AddressCommandFlagBitsKHR::FULLY_BOUND_KHR.0,
                    "FULLY_BOUND_KHR",
                ),
                (
                    AddressCommandFlagBitsKHR::STORAGE_BUFFER_USAGE_KHR.0,
                    "STORAGE_BUFFER_USAGE_KHR",
                ),
                (
                    AddressCommandFlagBitsKHR::UNKNOWN_STORAGE_BUFFER_USAGE_KHR.0,
                    "UNKNOWN_STORAGE_BUFFER_USAGE_KHR",
                ),
                (
                    AddressCommandFlagBitsKHR::TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR.0,
                    "TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR",
                ),
                (
                    AddressCommandFlagBitsKHR::UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR.0,
                    "UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAddressCommandFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct AddressCommandFlagBitsKHR(u32);

    impl AddressCommandFlagBitsKHR {
        pub const PROTECTED_KHR: Self = Self(1 << 0);
        pub const FULLY_BOUND_KHR: Self = Self(1 << 1);
        pub const STORAGE_BUFFER_USAGE_KHR: Self = Self(1 << 2);
        pub const UNKNOWN_STORAGE_BUFFER_USAGE_KHR: Self = Self(1 << 3);
        // VK_KHR_device_address_commands
        pub const TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR: Self = Self(1 << 4);
        pub const UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR: Self = Self(1 << 5);
    }

    impl fmt::Debug for AddressCommandFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::PROTECTED_KHR => Some("PROTECTED_KHR"),
                Self::FULLY_BOUND_KHR => Some("FULLY_BOUND_KHR"),
                Self::STORAGE_BUFFER_USAGE_KHR => Some("STORAGE_BUFFER_USAGE_KHR"),
                Self::UNKNOWN_STORAGE_BUFFER_USAGE_KHR => Some("UNKNOWN_STORAGE_BUFFER_USAGE_KHR"),
                Self::TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR => {
                    Some("TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR")
                }
                Self::UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR => {
                    Some("UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR")
                }
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryKHR.html>
    pub type PFN_vkCmdCopyMemoryKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_memory_info: *const CopyDeviceMemoryInfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryToImageKHR.html>
    pub type PFN_vkCmdCopyMemoryToImageKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_memory_info: *const CopyDeviceMemoryImageInfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyImageToMemoryKHR.html>
    pub type PFN_vkCmdCopyImageToMemoryKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_memory_info: *const CopyDeviceMemoryImageInfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdUpdateMemoryKHR.html>
    pub type PFN_vkCmdUpdateMemoryKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_dst_range: *const DeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        data_size: DeviceSize,
        p_data: *const c_void,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdFillMemoryKHR.html>
    pub type PFN_vkCmdFillMemoryKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_dst_range: *const DeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        data: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyQueryPoolResultsToMemoryKHR.html>
    pub type PFN_vkCmdCopyQueryPoolResultsToMemoryKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        p_dst_range: *const StridedDeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        query_result_flags: QueryResultFlags,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginConditionalRendering2EXT.html>
    pub type PFN_vkCmdBeginConditionalRendering2EXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_conditional_rendering_begin: *const ConditionalRenderingBeginInfo2EXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindTransformFeedbackBuffers2EXT.html>
    pub type PFN_vkCmdBindTransformFeedbackBuffers2EXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_binding_infos: *const BindTransformFeedbackBuffer2InfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginTransformFeedback2EXT.html>
    pub type PFN_vkCmdBeginTransformFeedback2EXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_counter_range: u32,
        counter_range_count: u32,
        p_counter_infos: *const BindTransformFeedbackBuffer2InfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndTransformFeedback2EXT.html>
    pub type PFN_vkCmdEndTransformFeedback2EXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_counter_range: u32,
        counter_range_count: u32,
        p_counter_infos: *const BindTransformFeedbackBuffer2InfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawIndirectByteCount2EXT.html>
    pub type PFN_vkCmdDrawIndirectByteCount2EXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        p_counter_info: *const BindTransformFeedbackBuffer2InfoEXT<'_>,
        counter_offset: u32,
        vertex_stride: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWriteMarkerToMemoryAMD.html>
    pub type PFN_vkCmdWriteMarkerToMemoryAMD = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const MemoryMarkerInfoAMD<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindIndexBuffer3KHR.html>
    pub type PFN_vkCmdBindIndexBuffer3KHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const BindIndexBuffer3InfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindVertexBuffers3KHR.html>
    pub type PFN_vkCmdBindVertexBuffers3KHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_binding_infos: *const BindVertexBuffer3InfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawIndirect2KHR.html>
    pub type PFN_vkCmdDrawIndirect2KHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirect2InfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawIndexedIndirect2KHR.html>
    pub type PFN_vkCmdDrawIndexedIndirect2KHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirect2InfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawIndirectCount2KHR.html>
    pub type PFN_vkCmdDrawIndirectCount2KHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirectCount2InfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawIndexedIndirectCount2KHR.html>
    pub type PFN_vkCmdDrawIndexedIndirectCount2KHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirectCount2InfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawMeshTasksIndirect2EXT.html>
    pub type PFN_vkCmdDrawMeshTasksIndirect2EXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirect2InfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawMeshTasksIndirectCount2EXT.html>
    pub type PFN_vkCmdDrawMeshTasksIndirectCount2EXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirectCount2InfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDispatchIndirect2KHR.html>
    pub type PFN_vkCmdDispatchIndirect2KHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const DispatchIndirect2InfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateAccelerationStructure2KHR.html>
    pub type PFN_vkCreateAccelerationStructure2KHR = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const AccelerationStructureCreateInfo2KHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_acceleration_structure: *mut AccelerationStructureKHR,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkStridedDeviceAddressRangeKHR = StridedDeviceAddressRangeKHR;
    pub type VkDeviceAddressRangeKHR = DeviceAddressRangeKHR;
    pub type VkDeviceMemoryCopyKHR = DeviceMemoryCopyKHR<'static>;
    pub type VkCopyDeviceMemoryInfoKHR = CopyDeviceMemoryInfoKHR<'static>;
    pub type VkDeviceMemoryImageCopyKHR = DeviceMemoryImageCopyKHR<'static>;
    pub type VkCopyDeviceMemoryImageInfoKHR = CopyDeviceMemoryImageInfoKHR<'static>;
    pub type VkMemoryRangeBarriersInfoKHR = MemoryRangeBarriersInfoKHR<'static>;
    pub type VkMemoryRangeBarrierKHR = MemoryRangeBarrierKHR<'static>;
    pub type VkPhysicalDeviceDeviceAddressCommandsFeaturesKHR =
        PhysicalDeviceDeviceAddressCommandsFeaturesKHR<'static>;
    pub type VkConditionalRenderingBeginInfo2EXT = ConditionalRenderingBeginInfo2EXT<'static>;
    pub type VkAccelerationStructureCreateInfo2KHR = AccelerationStructureCreateInfo2KHR<'static>;
    pub type VkBindIndexBuffer3InfoKHR = BindIndexBuffer3InfoKHR<'static>;
    pub type VkBindVertexBuffer3InfoKHR = BindVertexBuffer3InfoKHR<'static>;
    pub type VkDrawIndirect2InfoKHR = DrawIndirect2InfoKHR<'static>;
    pub type VkDrawIndirectCount2InfoKHR = DrawIndirectCount2InfoKHR<'static>;
    pub type VkDispatchIndirect2InfoKHR = DispatchIndirect2InfoKHR<'static>;
    pub type VkBindTransformFeedbackBuffer2InfoEXT = BindTransformFeedbackBuffer2InfoEXT<'static>;
    pub type VkMemoryMarkerInfoAMD = MemoryMarkerInfoAMD<'static>;
    pub type VkAddressCommandFlagsKHR = AddressCommandFlagsKHR;
    pub type VkAddressCommandFlagBitsKHR = AddressCommandFlagBitsKHR;
    impl DeviceMemoryCopyKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceMemoryCopyKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CopyDeviceMemoryInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCopyDeviceMemoryInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceMemoryImageCopyKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceMemoryImageCopyKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CopyDeviceMemoryImageInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCopyDeviceMemoryImageInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MemoryRangeBarriersInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMemoryRangeBarriersInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MemoryRangeBarrierKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMemoryRangeBarrierKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceDeviceAddressCommandsFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceDeviceAddressCommandsFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ConditionalRenderingBeginInfo2EXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkConditionalRenderingBeginInfo2EXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AccelerationStructureCreateInfo2KHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkAccelerationStructureCreateInfo2KHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BindIndexBuffer3InfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBindIndexBuffer3InfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BindVertexBuffer3InfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBindVertexBuffer3InfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DrawIndirect2InfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDrawIndirect2InfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DrawIndirectCount2InfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDrawIndirectCount2InfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DispatchIndirect2InfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDispatchIndirect2InfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BindTransformFeedbackBuffer2InfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBindTransformFeedbackBuffer2InfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MemoryMarkerInfoAMD<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMemoryMarkerInfoAMD {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    cmd_bind_index_buffer3: PFN_vkCmdBindIndexBuffer3KHR,
    cmd_bind_vertex_buffers3: PFN_vkCmdBindVertexBuffers3KHR,
    cmd_draw_indirect2: PFN_vkCmdDrawIndirect2KHR,
    cmd_draw_indexed_indirect2: PFN_vkCmdDrawIndexedIndirect2KHR,
    cmd_dispatch_indirect2: PFN_vkCmdDispatchIndirect2KHR,
    cmd_copy_memory: PFN_vkCmdCopyMemoryKHR,
    cmd_copy_memory_to_image: PFN_vkCmdCopyMemoryToImageKHR,
    cmd_copy_image_to_memory: PFN_vkCmdCopyImageToMemoryKHR,
    cmd_update_memory: PFN_vkCmdUpdateMemoryKHR,
    cmd_fill_memory: PFN_vkCmdFillMemoryKHR,
    cmd_copy_query_pool_results_to_memory: PFN_vkCmdCopyQueryPoolResultsToMemoryKHR,
    cmd_draw_indirect_count2: Option<PFN_vkCmdDrawIndirectCount2KHR>,
    cmd_draw_indexed_indirect_count2: Option<PFN_vkCmdDrawIndexedIndirectCount2KHR>,
    cmd_begin_conditional_rendering2: Option<PFN_vkCmdBeginConditionalRendering2EXT>,
    cmd_bind_transform_feedback_buffers2: Option<PFN_vkCmdBindTransformFeedbackBuffers2EXT>,
    cmd_begin_transform_feedback2: Option<PFN_vkCmdBeginTransformFeedback2EXT>,
    cmd_end_transform_feedback2: Option<PFN_vkCmdEndTransformFeedback2EXT>,
    cmd_draw_indirect_byte_count2: Option<PFN_vkCmdDrawIndirectByteCount2EXT>,
    cmd_draw_mesh_tasks_indirect2: Option<PFN_vkCmdDrawMeshTasksIndirect2EXT>,
    cmd_draw_mesh_tasks_indirect_count2: Option<PFN_vkCmdDrawMeshTasksIndirectCount2EXT>,
    cmd_write_marker_to_memory: Option<PFN_vkCmdWriteMarkerToMemoryAMD>,
    create_acceleration_structure2: Option<PFN_vkCreateAccelerationStructure2KHR>,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_bind_index_buffer3: transmute(
                    load(c"vkCmdBindIndexBuffer3KHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_bind_vertex_buffers3: transmute(
                    load(c"vkCmdBindVertexBuffers3KHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_draw_indirect2: transmute(
                    load(c"vkCmdDrawIndirect2KHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_draw_indexed_indirect2: transmute(
                    load(c"vkCmdDrawIndexedIndirect2KHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_dispatch_indirect2: transmute(
                    load(c"vkCmdDispatchIndirect2KHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_memory: transmute(
                    load(c"vkCmdCopyMemoryKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_memory_to_image: transmute(
                    load(c"vkCmdCopyMemoryToImageKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_image_to_memory: transmute(
                    load(c"vkCmdCopyImageToMemoryKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_update_memory: transmute(
                    load(c"vkCmdUpdateMemoryKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_fill_memory: transmute(
                    load(c"vkCmdFillMemoryKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_query_pool_results_to_memory: transmute(
                    load(c"vkCmdCopyQueryPoolResultsToMemoryKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_draw_indirect_count2: transmute(load(c"vkCmdDrawIndirectCount2KHR")),
                cmd_draw_indexed_indirect_count2: transmute(load(
                    c"vkCmdDrawIndexedIndirectCount2KHR",
                )),
                cmd_begin_conditional_rendering2: transmute(load(
                    c"vkCmdBeginConditionalRendering2EXT",
                )),
                cmd_bind_transform_feedback_buffers2: transmute(load(
                    c"vkCmdBindTransformFeedbackBuffers2EXT",
                )),
                cmd_begin_transform_feedback2: transmute(load(c"vkCmdBeginTransformFeedback2EXT")),
                cmd_end_transform_feedback2: transmute(load(c"vkCmdEndTransformFeedback2EXT")),
                cmd_draw_indirect_byte_count2: transmute(load(c"vkCmdDrawIndirectByteCount2EXT")),
                cmd_draw_mesh_tasks_indirect2: transmute(load(c"vkCmdDrawMeshTasksIndirect2EXT")),
                cmd_draw_mesh_tasks_indirect_count2: transmute(load(
                    c"vkCmdDrawMeshTasksIndirectCount2EXT",
                )),
                cmd_write_marker_to_memory: transmute(load(c"vkCmdWriteMarkerToMemoryAMD")),
                create_acceleration_structure2: transmute(load(
                    c"vkCreateAccelerationStructure2KHR",
                )),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindIndexBuffer3KHR.html>
    #[inline]
    pub unsafe fn cmd_bind_index_buffer3(
        &self,
        command_buffer: CommandBuffer,
        info: &BindIndexBuffer3InfoKHR<'_>,
    ) {
        unsafe { (self.cmd_bind_index_buffer3)(command_buffer, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindVertexBuffers3KHR.html>
    #[inline]
    pub unsafe fn cmd_bind_vertex_buffers3(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_infos: &[BindVertexBuffer3InfoKHR<'_>],
    ) {
        unsafe {
            (self.cmd_bind_vertex_buffers3)(
                command_buffer,
                first_binding,
                binding_infos.len().try_into().unwrap(),
                binding_infos.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawIndirect2KHR.html>
    #[inline]
    pub unsafe fn cmd_draw_indirect2(
        &self,
        command_buffer: CommandBuffer,
        info: &DrawIndirect2InfoKHR<'_>,
    ) {
        unsafe { (self.cmd_draw_indirect2)(command_buffer, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawIndexedIndirect2KHR.html>
    #[inline]
    pub unsafe fn cmd_draw_indexed_indirect2(
        &self,
        command_buffer: CommandBuffer,
        info: &DrawIndirect2InfoKHR<'_>,
    ) {
        unsafe { (self.cmd_draw_indexed_indirect2)(command_buffer, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDispatchIndirect2KHR.html>
    #[inline]
    pub unsafe fn cmd_dispatch_indirect2(
        &self,
        command_buffer: CommandBuffer,
        info: &DispatchIndirect2InfoKHR<'_>,
    ) {
        unsafe { (self.cmd_dispatch_indirect2)(command_buffer, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryKHR.html>
    #[inline]
    pub unsafe fn cmd_copy_memory(
        &self,
        command_buffer: CommandBuffer,
        copy_memory_info: Option<&CopyDeviceMemoryInfoKHR<'_>>,
    ) {
        unsafe { (self.cmd_copy_memory)(command_buffer, copy_memory_info.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryToImageKHR.html>
    #[inline]
    pub unsafe fn cmd_copy_memory_to_image(
        &self,
        command_buffer: CommandBuffer,
        copy_memory_info: Option<&CopyDeviceMemoryImageInfoKHR<'_>>,
    ) {
        unsafe { (self.cmd_copy_memory_to_image)(command_buffer, copy_memory_info.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyImageToMemoryKHR.html>
    #[inline]
    pub unsafe fn cmd_copy_image_to_memory(
        &self,
        command_buffer: CommandBuffer,
        copy_memory_info: Option<&CopyDeviceMemoryImageInfoKHR<'_>>,
    ) {
        unsafe { (self.cmd_copy_image_to_memory)(command_buffer, copy_memory_info.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdUpdateMemoryKHR.html>
    #[inline]
    pub unsafe fn cmd_update_memory(
        &self,
        command_buffer: CommandBuffer,
        dst_range: &DeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        data: &[u8],
    ) {
        unsafe {
            (self.cmd_update_memory)(
                command_buffer,
                dst_range,
                dst_flags,
                data.len().try_into().unwrap(),
                data.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdFillMemoryKHR.html>
    #[inline]
    pub unsafe fn cmd_fill_memory(
        &self,
        command_buffer: CommandBuffer,
        dst_range: &DeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        data: u32,
    ) {
        unsafe { (self.cmd_fill_memory)(command_buffer, dst_range, dst_flags, data) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyQueryPoolResultsToMemoryKHR.html>
    #[inline]
    pub unsafe fn cmd_copy_query_pool_results_to_memory(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_range: &StridedDeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        query_result_flags: QueryResultFlags,
    ) {
        unsafe {
            (self.cmd_copy_query_pool_results_to_memory)(
                command_buffer,
                query_pool,
                first_query,
                query_count,
                dst_range,
                dst_flags,
                query_result_flags,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawIndirectCount2KHR.html>
    #[inline]
    pub unsafe fn cmd_draw_indirect_count2(
        &self,
        command_buffer: CommandBuffer,
        info: &DrawIndirectCount2InfoKHR<'_>,
    ) {
        unsafe { (self.cmd_draw_indirect_count2.unwrap())(command_buffer, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawIndexedIndirectCount2KHR.html>
    #[inline]
    pub unsafe fn cmd_draw_indexed_indirect_count2(
        &self,
        command_buffer: CommandBuffer,
        info: &DrawIndirectCount2InfoKHR<'_>,
    ) {
        unsafe { (self.cmd_draw_indexed_indirect_count2.unwrap())(command_buffer, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginConditionalRendering2EXT.html>
    #[inline]
    pub unsafe fn cmd_begin_conditional_rendering2(
        &self,
        command_buffer: CommandBuffer,
        conditional_rendering_begin: &ConditionalRenderingBeginInfo2EXT<'_>,
    ) {
        unsafe {
            (self.cmd_begin_conditional_rendering2.unwrap())(
                command_buffer,
                conditional_rendering_begin,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindTransformFeedbackBuffers2EXT.html>
    #[inline]
    pub unsafe fn cmd_bind_transform_feedback_buffers2(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_infos: SliceOrLen<'_, BindTransformFeedbackBuffer2InfoEXT<'_>>,
    ) {
        unsafe {
            (self.cmd_bind_transform_feedback_buffers2.unwrap())(
                command_buffer,
                first_binding,
                binding_infos.len().try_into().unwrap(),
                binding_infos.to_raw_ptr(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginTransformFeedback2EXT.html>
    #[inline]
    pub unsafe fn cmd_begin_transform_feedback2(
        &self,
        command_buffer: CommandBuffer,
        first_counter_range: u32,
        counter_infos: Option<SliceOrLen<'_, BindTransformFeedbackBuffer2InfoEXT<'_>>>,
    ) {
        unsafe {
            (self.cmd_begin_transform_feedback2.unwrap())(
                command_buffer,
                first_counter_range,
                counter_infos
                    .as_ref()
                    .map_or(0, SliceOrLen::len)
                    .try_into()
                    .unwrap(),
                counter_infos.to_raw_ptr(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndTransformFeedback2EXT.html>
    #[inline]
    pub unsafe fn cmd_end_transform_feedback2(
        &self,
        command_buffer: CommandBuffer,
        first_counter_range: u32,
        counter_infos: Option<SliceOrLen<'_, BindTransformFeedbackBuffer2InfoEXT<'_>>>,
    ) {
        unsafe {
            (self.cmd_end_transform_feedback2.unwrap())(
                command_buffer,
                first_counter_range,
                counter_infos
                    .as_ref()
                    .map_or(0, SliceOrLen::len)
                    .try_into()
                    .unwrap(),
                counter_infos.to_raw_ptr(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawIndirectByteCount2EXT.html>
    #[inline]
    pub unsafe fn cmd_draw_indirect_byte_count2(
        &self,
        command_buffer: CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_info: &BindTransformFeedbackBuffer2InfoEXT<'_>,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_indirect_byte_count2.unwrap())(
                command_buffer,
                instance_count,
                first_instance,
                counter_info,
                counter_offset,
                vertex_stride,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawMeshTasksIndirect2EXT.html>
    #[inline]
    pub unsafe fn cmd_draw_mesh_tasks_indirect2(
        &self,
        command_buffer: CommandBuffer,
        info: &DrawIndirect2InfoKHR<'_>,
    ) {
        unsafe { (self.cmd_draw_mesh_tasks_indirect2.unwrap())(command_buffer, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawMeshTasksIndirectCount2EXT.html>
    #[inline]
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count2(
        &self,
        command_buffer: CommandBuffer,
        info: &DrawIndirectCount2InfoKHR<'_>,
    ) {
        unsafe { (self.cmd_draw_mesh_tasks_indirect_count2.unwrap())(command_buffer, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWriteMarkerToMemoryAMD.html>
    #[inline]
    pub unsafe fn cmd_write_marker_to_memory(
        &self,
        command_buffer: CommandBuffer,
        info: &MemoryMarkerInfoAMD<'_>,
    ) {
        unsafe { (self.cmd_write_marker_to_memory.unwrap())(command_buffer, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateAccelerationStructure2KHR.html>
    #[inline]
    pub unsafe fn create_acceleration_structure2(
        &self,
        device: Device,
        create_info: &AccelerationStructureCreateInfo2KHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<AccelerationStructureKHR> {
        unsafe {
            let mut acceleration_structure = core::mem::MaybeUninit::uninit();
            let result = (self.create_acceleration_structure2.unwrap())(
                device,
                create_info,
                allocator.to_raw_ptr(),
                acceleration_structure.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(acceleration_structure.assume_init()),
                err => Err(err),
            }
        }
    }
}
