//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_copy_memory_indirect.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_copy_memory_indirect";

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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyMemoryIndirectCommandKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct CopyMemoryIndirectCommandKHR {
        pub src_address: DeviceAddress,
        pub dst_address: DeviceAddress,
        pub size: DeviceSize,
    }

    impl CopyMemoryIndirectCommandKHR {
        #[inline]
        pub fn src_address(mut self, src_address: DeviceAddress) -> Self {
            self.src_address = src_address;
            self
        }

        #[inline]
        pub fn dst_address(mut self, dst_address: DeviceAddress) -> Self {
            self.dst_address = dst_address;
            self
        }

        #[inline]
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyMemoryIndirectInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CopyMemoryIndirectInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_copy_flags: AddressCopyFlagsKHR,
        pub dst_copy_flags: AddressCopyFlagsKHR,
        pub copy_count: u32,
        pub copy_address_range: StridedDeviceAddressRangeKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyMemoryIndirectInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyMemoryIndirectInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_copy_flags", &self.src_copy_flags)
                .field("dst_copy_flags", &self.dst_copy_flags)
                .field("copy_count", &self.copy_count)
                .field("copy_address_range", &self.copy_address_range)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyMemoryIndirectInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_MEMORY_INDIRECT_INFO_KHR;
    }

    impl Default for CopyMemoryIndirectInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_copy_flags: Default::default(),
                dst_copy_flags: Default::default(),
                copy_count: Default::default(),
                copy_address_range: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CopyMemoryIndirectInfoKHR<'a> {
        #[inline]
        pub fn src_copy_flags(mut self, src_copy_flags: AddressCopyFlagsKHR) -> Self {
            self.src_copy_flags = src_copy_flags;
            self
        }

        #[inline]
        pub fn dst_copy_flags(mut self, dst_copy_flags: AddressCopyFlagsKHR) -> Self {
            self.dst_copy_flags = dst_copy_flags;
            self
        }

        #[inline]
        pub fn copy_count(mut self, copy_count: u32) -> Self {
            self.copy_count = copy_count;
            self
        }

        #[inline]
        pub fn copy_address_range(
            mut self,
            copy_address_range: StridedDeviceAddressRangeKHR,
        ) -> Self {
            self.copy_address_range = copy_address_range;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyMemoryToImageIndirectCommandKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct CopyMemoryToImageIndirectCommandKHR {
        pub src_address: DeviceAddress,
        pub buffer_row_length: u32,
        pub buffer_image_height: u32,
        pub image_subresource: ImageSubresourceLayers,
        pub image_offset: Offset3D,
        pub image_extent: Extent3D,
    }

    impl CopyMemoryToImageIndirectCommandKHR {
        #[inline]
        pub fn src_address(mut self, src_address: DeviceAddress) -> Self {
            self.src_address = src_address;
            self
        }

        #[inline]
        pub fn buffer_row_length(mut self, buffer_row_length: u32) -> Self {
            self.buffer_row_length = buffer_row_length;
            self
        }

        #[inline]
        pub fn buffer_image_height(mut self, buffer_image_height: u32) -> Self {
            self.buffer_image_height = buffer_image_height;
            self
        }

        #[inline]
        pub fn image_subresource(mut self, image_subresource: ImageSubresourceLayers) -> Self {
            self.image_subresource = image_subresource;
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyMemoryToImageIndirectInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CopyMemoryToImageIndirectInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_copy_flags: AddressCopyFlagsKHR,
        pub copy_count: u32,
        pub copy_address_range: StridedDeviceAddressRangeKHR,
        pub dst_image: Image,
        pub dst_image_layout: ImageLayout,
        pub p_image_subresources: *const ImageSubresourceLayers,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyMemoryToImageIndirectInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyMemoryToImageIndirectInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_copy_flags", &self.src_copy_flags)
                .field("copy_count", &self.copy_count)
                .field("copy_address_range", &self.copy_address_range)
                .field("dst_image", &self.dst_image)
                .field("dst_image_layout", &self.dst_image_layout)
                .field("p_image_subresources", &self.p_image_subresources)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyMemoryToImageIndirectInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_MEMORY_TO_IMAGE_INDIRECT_INFO_KHR;
    }

    impl Default for CopyMemoryToImageIndirectInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_copy_flags: Default::default(),
                copy_count: Default::default(),
                copy_address_range: Default::default(),
                dst_image: Default::default(),
                dst_image_layout: Default::default(),
                p_image_subresources: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CopyMemoryToImageIndirectInfoKHR<'a> {
        #[inline]
        pub fn src_copy_flags(mut self, src_copy_flags: AddressCopyFlagsKHR) -> Self {
            self.src_copy_flags = src_copy_flags;
            self
        }

        #[inline]
        pub fn image_subresources(
            mut self,
            image_subresources: &'a [ImageSubresourceLayers],
        ) -> Self {
            self.copy_count = image_subresources.len().try_into().unwrap();
            self.p_image_subresources = image_subresources.as_ptr() as _;
            self
        }

        #[inline]
        pub fn copy_address_range(
            mut self,
            copy_address_range: StridedDeviceAddressRangeKHR,
        ) -> Self {
            self.copy_address_range = copy_address_range;
            self
        }

        #[inline]
        pub fn dst_image(mut self, dst_image: Image) -> Self {
            self.dst_image = dst_image;
            self
        }

        #[inline]
        pub fn dst_image_layout(mut self, dst_image_layout: ImageLayout) -> Self {
            self.dst_image_layout = dst_image_layout;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCopyMemoryIndirectFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCopyMemoryIndirectFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub indirect_memory_copy: Bool32,
        pub indirect_memory_to_image_copy: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCopyMemoryIndirectFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCopyMemoryIndirectFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("indirect_memory_copy", &self.indirect_memory_copy)
                .field(
                    "indirect_memory_to_image_copy",
                    &self.indirect_memory_to_image_copy,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCopyMemoryIndirectFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceCopyMemoryIndirectFeaturesKHR<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceCopyMemoryIndirectFeaturesKHR<'_> {}

    impl Default for PhysicalDeviceCopyMemoryIndirectFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                indirect_memory_copy: Default::default(),
                indirect_memory_to_image_copy: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCopyMemoryIndirectFeaturesKHR<'a> {
        #[inline]
        pub fn indirect_memory_copy(mut self, indirect_memory_copy: bool) -> Self {
            self.indirect_memory_copy = indirect_memory_copy.into();
            self
        }

        #[inline]
        pub fn indirect_memory_to_image_copy(
            mut self,
            indirect_memory_to_image_copy: bool,
        ) -> Self {
            self.indirect_memory_to_image_copy = indirect_memory_to_image_copy.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCopyMemoryIndirectPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCopyMemoryIndirectPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub supported_queues: QueueFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCopyMemoryIndirectPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCopyMemoryIndirectPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("supported_queues", &self.supported_queues)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCopyMemoryIndirectPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceCopyMemoryIndirectPropertiesKHR<'_>
    {
    }

    impl Default for PhysicalDeviceCopyMemoryIndirectPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                supported_queues: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCopyMemoryIndirectPropertiesKHR<'a> {
        #[inline]
        pub fn supported_queues(mut self, supported_queues: QueueFlags) -> Self {
            self.supported_queues = supported_queues;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAddressCopyFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct AddressCopyFlagsKHR(Flags);
    vk_bitflags_wrapped!(AddressCopyFlagsKHR, Flags, AddressCopyFlagBitsKHR);

    impl fmt::Debug for AddressCopyFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    AddressCopyFlagBitsKHR::DEVICE_LOCAL_KHR.0,
                    "DEVICE_LOCAL_KHR",
                ),
                (AddressCopyFlagBitsKHR::SPARSE_KHR.0, "SPARSE_KHR"),
                (AddressCopyFlagBitsKHR::PROTECTED_KHR.0, "PROTECTED_KHR"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAddressCopyFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct AddressCopyFlagBitsKHR(u32);

    impl AddressCopyFlagBitsKHR {
        pub const DEVICE_LOCAL_KHR: Self = Self(1 << 0);
        pub const SPARSE_KHR: Self = Self(1 << 1);
        pub const PROTECTED_KHR: Self = Self(1 << 2);
    }

    impl fmt::Debug for AddressCopyFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEVICE_LOCAL_KHR => Some("DEVICE_LOCAL_KHR"),
                Self::SPARSE_KHR => Some("SPARSE_KHR"),
                Self::PROTECTED_KHR => Some("PROTECTED_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryIndirectKHR.html>
    pub type PFN_vkCmdCopyMemoryIndirectKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_memory_indirect_info: *const CopyMemoryIndirectInfoKHR<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryToImageIndirectKHR.html>
    pub type PFN_vkCmdCopyMemoryToImageIndirectKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_memory_to_image_indirect_info: *const CopyMemoryToImageIndirectInfoKHR<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkStridedDeviceAddressRangeKHR = StridedDeviceAddressRangeKHR;
    pub type VkCopyMemoryIndirectCommandKHR = CopyMemoryIndirectCommandKHR;
    pub type VkCopyMemoryIndirectInfoKHR = CopyMemoryIndirectInfoKHR<'static>;
    pub type VkCopyMemoryToImageIndirectCommandKHR = CopyMemoryToImageIndirectCommandKHR;
    pub type VkCopyMemoryToImageIndirectInfoKHR = CopyMemoryToImageIndirectInfoKHR<'static>;
    pub type VkPhysicalDeviceCopyMemoryIndirectFeaturesKHR =
        PhysicalDeviceCopyMemoryIndirectFeaturesKHR<'static>;
    pub type VkPhysicalDeviceCopyMemoryIndirectPropertiesKHR =
        PhysicalDeviceCopyMemoryIndirectPropertiesKHR<'static>;
    pub type VkAddressCopyFlagsKHR = AddressCopyFlagsKHR;
    pub type VkAddressCopyFlagBitsKHR = AddressCopyFlagBitsKHR;
    impl CopyMemoryIndirectInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCopyMemoryIndirectInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CopyMemoryToImageIndirectInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCopyMemoryToImageIndirectInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceCopyMemoryIndirectFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceCopyMemoryIndirectFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceCopyMemoryIndirectPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceCopyMemoryIndirectPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    cmd_copy_memory_indirect: PFN_vkCmdCopyMemoryIndirectKHR,
    cmd_copy_memory_to_image_indirect: PFN_vkCmdCopyMemoryToImageIndirectKHR,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_copy_memory_indirect: transmute(
                    load(c"vkCmdCopyMemoryIndirectKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_memory_to_image_indirect: transmute(
                    load(c"vkCmdCopyMemoryToImageIndirectKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryIndirectKHR.html>
    #[inline]
    pub unsafe fn cmd_copy_memory_indirect(
        &self,
        command_buffer: CommandBuffer,
        copy_memory_indirect_info: &CopyMemoryIndirectInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_copy_memory_indirect)(command_buffer, copy_memory_indirect_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryToImageIndirectKHR.html>
    #[inline]
    pub unsafe fn cmd_copy_memory_to_image_indirect(
        &self,
        command_buffer: CommandBuffer,
        copy_memory_to_image_indirect_info: &CopyMemoryToImageIndirectInfoKHR<'_>,
    ) {
        unsafe {
            (self.cmd_copy_memory_to_image_indirect)(
                command_buffer,
                copy_memory_to_image_indirect_info,
            )
        }
    }
}
