#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const API_VERSION: ApiVersion = ApiVersion::new(0, 1, 3, 0);

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    pub type Flags64 = u64;

    handle_nondispatchable!(
        PrivateDataSlot,
        PRIVATE_DATA_SLOT,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkPrivateDataSlot.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDevicePrivateDataCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DevicePrivateDataCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub private_data_slot_request_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DevicePrivateDataCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DevicePrivateDataCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "private_data_slot_request_count",
                    &self.private_data_slot_request_count,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DevicePrivateDataCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_PRIVATE_DATA_CREATE_INFO;
    }

    unsafe impl Extends<DeviceCreateInfo<'_>> for DevicePrivateDataCreateInfo<'_> {}

    impl Default for DevicePrivateDataCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                private_data_slot_request_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DevicePrivateDataCreateInfo<'a> {
        #[inline]
        pub fn private_data_slot_request_count(
            mut self,
            private_data_slot_request_count: u32,
        ) -> Self {
            self.private_data_slot_request_count = private_data_slot_request_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPrivateDataSlotCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PrivateDataSlotCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PrivateDataSlotCreateFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PrivateDataSlotCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PrivateDataSlotCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PrivateDataSlotCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PRIVATE_DATA_SLOT_CREATE_INFO;
    }

    impl Default for PrivateDataSlotCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PrivateDataSlotCreateInfo<'a> {
        #[inline]
        pub fn flags(mut self, flags: PrivateDataSlotCreateFlags) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePrivateDataFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePrivateDataFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub private_data: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePrivateDataFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePrivateDataFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("private_data", &self.private_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePrivateDataFeatures<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDevicePrivateDataFeatures<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDevicePrivateDataFeatures<'_> {}

    impl Default for PhysicalDevicePrivateDataFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                private_data: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePrivateDataFeatures<'a> {
        #[inline]
        pub fn private_data(mut self, private_data: bool) -> Self {
            self.private_data = private_data.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceBufferMemoryRequirements.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceBufferMemoryRequirements<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_create_info: *const BufferCreateInfo<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceBufferMemoryRequirements<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceBufferMemoryRequirements")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_create_info", &self.p_create_info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceBufferMemoryRequirements<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_BUFFER_MEMORY_REQUIREMENTS;
    }

    impl Default for DeviceBufferMemoryRequirements<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_create_info: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceBufferMemoryRequirements<'a> {
        #[inline]
        pub fn create_info(mut self, create_info: &'a BufferCreateInfo<'a>) -> Self {
            self.p_create_info = create_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceImageMemoryRequirements.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceImageMemoryRequirements<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_create_info: *const ImageCreateInfo<'a>,
        pub plane_aspect: ImageAspectFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceImageMemoryRequirements<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceImageMemoryRequirements")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_create_info", &self.p_create_info)
                .field("plane_aspect", &self.plane_aspect)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceImageMemoryRequirements<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_IMAGE_MEMORY_REQUIREMENTS;
    }

    impl Default for DeviceImageMemoryRequirements<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_create_info: ptr::null(),
                plane_aspect: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceImageMemoryRequirements<'a> {
        #[inline]
        pub fn create_info(mut self, create_info: &'a ImageCreateInfo<'a>) -> Self {
            self.p_create_info = create_info;
            self
        }

        #[inline]
        pub fn plane_aspect(mut self, plane_aspect: ImageAspectFlagBits) -> Self {
            self.plane_aspect = plane_aspect;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceInlineUniformBlockFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceInlineUniformBlockFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub inline_uniform_block: Bool32,
        pub descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceInlineUniformBlockFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceInlineUniformBlockFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("inline_uniform_block", &self.inline_uniform_block)
                .field(
                    "descriptor_binding_inline_uniform_block_update_after_bind",
                    &self.descriptor_binding_inline_uniform_block_update_after_bind,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceInlineUniformBlockFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceInlineUniformBlockFeatures<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceInlineUniformBlockFeatures<'_> {}

    impl Default for PhysicalDeviceInlineUniformBlockFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                inline_uniform_block: Default::default(),
                descriptor_binding_inline_uniform_block_update_after_bind: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceInlineUniformBlockFeatures<'a> {
        #[inline]
        pub fn inline_uniform_block(mut self, inline_uniform_block: bool) -> Self {
            self.inline_uniform_block = inline_uniform_block.into();
            self
        }

        #[inline]
        pub fn descriptor_binding_inline_uniform_block_update_after_bind(
            mut self,
            descriptor_binding_inline_uniform_block_update_after_bind: bool,
        ) -> Self {
            self.descriptor_binding_inline_uniform_block_update_after_bind =
                descriptor_binding_inline_uniform_block_update_after_bind.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceInlineUniformBlockProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceInlineUniformBlockProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_inline_uniform_block_size: u32,
        pub max_per_stage_descriptor_inline_uniform_blocks: u32,
        pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
        pub max_descriptor_set_inline_uniform_blocks: u32,
        pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceInlineUniformBlockProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceInlineUniformBlockProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_inline_uniform_block_size",
                    &self.max_inline_uniform_block_size,
                )
                .field(
                    "max_per_stage_descriptor_inline_uniform_blocks",
                    &self.max_per_stage_descriptor_inline_uniform_blocks,
                )
                .field(
                    "max_per_stage_descriptor_update_after_bind_inline_uniform_blocks",
                    &self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks,
                )
                .field(
                    "max_descriptor_set_inline_uniform_blocks",
                    &self.max_descriptor_set_inline_uniform_blocks,
                )
                .field(
                    "max_descriptor_set_update_after_bind_inline_uniform_blocks",
                    &self.max_descriptor_set_update_after_bind_inline_uniform_blocks,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceInlineUniformBlockProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceInlineUniformBlockProperties<'_>
    {
    }

    impl Default for PhysicalDeviceInlineUniformBlockProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_inline_uniform_block_size: Default::default(),
                max_per_stage_descriptor_inline_uniform_blocks: Default::default(),
                max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: Default::default(
                ),
                max_descriptor_set_inline_uniform_blocks: Default::default(),
                max_descriptor_set_update_after_bind_inline_uniform_blocks: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceInlineUniformBlockProperties<'a> {
        #[inline]
        pub fn max_inline_uniform_block_size(mut self, max_inline_uniform_block_size: u32) -> Self {
            self.max_inline_uniform_block_size = max_inline_uniform_block_size;
            self
        }

        #[inline]
        pub fn max_per_stage_descriptor_inline_uniform_blocks(
            mut self,
            max_per_stage_descriptor_inline_uniform_blocks: u32,
        ) -> Self {
            self.max_per_stage_descriptor_inline_uniform_blocks =
                max_per_stage_descriptor_inline_uniform_blocks;
            self
        }

        #[inline]
        pub fn max_per_stage_descriptor_update_after_bind_inline_uniform_blocks(
            mut self,
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks =
                max_per_stage_descriptor_update_after_bind_inline_uniform_blocks;
            self
        }

        #[inline]
        pub fn max_descriptor_set_inline_uniform_blocks(
            mut self,
            max_descriptor_set_inline_uniform_blocks: u32,
        ) -> Self {
            self.max_descriptor_set_inline_uniform_blocks =
                max_descriptor_set_inline_uniform_blocks;
            self
        }

        #[inline]
        pub fn max_descriptor_set_update_after_bind_inline_uniform_blocks(
            mut self,
            max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_inline_uniform_blocks =
                max_descriptor_set_update_after_bind_inline_uniform_blocks;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkWriteDescriptorSetInlineUniformBlock.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct WriteDescriptorSetInlineUniformBlock<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub data_size: u32,
        pub p_data: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for WriteDescriptorSetInlineUniformBlock<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("WriteDescriptorSetInlineUniformBlock")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("data_size", &self.data_size)
                .field("p_data", &self.p_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for WriteDescriptorSetInlineUniformBlock<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK;
    }

    unsafe impl Extends<WriteDescriptorSet<'_>> for WriteDescriptorSetInlineUniformBlock<'_> {}

    impl Default for WriteDescriptorSetInlineUniformBlock<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                data_size: Default::default(),
                p_data: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> WriteDescriptorSetInlineUniformBlock<'a> {
        #[inline]
        pub fn data(mut self, data: &'a [u8]) -> Self {
            self.data_size = data.len().try_into().unwrap();
            self.p_data = data.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorPoolInlineUniformBlockCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DescriptorPoolInlineUniformBlockCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub max_inline_uniform_block_bindings: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorPoolInlineUniformBlockCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorPoolInlineUniformBlockCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_inline_uniform_block_bindings",
                    &self.max_inline_uniform_block_bindings,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DescriptorPoolInlineUniformBlockCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO;
    }

    unsafe impl Extends<DescriptorPoolCreateInfo<'_>>
        for DescriptorPoolInlineUniformBlockCreateInfo<'_>
    {
    }

    impl Default for DescriptorPoolInlineUniformBlockCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                max_inline_uniform_block_bindings: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DescriptorPoolInlineUniformBlockCreateInfo<'a> {
        #[inline]
        pub fn max_inline_uniform_block_bindings(
            mut self,
            max_inline_uniform_block_bindings: u32,
        ) -> Self {
            self.max_inline_uniform_block_bindings = max_inline_uniform_block_bindings;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance4Features.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMaintenance4Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub maintenance4: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMaintenance4Features<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMaintenance4Features")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("maintenance4", &self.maintenance4)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance4Features<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceMaintenance4Features<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceMaintenance4Features<'_> {}

    impl Default for PhysicalDeviceMaintenance4Features<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                maintenance4: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMaintenance4Features<'a> {
        #[inline]
        pub fn maintenance4(mut self, maintenance4: bool) -> Self {
            self.maintenance4 = maintenance4.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance4Properties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMaintenance4Properties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_buffer_size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMaintenance4Properties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMaintenance4Properties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_buffer_size", &self.max_buffer_size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance4Properties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>> for PhysicalDeviceMaintenance4Properties<'_> {}

    impl Default for PhysicalDeviceMaintenance4Properties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_buffer_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMaintenance4Properties<'a> {
        #[inline]
        pub fn max_buffer_size(mut self, max_buffer_size: DeviceSize) -> Self {
            self.max_buffer_size = max_buffer_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceTextureCompressionASTCHDRFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceTextureCompressionASTCHDRFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub texture_compression_astc_hdr: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceTextureCompressionASTCHDRFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceTextureCompressionASTCHDRFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "texture_compression_astc_hdr",
                    &self.texture_compression_astc_hdr,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTextureCompressionASTCHDRFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceTextureCompressionASTCHDRFeatures<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceTextureCompressionASTCHDRFeatures<'_> {}

    impl Default for PhysicalDeviceTextureCompressionASTCHDRFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                texture_compression_astc_hdr: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceTextureCompressionASTCHDRFeatures<'a> {
        #[inline]
        pub fn texture_compression_astc_hdr(mut self, texture_compression_astc_hdr: bool) -> Self {
            self.texture_compression_astc_hdr = texture_compression_astc_hdr.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCreationFeedback.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct PipelineCreationFeedback {
        pub flags: PipelineCreationFeedbackFlags,
        pub duration: u64,
    }

    impl PipelineCreationFeedback {
        #[inline]
        pub fn flags(mut self, flags: PipelineCreationFeedbackFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn duration(mut self, duration: u64) -> Self {
            self.duration = duration;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCreationFeedbackCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineCreationFeedbackCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_pipeline_creation_feedback: *mut PipelineCreationFeedback,
        pub pipeline_stage_creation_feedback_count: u32,
        pub p_pipeline_stage_creation_feedbacks: *mut PipelineCreationFeedback,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineCreationFeedbackCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineCreationFeedbackCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "p_pipeline_creation_feedback",
                    &self.p_pipeline_creation_feedback,
                )
                .field(
                    "pipeline_stage_creation_feedback_count",
                    &self.pipeline_stage_creation_feedback_count,
                )
                .field(
                    "p_pipeline_stage_creation_feedbacks",
                    &self.p_pipeline_stage_creation_feedbacks,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineCreationFeedbackCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_CREATION_FEEDBACK_CREATE_INFO;
    }

    unsafe impl Extends<GraphicsPipelineCreateInfo<'_>> for PipelineCreationFeedbackCreateInfo<'_> {}
    unsafe impl Extends<ComputePipelineCreateInfo<'_>> for PipelineCreationFeedbackCreateInfo<'_> {}
    unsafe impl Extends<RayTracingPipelineCreateInfoNV<'_>> for PipelineCreationFeedbackCreateInfo<'_> {}
    unsafe impl Extends<RayTracingPipelineCreateInfoKHR<'_>>
        for PipelineCreationFeedbackCreateInfo<'_>
    {
    }
    #[cfg(feature = "provisional")]
    unsafe impl Extends<ExecutionGraphPipelineCreateInfoAMDX<'_>>
        for PipelineCreationFeedbackCreateInfo<'_>
    {
    }
    unsafe impl Extends<DataGraphPipelineCreateInfoARM<'_>> for PipelineCreationFeedbackCreateInfo<'_> {}

    impl Default for PipelineCreationFeedbackCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_pipeline_creation_feedback: ptr::null_mut(),
                pipeline_stage_creation_feedback_count: Default::default(),
                p_pipeline_stage_creation_feedbacks: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineCreationFeedbackCreateInfo<'a> {
        #[inline]
        pub fn pipeline_creation_feedback(
            mut self,
            pipeline_creation_feedback: &'a mut PipelineCreationFeedback,
        ) -> Self {
            self.p_pipeline_creation_feedback = pipeline_creation_feedback;
            self
        }

        #[inline]
        pub fn pipeline_stage_creation_feedbacks(
            mut self,
            pipeline_stage_creation_feedbacks: &'a mut [PipelineCreationFeedback],
        ) -> Self {
            self.pipeline_stage_creation_feedback_count =
                pipeline_stage_creation_feedbacks.len().try_into().unwrap();
            self.p_pipeline_stage_creation_feedbacks =
                pipeline_stage_creation_feedbacks.as_mut_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_demote_to_helper_invocation: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderDemoteToHelperInvocationFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "shader_demote_to_helper_invocation",
                    &self.shader_demote_to_helper_invocation,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'_>
    {
    }

    impl Default for PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_demote_to_helper_invocation: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'a> {
        #[inline]
        pub fn shader_demote_to_helper_invocation(
            mut self,
            shader_demote_to_helper_invocation: bool,
        ) -> Self {
            self.shader_demote_to_helper_invocation = shader_demote_to_helper_invocation.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceTexelBufferAlignmentProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceTexelBufferAlignmentProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub storage_texel_buffer_offset_alignment_bytes: DeviceSize,
        pub storage_texel_buffer_offset_single_texel_alignment: Bool32,
        pub uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
        pub uniform_texel_buffer_offset_single_texel_alignment: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceTexelBufferAlignmentProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceTexelBufferAlignmentProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "storage_texel_buffer_offset_alignment_bytes",
                    &self.storage_texel_buffer_offset_alignment_bytes,
                )
                .field(
                    "storage_texel_buffer_offset_single_texel_alignment",
                    &self.storage_texel_buffer_offset_single_texel_alignment,
                )
                .field(
                    "uniform_texel_buffer_offset_alignment_bytes",
                    &self.uniform_texel_buffer_offset_alignment_bytes,
                )
                .field(
                    "uniform_texel_buffer_offset_single_texel_alignment",
                    &self.uniform_texel_buffer_offset_single_texel_alignment,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTexelBufferAlignmentProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceTexelBufferAlignmentProperties<'_>
    {
    }

    impl Default for PhysicalDeviceTexelBufferAlignmentProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                storage_texel_buffer_offset_alignment_bytes: Default::default(),
                storage_texel_buffer_offset_single_texel_alignment: Default::default(),
                uniform_texel_buffer_offset_alignment_bytes: Default::default(),
                uniform_texel_buffer_offset_single_texel_alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceTexelBufferAlignmentProperties<'a> {
        #[inline]
        pub fn storage_texel_buffer_offset_alignment_bytes(
            mut self,
            storage_texel_buffer_offset_alignment_bytes: DeviceSize,
        ) -> Self {
            self.storage_texel_buffer_offset_alignment_bytes =
                storage_texel_buffer_offset_alignment_bytes;
            self
        }

        #[inline]
        pub fn storage_texel_buffer_offset_single_texel_alignment(
            mut self,
            storage_texel_buffer_offset_single_texel_alignment: bool,
        ) -> Self {
            self.storage_texel_buffer_offset_single_texel_alignment =
                storage_texel_buffer_offset_single_texel_alignment.into();
            self
        }

        #[inline]
        pub fn uniform_texel_buffer_offset_alignment_bytes(
            mut self,
            uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
        ) -> Self {
            self.uniform_texel_buffer_offset_alignment_bytes =
                uniform_texel_buffer_offset_alignment_bytes;
            self
        }

        #[inline]
        pub fn uniform_texel_buffer_offset_single_texel_alignment(
            mut self,
            uniform_texel_buffer_offset_single_texel_alignment: bool,
        ) -> Self {
            self.uniform_texel_buffer_offset_single_texel_alignment =
                uniform_texel_buffer_offset_single_texel_alignment.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSubgroupSizeControlFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceSubgroupSizeControlFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub subgroup_size_control: Bool32,
        pub compute_full_subgroups: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceSubgroupSizeControlFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceSubgroupSizeControlFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("subgroup_size_control", &self.subgroup_size_control)
                .field("compute_full_subgroups", &self.compute_full_subgroups)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSubgroupSizeControlFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceSubgroupSizeControlFeatures<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceSubgroupSizeControlFeatures<'_> {}

    impl Default for PhysicalDeviceSubgroupSizeControlFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                subgroup_size_control: Default::default(),
                compute_full_subgroups: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceSubgroupSizeControlFeatures<'a> {
        #[inline]
        pub fn subgroup_size_control(mut self, subgroup_size_control: bool) -> Self {
            self.subgroup_size_control = subgroup_size_control.into();
            self
        }

        #[inline]
        pub fn compute_full_subgroups(mut self, compute_full_subgroups: bool) -> Self {
            self.compute_full_subgroups = compute_full_subgroups.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSubgroupSizeControlProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceSubgroupSizeControlProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_subgroup_size: u32,
        pub max_subgroup_size: u32,
        pub max_compute_workgroup_subgroups: u32,
        pub required_subgroup_size_stages: ShaderStageFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceSubgroupSizeControlProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceSubgroupSizeControlProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("min_subgroup_size", &self.min_subgroup_size)
                .field("max_subgroup_size", &self.max_subgroup_size)
                .field(
                    "max_compute_workgroup_subgroups",
                    &self.max_compute_workgroup_subgroups,
                )
                .field(
                    "required_subgroup_size_stages",
                    &self.required_subgroup_size_stages,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSubgroupSizeControlProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceSubgroupSizeControlProperties<'_>
    {
    }

    impl Default for PhysicalDeviceSubgroupSizeControlProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                min_subgroup_size: Default::default(),
                max_subgroup_size: Default::default(),
                max_compute_workgroup_subgroups: Default::default(),
                required_subgroup_size_stages: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceSubgroupSizeControlProperties<'a> {
        #[inline]
        pub fn min_subgroup_size(mut self, min_subgroup_size: u32) -> Self {
            self.min_subgroup_size = min_subgroup_size;
            self
        }

        #[inline]
        pub fn max_subgroup_size(mut self, max_subgroup_size: u32) -> Self {
            self.max_subgroup_size = max_subgroup_size;
            self
        }

        #[inline]
        pub fn max_compute_workgroup_subgroups(
            mut self,
            max_compute_workgroup_subgroups: u32,
        ) -> Self {
            self.max_compute_workgroup_subgroups = max_compute_workgroup_subgroups;
            self
        }

        #[inline]
        pub fn required_subgroup_size_stages(
            mut self,
            required_subgroup_size_stages: ShaderStageFlags,
        ) -> Self {
            self.required_subgroup_size_stages = required_subgroup_size_stages;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineShaderStageRequiredSubgroupSizeCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub required_subgroup_size: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineShaderStageRequiredSubgroupSizeCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineShaderStageRequiredSubgroupSizeCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("required_subgroup_size", &self.required_subgroup_size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO;
    }

    unsafe impl Extends<PipelineShaderStageCreateInfo<'_>>
        for PipelineShaderStageRequiredSubgroupSizeCreateInfo<'_>
    {
    }
    unsafe impl Extends<ShaderCreateInfoEXT<'_>>
        for PipelineShaderStageRequiredSubgroupSizeCreateInfo<'_>
    {
    }

    impl Default for PipelineShaderStageRequiredSubgroupSizeCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                required_subgroup_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a> {
        #[inline]
        pub fn required_subgroup_size(mut self, required_subgroup_size: u32) -> Self {
            self.required_subgroup_size = required_subgroup_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePipelineCreationCacheControlFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePipelineCreationCacheControlFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_creation_cache_control: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePipelineCreationCacheControlFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePipelineCreationCacheControlFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "pipeline_creation_cache_control",
                    &self.pipeline_creation_cache_control,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePipelineCreationCacheControlFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDevicePipelineCreationCacheControlFeatures<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDevicePipelineCreationCacheControlFeatures<'_>
    {
    }

    impl Default for PhysicalDevicePipelineCreationCacheControlFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                pipeline_creation_cache_control: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePipelineCreationCacheControlFeatures<'a> {
        #[inline]
        pub fn pipeline_creation_cache_control(
            mut self,
            pipeline_creation_cache_control: bool,
        ) -> Self {
            self.pipeline_creation_cache_control = pipeline_creation_cache_control.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVulkan13Features.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceVulkan13Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub robust_image_access: Bool32,
        pub inline_uniform_block: Bool32,
        pub descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
        pub pipeline_creation_cache_control: Bool32,
        pub private_data: Bool32,
        pub shader_demote_to_helper_invocation: Bool32,
        pub shader_terminate_invocation: Bool32,
        pub subgroup_size_control: Bool32,
        pub compute_full_subgroups: Bool32,
        pub synchronization2: Bool32,
        pub texture_compression_astc_hdr: Bool32,
        pub shader_zero_initialize_workgroup_memory: Bool32,
        pub dynamic_rendering: Bool32,
        pub shader_integer_dot_product: Bool32,
        pub maintenance4: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceVulkan13Features<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVulkan13Features")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("robust_image_access", &self.robust_image_access)
                .field("inline_uniform_block", &self.inline_uniform_block)
                .field(
                    "descriptor_binding_inline_uniform_block_update_after_bind",
                    &self.descriptor_binding_inline_uniform_block_update_after_bind,
                )
                .field(
                    "pipeline_creation_cache_control",
                    &self.pipeline_creation_cache_control,
                )
                .field("private_data", &self.private_data)
                .field(
                    "shader_demote_to_helper_invocation",
                    &self.shader_demote_to_helper_invocation,
                )
                .field(
                    "shader_terminate_invocation",
                    &self.shader_terminate_invocation,
                )
                .field("subgroup_size_control", &self.subgroup_size_control)
                .field("compute_full_subgroups", &self.compute_full_subgroups)
                .field("synchronization2", &self.synchronization2)
                .field(
                    "texture_compression_astc_hdr",
                    &self.texture_compression_astc_hdr,
                )
                .field(
                    "shader_zero_initialize_workgroup_memory",
                    &self.shader_zero_initialize_workgroup_memory,
                )
                .field("dynamic_rendering", &self.dynamic_rendering)
                .field(
                    "shader_integer_dot_product",
                    &self.shader_integer_dot_product,
                )
                .field("maintenance4", &self.maintenance4)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVulkan13Features<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VULKAN_1_3_FEATURES;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceVulkan13Features<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceVulkan13Features<'_> {}

    impl Default for PhysicalDeviceVulkan13Features<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                robust_image_access: Default::default(),
                inline_uniform_block: Default::default(),
                descriptor_binding_inline_uniform_block_update_after_bind: Default::default(),
                pipeline_creation_cache_control: Default::default(),
                private_data: Default::default(),
                shader_demote_to_helper_invocation: Default::default(),
                shader_terminate_invocation: Default::default(),
                subgroup_size_control: Default::default(),
                compute_full_subgroups: Default::default(),
                synchronization2: Default::default(),
                texture_compression_astc_hdr: Default::default(),
                shader_zero_initialize_workgroup_memory: Default::default(),
                dynamic_rendering: Default::default(),
                shader_integer_dot_product: Default::default(),
                maintenance4: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceVulkan13Features<'a> {
        #[inline]
        pub fn robust_image_access(mut self, robust_image_access: bool) -> Self {
            self.robust_image_access = robust_image_access.into();
            self
        }

        #[inline]
        pub fn inline_uniform_block(mut self, inline_uniform_block: bool) -> Self {
            self.inline_uniform_block = inline_uniform_block.into();
            self
        }

        #[inline]
        pub fn descriptor_binding_inline_uniform_block_update_after_bind(
            mut self,
            descriptor_binding_inline_uniform_block_update_after_bind: bool,
        ) -> Self {
            self.descriptor_binding_inline_uniform_block_update_after_bind =
                descriptor_binding_inline_uniform_block_update_after_bind.into();
            self
        }

        #[inline]
        pub fn pipeline_creation_cache_control(
            mut self,
            pipeline_creation_cache_control: bool,
        ) -> Self {
            self.pipeline_creation_cache_control = pipeline_creation_cache_control.into();
            self
        }

        #[inline]
        pub fn private_data(mut self, private_data: bool) -> Self {
            self.private_data = private_data.into();
            self
        }

        #[inline]
        pub fn shader_demote_to_helper_invocation(
            mut self,
            shader_demote_to_helper_invocation: bool,
        ) -> Self {
            self.shader_demote_to_helper_invocation = shader_demote_to_helper_invocation.into();
            self
        }

        #[inline]
        pub fn shader_terminate_invocation(mut self, shader_terminate_invocation: bool) -> Self {
            self.shader_terminate_invocation = shader_terminate_invocation.into();
            self
        }

        #[inline]
        pub fn subgroup_size_control(mut self, subgroup_size_control: bool) -> Self {
            self.subgroup_size_control = subgroup_size_control.into();
            self
        }

        #[inline]
        pub fn compute_full_subgroups(mut self, compute_full_subgroups: bool) -> Self {
            self.compute_full_subgroups = compute_full_subgroups.into();
            self
        }

        #[inline]
        pub fn synchronization2(mut self, synchronization2: bool) -> Self {
            self.synchronization2 = synchronization2.into();
            self
        }

        #[inline]
        pub fn texture_compression_astc_hdr(mut self, texture_compression_astc_hdr: bool) -> Self {
            self.texture_compression_astc_hdr = texture_compression_astc_hdr.into();
            self
        }

        #[inline]
        pub fn shader_zero_initialize_workgroup_memory(
            mut self,
            shader_zero_initialize_workgroup_memory: bool,
        ) -> Self {
            self.shader_zero_initialize_workgroup_memory =
                shader_zero_initialize_workgroup_memory.into();
            self
        }

        #[inline]
        pub fn dynamic_rendering(mut self, dynamic_rendering: bool) -> Self {
            self.dynamic_rendering = dynamic_rendering.into();
            self
        }

        #[inline]
        pub fn shader_integer_dot_product(mut self, shader_integer_dot_product: bool) -> Self {
            self.shader_integer_dot_product = shader_integer_dot_product.into();
            self
        }

        #[inline]
        pub fn maintenance4(mut self, maintenance4: bool) -> Self {
            self.maintenance4 = maintenance4.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVulkan13Properties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceVulkan13Properties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_subgroup_size: u32,
        pub max_subgroup_size: u32,
        pub max_compute_workgroup_subgroups: u32,
        pub required_subgroup_size_stages: ShaderStageFlags,
        pub max_inline_uniform_block_size: u32,
        pub max_per_stage_descriptor_inline_uniform_blocks: u32,
        pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
        pub max_descriptor_set_inline_uniform_blocks: u32,
        pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
        pub max_inline_uniform_total_size: u32,
        pub integer_dot_product8_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product8_bit_signed_accelerated: Bool32,
        pub integer_dot_product8_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product4x8_bit_packed_unsigned_accelerated: Bool32,
        pub integer_dot_product4x8_bit_packed_signed_accelerated: Bool32,
        pub integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product16_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product16_bit_signed_accelerated: Bool32,
        pub integer_dot_product16_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product32_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product32_bit_signed_accelerated: Bool32,
        pub integer_dot_product32_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product64_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product64_bit_signed_accelerated: Bool32,
        pub integer_dot_product64_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating8_bit_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated:
            Bool32,
        pub integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating16_bit_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating32_bit_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating64_bit_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: Bool32,
        pub storage_texel_buffer_offset_alignment_bytes: DeviceSize,
        pub storage_texel_buffer_offset_single_texel_alignment: Bool32,
        pub uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
        pub uniform_texel_buffer_offset_single_texel_alignment: Bool32,
        pub max_buffer_size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceVulkan13Properties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVulkan13Properties")
.field("s_type", &self.s_type)
.field("p_next", &self.p_next)
.field("min_subgroup_size", &self.min_subgroup_size)
.field("max_subgroup_size", &self.max_subgroup_size)
.field("max_compute_workgroup_subgroups", &self.max_compute_workgroup_subgroups)
.field("required_subgroup_size_stages", &self.required_subgroup_size_stages)
.field("max_inline_uniform_block_size", &self.max_inline_uniform_block_size)
.field("max_per_stage_descriptor_inline_uniform_blocks", &self.max_per_stage_descriptor_inline_uniform_blocks)
.field("max_per_stage_descriptor_update_after_bind_inline_uniform_blocks", &self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks)
.field("max_descriptor_set_inline_uniform_blocks", &self.max_descriptor_set_inline_uniform_blocks)
.field("max_descriptor_set_update_after_bind_inline_uniform_blocks", &self.max_descriptor_set_update_after_bind_inline_uniform_blocks)
.field("max_inline_uniform_total_size", &self.max_inline_uniform_total_size)
.field("integer_dot_product8_bit_unsigned_accelerated", &self.integer_dot_product8_bit_unsigned_accelerated)
.field("integer_dot_product8_bit_signed_accelerated", &self.integer_dot_product8_bit_signed_accelerated)
.field("integer_dot_product8_bit_mixed_signedness_accelerated", &self.integer_dot_product8_bit_mixed_signedness_accelerated)
.field("integer_dot_product4x8_bit_packed_unsigned_accelerated", &self.integer_dot_product4x8_bit_packed_unsigned_accelerated)
.field("integer_dot_product4x8_bit_packed_signed_accelerated", &self.integer_dot_product4x8_bit_packed_signed_accelerated)
.field("integer_dot_product4x8_bit_packed_mixed_signedness_accelerated", &self.integer_dot_product4x8_bit_packed_mixed_signedness_accelerated)
.field("integer_dot_product16_bit_unsigned_accelerated", &self.integer_dot_product16_bit_unsigned_accelerated)
.field("integer_dot_product16_bit_signed_accelerated", &self.integer_dot_product16_bit_signed_accelerated)
.field("integer_dot_product16_bit_mixed_signedness_accelerated", &self.integer_dot_product16_bit_mixed_signedness_accelerated)
.field("integer_dot_product32_bit_unsigned_accelerated", &self.integer_dot_product32_bit_unsigned_accelerated)
.field("integer_dot_product32_bit_signed_accelerated", &self.integer_dot_product32_bit_signed_accelerated)
.field("integer_dot_product32_bit_mixed_signedness_accelerated", &self.integer_dot_product32_bit_mixed_signedness_accelerated)
.field("integer_dot_product64_bit_unsigned_accelerated", &self.integer_dot_product64_bit_unsigned_accelerated)
.field("integer_dot_product64_bit_signed_accelerated", &self.integer_dot_product64_bit_signed_accelerated)
.field("integer_dot_product64_bit_mixed_signedness_accelerated", &self.integer_dot_product64_bit_mixed_signedness_accelerated)
.field("integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated", &self.integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated)
.field("integer_dot_product_accumulating_saturating8_bit_signed_accelerated", &self.integer_dot_product_accumulating_saturating8_bit_signed_accelerated)
.field("integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated", &self.integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated)
.field("integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated", &self.integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated)
.field("integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated", &self.integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated)
.field("integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated", &self.integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated)
.field("integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated", &self.integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated)
.field("integer_dot_product_accumulating_saturating16_bit_signed_accelerated", &self.integer_dot_product_accumulating_saturating16_bit_signed_accelerated)
.field("integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated", &self.integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated)
.field("integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated", &self.integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated)
.field("integer_dot_product_accumulating_saturating32_bit_signed_accelerated", &self.integer_dot_product_accumulating_saturating32_bit_signed_accelerated)
.field("integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated", &self.integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated)
.field("integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated", &self.integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated)
.field("integer_dot_product_accumulating_saturating64_bit_signed_accelerated", &self.integer_dot_product_accumulating_saturating64_bit_signed_accelerated)
.field("integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated", &self.integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated)
.field("storage_texel_buffer_offset_alignment_bytes", &self.storage_texel_buffer_offset_alignment_bytes)
.field("storage_texel_buffer_offset_single_texel_alignment", &self.storage_texel_buffer_offset_single_texel_alignment)
.field("uniform_texel_buffer_offset_alignment_bytes", &self.uniform_texel_buffer_offset_alignment_bytes)
.field("uniform_texel_buffer_offset_single_texel_alignment", &self.uniform_texel_buffer_offset_single_texel_alignment)
.field("max_buffer_size", &self.max_buffer_size)
.finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVulkan13Properties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>> for PhysicalDeviceVulkan13Properties<'_> {}

    impl Default for PhysicalDeviceVulkan13Properties<'_> {
        fn default() -> Self {
            Self {
s_type: Self::STRUCTURE_TYPE
,
p_next: ptr::null_mut(),
min_subgroup_size: Default::default(),
max_subgroup_size: Default::default(),
max_compute_workgroup_subgroups: Default::default(),
required_subgroup_size_stages: Default::default(),
max_inline_uniform_block_size: Default::default(),
max_per_stage_descriptor_inline_uniform_blocks: Default::default(),
max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: Default::default(),
max_descriptor_set_inline_uniform_blocks: Default::default(),
max_descriptor_set_update_after_bind_inline_uniform_blocks: Default::default(),
max_inline_uniform_total_size: Default::default(),
integer_dot_product8_bit_unsigned_accelerated: Default::default(),
integer_dot_product8_bit_signed_accelerated: Default::default(),
integer_dot_product8_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product4x8_bit_packed_unsigned_accelerated: Default::default(),
integer_dot_product4x8_bit_packed_signed_accelerated: Default::default(),
integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: Default::default(),
integer_dot_product16_bit_unsigned_accelerated: Default::default(),
integer_dot_product16_bit_signed_accelerated: Default::default(),
integer_dot_product16_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product32_bit_unsigned_accelerated: Default::default(),
integer_dot_product32_bit_signed_accelerated: Default::default(),
integer_dot_product32_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product64_bit_unsigned_accelerated: Default::default(),
integer_dot_product64_bit_signed_accelerated: Default::default(),
integer_dot_product64_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating8_bit_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating16_bit_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating32_bit_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating64_bit_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: Default::default(),
storage_texel_buffer_offset_alignment_bytes: Default::default(),
storage_texel_buffer_offset_single_texel_alignment: Default::default(),
uniform_texel_buffer_offset_alignment_bytes: Default::default(),
uniform_texel_buffer_offset_single_texel_alignment: Default::default(),
max_buffer_size: Default::default(),
_marker: PhantomData
}
        }
    }

    impl<'a> PhysicalDeviceVulkan13Properties<'a> {
        #[inline]
        pub fn min_subgroup_size(mut self, min_subgroup_size: u32) -> Self {
            self.min_subgroup_size = min_subgroup_size;
            self
        }

        #[inline]
        pub fn max_subgroup_size(mut self, max_subgroup_size: u32) -> Self {
            self.max_subgroup_size = max_subgroup_size;
            self
        }

        #[inline]
        pub fn max_compute_workgroup_subgroups(
            mut self,
            max_compute_workgroup_subgroups: u32,
        ) -> Self {
            self.max_compute_workgroup_subgroups = max_compute_workgroup_subgroups;
            self
        }

        #[inline]
        pub fn required_subgroup_size_stages(
            mut self,
            required_subgroup_size_stages: ShaderStageFlags,
        ) -> Self {
            self.required_subgroup_size_stages = required_subgroup_size_stages;
            self
        }

        #[inline]
        pub fn max_inline_uniform_block_size(mut self, max_inline_uniform_block_size: u32) -> Self {
            self.max_inline_uniform_block_size = max_inline_uniform_block_size;
            self
        }

        #[inline]
        pub fn max_per_stage_descriptor_inline_uniform_blocks(
            mut self,
            max_per_stage_descriptor_inline_uniform_blocks: u32,
        ) -> Self {
            self.max_per_stage_descriptor_inline_uniform_blocks =
                max_per_stage_descriptor_inline_uniform_blocks;
            self
        }

        #[inline]
        pub fn max_per_stage_descriptor_update_after_bind_inline_uniform_blocks(
            mut self,
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks =
                max_per_stage_descriptor_update_after_bind_inline_uniform_blocks;
            self
        }

        #[inline]
        pub fn max_descriptor_set_inline_uniform_blocks(
            mut self,
            max_descriptor_set_inline_uniform_blocks: u32,
        ) -> Self {
            self.max_descriptor_set_inline_uniform_blocks =
                max_descriptor_set_inline_uniform_blocks;
            self
        }

        #[inline]
        pub fn max_descriptor_set_update_after_bind_inline_uniform_blocks(
            mut self,
            max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_inline_uniform_blocks =
                max_descriptor_set_update_after_bind_inline_uniform_blocks;
            self
        }

        #[inline]
        pub fn max_inline_uniform_total_size(mut self, max_inline_uniform_total_size: u32) -> Self {
            self.max_inline_uniform_total_size = max_inline_uniform_total_size;
            self
        }

        #[inline]
        pub fn integer_dot_product8_bit_unsigned_accelerated(
            mut self,
            integer_dot_product8_bit_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product8_bit_unsigned_accelerated =
                integer_dot_product8_bit_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product8_bit_signed_accelerated(
            mut self,
            integer_dot_product8_bit_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product8_bit_signed_accelerated =
                integer_dot_product8_bit_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product8_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product8_bit_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product8_bit_mixed_signedness_accelerated =
                integer_dot_product8_bit_mixed_signedness_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product4x8_bit_packed_unsigned_accelerated(
            mut self,
            integer_dot_product4x8_bit_packed_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product4x8_bit_packed_unsigned_accelerated =
                integer_dot_product4x8_bit_packed_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product4x8_bit_packed_signed_accelerated(
            mut self,
            integer_dot_product4x8_bit_packed_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product4x8_bit_packed_signed_accelerated =
                integer_dot_product4x8_bit_packed_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product4x8_bit_packed_mixed_signedness_accelerated(
            mut self,
            integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product4x8_bit_packed_mixed_signedness_accelerated =
                integer_dot_product4x8_bit_packed_mixed_signedness_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product16_bit_unsigned_accelerated(
            mut self,
            integer_dot_product16_bit_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product16_bit_unsigned_accelerated =
                integer_dot_product16_bit_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product16_bit_signed_accelerated(
            mut self,
            integer_dot_product16_bit_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product16_bit_signed_accelerated =
                integer_dot_product16_bit_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product16_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product16_bit_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product16_bit_mixed_signedness_accelerated =
                integer_dot_product16_bit_mixed_signedness_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product32_bit_unsigned_accelerated(
            mut self,
            integer_dot_product32_bit_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product32_bit_unsigned_accelerated =
                integer_dot_product32_bit_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product32_bit_signed_accelerated(
            mut self,
            integer_dot_product32_bit_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product32_bit_signed_accelerated =
                integer_dot_product32_bit_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product32_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product32_bit_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product32_bit_mixed_signedness_accelerated =
                integer_dot_product32_bit_mixed_signedness_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product64_bit_unsigned_accelerated(
            mut self,
            integer_dot_product64_bit_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product64_bit_unsigned_accelerated =
                integer_dot_product64_bit_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product64_bit_signed_accelerated(
            mut self,
            integer_dot_product64_bit_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product64_bit_signed_accelerated =
                integer_dot_product64_bit_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product64_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product64_bit_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product64_bit_mixed_signedness_accelerated =
                integer_dot_product64_bit_mixed_signedness_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated =
                integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating8_bit_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating8_bit_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating8_bit_signed_accelerated =
                integer_dot_product_accumulating_saturating8_bit_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated =
                integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated
                    .into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated =
                integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated
                    .into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated =
                integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated = integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated =
                integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating16_bit_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating16_bit_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating16_bit_signed_accelerated =
                integer_dot_product_accumulating_saturating16_bit_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated =
                integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated
                    .into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated =
                integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating32_bit_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating32_bit_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating32_bit_signed_accelerated =
                integer_dot_product_accumulating_saturating32_bit_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated =
                integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated
                    .into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated =
                integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating64_bit_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating64_bit_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating64_bit_signed_accelerated =
                integer_dot_product_accumulating_saturating64_bit_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated =
                integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated
                    .into();
            self
        }

        #[inline]
        pub fn storage_texel_buffer_offset_alignment_bytes(
            mut self,
            storage_texel_buffer_offset_alignment_bytes: DeviceSize,
        ) -> Self {
            self.storage_texel_buffer_offset_alignment_bytes =
                storage_texel_buffer_offset_alignment_bytes;
            self
        }

        #[inline]
        pub fn storage_texel_buffer_offset_single_texel_alignment(
            mut self,
            storage_texel_buffer_offset_single_texel_alignment: bool,
        ) -> Self {
            self.storage_texel_buffer_offset_single_texel_alignment =
                storage_texel_buffer_offset_single_texel_alignment.into();
            self
        }

        #[inline]
        pub fn uniform_texel_buffer_offset_alignment_bytes(
            mut self,
            uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
        ) -> Self {
            self.uniform_texel_buffer_offset_alignment_bytes =
                uniform_texel_buffer_offset_alignment_bytes;
            self
        }

        #[inline]
        pub fn uniform_texel_buffer_offset_single_texel_alignment(
            mut self,
            uniform_texel_buffer_offset_single_texel_alignment: bool,
        ) -> Self {
            self.uniform_texel_buffer_offset_single_texel_alignment =
                uniform_texel_buffer_offset_single_texel_alignment.into();
            self
        }

        #[inline]
        pub fn max_buffer_size(mut self, max_buffer_size: DeviceSize) -> Self {
            self.max_buffer_size = max_buffer_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceToolProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceToolProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub name: ArrayCStr<{ MAX_EXTENSION_NAME_SIZE as usize }>,
        pub version: ArrayCStr<{ MAX_EXTENSION_NAME_SIZE as usize }>,
        pub purposes: ToolPurposeFlags,
        pub description: ArrayCStr<{ MAX_DESCRIPTION_SIZE as usize }>,
        pub layer: ArrayCStr<{ MAX_EXTENSION_NAME_SIZE as usize }>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceToolProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceToolProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("name", &self.name)
                .field("version", &self.version)
                .field("purposes", &self.purposes)
                .field("description", &self.description)
                .field("layer", &self.layer)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceToolProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_TOOL_PROPERTIES;
    }

    impl Default for PhysicalDeviceToolProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                name: Default::default(),
                version: Default::default(),
                purposes: Default::default(),
                description: Default::default(),
                layer: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceToolProperties<'a> {
        #[inline]
        pub fn name(
            mut self,
            name: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            self.name.write_c_str(name)?;
            Ok(self)
        }

        #[inline]
        pub fn version(
            mut self,
            version: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            self.version.write_c_str(version)?;
            Ok(self)
        }

        #[inline]
        pub fn purposes(mut self, purposes: ToolPurposeFlags) -> Self {
            self.purposes = purposes;
            self
        }

        #[inline]
        pub fn description(
            mut self,
            description: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            self.description.write_c_str(description)?;
            Ok(self)
        }

        #[inline]
        pub fn layer(
            mut self,
            layer: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            self.layer.write_c_str(layer)?;
            Ok(self)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_zero_initialize_workgroup_memory: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "shader_zero_initialize_workgroup_memory",
                    &self.shader_zero_initialize_workgroup_memory,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'_>
    {
    }

    impl Default for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_zero_initialize_workgroup_memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'a> {
        #[inline]
        pub fn shader_zero_initialize_workgroup_memory(
            mut self,
            shader_zero_initialize_workgroup_memory: bool,
        ) -> Self {
            self.shader_zero_initialize_workgroup_memory =
                shader_zero_initialize_workgroup_memory.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImageRobustnessFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceImageRobustnessFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub robust_image_access: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceImageRobustnessFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceImageRobustnessFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("robust_image_access", &self.robust_image_access)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageRobustnessFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceImageRobustnessFeatures<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceImageRobustnessFeatures<'_> {}

    impl Default for PhysicalDeviceImageRobustnessFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                robust_image_access: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceImageRobustnessFeatures<'a> {
        #[inline]
        pub fn robust_image_access(mut self, robust_image_access: bool) -> Self {
            self.robust_image_access = robust_image_access.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferCopy2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BufferCopy2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_offset: DeviceSize,
        pub dst_offset: DeviceSize,
        pub size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BufferCopy2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BufferCopy2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_offset", &self.src_offset)
                .field("dst_offset", &self.dst_offset)
                .field("size", &self.size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BufferCopy2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_COPY_2;
    }

    impl Default for BufferCopy2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_offset: Default::default(),
                dst_offset: Default::default(),
                size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BufferCopy2<'a> {
        #[inline]
        pub fn src_offset(mut self, src_offset: DeviceSize) -> Self {
            self.src_offset = src_offset;
            self
        }

        #[inline]
        pub fn dst_offset(mut self, dst_offset: DeviceSize) -> Self {
            self.dst_offset = dst_offset;
            self
        }

        #[inline]
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageCopy2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageCopy2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_subresource: ImageSubresourceLayers,
        pub src_offset: Offset3D,
        pub dst_subresource: ImageSubresourceLayers,
        pub dst_offset: Offset3D,
        pub extent: Extent3D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageCopy2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageCopy2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_subresource", &self.src_subresource)
                .field("src_offset", &self.src_offset)
                .field("dst_subresource", &self.dst_subresource)
                .field("dst_offset", &self.dst_offset)
                .field("extent", &self.extent)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageCopy2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_COPY_2;
    }

    impl Default for ImageCopy2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_subresource: Default::default(),
                src_offset: Default::default(),
                dst_subresource: Default::default(),
                dst_offset: Default::default(),
                extent: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageCopy2<'a> {
        #[inline]
        pub fn src_subresource(mut self, src_subresource: ImageSubresourceLayers) -> Self {
            self.src_subresource = src_subresource;
            self
        }

        #[inline]
        pub fn src_offset(mut self, src_offset: Offset3D) -> Self {
            self.src_offset = src_offset;
            self
        }

        #[inline]
        pub fn dst_subresource(mut self, dst_subresource: ImageSubresourceLayers) -> Self {
            self.dst_subresource = dst_subresource;
            self
        }

        #[inline]
        pub fn dst_offset(mut self, dst_offset: Offset3D) -> Self {
            self.dst_offset = dst_offset;
            self
        }

        #[inline]
        pub fn extent(mut self, extent: Extent3D) -> Self {
            self.extent = extent;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageBlit2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageBlit2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_subresource: ImageSubresourceLayers,
        pub src_offsets: [Offset3D; 2],
        pub dst_subresource: ImageSubresourceLayers,
        pub dst_offsets: [Offset3D; 2],
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageBlit2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageBlit2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_subresource", &self.src_subresource)
                .field("src_offsets", &self.src_offsets)
                .field("dst_subresource", &self.dst_subresource)
                .field("dst_offsets", &self.dst_offsets)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageBlit2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_BLIT_2;
    }

    impl Default for ImageBlit2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_subresource: Default::default(),
                src_offsets: [Default::default(); _],
                dst_subresource: Default::default(),
                dst_offsets: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageBlit2<'a> {
        #[inline]
        pub fn src_subresource(mut self, src_subresource: ImageSubresourceLayers) -> Self {
            self.src_subresource = src_subresource;
            self
        }

        #[inline]
        pub fn src_offsets(mut self, src_offsets: [Offset3D; 2]) -> Self {
            self.src_offsets = src_offsets;
            self
        }

        #[inline]
        pub fn dst_subresource(mut self, dst_subresource: ImageSubresourceLayers) -> Self {
            self.dst_subresource = dst_subresource;
            self
        }

        #[inline]
        pub fn dst_offsets(mut self, dst_offsets: [Offset3D; 2]) -> Self {
            self.dst_offsets = dst_offsets;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferImageCopy2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BufferImageCopy2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub buffer_offset: DeviceSize,
        pub buffer_row_length: u32,
        pub buffer_image_height: u32,
        pub image_subresource: ImageSubresourceLayers,
        pub image_offset: Offset3D,
        pub image_extent: Extent3D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BufferImageCopy2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BufferImageCopy2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("buffer_offset", &self.buffer_offset)
                .field("buffer_row_length", &self.buffer_row_length)
                .field("buffer_image_height", &self.buffer_image_height)
                .field("image_subresource", &self.image_subresource)
                .field("image_offset", &self.image_offset)
                .field("image_extent", &self.image_extent)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BufferImageCopy2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_IMAGE_COPY_2;
    }

    impl Default for BufferImageCopy2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                buffer_offset: Default::default(),
                buffer_row_length: Default::default(),
                buffer_image_height: Default::default(),
                image_subresource: Default::default(),
                image_offset: Default::default(),
                image_extent: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BufferImageCopy2<'a> {
        #[inline]
        pub fn buffer_offset(mut self, buffer_offset: DeviceSize) -> Self {
            self.buffer_offset = buffer_offset;
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageResolve2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageResolve2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_subresource: ImageSubresourceLayers,
        pub src_offset: Offset3D,
        pub dst_subresource: ImageSubresourceLayers,
        pub dst_offset: Offset3D,
        pub extent: Extent3D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageResolve2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageResolve2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_subresource", &self.src_subresource)
                .field("src_offset", &self.src_offset)
                .field("dst_subresource", &self.dst_subresource)
                .field("dst_offset", &self.dst_offset)
                .field("extent", &self.extent)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageResolve2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_RESOLVE_2;
    }

    impl Default for ImageResolve2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_subresource: Default::default(),
                src_offset: Default::default(),
                dst_subresource: Default::default(),
                dst_offset: Default::default(),
                extent: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageResolve2<'a> {
        #[inline]
        pub fn src_subresource(mut self, src_subresource: ImageSubresourceLayers) -> Self {
            self.src_subresource = src_subresource;
            self
        }

        #[inline]
        pub fn src_offset(mut self, src_offset: Offset3D) -> Self {
            self.src_offset = src_offset;
            self
        }

        #[inline]
        pub fn dst_subresource(mut self, dst_subresource: ImageSubresourceLayers) -> Self {
            self.dst_subresource = dst_subresource;
            self
        }

        #[inline]
        pub fn dst_offset(mut self, dst_offset: Offset3D) -> Self {
            self.dst_offset = dst_offset;
            self
        }

        #[inline]
        pub fn extent(mut self, extent: Extent3D) -> Self {
            self.extent = extent;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyBufferInfo2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CopyBufferInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_buffer: Buffer,
        pub dst_buffer: Buffer,
        pub region_count: u32,
        pub p_regions: *const BufferCopy2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyBufferInfo2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyBufferInfo2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_buffer", &self.src_buffer)
                .field("dst_buffer", &self.dst_buffer)
                .field("region_count", &self.region_count)
                .field("p_regions", &self.p_regions)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyBufferInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_BUFFER_INFO_2;
    }

    impl Default for CopyBufferInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_buffer: Default::default(),
                dst_buffer: Default::default(),
                region_count: Default::default(),
                p_regions: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CopyBufferInfo2<'a> {
        #[inline]
        pub fn src_buffer(mut self, src_buffer: Buffer) -> Self {
            self.src_buffer = src_buffer;
            self
        }

        #[inline]
        pub fn dst_buffer(mut self, dst_buffer: Buffer) -> Self {
            self.dst_buffer = dst_buffer;
            self
        }

        #[inline]
        pub fn regions(mut self, regions: &'a [BufferCopy2<'_>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyImageInfo2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CopyImageInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_image: Image,
        pub src_image_layout: ImageLayout,
        pub dst_image: Image,
        pub dst_image_layout: ImageLayout,
        pub region_count: u32,
        pub p_regions: *const ImageCopy2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyImageInfo2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyImageInfo2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_image", &self.src_image)
                .field("src_image_layout", &self.src_image_layout)
                .field("dst_image", &self.dst_image)
                .field("dst_image_layout", &self.dst_image_layout)
                .field("region_count", &self.region_count)
                .field("p_regions", &self.p_regions)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyImageInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_IMAGE_INFO_2;
    }

    impl Default for CopyImageInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_image: Default::default(),
                src_image_layout: Default::default(),
                dst_image: Default::default(),
                dst_image_layout: Default::default(),
                region_count: Default::default(),
                p_regions: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CopyImageInfo2<'a> {
        #[inline]
        pub fn src_image(mut self, src_image: Image) -> Self {
            self.src_image = src_image;
            self
        }

        #[inline]
        pub fn src_image_layout(mut self, src_image_layout: ImageLayout) -> Self {
            self.src_image_layout = src_image_layout;
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

        #[inline]
        pub fn regions(mut self, regions: &'a [ImageCopy2<'_>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBlitImageInfo2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BlitImageInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_image: Image,
        pub src_image_layout: ImageLayout,
        pub dst_image: Image,
        pub dst_image_layout: ImageLayout,
        pub region_count: u32,
        pub p_regions: *const ImageBlit2<'a>,
        pub filter: Filter,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BlitImageInfo2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BlitImageInfo2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_image", &self.src_image)
                .field("src_image_layout", &self.src_image_layout)
                .field("dst_image", &self.dst_image)
                .field("dst_image_layout", &self.dst_image_layout)
                .field("region_count", &self.region_count)
                .field("p_regions", &self.p_regions)
                .field("filter", &self.filter)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BlitImageInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BLIT_IMAGE_INFO_2;
    }

    impl Default for BlitImageInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_image: Default::default(),
                src_image_layout: Default::default(),
                dst_image: Default::default(),
                dst_image_layout: Default::default(),
                region_count: Default::default(),
                p_regions: ptr::null(),
                filter: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BlitImageInfo2<'a> {
        #[inline]
        pub fn src_image(mut self, src_image: Image) -> Self {
            self.src_image = src_image;
            self
        }

        #[inline]
        pub fn src_image_layout(mut self, src_image_layout: ImageLayout) -> Self {
            self.src_image_layout = src_image_layout;
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

        #[inline]
        pub fn regions(mut self, regions: &'a [ImageBlit2<'_>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr() as _;
            self
        }

        #[inline]
        pub fn filter(mut self, filter: Filter) -> Self {
            self.filter = filter;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyBufferToImageInfo2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CopyBufferToImageInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_buffer: Buffer,
        pub dst_image: Image,
        pub dst_image_layout: ImageLayout,
        pub region_count: u32,
        pub p_regions: *const BufferImageCopy2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyBufferToImageInfo2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyBufferToImageInfo2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_buffer", &self.src_buffer)
                .field("dst_image", &self.dst_image)
                .field("dst_image_layout", &self.dst_image_layout)
                .field("region_count", &self.region_count)
                .field("p_regions", &self.p_regions)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyBufferToImageInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_BUFFER_TO_IMAGE_INFO_2;
    }

    impl Default for CopyBufferToImageInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_buffer: Default::default(),
                dst_image: Default::default(),
                dst_image_layout: Default::default(),
                region_count: Default::default(),
                p_regions: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CopyBufferToImageInfo2<'a> {
        #[inline]
        pub fn src_buffer(mut self, src_buffer: Buffer) -> Self {
            self.src_buffer = src_buffer;
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

        #[inline]
        pub fn regions(mut self, regions: &'a [BufferImageCopy2<'_>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyImageToBufferInfo2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CopyImageToBufferInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_image: Image,
        pub src_image_layout: ImageLayout,
        pub dst_buffer: Buffer,
        pub region_count: u32,
        pub p_regions: *const BufferImageCopy2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyImageToBufferInfo2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyImageToBufferInfo2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_image", &self.src_image)
                .field("src_image_layout", &self.src_image_layout)
                .field("dst_buffer", &self.dst_buffer)
                .field("region_count", &self.region_count)
                .field("p_regions", &self.p_regions)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyImageToBufferInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_IMAGE_TO_BUFFER_INFO_2;
    }

    impl Default for CopyImageToBufferInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_image: Default::default(),
                src_image_layout: Default::default(),
                dst_buffer: Default::default(),
                region_count: Default::default(),
                p_regions: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CopyImageToBufferInfo2<'a> {
        #[inline]
        pub fn src_image(mut self, src_image: Image) -> Self {
            self.src_image = src_image;
            self
        }

        #[inline]
        pub fn src_image_layout(mut self, src_image_layout: ImageLayout) -> Self {
            self.src_image_layout = src_image_layout;
            self
        }

        #[inline]
        pub fn dst_buffer(mut self, dst_buffer: Buffer) -> Self {
            self.dst_buffer = dst_buffer;
            self
        }

        #[inline]
        pub fn regions(mut self, regions: &'a [BufferImageCopy2<'_>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkResolveImageInfo2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ResolveImageInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_image: Image,
        pub src_image_layout: ImageLayout,
        pub dst_image: Image,
        pub dst_image_layout: ImageLayout,
        pub region_count: u32,
        pub p_regions: *const ImageResolve2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ResolveImageInfo2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ResolveImageInfo2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_image", &self.src_image)
                .field("src_image_layout", &self.src_image_layout)
                .field("dst_image", &self.dst_image)
                .field("dst_image_layout", &self.dst_image_layout)
                .field("region_count", &self.region_count)
                .field("p_regions", &self.p_regions)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ResolveImageInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RESOLVE_IMAGE_INFO_2;
    }

    impl Default for ResolveImageInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_image: Default::default(),
                src_image_layout: Default::default(),
                dst_image: Default::default(),
                dst_image_layout: Default::default(),
                region_count: Default::default(),
                p_regions: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ResolveImageInfo2<'a> {
        #[inline]
        pub fn src_image(mut self, src_image: Image) -> Self {
            self.src_image = src_image;
            self
        }

        #[inline]
        pub fn src_image_layout(mut self, src_image_layout: ImageLayout) -> Self {
            self.src_image_layout = src_image_layout;
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

        #[inline]
        pub fn regions(mut self, regions: &'a [ImageResolve2<'_>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderTerminateInvocationFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderTerminateInvocationFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_terminate_invocation: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderTerminateInvocationFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderTerminateInvocationFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "shader_terminate_invocation",
                    &self.shader_terminate_invocation,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderTerminateInvocationFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceShaderTerminateInvocationFeatures<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceShaderTerminateInvocationFeatures<'_> {}

    impl Default for PhysicalDeviceShaderTerminateInvocationFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_terminate_invocation: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderTerminateInvocationFeatures<'a> {
        #[inline]
        pub fn shader_terminate_invocation(mut self, shader_terminate_invocation: bool) -> Self {
            self.shader_terminate_invocation = shader_terminate_invocation.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryBarrier2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryBarrier2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_stage_mask: PipelineStageFlags2,
        pub src_access_mask: AccessFlags2,
        pub dst_stage_mask: PipelineStageFlags2,
        pub dst_access_mask: AccessFlags2,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryBarrier2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryBarrier2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_stage_mask", &self.src_stage_mask)
                .field("src_access_mask", &self.src_access_mask)
                .field("dst_stage_mask", &self.dst_stage_mask)
                .field("dst_access_mask", &self.dst_access_mask)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryBarrier2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_BARRIER_2;
    }

    unsafe impl Extends<SubpassDependency2<'_>> for MemoryBarrier2<'_> {}

    impl Default for MemoryBarrier2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_stage_mask: Default::default(),
                src_access_mask: Default::default(),
                dst_stage_mask: Default::default(),
                dst_access_mask: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryBarrier2<'a> {
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
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageMemoryBarrier2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageMemoryBarrier2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_stage_mask: PipelineStageFlags2,
        pub src_access_mask: AccessFlags2,
        pub dst_stage_mask: PipelineStageFlags2,
        pub dst_access_mask: AccessFlags2,
        pub old_layout: ImageLayout,
        pub new_layout: ImageLayout,
        pub src_queue_family_index: u32,
        pub dst_queue_family_index: u32,
        pub image: Image,
        pub subresource_range: ImageSubresourceRange,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageMemoryBarrier2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageMemoryBarrier2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_stage_mask", &self.src_stage_mask)
                .field("src_access_mask", &self.src_access_mask)
                .field("dst_stage_mask", &self.dst_stage_mask)
                .field("dst_access_mask", &self.dst_access_mask)
                .field("old_layout", &self.old_layout)
                .field("new_layout", &self.new_layout)
                .field("src_queue_family_index", &self.src_queue_family_index)
                .field("dst_queue_family_index", &self.dst_queue_family_index)
                .field("image", &self.image)
                .field("subresource_range", &self.subresource_range)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageMemoryBarrier2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_MEMORY_BARRIER_2;
    }

    impl Default for ImageMemoryBarrier2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_stage_mask: Default::default(),
                src_access_mask: Default::default(),
                dst_stage_mask: Default::default(),
                dst_access_mask: Default::default(),
                old_layout: Default::default(),
                new_layout: Default::default(),
                src_queue_family_index: Default::default(),
                dst_queue_family_index: Default::default(),
                image: Default::default(),
                subresource_range: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageMemoryBarrier2<'a> {
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
        pub fn old_layout(mut self, old_layout: ImageLayout) -> Self {
            self.old_layout = old_layout;
            self
        }

        #[inline]
        pub fn new_layout(mut self, new_layout: ImageLayout) -> Self {
            self.new_layout = new_layout;
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
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }

        #[inline]
        pub fn subresource_range(mut self, subresource_range: ImageSubresourceRange) -> Self {
            self.subresource_range = subresource_range;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferMemoryBarrier2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BufferMemoryBarrier2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_stage_mask: PipelineStageFlags2,
        pub src_access_mask: AccessFlags2,
        pub dst_stage_mask: PipelineStageFlags2,
        pub dst_access_mask: AccessFlags2,
        pub src_queue_family_index: u32,
        pub dst_queue_family_index: u32,
        pub buffer: Buffer,
        pub offset: DeviceSize,
        pub size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BufferMemoryBarrier2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BufferMemoryBarrier2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_stage_mask", &self.src_stage_mask)
                .field("src_access_mask", &self.src_access_mask)
                .field("dst_stage_mask", &self.dst_stage_mask)
                .field("dst_access_mask", &self.dst_access_mask)
                .field("src_queue_family_index", &self.src_queue_family_index)
                .field("dst_queue_family_index", &self.dst_queue_family_index)
                .field("buffer", &self.buffer)
                .field("offset", &self.offset)
                .field("size", &self.size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BufferMemoryBarrier2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_MEMORY_BARRIER_2;
    }

    impl Default for BufferMemoryBarrier2<'_> {
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
                buffer: Default::default(),
                offset: Default::default(),
                size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BufferMemoryBarrier2<'a> {
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
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }

        #[inline]
        pub fn offset(mut self, offset: DeviceSize) -> Self {
            self.offset = offset;
            self
        }

        #[inline]
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDependencyInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DependencyInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub dependency_flags: DependencyFlags,
        pub memory_barrier_count: u32,
        pub p_memory_barriers: *const MemoryBarrier2<'a>,
        pub buffer_memory_barrier_count: u32,
        pub p_buffer_memory_barriers: *const BufferMemoryBarrier2<'a>,
        pub image_memory_barrier_count: u32,
        pub p_image_memory_barriers: *const ImageMemoryBarrier2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DependencyInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DependencyInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("dependency_flags", &self.dependency_flags)
                .field("memory_barrier_count", &self.memory_barrier_count)
                .field("p_memory_barriers", &self.p_memory_barriers)
                .field(
                    "buffer_memory_barrier_count",
                    &self.buffer_memory_barrier_count,
                )
                .field("p_buffer_memory_barriers", &self.p_buffer_memory_barriers)
                .field(
                    "image_memory_barrier_count",
                    &self.image_memory_barrier_count,
                )
                .field("p_image_memory_barriers", &self.p_image_memory_barriers)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DependencyInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEPENDENCY_INFO;
    }

    impl Default for DependencyInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                dependency_flags: Default::default(),
                memory_barrier_count: Default::default(),
                p_memory_barriers: ptr::null(),
                buffer_memory_barrier_count: Default::default(),
                p_buffer_memory_barriers: ptr::null(),
                image_memory_barrier_count: Default::default(),
                p_image_memory_barriers: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DependencyInfo<'a> {
        #[inline]
        pub fn dependency_flags(mut self, dependency_flags: DependencyFlags) -> Self {
            self.dependency_flags = dependency_flags;
            self
        }

        #[inline]
        pub fn memory_barriers(mut self, memory_barriers: &'a [MemoryBarrier2<'_>]) -> Self {
            self.memory_barrier_count = memory_barriers.len().try_into().unwrap();
            self.p_memory_barriers = memory_barriers.as_ptr() as _;
            self
        }

        #[inline]
        pub fn buffer_memory_barriers(
            mut self,
            buffer_memory_barriers: &'a [BufferMemoryBarrier2<'_>],
        ) -> Self {
            self.buffer_memory_barrier_count = buffer_memory_barriers.len().try_into().unwrap();
            self.p_buffer_memory_barriers = buffer_memory_barriers.as_ptr() as _;
            self
        }

        #[inline]
        pub fn image_memory_barriers(
            mut self,
            image_memory_barriers: &'a [ImageMemoryBarrier2<'_>],
        ) -> Self {
            self.image_memory_barrier_count = image_memory_barriers.len().try_into().unwrap();
            self.p_image_memory_barriers = image_memory_barriers.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSemaphoreSubmitInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SemaphoreSubmitInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub semaphore: Semaphore,
        pub value: u64,
        pub stage_mask: PipelineStageFlags2,
        pub device_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SemaphoreSubmitInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SemaphoreSubmitInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("semaphore", &self.semaphore)
                .field("value", &self.value)
                .field("stage_mask", &self.stage_mask)
                .field("device_index", &self.device_index)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SemaphoreSubmitInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SEMAPHORE_SUBMIT_INFO;
    }

    impl Default for SemaphoreSubmitInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                semaphore: Default::default(),
                value: Default::default(),
                stage_mask: Default::default(),
                device_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SemaphoreSubmitInfo<'a> {
        #[inline]
        pub fn semaphore(mut self, semaphore: Semaphore) -> Self {
            self.semaphore = semaphore;
            self
        }

        #[inline]
        pub fn value(mut self, value: u64) -> Self {
            self.value = value;
            self
        }

        #[inline]
        pub fn stage_mask(mut self, stage_mask: PipelineStageFlags2) -> Self {
            self.stage_mask = stage_mask;
            self
        }

        #[inline]
        pub fn device_index(mut self, device_index: u32) -> Self {
            self.device_index = device_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCommandBufferSubmitInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CommandBufferSubmitInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub command_buffer: CommandBuffer,
        pub device_mask: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CommandBufferSubmitInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CommandBufferSubmitInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("command_buffer", &self.command_buffer)
                .field("device_mask", &self.device_mask)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CommandBufferSubmitInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COMMAND_BUFFER_SUBMIT_INFO;
    }

    impl Default for CommandBufferSubmitInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                command_buffer: Default::default(),
                device_mask: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CommandBufferSubmitInfo<'a> {
        #[inline]
        pub fn command_buffer(mut self, command_buffer: CommandBuffer) -> Self {
            self.command_buffer = command_buffer;
            self
        }

        #[inline]
        pub fn device_mask(mut self, device_mask: u32) -> Self {
            self.device_mask = device_mask;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubmitInfo2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SubmitInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: SubmitFlags,
        pub wait_semaphore_info_count: u32,
        pub p_wait_semaphore_infos: *const SemaphoreSubmitInfo<'a>,
        pub command_buffer_info_count: u32,
        pub p_command_buffer_infos: *const CommandBufferSubmitInfo<'a>,
        pub signal_semaphore_info_count: u32,
        pub p_signal_semaphore_infos: *const SemaphoreSubmitInfo<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SubmitInfo2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SubmitInfo2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("wait_semaphore_info_count", &self.wait_semaphore_info_count)
                .field("p_wait_semaphore_infos", &self.p_wait_semaphore_infos)
                .field("command_buffer_info_count", &self.command_buffer_info_count)
                .field("p_command_buffer_infos", &self.p_command_buffer_infos)
                .field(
                    "signal_semaphore_info_count",
                    &self.signal_semaphore_info_count,
                )
                .field("p_signal_semaphore_infos", &self.p_signal_semaphore_infos)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SubmitInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SUBMIT_INFO_2;
    }

    impl Default for SubmitInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                wait_semaphore_info_count: Default::default(),
                p_wait_semaphore_infos: ptr::null(),
                command_buffer_info_count: Default::default(),
                p_command_buffer_infos: ptr::null(),
                signal_semaphore_info_count: Default::default(),
                p_signal_semaphore_infos: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SubmitInfo2<'a> {
        #[inline]
        pub fn flags(mut self, flags: SubmitFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn wait_semaphore_infos(
            mut self,
            wait_semaphore_infos: &'a [SemaphoreSubmitInfo<'_>],
        ) -> Self {
            self.wait_semaphore_info_count = wait_semaphore_infos.len().try_into().unwrap();
            self.p_wait_semaphore_infos = wait_semaphore_infos.as_ptr() as _;
            self
        }

        #[inline]
        pub fn command_buffer_infos(
            mut self,
            command_buffer_infos: &'a [CommandBufferSubmitInfo<'_>],
        ) -> Self {
            self.command_buffer_info_count = command_buffer_infos.len().try_into().unwrap();
            self.p_command_buffer_infos = command_buffer_infos.as_ptr() as _;
            self
        }

        #[inline]
        pub fn signal_semaphore_infos(
            mut self,
            signal_semaphore_infos: &'a [SemaphoreSubmitInfo<'_>],
        ) -> Self {
            self.signal_semaphore_info_count = signal_semaphore_infos.len().try_into().unwrap();
            self.p_signal_semaphore_infos = signal_semaphore_infos.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSynchronization2Features.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceSynchronization2Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub synchronization2: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceSynchronization2Features<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceSynchronization2Features")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("synchronization2", &self.synchronization2)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSynchronization2Features<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceSynchronization2Features<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceSynchronization2Features<'_> {}

    impl Default for PhysicalDeviceSynchronization2Features<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                synchronization2: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceSynchronization2Features<'a> {
        #[inline]
        pub fn synchronization2(mut self, synchronization2: bool) -> Self {
            self.synchronization2 = synchronization2.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderIntegerDotProductFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderIntegerDotProductFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_integer_dot_product: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderIntegerDotProductFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderIntegerDotProductFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "shader_integer_dot_product",
                    &self.shader_integer_dot_product,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderIntegerDotProductFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceShaderIntegerDotProductFeatures<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceShaderIntegerDotProductFeatures<'_> {}

    impl Default for PhysicalDeviceShaderIntegerDotProductFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_integer_dot_product: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderIntegerDotProductFeatures<'a> {
        #[inline]
        pub fn shader_integer_dot_product(mut self, shader_integer_dot_product: bool) -> Self {
            self.shader_integer_dot_product = shader_integer_dot_product.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderIntegerDotProductProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderIntegerDotProductProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub integer_dot_product8_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product8_bit_signed_accelerated: Bool32,
        pub integer_dot_product8_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product4x8_bit_packed_unsigned_accelerated: Bool32,
        pub integer_dot_product4x8_bit_packed_signed_accelerated: Bool32,
        pub integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product16_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product16_bit_signed_accelerated: Bool32,
        pub integer_dot_product16_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product32_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product32_bit_signed_accelerated: Bool32,
        pub integer_dot_product32_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product64_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product64_bit_signed_accelerated: Bool32,
        pub integer_dot_product64_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating8_bit_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated:
            Bool32,
        pub integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating16_bit_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating32_bit_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating64_bit_signed_accelerated: Bool32,
        pub integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderIntegerDotProductProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderIntegerDotProductProperties")
.field("s_type", &self.s_type)
.field("p_next", &self.p_next)
.field("integer_dot_product8_bit_unsigned_accelerated", &self.integer_dot_product8_bit_unsigned_accelerated)
.field("integer_dot_product8_bit_signed_accelerated", &self.integer_dot_product8_bit_signed_accelerated)
.field("integer_dot_product8_bit_mixed_signedness_accelerated", &self.integer_dot_product8_bit_mixed_signedness_accelerated)
.field("integer_dot_product4x8_bit_packed_unsigned_accelerated", &self.integer_dot_product4x8_bit_packed_unsigned_accelerated)
.field("integer_dot_product4x8_bit_packed_signed_accelerated", &self.integer_dot_product4x8_bit_packed_signed_accelerated)
.field("integer_dot_product4x8_bit_packed_mixed_signedness_accelerated", &self.integer_dot_product4x8_bit_packed_mixed_signedness_accelerated)
.field("integer_dot_product16_bit_unsigned_accelerated", &self.integer_dot_product16_bit_unsigned_accelerated)
.field("integer_dot_product16_bit_signed_accelerated", &self.integer_dot_product16_bit_signed_accelerated)
.field("integer_dot_product16_bit_mixed_signedness_accelerated", &self.integer_dot_product16_bit_mixed_signedness_accelerated)
.field("integer_dot_product32_bit_unsigned_accelerated", &self.integer_dot_product32_bit_unsigned_accelerated)
.field("integer_dot_product32_bit_signed_accelerated", &self.integer_dot_product32_bit_signed_accelerated)
.field("integer_dot_product32_bit_mixed_signedness_accelerated", &self.integer_dot_product32_bit_mixed_signedness_accelerated)
.field("integer_dot_product64_bit_unsigned_accelerated", &self.integer_dot_product64_bit_unsigned_accelerated)
.field("integer_dot_product64_bit_signed_accelerated", &self.integer_dot_product64_bit_signed_accelerated)
.field("integer_dot_product64_bit_mixed_signedness_accelerated", &self.integer_dot_product64_bit_mixed_signedness_accelerated)
.field("integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated", &self.integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated)
.field("integer_dot_product_accumulating_saturating8_bit_signed_accelerated", &self.integer_dot_product_accumulating_saturating8_bit_signed_accelerated)
.field("integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated", &self.integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated)
.field("integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated", &self.integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated)
.field("integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated", &self.integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated)
.field("integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated", &self.integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated)
.field("integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated", &self.integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated)
.field("integer_dot_product_accumulating_saturating16_bit_signed_accelerated", &self.integer_dot_product_accumulating_saturating16_bit_signed_accelerated)
.field("integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated", &self.integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated)
.field("integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated", &self.integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated)
.field("integer_dot_product_accumulating_saturating32_bit_signed_accelerated", &self.integer_dot_product_accumulating_saturating32_bit_signed_accelerated)
.field("integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated", &self.integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated)
.field("integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated", &self.integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated)
.field("integer_dot_product_accumulating_saturating64_bit_signed_accelerated", &self.integer_dot_product_accumulating_saturating64_bit_signed_accelerated)
.field("integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated", &self.integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated)
.finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderIntegerDotProductProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceShaderIntegerDotProductProperties<'_>
    {
    }

    impl Default for PhysicalDeviceShaderIntegerDotProductProperties<'_> {
        fn default() -> Self {
            Self {
s_type: Self::STRUCTURE_TYPE
,
p_next: ptr::null_mut(),
integer_dot_product8_bit_unsigned_accelerated: Default::default(),
integer_dot_product8_bit_signed_accelerated: Default::default(),
integer_dot_product8_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product4x8_bit_packed_unsigned_accelerated: Default::default(),
integer_dot_product4x8_bit_packed_signed_accelerated: Default::default(),
integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: Default::default(),
integer_dot_product16_bit_unsigned_accelerated: Default::default(),
integer_dot_product16_bit_signed_accelerated: Default::default(),
integer_dot_product16_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product32_bit_unsigned_accelerated: Default::default(),
integer_dot_product32_bit_signed_accelerated: Default::default(),
integer_dot_product32_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product64_bit_unsigned_accelerated: Default::default(),
integer_dot_product64_bit_signed_accelerated: Default::default(),
integer_dot_product64_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating8_bit_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating16_bit_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating32_bit_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: Default::default(),
integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: Default::default(),
integer_dot_product_accumulating_saturating64_bit_signed_accelerated: Default::default(),
integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: Default::default(),
_marker: PhantomData
}
        }
    }

    impl<'a> PhysicalDeviceShaderIntegerDotProductProperties<'a> {
        #[inline]
        pub fn integer_dot_product8_bit_unsigned_accelerated(
            mut self,
            integer_dot_product8_bit_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product8_bit_unsigned_accelerated =
                integer_dot_product8_bit_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product8_bit_signed_accelerated(
            mut self,
            integer_dot_product8_bit_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product8_bit_signed_accelerated =
                integer_dot_product8_bit_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product8_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product8_bit_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product8_bit_mixed_signedness_accelerated =
                integer_dot_product8_bit_mixed_signedness_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product4x8_bit_packed_unsigned_accelerated(
            mut self,
            integer_dot_product4x8_bit_packed_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product4x8_bit_packed_unsigned_accelerated =
                integer_dot_product4x8_bit_packed_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product4x8_bit_packed_signed_accelerated(
            mut self,
            integer_dot_product4x8_bit_packed_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product4x8_bit_packed_signed_accelerated =
                integer_dot_product4x8_bit_packed_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product4x8_bit_packed_mixed_signedness_accelerated(
            mut self,
            integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product4x8_bit_packed_mixed_signedness_accelerated =
                integer_dot_product4x8_bit_packed_mixed_signedness_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product16_bit_unsigned_accelerated(
            mut self,
            integer_dot_product16_bit_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product16_bit_unsigned_accelerated =
                integer_dot_product16_bit_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product16_bit_signed_accelerated(
            mut self,
            integer_dot_product16_bit_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product16_bit_signed_accelerated =
                integer_dot_product16_bit_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product16_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product16_bit_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product16_bit_mixed_signedness_accelerated =
                integer_dot_product16_bit_mixed_signedness_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product32_bit_unsigned_accelerated(
            mut self,
            integer_dot_product32_bit_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product32_bit_unsigned_accelerated =
                integer_dot_product32_bit_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product32_bit_signed_accelerated(
            mut self,
            integer_dot_product32_bit_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product32_bit_signed_accelerated =
                integer_dot_product32_bit_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product32_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product32_bit_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product32_bit_mixed_signedness_accelerated =
                integer_dot_product32_bit_mixed_signedness_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product64_bit_unsigned_accelerated(
            mut self,
            integer_dot_product64_bit_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product64_bit_unsigned_accelerated =
                integer_dot_product64_bit_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product64_bit_signed_accelerated(
            mut self,
            integer_dot_product64_bit_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product64_bit_signed_accelerated =
                integer_dot_product64_bit_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product64_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product64_bit_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product64_bit_mixed_signedness_accelerated =
                integer_dot_product64_bit_mixed_signedness_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated =
                integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating8_bit_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating8_bit_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating8_bit_signed_accelerated =
                integer_dot_product_accumulating_saturating8_bit_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated =
                integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated
                    .into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated =
                integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated
                    .into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated =
                integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated = integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated =
                integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating16_bit_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating16_bit_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating16_bit_signed_accelerated =
                integer_dot_product_accumulating_saturating16_bit_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated =
                integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated
                    .into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated =
                integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating32_bit_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating32_bit_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating32_bit_signed_accelerated =
                integer_dot_product_accumulating_saturating32_bit_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated =
                integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated
                    .into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated =
                integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating64_bit_signed_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating64_bit_signed_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating64_bit_signed_accelerated =
                integer_dot_product_accumulating_saturating64_bit_signed_accelerated.into();
            self
        }

        #[inline]
        pub fn integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated(
            mut self,
            integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: bool,
        ) -> Self {
            self.integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated =
                integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated
                    .into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFormatProperties3.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct FormatProperties3<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub linear_tiling_features: FormatFeatureFlags2,
        pub optimal_tiling_features: FormatFeatureFlags2,
        pub buffer_features: FormatFeatureFlags2,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for FormatProperties3<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("FormatProperties3")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("linear_tiling_features", &self.linear_tiling_features)
                .field("optimal_tiling_features", &self.optimal_tiling_features)
                .field("buffer_features", &self.buffer_features)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for FormatProperties3<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::FORMAT_PROPERTIES_3;
    }

    unsafe impl Extends<FormatProperties2<'_>> for FormatProperties3<'_> {}

    impl Default for FormatProperties3<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                linear_tiling_features: Default::default(),
                optimal_tiling_features: Default::default(),
                buffer_features: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> FormatProperties3<'a> {
        #[inline]
        pub fn linear_tiling_features(
            mut self,
            linear_tiling_features: FormatFeatureFlags2,
        ) -> Self {
            self.linear_tiling_features = linear_tiling_features;
            self
        }

        #[inline]
        pub fn optimal_tiling_features(
            mut self,
            optimal_tiling_features: FormatFeatureFlags2,
        ) -> Self {
            self.optimal_tiling_features = optimal_tiling_features;
            self
        }

        #[inline]
        pub fn buffer_features(mut self, buffer_features: FormatFeatureFlags2) -> Self {
            self.buffer_features = buffer_features;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRenderingCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineRenderingCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub view_mask: u32,
        pub color_attachment_count: u32,
        pub p_color_attachment_formats: *const Format,
        pub depth_attachment_format: Format,
        pub stencil_attachment_format: Format,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineRenderingCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineRenderingCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("view_mask", &self.view_mask)
                .field("color_attachment_count", &self.color_attachment_count)
                .field(
                    "p_color_attachment_formats",
                    &self.p_color_attachment_formats,
                )
                .field("depth_attachment_format", &self.depth_attachment_format)
                .field("stencil_attachment_format", &self.stencil_attachment_format)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineRenderingCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_RENDERING_CREATE_INFO;
    }

    unsafe impl Extends<GraphicsPipelineCreateInfo<'_>> for PipelineRenderingCreateInfo<'_> {}

    impl Default for PipelineRenderingCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                view_mask: Default::default(),
                color_attachment_count: Default::default(),
                p_color_attachment_formats: ptr::null(),
                depth_attachment_format: Default::default(),
                stencil_attachment_format: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineRenderingCreateInfo<'a> {
        #[inline]
        pub fn view_mask(mut self, view_mask: u32) -> Self {
            self.view_mask = view_mask;
            self
        }

        #[inline]
        pub fn color_attachment_formats(mut self, color_attachment_formats: &'a [Format]) -> Self {
            self.color_attachment_count = color_attachment_formats.len().try_into().unwrap();
            self.p_color_attachment_formats = color_attachment_formats.as_ptr() as _;
            self
        }

        #[inline]
        pub fn depth_attachment_format(mut self, depth_attachment_format: Format) -> Self {
            self.depth_attachment_format = depth_attachment_format;
            self
        }

        #[inline]
        pub fn stencil_attachment_format(mut self, stencil_attachment_format: Format) -> Self {
            self.stencil_attachment_format = stencil_attachment_format;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RenderingInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: RenderingFlags,
        pub render_area: Rect2D,
        pub layer_count: u32,
        pub view_mask: u32,
        pub color_attachment_count: u32,
        pub p_color_attachments: *const RenderingAttachmentInfo<'a>,
        pub p_depth_attachment: *const RenderingAttachmentInfo<'a>,
        pub p_stencil_attachment: *const RenderingAttachmentInfo<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderingInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderingInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("render_area", &self.render_area)
                .field("layer_count", &self.layer_count)
                .field("view_mask", &self.view_mask)
                .field("color_attachment_count", &self.color_attachment_count)
                .field("p_color_attachments", &self.p_color_attachments)
                .field("p_depth_attachment", &self.p_depth_attachment)
                .field("p_stencil_attachment", &self.p_stencil_attachment)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderingInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDERING_INFO;
    }

    impl Default for RenderingInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                render_area: Default::default(),
                layer_count: Default::default(),
                view_mask: Default::default(),
                color_attachment_count: Default::default(),
                p_color_attachments: ptr::null(),
                p_depth_attachment: ptr::null(),
                p_stencil_attachment: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RenderingInfo<'a> {
        #[inline]
        pub fn flags(mut self, flags: RenderingFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn render_area(mut self, render_area: Rect2D) -> Self {
            self.render_area = render_area;
            self
        }

        #[inline]
        pub fn layer_count(mut self, layer_count: u32) -> Self {
            self.layer_count = layer_count;
            self
        }

        #[inline]
        pub fn view_mask(mut self, view_mask: u32) -> Self {
            self.view_mask = view_mask;
            self
        }

        #[inline]
        pub fn color_attachments(
            mut self,
            color_attachments: &'a [RenderingAttachmentInfo<'_>],
        ) -> Self {
            self.color_attachment_count = color_attachments.len().try_into().unwrap();
            self.p_color_attachments = color_attachments.as_ptr() as _;
            self
        }

        #[inline]
        pub fn depth_attachment(
            mut self,
            depth_attachment: &'a RenderingAttachmentInfo<'a>,
        ) -> Self {
            self.p_depth_attachment = depth_attachment;
            self
        }

        #[inline]
        pub fn stencil_attachment(
            mut self,
            stencil_attachment: &'a RenderingAttachmentInfo<'a>,
        ) -> Self {
            self.p_stencil_attachment = stencil_attachment;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingAttachmentInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RenderingAttachmentInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image_view: ImageView,
        pub image_layout: ImageLayout,
        pub resolve_mode: ResolveModeFlagBits,
        pub resolve_image_view: ImageView,
        pub resolve_image_layout: ImageLayout,
        pub load_op: AttachmentLoadOp,
        pub store_op: AttachmentStoreOp,
        pub clear_value: ClearValue,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderingAttachmentInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderingAttachmentInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image_view", &self.image_view)
                .field("image_layout", &self.image_layout)
                .field("resolve_mode", &self.resolve_mode)
                .field("resolve_image_view", &self.resolve_image_view)
                .field("resolve_image_layout", &self.resolve_image_layout)
                .field("load_op", &self.load_op)
                .field("store_op", &self.store_op)
                .field("clear_value", &self.clear_value)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderingAttachmentInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDERING_ATTACHMENT_INFO;
    }

    impl Default for RenderingAttachmentInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                image_view: Default::default(),
                image_layout: Default::default(),
                resolve_mode: Default::default(),
                resolve_image_view: Default::default(),
                resolve_image_layout: Default::default(),
                load_op: Default::default(),
                store_op: Default::default(),
                clear_value: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RenderingAttachmentInfo<'a> {
        #[inline]
        pub fn image_view(mut self, image_view: ImageView) -> Self {
            self.image_view = image_view;
            self
        }

        #[inline]
        pub fn image_layout(mut self, image_layout: ImageLayout) -> Self {
            self.image_layout = image_layout;
            self
        }

        #[inline]
        pub fn resolve_mode(mut self, resolve_mode: ResolveModeFlagBits) -> Self {
            self.resolve_mode = resolve_mode;
            self
        }

        #[inline]
        pub fn resolve_image_view(mut self, resolve_image_view: ImageView) -> Self {
            self.resolve_image_view = resolve_image_view;
            self
        }

        #[inline]
        pub fn resolve_image_layout(mut self, resolve_image_layout: ImageLayout) -> Self {
            self.resolve_image_layout = resolve_image_layout;
            self
        }

        #[inline]
        pub fn load_op(mut self, load_op: AttachmentLoadOp) -> Self {
            self.load_op = load_op;
            self
        }

        #[inline]
        pub fn store_op(mut self, store_op: AttachmentStoreOp) -> Self {
            self.store_op = store_op;
            self
        }

        #[inline]
        pub fn clear_value(mut self, clear_value: ClearValue) -> Self {
            self.clear_value = clear_value;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDynamicRenderingFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDynamicRenderingFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub dynamic_rendering: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDynamicRenderingFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDynamicRenderingFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("dynamic_rendering", &self.dynamic_rendering)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDynamicRenderingFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceDynamicRenderingFeatures<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceDynamicRenderingFeatures<'_> {}

    impl Default for PhysicalDeviceDynamicRenderingFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                dynamic_rendering: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDynamicRenderingFeatures<'a> {
        #[inline]
        pub fn dynamic_rendering(mut self, dynamic_rendering: bool) -> Self {
            self.dynamic_rendering = dynamic_rendering.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCommandBufferInheritanceRenderingInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CommandBufferInheritanceRenderingInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: RenderingFlags,
        pub view_mask: u32,
        pub color_attachment_count: u32,
        pub p_color_attachment_formats: *const Format,
        pub depth_attachment_format: Format,
        pub stencil_attachment_format: Format,
        pub rasterization_samples: SampleCountFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CommandBufferInheritanceRenderingInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CommandBufferInheritanceRenderingInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("view_mask", &self.view_mask)
                .field("color_attachment_count", &self.color_attachment_count)
                .field(
                    "p_color_attachment_formats",
                    &self.p_color_attachment_formats,
                )
                .field("depth_attachment_format", &self.depth_attachment_format)
                .field("stencil_attachment_format", &self.stencil_attachment_format)
                .field("rasterization_samples", &self.rasterization_samples)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CommandBufferInheritanceRenderingInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO;
    }

    unsafe impl Extends<CommandBufferInheritanceInfo<'_>>
        for CommandBufferInheritanceRenderingInfo<'_>
    {
    }

    impl Default for CommandBufferInheritanceRenderingInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                view_mask: Default::default(),
                color_attachment_count: Default::default(),
                p_color_attachment_formats: ptr::null(),
                depth_attachment_format: Default::default(),
                stencil_attachment_format: Default::default(),
                rasterization_samples: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CommandBufferInheritanceRenderingInfo<'a> {
        #[inline]
        pub fn flags(mut self, flags: RenderingFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn view_mask(mut self, view_mask: u32) -> Self {
            self.view_mask = view_mask;
            self
        }

        #[inline]
        pub fn color_attachment_formats(mut self, color_attachment_formats: &'a [Format]) -> Self {
            self.color_attachment_count = color_attachment_formats.len().try_into().unwrap();
            self.p_color_attachment_formats = color_attachment_formats.as_ptr() as _;
            self
        }

        #[inline]
        pub fn depth_attachment_format(mut self, depth_attachment_format: Format) -> Self {
            self.depth_attachment_format = depth_attachment_format;
            self
        }

        #[inline]
        pub fn stencil_attachment_format(mut self, stencil_attachment_format: Format) -> Self {
            self.stencil_attachment_format = stencil_attachment_format;
            self
        }

        #[inline]
        pub fn rasterization_samples(mut self, rasterization_samples: SampleCountFlagBits) -> Self {
            self.rasterization_samples = rasterization_samples;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPrivateDataSlotCreateFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PrivateDataSlotCreateFlags(Flags);
    vk_bitflags_wrapped!(PrivateDataSlotCreateFlags, Flags);

    impl fmt::Debug for PrivateDataSlotCreateFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCreationFeedbackFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineCreationFeedbackFlags(Flags);
    vk_bitflags_wrapped!(
        PipelineCreationFeedbackFlags,
        Flags,
        PipelineCreationFeedbackFlagBits
    );

    impl fmt::Debug for PipelineCreationFeedbackFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (PipelineCreationFeedbackFlagBits::VALID.0, "VALID"),
                (
                    PipelineCreationFeedbackFlagBits::APPLICATION_PIPELINE_CACHE_HIT.0,
                    "APPLICATION_PIPELINE_CACHE_HIT",
                ),
                (
                    PipelineCreationFeedbackFlagBits::BASE_PIPELINE_ACCELERATION.0,
                    "BASE_PIPELINE_ACCELERATION",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCreationFeedbackFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PipelineCreationFeedbackFlagBits(u32);

    impl PipelineCreationFeedbackFlagBits {
        pub const VALID: Self = Self(1 << 0);
        pub const APPLICATION_PIPELINE_CACHE_HIT: Self = Self(1 << 1);
        pub const BASE_PIPELINE_ACCELERATION: Self = Self(1 << 2);
        // VK_EXT_pipeline_creation_feedback
        pub const VALID_EXT: Self = Self::VALID;
        pub const APPLICATION_PIPELINE_CACHE_HIT_EXT: Self = Self::APPLICATION_PIPELINE_CACHE_HIT;
        pub const BASE_PIPELINE_ACCELERATION_EXT: Self = Self::BASE_PIPELINE_ACCELERATION;
    }

    impl fmt::Debug for PipelineCreationFeedbackFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::VALID => Some("VALID"),
                Self::APPLICATION_PIPELINE_CACHE_HIT => Some("APPLICATION_PIPELINE_CACHE_HIT"),
                Self::BASE_PIPELINE_ACCELERATION => Some("BASE_PIPELINE_ACCELERATION"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccessFlags2.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct AccessFlags2(Flags64);
    vk_bitflags_wrapped!(AccessFlags2, Flags64, AccessFlagBits2);

    impl AccessFlags2 {
        pub const NONE: Self = Self(0);
    }

    impl fmt::Debug for AccessFlags2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags64, &str)] = &[
                (
                    AccessFlagBits2::INDIRECT_COMMAND_READ.0,
                    "INDIRECT_COMMAND_READ",
                ),
                (AccessFlagBits2::INDEX_READ.0, "INDEX_READ"),
                (
                    AccessFlagBits2::VERTEX_ATTRIBUTE_READ.0,
                    "VERTEX_ATTRIBUTE_READ",
                ),
                (AccessFlagBits2::UNIFORM_READ.0, "UNIFORM_READ"),
                (
                    AccessFlagBits2::INPUT_ATTACHMENT_READ.0,
                    "INPUT_ATTACHMENT_READ",
                ),
                (AccessFlagBits2::SHADER_READ.0, "SHADER_READ"),
                (AccessFlagBits2::SHADER_WRITE.0, "SHADER_WRITE"),
                (
                    AccessFlagBits2::COLOR_ATTACHMENT_READ.0,
                    "COLOR_ATTACHMENT_READ",
                ),
                (
                    AccessFlagBits2::COLOR_ATTACHMENT_WRITE.0,
                    "COLOR_ATTACHMENT_WRITE",
                ),
                (
                    AccessFlagBits2::DEPTH_STENCIL_ATTACHMENT_READ.0,
                    "DEPTH_STENCIL_ATTACHMENT_READ",
                ),
                (
                    AccessFlagBits2::DEPTH_STENCIL_ATTACHMENT_WRITE.0,
                    "DEPTH_STENCIL_ATTACHMENT_WRITE",
                ),
                (AccessFlagBits2::TRANSFER_READ.0, "TRANSFER_READ"),
                (AccessFlagBits2::TRANSFER_WRITE.0, "TRANSFER_WRITE"),
                (AccessFlagBits2::HOST_READ.0, "HOST_READ"),
                (AccessFlagBits2::HOST_WRITE.0, "HOST_WRITE"),
                (AccessFlagBits2::MEMORY_READ.0, "MEMORY_READ"),
                (AccessFlagBits2::MEMORY_WRITE.0, "MEMORY_WRITE"),
                (
                    AccessFlagBits2::SHADER_SAMPLED_READ.0,
                    "SHADER_SAMPLED_READ",
                ),
                (
                    AccessFlagBits2::SHADER_STORAGE_READ.0,
                    "SHADER_STORAGE_READ",
                ),
                (
                    AccessFlagBits2::SHADER_STORAGE_WRITE.0,
                    "SHADER_STORAGE_WRITE",
                ),
                (
                    AccessFlagBits2::DATA_GRAPH_READ_ARM.0,
                    "DATA_GRAPH_READ_ARM",
                ),
                (
                    AccessFlagBits2::DATA_GRAPH_WRITE_ARM.0,
                    "DATA_GRAPH_WRITE_ARM",
                ),
                (
                    AccessFlagBits2::DESCRIPTOR_BUFFER_READ_EXT.0,
                    "DESCRIPTOR_BUFFER_READ_EXT",
                ),
                (
                    AccessFlagBits2::SAMPLER_HEAP_READ_EXT.0,
                    "SAMPLER_HEAP_READ_EXT",
                ),
                (
                    AccessFlagBits2::RESOURCE_HEAP_READ_EXT.0,
                    "RESOURCE_HEAP_READ_EXT",
                ),
                (
                    AccessFlagBits2::MEMORY_DECOMPRESSION_READ_EXT.0,
                    "MEMORY_DECOMPRESSION_READ_EXT",
                ),
                (
                    AccessFlagBits2::MEMORY_DECOMPRESSION_WRITE_EXT.0,
                    "MEMORY_DECOMPRESSION_WRITE_EXT",
                ),
                (AccessFlagBits2::MICROMAP_READ_EXT.0, "MICROMAP_READ_EXT"),
                (AccessFlagBits2::MICROMAP_WRITE_EXT.0, "MICROMAP_WRITE_EXT"),
                (
                    AccessFlagBits2::INVOCATION_MASK_READ_HUAWEI.0,
                    "INVOCATION_MASK_READ_HUAWEI",
                ),
                (
                    AccessFlagBits2::SHADER_BINDING_TABLE_READ_KHR.0,
                    "SHADER_BINDING_TABLE_READ_KHR",
                ),
                (
                    AccessFlagBits2::COMMAND_PREPROCESS_READ_EXT.0,
                    "COMMAND_PREPROCESS_READ_EXT",
                ),
                (
                    AccessFlagBits2::COMMAND_PREPROCESS_WRITE_EXT.0,
                    "COMMAND_PREPROCESS_WRITE_EXT",
                ),
                (
                    AccessFlagBits2::COLOR_ATTACHMENT_READ_NONCOHERENT_EXT.0,
                    "COLOR_ATTACHMENT_READ_NONCOHERENT_EXT",
                ),
                (
                    AccessFlagBits2::CONDITIONAL_RENDERING_READ_EXT.0,
                    "CONDITIONAL_RENDERING_READ_EXT",
                ),
                (
                    AccessFlagBits2::ACCELERATION_STRUCTURE_READ_KHR.0,
                    "ACCELERATION_STRUCTURE_READ_KHR",
                ),
                (
                    AccessFlagBits2::ACCELERATION_STRUCTURE_WRITE_KHR.0,
                    "ACCELERATION_STRUCTURE_WRITE_KHR",
                ),
                (
                    AccessFlagBits2::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR.0,
                    "FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR",
                ),
                (
                    AccessFlagBits2::FRAGMENT_DENSITY_MAP_READ_EXT.0,
                    "FRAGMENT_DENSITY_MAP_READ_EXT",
                ),
                (
                    AccessFlagBits2::TRANSFORM_FEEDBACK_WRITE_EXT.0,
                    "TRANSFORM_FEEDBACK_WRITE_EXT",
                ),
                (
                    AccessFlagBits2::TRANSFORM_FEEDBACK_COUNTER_READ_EXT.0,
                    "TRANSFORM_FEEDBACK_COUNTER_READ_EXT",
                ),
                (
                    AccessFlagBits2::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT.0,
                    "TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT",
                ),
                (
                    AccessFlagBits2::VIDEO_DECODE_READ_KHR.0,
                    "VIDEO_DECODE_READ_KHR",
                ),
                (
                    AccessFlagBits2::VIDEO_DECODE_WRITE_KHR.0,
                    "VIDEO_DECODE_WRITE_KHR",
                ),
                (
                    AccessFlagBits2::VIDEO_ENCODE_READ_KHR.0,
                    "VIDEO_ENCODE_READ_KHR",
                ),
                (
                    AccessFlagBits2::VIDEO_ENCODE_WRITE_KHR.0,
                    "VIDEO_ENCODE_WRITE_KHR",
                ),
                (
                    AccessFlagBits2::OPTICAL_FLOW_READ_NV.0,
                    "OPTICAL_FLOW_READ_NV",
                ),
                (
                    AccessFlagBits2::OPTICAL_FLOW_WRITE_NV.0,
                    "OPTICAL_FLOW_WRITE_NV",
                ),
                (
                    AccessFlagBits2::SHADER_TILE_ATTACHMENT_READ_QCOM.0,
                    "SHADER_TILE_ATTACHMENT_READ_QCOM",
                ),
                (
                    AccessFlagBits2::SHADER_TILE_ATTACHMENT_WRITE_QCOM.0,
                    "SHADER_TILE_ATTACHMENT_WRITE_QCOM",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccessFlagBits2.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct AccessFlagBits2(u64);

    impl AccessFlagBits2 {
        pub const INDIRECT_COMMAND_READ: Self = Self(1 << 0);
        pub const INDEX_READ: Self = Self(1 << 1);
        pub const VERTEX_ATTRIBUTE_READ: Self = Self(1 << 2);
        pub const UNIFORM_READ: Self = Self(1 << 3);
        pub const INPUT_ATTACHMENT_READ: Self = Self(1 << 4);
        pub const SHADER_READ: Self = Self(1 << 5);
        pub const SHADER_WRITE: Self = Self(1 << 6);
        pub const COLOR_ATTACHMENT_READ: Self = Self(1 << 7);
        pub const COLOR_ATTACHMENT_WRITE: Self = Self(1 << 8);
        pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(1 << 9);
        pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(1 << 10);
        pub const TRANSFER_READ: Self = Self(1 << 11);
        pub const TRANSFER_WRITE: Self = Self(1 << 12);
        pub const HOST_READ: Self = Self(1 << 13);
        pub const HOST_WRITE: Self = Self(1 << 14);
        pub const MEMORY_READ: Self = Self(1 << 15);
        pub const MEMORY_WRITE: Self = Self(1 << 16);
        pub const SHADER_SAMPLED_READ: Self = Self(1 << 32);
        pub const SHADER_STORAGE_READ: Self = Self(1 << 33);
        pub const SHADER_STORAGE_WRITE: Self = Self(1 << 34);
        // VK_ARM_data_graph
        pub const DATA_GRAPH_READ_ARM: Self = Self(1 << 47);
        pub const DATA_GRAPH_WRITE_ARM: Self = Self(1 << 48);

        // VK_EXT_descriptor_buffer
        pub const DESCRIPTOR_BUFFER_READ_EXT: Self = Self(1 << 41);

        // VK_EXT_descriptor_heap
        pub const SAMPLER_HEAP_READ_EXT: Self = Self(1 << 57);
        pub const RESOURCE_HEAP_READ_EXT: Self = Self(1 << 58);

        // VK_EXT_memory_decompression
        pub const MEMORY_DECOMPRESSION_READ_EXT: Self = Self(1 << 55);
        pub const MEMORY_DECOMPRESSION_WRITE_EXT: Self = Self(1 << 56);

        // VK_EXT_opacity_micromap
        pub const MICROMAP_READ_EXT: Self = Self(1 << 44);
        pub const MICROMAP_WRITE_EXT: Self = Self(1 << 45);

        // VK_HUAWEI_invocation_mask
        pub const INVOCATION_MASK_READ_HUAWEI: Self = Self(1 << 39);

        // VK_KHR_ray_tracing_maintenance1
        pub const SHADER_BINDING_TABLE_READ_KHR: Self = Self(1 << 40);

        // VK_KHR_synchronization2
        pub const COMMAND_PREPROCESS_READ_EXT: Self = Self(1 << 17);
        pub const COMMAND_PREPROCESS_WRITE_EXT: Self = Self(1 << 18);
        pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(1 << 19);
        /// read access flag for reading conditional rendering predicate
        pub const CONDITIONAL_RENDERING_READ_EXT: Self = Self(1 << 20);
        pub const ACCELERATION_STRUCTURE_READ_KHR: Self = Self(1 << 21);
        pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(1 << 22);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self = Self(1 << 23);
        pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(1 << 24);
        pub const TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(1 << 25);
        pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(1 << 26);
        pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self = Self(1 << 27);
        pub const INDIRECT_COMMAND_READ_KHR: Self = Self::INDIRECT_COMMAND_READ;
        pub const INDEX_READ_KHR: Self = Self::INDEX_READ;
        pub const VERTEX_ATTRIBUTE_READ_KHR: Self = Self::VERTEX_ATTRIBUTE_READ;
        pub const UNIFORM_READ_KHR: Self = Self::UNIFORM_READ;
        pub const INPUT_ATTACHMENT_READ_KHR: Self = Self::INPUT_ATTACHMENT_READ;
        pub const SHADER_READ_KHR: Self = Self::SHADER_READ;
        pub const SHADER_WRITE_KHR: Self = Self::SHADER_WRITE;
        pub const COLOR_ATTACHMENT_READ_KHR: Self = Self::COLOR_ATTACHMENT_READ;
        pub const COLOR_ATTACHMENT_WRITE_KHR: Self = Self::COLOR_ATTACHMENT_WRITE;
        pub const DEPTH_STENCIL_ATTACHMENT_READ_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT_READ;
        pub const DEPTH_STENCIL_ATTACHMENT_WRITE_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT_WRITE;
        pub const TRANSFER_READ_KHR: Self = Self::TRANSFER_READ;
        pub const TRANSFER_WRITE_KHR: Self = Self::TRANSFER_WRITE;
        pub const HOST_READ_KHR: Self = Self::HOST_READ;
        pub const HOST_WRITE_KHR: Self = Self::HOST_WRITE;
        pub const MEMORY_READ_KHR: Self = Self::MEMORY_READ;
        pub const MEMORY_WRITE_KHR: Self = Self::MEMORY_WRITE;
        pub const SHADER_SAMPLED_READ_KHR: Self = Self::SHADER_SAMPLED_READ;
        pub const SHADER_STORAGE_READ_KHR: Self = Self::SHADER_STORAGE_READ;
        pub const SHADER_STORAGE_WRITE_KHR: Self = Self::SHADER_STORAGE_WRITE;
        pub const COMMAND_PREPROCESS_READ_NV: Self = Self::COMMAND_PREPROCESS_READ_EXT;
        pub const COMMAND_PREPROCESS_WRITE_NV: Self = Self::COMMAND_PREPROCESS_WRITE_EXT;
        pub const SHADING_RATE_IMAGE_READ_NV: Self =
            Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
        pub const ACCELERATION_STRUCTURE_READ_NV: Self = Self::ACCELERATION_STRUCTURE_READ_KHR;
        pub const ACCELERATION_STRUCTURE_WRITE_NV: Self = Self::ACCELERATION_STRUCTURE_WRITE_KHR;

        // VK_KHR_video_decode_queue
        pub const VIDEO_DECODE_READ_KHR: Self = Self(1 << 35);
        pub const VIDEO_DECODE_WRITE_KHR: Self = Self(1 << 36);

        // VK_KHR_video_encode_queue
        pub const VIDEO_ENCODE_READ_KHR: Self = Self(1 << 37);
        pub const VIDEO_ENCODE_WRITE_KHR: Self = Self(1 << 38);

        // VK_NV_optical_flow
        pub const OPTICAL_FLOW_READ_NV: Self = Self(1 << 42);
        pub const OPTICAL_FLOW_WRITE_NV: Self = Self(1 << 43);

        // VK_QCOM_tile_shading
        pub const SHADER_TILE_ATTACHMENT_READ_QCOM: Self = Self(1 << 51);
        pub const SHADER_TILE_ATTACHMENT_WRITE_QCOM: Self = Self(1 << 52);
    }

    impl fmt::Debug for AccessFlagBits2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::INDIRECT_COMMAND_READ => Some("INDIRECT_COMMAND_READ"),
                Self::INDEX_READ => Some("INDEX_READ"),
                Self::VERTEX_ATTRIBUTE_READ => Some("VERTEX_ATTRIBUTE_READ"),
                Self::UNIFORM_READ => Some("UNIFORM_READ"),
                Self::INPUT_ATTACHMENT_READ => Some("INPUT_ATTACHMENT_READ"),
                Self::SHADER_READ => Some("SHADER_READ"),
                Self::SHADER_WRITE => Some("SHADER_WRITE"),
                Self::COLOR_ATTACHMENT_READ => Some("COLOR_ATTACHMENT_READ"),
                Self::COLOR_ATTACHMENT_WRITE => Some("COLOR_ATTACHMENT_WRITE"),
                Self::DEPTH_STENCIL_ATTACHMENT_READ => Some("DEPTH_STENCIL_ATTACHMENT_READ"),
                Self::DEPTH_STENCIL_ATTACHMENT_WRITE => Some("DEPTH_STENCIL_ATTACHMENT_WRITE"),
                Self::TRANSFER_READ => Some("TRANSFER_READ"),
                Self::TRANSFER_WRITE => Some("TRANSFER_WRITE"),
                Self::HOST_READ => Some("HOST_READ"),
                Self::HOST_WRITE => Some("HOST_WRITE"),
                Self::MEMORY_READ => Some("MEMORY_READ"),
                Self::MEMORY_WRITE => Some("MEMORY_WRITE"),
                Self::SHADER_SAMPLED_READ => Some("SHADER_SAMPLED_READ"),
                Self::SHADER_STORAGE_READ => Some("SHADER_STORAGE_READ"),
                Self::SHADER_STORAGE_WRITE => Some("SHADER_STORAGE_WRITE"),
                Self::DATA_GRAPH_READ_ARM => Some("DATA_GRAPH_READ_ARM"),
                Self::DATA_GRAPH_WRITE_ARM => Some("DATA_GRAPH_WRITE_ARM"),
                Self::DESCRIPTOR_BUFFER_READ_EXT => Some("DESCRIPTOR_BUFFER_READ_EXT"),
                Self::SAMPLER_HEAP_READ_EXT => Some("SAMPLER_HEAP_READ_EXT"),
                Self::RESOURCE_HEAP_READ_EXT => Some("RESOURCE_HEAP_READ_EXT"),
                Self::MEMORY_DECOMPRESSION_READ_EXT => Some("MEMORY_DECOMPRESSION_READ_EXT"),
                Self::MEMORY_DECOMPRESSION_WRITE_EXT => Some("MEMORY_DECOMPRESSION_WRITE_EXT"),
                Self::MICROMAP_READ_EXT => Some("MICROMAP_READ_EXT"),
                Self::MICROMAP_WRITE_EXT => Some("MICROMAP_WRITE_EXT"),
                Self::INVOCATION_MASK_READ_HUAWEI => Some("INVOCATION_MASK_READ_HUAWEI"),
                Self::SHADER_BINDING_TABLE_READ_KHR => Some("SHADER_BINDING_TABLE_READ_KHR"),
                Self::COMMAND_PREPROCESS_READ_EXT => Some("COMMAND_PREPROCESS_READ_EXT"),
                Self::COMMAND_PREPROCESS_WRITE_EXT => Some("COMMAND_PREPROCESS_WRITE_EXT"),
                Self::COLOR_ATTACHMENT_READ_NONCOHERENT_EXT => {
                    Some("COLOR_ATTACHMENT_READ_NONCOHERENT_EXT")
                }
                Self::CONDITIONAL_RENDERING_READ_EXT => Some("CONDITIONAL_RENDERING_READ_EXT"),
                Self::ACCELERATION_STRUCTURE_READ_KHR => Some("ACCELERATION_STRUCTURE_READ_KHR"),
                Self::ACCELERATION_STRUCTURE_WRITE_KHR => Some("ACCELERATION_STRUCTURE_WRITE_KHR"),
                Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR => {
                    Some("FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR")
                }
                Self::FRAGMENT_DENSITY_MAP_READ_EXT => Some("FRAGMENT_DENSITY_MAP_READ_EXT"),
                Self::TRANSFORM_FEEDBACK_WRITE_EXT => Some("TRANSFORM_FEEDBACK_WRITE_EXT"),
                Self::TRANSFORM_FEEDBACK_COUNTER_READ_EXT => {
                    Some("TRANSFORM_FEEDBACK_COUNTER_READ_EXT")
                }
                Self::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT => {
                    Some("TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT")
                }
                Self::VIDEO_DECODE_READ_KHR => Some("VIDEO_DECODE_READ_KHR"),
                Self::VIDEO_DECODE_WRITE_KHR => Some("VIDEO_DECODE_WRITE_KHR"),
                Self::VIDEO_ENCODE_READ_KHR => Some("VIDEO_ENCODE_READ_KHR"),
                Self::VIDEO_ENCODE_WRITE_KHR => Some("VIDEO_ENCODE_WRITE_KHR"),
                Self::OPTICAL_FLOW_READ_NV => Some("OPTICAL_FLOW_READ_NV"),
                Self::OPTICAL_FLOW_WRITE_NV => Some("OPTICAL_FLOW_WRITE_NV"),
                Self::SHADER_TILE_ATTACHMENT_READ_QCOM => Some("SHADER_TILE_ATTACHMENT_READ_QCOM"),
                Self::SHADER_TILE_ATTACHMENT_WRITE_QCOM => {
                    Some("SHADER_TILE_ATTACHMENT_WRITE_QCOM")
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineStageFlags2.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineStageFlags2(Flags64);
    vk_bitflags_wrapped!(PipelineStageFlags2, Flags64, PipelineStageFlagBits2);

    impl PipelineStageFlags2 {
        pub const NONE: Self = Self(0);
    }

    impl fmt::Debug for PipelineStageFlags2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags64, &str)] = &[
                (PipelineStageFlagBits2::TOP_OF_PIPE.0, "TOP_OF_PIPE"),
                (PipelineStageFlagBits2::DRAW_INDIRECT.0, "DRAW_INDIRECT"),
                (PipelineStageFlagBits2::VERTEX_INPUT.0, "VERTEX_INPUT"),
                (PipelineStageFlagBits2::VERTEX_SHADER.0, "VERTEX_SHADER"),
                (
                    PipelineStageFlagBits2::TESSELLATION_CONTROL_SHADER.0,
                    "TESSELLATION_CONTROL_SHADER",
                ),
                (
                    PipelineStageFlagBits2::TESSELLATION_EVALUATION_SHADER.0,
                    "TESSELLATION_EVALUATION_SHADER",
                ),
                (PipelineStageFlagBits2::GEOMETRY_SHADER.0, "GEOMETRY_SHADER"),
                (PipelineStageFlagBits2::FRAGMENT_SHADER.0, "FRAGMENT_SHADER"),
                (
                    PipelineStageFlagBits2::EARLY_FRAGMENT_TESTS.0,
                    "EARLY_FRAGMENT_TESTS",
                ),
                (
                    PipelineStageFlagBits2::LATE_FRAGMENT_TESTS.0,
                    "LATE_FRAGMENT_TESTS",
                ),
                (
                    PipelineStageFlagBits2::COLOR_ATTACHMENT_OUTPUT.0,
                    "COLOR_ATTACHMENT_OUTPUT",
                ),
                (PipelineStageFlagBits2::COMPUTE_SHADER.0, "COMPUTE_SHADER"),
                (PipelineStageFlagBits2::ALL_TRANSFER.0, "ALL_TRANSFER"),
                (PipelineStageFlagBits2::BOTTOM_OF_PIPE.0, "BOTTOM_OF_PIPE"),
                (PipelineStageFlagBits2::HOST.0, "HOST"),
                (PipelineStageFlagBits2::ALL_GRAPHICS.0, "ALL_GRAPHICS"),
                (PipelineStageFlagBits2::ALL_COMMANDS.0, "ALL_COMMANDS"),
                (PipelineStageFlagBits2::COPY.0, "COPY"),
                (PipelineStageFlagBits2::RESOLVE.0, "RESOLVE"),
                (PipelineStageFlagBits2::BLIT.0, "BLIT"),
                (PipelineStageFlagBits2::CLEAR.0, "CLEAR"),
                (PipelineStageFlagBits2::INDEX_INPUT.0, "INDEX_INPUT"),
                (
                    PipelineStageFlagBits2::VERTEX_ATTRIBUTE_INPUT.0,
                    "VERTEX_ATTRIBUTE_INPUT",
                ),
                (
                    PipelineStageFlagBits2::PRE_RASTERIZATION_SHADERS.0,
                    "PRE_RASTERIZATION_SHADERS",
                ),
                (PipelineStageFlagBits2::DATA_GRAPH_ARM.0, "DATA_GRAPH_ARM"),
                (
                    PipelineStageFlagBits2::MEMORY_DECOMPRESSION_EXT.0,
                    "MEMORY_DECOMPRESSION_EXT",
                ),
                (
                    PipelineStageFlagBits2::MICROMAP_BUILD_EXT.0,
                    "MICROMAP_BUILD_EXT",
                ),
                (
                    PipelineStageFlagBits2::CLUSTER_CULLING_SHADER_HUAWEI.0,
                    "CLUSTER_CULLING_SHADER_HUAWEI",
                ),
                (
                    PipelineStageFlagBits2::INVOCATION_MASK_HUAWEI.0,
                    "INVOCATION_MASK_HUAWEI",
                ),
                (
                    PipelineStageFlagBits2::SUBPASS_SHADER_HUAWEI.0,
                    "SUBPASS_SHADER_HUAWEI",
                ),
                (
                    PipelineStageFlagBits2::COPY_INDIRECT_KHR.0,
                    "COPY_INDIRECT_KHR",
                ),
                (
                    PipelineStageFlagBits2::ACCELERATION_STRUCTURE_COPY_KHR.0,
                    "ACCELERATION_STRUCTURE_COPY_KHR",
                ),
                (
                    PipelineStageFlagBits2::COMMAND_PREPROCESS_EXT.0,
                    "COMMAND_PREPROCESS_EXT",
                ),
                (
                    PipelineStageFlagBits2::CONDITIONAL_RENDERING_EXT.0,
                    "CONDITIONAL_RENDERING_EXT",
                ),
                (PipelineStageFlagBits2::TASK_SHADER_EXT.0, "TASK_SHADER_EXT"),
                (PipelineStageFlagBits2::MESH_SHADER_EXT.0, "MESH_SHADER_EXT"),
                (
                    PipelineStageFlagBits2::RAY_TRACING_SHADER_KHR.0,
                    "RAY_TRACING_SHADER_KHR",
                ),
                (
                    PipelineStageFlagBits2::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0,
                    "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR",
                ),
                (
                    PipelineStageFlagBits2::FRAGMENT_DENSITY_PROCESS_EXT.0,
                    "FRAGMENT_DENSITY_PROCESS_EXT",
                ),
                (
                    PipelineStageFlagBits2::TRANSFORM_FEEDBACK_EXT.0,
                    "TRANSFORM_FEEDBACK_EXT",
                ),
                (
                    PipelineStageFlagBits2::ACCELERATION_STRUCTURE_BUILD_KHR.0,
                    "ACCELERATION_STRUCTURE_BUILD_KHR",
                ),
                (
                    PipelineStageFlagBits2::VIDEO_DECODE_KHR.0,
                    "VIDEO_DECODE_KHR",
                ),
                (
                    PipelineStageFlagBits2::VIDEO_ENCODE_KHR.0,
                    "VIDEO_ENCODE_KHR",
                ),
                (
                    PipelineStageFlagBits2::CONVERT_COOPERATIVE_VECTOR_MATRIX_NV.0,
                    "CONVERT_COOPERATIVE_VECTOR_MATRIX_NV",
                ),
                (PipelineStageFlagBits2::OPTICAL_FLOW_NV.0, "OPTICAL_FLOW_NV"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineStageFlagBits2.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PipelineStageFlagBits2(u64);

    impl PipelineStageFlagBits2 {
        pub const TOP_OF_PIPE: Self = Self(1 << 0);
        pub const DRAW_INDIRECT: Self = Self(1 << 1);
        pub const VERTEX_INPUT: Self = Self(1 << 2);
        pub const VERTEX_SHADER: Self = Self(1 << 3);
        pub const TESSELLATION_CONTROL_SHADER: Self = Self(1 << 4);
        pub const TESSELLATION_EVALUATION_SHADER: Self = Self(1 << 5);
        pub const GEOMETRY_SHADER: Self = Self(1 << 6);
        pub const FRAGMENT_SHADER: Self = Self(1 << 7);
        pub const EARLY_FRAGMENT_TESTS: Self = Self(1 << 8);
        pub const LATE_FRAGMENT_TESTS: Self = Self(1 << 9);
        pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(1 << 10);
        pub const COMPUTE_SHADER: Self = Self(1 << 11);
        pub const ALL_TRANSFER: Self = Self(1 << 12);
        pub const BOTTOM_OF_PIPE: Self = Self(1 << 13);
        pub const HOST: Self = Self(1 << 14);
        pub const ALL_GRAPHICS: Self = Self(1 << 15);
        pub const ALL_COMMANDS: Self = Self(1 << 16);
        pub const COPY: Self = Self(1 << 32);
        pub const RESOLVE: Self = Self(1 << 33);
        pub const BLIT: Self = Self(1 << 34);
        pub const CLEAR: Self = Self(1 << 35);
        pub const INDEX_INPUT: Self = Self(1 << 36);
        pub const VERTEX_ATTRIBUTE_INPUT: Self = Self(1 << 37);
        pub const PRE_RASTERIZATION_SHADERS: Self = Self(1 << 38);
        pub const TRANSFER: Self = Self::ALL_TRANSFER;
        // VK_ARM_data_graph
        pub const DATA_GRAPH_ARM: Self = Self(1 << 42);

        // VK_EXT_memory_decompression
        pub const MEMORY_DECOMPRESSION_EXT: Self = Self(1 << 45);

        // VK_EXT_opacity_micromap
        pub const MICROMAP_BUILD_EXT: Self = Self(1 << 30);

        // VK_HUAWEI_cluster_culling_shader
        pub const CLUSTER_CULLING_SHADER_HUAWEI: Self = Self(1 << 41);

        // VK_HUAWEI_invocation_mask
        pub const INVOCATION_MASK_HUAWEI: Self = Self(1 << 40);

        // VK_HUAWEI_subpass_shading
        pub const SUBPASS_SHADER_HUAWEI: Self = Self(1 << 39);
        pub const SUBPASS_SHADING_HUAWEI: Self = Self::SUBPASS_SHADER_HUAWEI;

        // VK_KHR_copy_memory_indirect
        pub const COPY_INDIRECT_KHR: Self = Self(1 << 46);

        // VK_KHR_ray_tracing_maintenance1
        pub const ACCELERATION_STRUCTURE_COPY_KHR: Self = Self(1 << 28);

        // VK_KHR_synchronization2
        pub const COMMAND_PREPROCESS_EXT: Self = Self(1 << 17);
        /// A pipeline stage for conditional rendering predicate fetch
        pub const CONDITIONAL_RENDERING_EXT: Self = Self(1 << 18);
        pub const TASK_SHADER_EXT: Self = Self(1 << 19);
        pub const MESH_SHADER_EXT: Self = Self(1 << 20);
        pub const RAY_TRACING_SHADER_KHR: Self = Self(1 << 21);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 22);
        pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(1 << 23);
        pub const TRANSFORM_FEEDBACK_EXT: Self = Self(1 << 24);
        pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(1 << 25);
        pub const TOP_OF_PIPE_KHR: Self = Self::TOP_OF_PIPE;
        pub const DRAW_INDIRECT_KHR: Self = Self::DRAW_INDIRECT;
        pub const VERTEX_INPUT_KHR: Self = Self::VERTEX_INPUT;
        pub const VERTEX_SHADER_KHR: Self = Self::VERTEX_SHADER;
        pub const TESSELLATION_CONTROL_SHADER_KHR: Self = Self::TESSELLATION_CONTROL_SHADER;
        pub const TESSELLATION_EVALUATION_SHADER_KHR: Self = Self::TESSELLATION_EVALUATION_SHADER;
        pub const GEOMETRY_SHADER_KHR: Self = Self::GEOMETRY_SHADER;
        pub const FRAGMENT_SHADER_KHR: Self = Self::FRAGMENT_SHADER;
        pub const EARLY_FRAGMENT_TESTS_KHR: Self = Self::EARLY_FRAGMENT_TESTS;
        pub const LATE_FRAGMENT_TESTS_KHR: Self = Self::LATE_FRAGMENT_TESTS;
        pub const COLOR_ATTACHMENT_OUTPUT_KHR: Self = Self::COLOR_ATTACHMENT_OUTPUT;
        pub const COMPUTE_SHADER_KHR: Self = Self::COMPUTE_SHADER;
        pub const ALL_TRANSFER_KHR: Self = Self::ALL_TRANSFER;
        pub const TRANSFER_KHR: Self = Self::ALL_TRANSFER;
        pub const BOTTOM_OF_PIPE_KHR: Self = Self::BOTTOM_OF_PIPE;
        pub const HOST_KHR: Self = Self::HOST;
        pub const ALL_GRAPHICS_KHR: Self = Self::ALL_GRAPHICS;
        pub const ALL_COMMANDS_KHR: Self = Self::ALL_COMMANDS;
        pub const COPY_KHR: Self = Self::COPY;
        pub const RESOLVE_KHR: Self = Self::RESOLVE;
        pub const BLIT_KHR: Self = Self::BLIT;
        pub const CLEAR_KHR: Self = Self::CLEAR;
        pub const INDEX_INPUT_KHR: Self = Self::INDEX_INPUT;
        pub const VERTEX_ATTRIBUTE_INPUT_KHR: Self = Self::VERTEX_ATTRIBUTE_INPUT;
        pub const PRE_RASTERIZATION_SHADERS_KHR: Self = Self::PRE_RASTERIZATION_SHADERS;
        pub const COMMAND_PREPROCESS_NV: Self = Self::COMMAND_PREPROCESS_EXT;
        pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
        pub const RAY_TRACING_SHADER_NV: Self = Self::RAY_TRACING_SHADER_KHR;
        pub const ACCELERATION_STRUCTURE_BUILD_NV: Self = Self::ACCELERATION_STRUCTURE_BUILD_KHR;
        pub const TASK_SHADER_NV: Self = Self::TASK_SHADER_EXT;
        pub const MESH_SHADER_NV: Self = Self::MESH_SHADER_EXT;

        // VK_KHR_video_decode_queue
        pub const VIDEO_DECODE_KHR: Self = Self(1 << 26);

        // VK_KHR_video_encode_queue
        pub const VIDEO_ENCODE_KHR: Self = Self(1 << 27);

        // VK_NV_cooperative_vector
        pub const CONVERT_COOPERATIVE_VECTOR_MATRIX_NV: Self = Self(1 << 44);

        // VK_NV_optical_flow
        pub const OPTICAL_FLOW_NV: Self = Self(1 << 29);
    }

    impl fmt::Debug for PipelineStageFlagBits2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::TOP_OF_PIPE => Some("TOP_OF_PIPE"),
                Self::DRAW_INDIRECT => Some("DRAW_INDIRECT"),
                Self::VERTEX_INPUT => Some("VERTEX_INPUT"),
                Self::VERTEX_SHADER => Some("VERTEX_SHADER"),
                Self::TESSELLATION_CONTROL_SHADER => Some("TESSELLATION_CONTROL_SHADER"),
                Self::TESSELLATION_EVALUATION_SHADER => Some("TESSELLATION_EVALUATION_SHADER"),
                Self::GEOMETRY_SHADER => Some("GEOMETRY_SHADER"),
                Self::FRAGMENT_SHADER => Some("FRAGMENT_SHADER"),
                Self::EARLY_FRAGMENT_TESTS => Some("EARLY_FRAGMENT_TESTS"),
                Self::LATE_FRAGMENT_TESTS => Some("LATE_FRAGMENT_TESTS"),
                Self::COLOR_ATTACHMENT_OUTPUT => Some("COLOR_ATTACHMENT_OUTPUT"),
                Self::COMPUTE_SHADER => Some("COMPUTE_SHADER"),
                Self::ALL_TRANSFER => Some("ALL_TRANSFER"),
                Self::BOTTOM_OF_PIPE => Some("BOTTOM_OF_PIPE"),
                Self::HOST => Some("HOST"),
                Self::ALL_GRAPHICS => Some("ALL_GRAPHICS"),
                Self::ALL_COMMANDS => Some("ALL_COMMANDS"),
                Self::COPY => Some("COPY"),
                Self::RESOLVE => Some("RESOLVE"),
                Self::BLIT => Some("BLIT"),
                Self::CLEAR => Some("CLEAR"),
                Self::INDEX_INPUT => Some("INDEX_INPUT"),
                Self::VERTEX_ATTRIBUTE_INPUT => Some("VERTEX_ATTRIBUTE_INPUT"),
                Self::PRE_RASTERIZATION_SHADERS => Some("PRE_RASTERIZATION_SHADERS"),
                Self::DATA_GRAPH_ARM => Some("DATA_GRAPH_ARM"),
                Self::MEMORY_DECOMPRESSION_EXT => Some("MEMORY_DECOMPRESSION_EXT"),
                Self::MICROMAP_BUILD_EXT => Some("MICROMAP_BUILD_EXT"),
                Self::CLUSTER_CULLING_SHADER_HUAWEI => Some("CLUSTER_CULLING_SHADER_HUAWEI"),
                Self::INVOCATION_MASK_HUAWEI => Some("INVOCATION_MASK_HUAWEI"),
                Self::SUBPASS_SHADER_HUAWEI => Some("SUBPASS_SHADER_HUAWEI"),
                Self::COPY_INDIRECT_KHR => Some("COPY_INDIRECT_KHR"),
                Self::ACCELERATION_STRUCTURE_COPY_KHR => Some("ACCELERATION_STRUCTURE_COPY_KHR"),
                Self::COMMAND_PREPROCESS_EXT => Some("COMMAND_PREPROCESS_EXT"),
                Self::CONDITIONAL_RENDERING_EXT => Some("CONDITIONAL_RENDERING_EXT"),
                Self::TASK_SHADER_EXT => Some("TASK_SHADER_EXT"),
                Self::MESH_SHADER_EXT => Some("MESH_SHADER_EXT"),
                Self::RAY_TRACING_SHADER_KHR => Some("RAY_TRACING_SHADER_KHR"),
                Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR => {
                    Some("FRAGMENT_SHADING_RATE_ATTACHMENT_KHR")
                }
                Self::FRAGMENT_DENSITY_PROCESS_EXT => Some("FRAGMENT_DENSITY_PROCESS_EXT"),
                Self::TRANSFORM_FEEDBACK_EXT => Some("TRANSFORM_FEEDBACK_EXT"),
                Self::ACCELERATION_STRUCTURE_BUILD_KHR => Some("ACCELERATION_STRUCTURE_BUILD_KHR"),
                Self::VIDEO_DECODE_KHR => Some("VIDEO_DECODE_KHR"),
                Self::VIDEO_ENCODE_KHR => Some("VIDEO_ENCODE_KHR"),
                Self::CONVERT_COOPERATIVE_VECTOR_MATRIX_NV => {
                    Some("CONVERT_COOPERATIVE_VECTOR_MATRIX_NV")
                }
                Self::OPTICAL_FLOW_NV => Some("OPTICAL_FLOW_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFormatFeatureFlags2.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct FormatFeatureFlags2(Flags64);
    vk_bitflags_wrapped!(FormatFeatureFlags2, Flags64, FormatFeatureFlagBits2);

    impl fmt::Debug for FormatFeatureFlags2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags64, &str)] = &[
                (FormatFeatureFlagBits2::SAMPLED_IMAGE.0, "SAMPLED_IMAGE"),
                (FormatFeatureFlagBits2::STORAGE_IMAGE.0, "STORAGE_IMAGE"),
                (FormatFeatureFlagBits2::STORAGE_IMAGE_ATOMIC.0, "STORAGE_IMAGE_ATOMIC"),
                (FormatFeatureFlagBits2::UNIFORM_TEXEL_BUFFER.0, "UNIFORM_TEXEL_BUFFER"),
                (FormatFeatureFlagBits2::STORAGE_TEXEL_BUFFER.0, "STORAGE_TEXEL_BUFFER"),
                (FormatFeatureFlagBits2::STORAGE_TEXEL_BUFFER_ATOMIC.0, "STORAGE_TEXEL_BUFFER_ATOMIC"),
                (FormatFeatureFlagBits2::VERTEX_BUFFER.0, "VERTEX_BUFFER"),
                (FormatFeatureFlagBits2::COLOR_ATTACHMENT.0, "COLOR_ATTACHMENT"),
                (FormatFeatureFlagBits2::COLOR_ATTACHMENT_BLEND.0, "COLOR_ATTACHMENT_BLEND"),
                (FormatFeatureFlagBits2::DEPTH_STENCIL_ATTACHMENT.0, "DEPTH_STENCIL_ATTACHMENT"),
                (FormatFeatureFlagBits2::BLIT_SRC.0, "BLIT_SRC"),
                (FormatFeatureFlagBits2::BLIT_DST.0, "BLIT_DST"),
                (FormatFeatureFlagBits2::SAMPLED_IMAGE_FILTER_LINEAR.0, "SAMPLED_IMAGE_FILTER_LINEAR"),
                (FormatFeatureFlagBits2::TRANSFER_SRC.0, "TRANSFER_SRC"),
                (FormatFeatureFlagBits2::TRANSFER_DST.0, "TRANSFER_DST"),
                (FormatFeatureFlagBits2::SAMPLED_IMAGE_FILTER_MINMAX.0, "SAMPLED_IMAGE_FILTER_MINMAX"),
                (FormatFeatureFlagBits2::MIDPOINT_CHROMA_SAMPLES.0, "MIDPOINT_CHROMA_SAMPLES"),
                (FormatFeatureFlagBits2::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER.0, "SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER"),
                (FormatFeatureFlagBits2::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER.0, "SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER"),
                (FormatFeatureFlagBits2::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT.0, "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT"),
                (FormatFeatureFlagBits2::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE.0, "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE"),
                (FormatFeatureFlagBits2::DISJOINT.0, "DISJOINT"),
                (FormatFeatureFlagBits2::COSITED_CHROMA_SAMPLES.0, "COSITED_CHROMA_SAMPLES"),
                (FormatFeatureFlagBits2::STORAGE_READ_WITHOUT_FORMAT.0, "STORAGE_READ_WITHOUT_FORMAT"),
                (FormatFeatureFlagBits2::STORAGE_WRITE_WITHOUT_FORMAT.0, "STORAGE_WRITE_WITHOUT_FORMAT"),
                (FormatFeatureFlagBits2::SAMPLED_IMAGE_DEPTH_COMPARISON.0, "SAMPLED_IMAGE_DEPTH_COMPARISON"),
                (FormatFeatureFlagBits2::TENSOR_DATA_GRAPH_ARM.0, "TENSOR_DATA_GRAPH_ARM"),
                (FormatFeatureFlagBits2::TENSOR_SHADER_ARM.0, "TENSOR_SHADER_ARM"),
                (FormatFeatureFlagBits2::TENSOR_IMAGE_ALIASING_ARM.0, "TENSOR_IMAGE_ALIASING_ARM"),
                (FormatFeatureFlagBits2::FRAGMENT_DENSITY_MAP_EXT.0, "FRAGMENT_DENSITY_MAP_EXT"),
                (FormatFeatureFlagBits2::ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR.0, "ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR"),
                (FormatFeatureFlagBits2::COPY_IMAGE_INDIRECT_DST_KHR.0, "COPY_IMAGE_INDIRECT_DST_KHR"),
                (FormatFeatureFlagBits2::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0, "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR"),
                (FormatFeatureFlagBits2::DEPTH_COPY_ON_COMPUTE_QUEUE_KHR.0, "DEPTH_COPY_ON_COMPUTE_QUEUE_KHR"),
                (FormatFeatureFlagBits2::DEPTH_COPY_ON_TRANSFER_QUEUE_KHR.0, "DEPTH_COPY_ON_TRANSFER_QUEUE_KHR"),
                (FormatFeatureFlagBits2::STENCIL_COPY_ON_COMPUTE_QUEUE_KHR.0, "STENCIL_COPY_ON_COMPUTE_QUEUE_KHR"),
                (FormatFeatureFlagBits2::STENCIL_COPY_ON_TRANSFER_QUEUE_KHR.0, "STENCIL_COPY_ON_TRANSFER_QUEUE_KHR"),
                (FormatFeatureFlagBits2::VIDEO_DECODE_OUTPUT_KHR.0, "VIDEO_DECODE_OUTPUT_KHR"),
                (FormatFeatureFlagBits2::VIDEO_DECODE_DPB_KHR.0, "VIDEO_DECODE_DPB_KHR"),
                (FormatFeatureFlagBits2::VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR.0, "VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR"),
                (FormatFeatureFlagBits2::VIDEO_ENCODE_EMPHASIS_MAP_KHR.0, "VIDEO_ENCODE_EMPHASIS_MAP_KHR"),
                (FormatFeatureFlagBits2::VIDEO_ENCODE_INPUT_KHR.0, "VIDEO_ENCODE_INPUT_KHR"),
                (FormatFeatureFlagBits2::VIDEO_ENCODE_DPB_KHR.0, "VIDEO_ENCODE_DPB_KHR"),
                (FormatFeatureFlagBits2::LINEAR_COLOR_ATTACHMENT_NV.0, "LINEAR_COLOR_ATTACHMENT_NV"),
                (FormatFeatureFlagBits2::OPTICAL_FLOW_IMAGE_NV.0, "OPTICAL_FLOW_IMAGE_NV"),
                (FormatFeatureFlagBits2::OPTICAL_FLOW_VECTOR_NV.0, "OPTICAL_FLOW_VECTOR_NV"),
                (FormatFeatureFlagBits2::OPTICAL_FLOW_COST_NV.0, "OPTICAL_FLOW_COST_NV"),
                (FormatFeatureFlagBits2::ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV.0, "ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV"),
                (FormatFeatureFlagBits2::WEIGHT_IMAGE_QCOM.0, "WEIGHT_IMAGE_QCOM"),
                (FormatFeatureFlagBits2::WEIGHT_SAMPLED_IMAGE_QCOM.0, "WEIGHT_SAMPLED_IMAGE_QCOM"),
                (FormatFeatureFlagBits2::BLOCK_MATCHING_QCOM.0, "BLOCK_MATCHING_QCOM"),
                (FormatFeatureFlagBits2::BOX_FILTER_SAMPLED_QCOM.0, "BOX_FILTER_SAMPLED_QCOM"),
                (FormatFeatureFlagBits2::SAMPLED_IMAGE_FILTER_CUBIC.0, "SAMPLED_IMAGE_FILTER_CUBIC"),
                (FormatFeatureFlagBits2::HOST_IMAGE_TRANSFER.0, "HOST_IMAGE_TRANSFER"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFormatFeatureFlagBits2.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct FormatFeatureFlagBits2(u64);

    impl FormatFeatureFlagBits2 {
        pub const SAMPLED_IMAGE: Self = Self(1 << 0);
        pub const STORAGE_IMAGE: Self = Self(1 << 1);
        pub const STORAGE_IMAGE_ATOMIC: Self = Self(1 << 2);
        pub const UNIFORM_TEXEL_BUFFER: Self = Self(1 << 3);
        pub const STORAGE_TEXEL_BUFFER: Self = Self(1 << 4);
        pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(1 << 5);
        pub const VERTEX_BUFFER: Self = Self(1 << 6);
        pub const COLOR_ATTACHMENT: Self = Self(1 << 7);
        pub const COLOR_ATTACHMENT_BLEND: Self = Self(1 << 8);
        pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(1 << 9);
        pub const BLIT_SRC: Self = Self(1 << 10);
        pub const BLIT_DST: Self = Self(1 << 11);
        pub const SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(1 << 12);
        pub const TRANSFER_SRC: Self = Self(1 << 14);
        pub const TRANSFER_DST: Self = Self(1 << 15);
        pub const SAMPLED_IMAGE_FILTER_MINMAX: Self = Self(1 << 16);
        pub const MIDPOINT_CHROMA_SAMPLES: Self = Self(1 << 17);
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self(1 << 18);
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self =
            Self(1 << 19);
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self =
            Self(1 << 20);
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self =
            Self(1 << 21);
        pub const DISJOINT: Self = Self(1 << 22);
        pub const COSITED_CHROMA_SAMPLES: Self = Self(1 << 23);
        pub const STORAGE_READ_WITHOUT_FORMAT: Self = Self(1 << 31);
        pub const STORAGE_WRITE_WITHOUT_FORMAT: Self = Self(1 << 32);
        pub const SAMPLED_IMAGE_DEPTH_COMPARISON: Self = Self(1 << 33);
        // VK_ARM_data_graph
        pub const TENSOR_DATA_GRAPH_ARM: Self = Self(1 << 48);

        // VK_ARM_tensors
        pub const TENSOR_SHADER_ARM: Self = Self(1 << 39);
        pub const TENSOR_IMAGE_ALIASING_ARM: Self = Self(1 << 43);

        // VK_EXT_fragment_density_map
        pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(1 << 24);

        // VK_EXT_host_image_copy
        pub const HOST_IMAGE_TRANSFER_EXT: Self = Self::HOST_IMAGE_TRANSFER;

        // VK_KHR_acceleration_structure
        pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self = Self(1 << 29);

        // VK_KHR_copy_memory_indirect
        pub const COPY_IMAGE_INDIRECT_DST_KHR: Self = Self(1 << 59);

        // VK_KHR_format_feature_flags2
        pub const SAMPLED_IMAGE_KHR: Self = Self::SAMPLED_IMAGE;
        pub const STORAGE_IMAGE_KHR: Self = Self::STORAGE_IMAGE;
        pub const STORAGE_IMAGE_ATOMIC_KHR: Self = Self::STORAGE_IMAGE_ATOMIC;
        pub const UNIFORM_TEXEL_BUFFER_KHR: Self = Self::UNIFORM_TEXEL_BUFFER;
        pub const STORAGE_TEXEL_BUFFER_KHR: Self = Self::STORAGE_TEXEL_BUFFER;
        pub const STORAGE_TEXEL_BUFFER_ATOMIC_KHR: Self = Self::STORAGE_TEXEL_BUFFER_ATOMIC;
        pub const VERTEX_BUFFER_KHR: Self = Self::VERTEX_BUFFER;
        pub const COLOR_ATTACHMENT_KHR: Self = Self::COLOR_ATTACHMENT;
        pub const COLOR_ATTACHMENT_BLEND_KHR: Self = Self::COLOR_ATTACHMENT_BLEND;
        pub const DEPTH_STENCIL_ATTACHMENT_KHR: Self = Self::DEPTH_STENCIL_ATTACHMENT;
        pub const BLIT_SRC_KHR: Self = Self::BLIT_SRC;
        pub const BLIT_DST_KHR: Self = Self::BLIT_DST;
        pub const SAMPLED_IMAGE_FILTER_LINEAR_KHR: Self = Self::SAMPLED_IMAGE_FILTER_LINEAR;
        pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
        pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
        pub const MIDPOINT_CHROMA_SAMPLES_KHR: Self = Self::MIDPOINT_CHROMA_SAMPLES;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR: Self =
            Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR: Self =
            Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR: Self =
            Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR:
            Self = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
        pub const DISJOINT_KHR: Self = Self::DISJOINT;
        pub const COSITED_CHROMA_SAMPLES_KHR: Self = Self::COSITED_CHROMA_SAMPLES;
        pub const STORAGE_READ_WITHOUT_FORMAT_KHR: Self = Self::STORAGE_READ_WITHOUT_FORMAT;
        pub const STORAGE_WRITE_WITHOUT_FORMAT_KHR: Self = Self::STORAGE_WRITE_WITHOUT_FORMAT;
        pub const SAMPLED_IMAGE_DEPTH_COMPARISON_KHR: Self = Self::SAMPLED_IMAGE_DEPTH_COMPARISON;
        pub const SAMPLED_IMAGE_FILTER_MINMAX_KHR: Self = Self::SAMPLED_IMAGE_FILTER_MINMAX;
        pub const SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self = Self::SAMPLED_IMAGE_FILTER_CUBIC;

        // VK_KHR_fragment_shading_rate
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 30);

        // VK_KHR_maintenance10
        pub const DEPTH_COPY_ON_COMPUTE_QUEUE_KHR: Self = Self(1 << 52);
        pub const DEPTH_COPY_ON_TRANSFER_QUEUE_KHR: Self = Self(1 << 53);
        pub const STENCIL_COPY_ON_COMPUTE_QUEUE_KHR: Self = Self(1 << 54);
        pub const STENCIL_COPY_ON_TRANSFER_QUEUE_KHR: Self = Self(1 << 55);

        // VK_KHR_video_decode_queue
        pub const VIDEO_DECODE_OUTPUT_KHR: Self = Self(1 << 25);
        pub const VIDEO_DECODE_DPB_KHR: Self = Self(1 << 26);

        // VK_KHR_video_encode_quantization_map
        pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self = Self(1 << 49);
        pub const VIDEO_ENCODE_EMPHASIS_MAP_KHR: Self = Self(1 << 50);

        // VK_KHR_video_encode_queue
        pub const VIDEO_ENCODE_INPUT_KHR: Self = Self(1 << 27);
        pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1 << 28);

        // VK_NV_linear_color_attachment
        /// Format support linear image as render target, it cannot be mixed with non linear attachment
        pub const LINEAR_COLOR_ATTACHMENT_NV: Self = Self(1 << 38);

        // VK_NV_optical_flow
        pub const OPTICAL_FLOW_IMAGE_NV: Self = Self(1 << 40);
        pub const OPTICAL_FLOW_VECTOR_NV: Self = Self(1 << 41);
        pub const OPTICAL_FLOW_COST_NV: Self = Self(1 << 42);

        // VK_NV_ray_tracing_linear_swept_spheres
        pub const ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV: Self = Self(1 << 51);

        // VK_QCOM_image_processing
        pub const WEIGHT_IMAGE_QCOM: Self = Self(1 << 34);
        pub const WEIGHT_SAMPLED_IMAGE_QCOM: Self = Self(1 << 35);
        pub const BLOCK_MATCHING_QCOM: Self = Self(1 << 36);
        pub const BOX_FILTER_SAMPLED_QCOM: Self = Self(1 << 37);

        // VK_VERSION_1_3
        /// This is an interaction with EXT_filter_cubic, though not tagged that way
        pub const SAMPLED_IMAGE_FILTER_CUBIC: Self = Self(1 << 13);

        // VK_VERSION_1_4
        pub const HOST_IMAGE_TRANSFER: Self = Self(1 << 46);
    }

    impl fmt::Debug for FormatFeatureFlagBits2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SAMPLED_IMAGE => Some("SAMPLED_IMAGE"),
                Self::STORAGE_IMAGE => Some("STORAGE_IMAGE"),
                Self::STORAGE_IMAGE_ATOMIC => Some("STORAGE_IMAGE_ATOMIC"),
                Self::UNIFORM_TEXEL_BUFFER => Some("UNIFORM_TEXEL_BUFFER"),
                Self::STORAGE_TEXEL_BUFFER => Some("STORAGE_TEXEL_BUFFER"),
                Self::STORAGE_TEXEL_BUFFER_ATOMIC => Some("STORAGE_TEXEL_BUFFER_ATOMIC"),
                Self::VERTEX_BUFFER => Some("VERTEX_BUFFER"),
                Self::COLOR_ATTACHMENT => Some("COLOR_ATTACHMENT"),
                Self::COLOR_ATTACHMENT_BLEND => Some("COLOR_ATTACHMENT_BLEND"),
                Self::DEPTH_STENCIL_ATTACHMENT => Some("DEPTH_STENCIL_ATTACHMENT"),
                Self::BLIT_SRC => Some("BLIT_SRC"),
                Self::BLIT_DST => Some("BLIT_DST"),
                Self::SAMPLED_IMAGE_FILTER_LINEAR => Some("SAMPLED_IMAGE_FILTER_LINEAR"),
                Self::TRANSFER_SRC => Some("TRANSFER_SRC"),
                Self::TRANSFER_DST => Some("TRANSFER_DST"),
                Self::SAMPLED_IMAGE_FILTER_MINMAX => Some("SAMPLED_IMAGE_FILTER_MINMAX"),
                Self::MIDPOINT_CHROMA_SAMPLES => Some("MIDPOINT_CHROMA_SAMPLES"),
                Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER => {
                    Some("SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER")
                }
                Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER => {
                    Some("SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER")
                }
                Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT => {
                    Some("SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT")
                }
                Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE => {
                    Some("SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE")
                }
                Self::DISJOINT => Some("DISJOINT"),
                Self::COSITED_CHROMA_SAMPLES => Some("COSITED_CHROMA_SAMPLES"),
                Self::STORAGE_READ_WITHOUT_FORMAT => Some("STORAGE_READ_WITHOUT_FORMAT"),
                Self::STORAGE_WRITE_WITHOUT_FORMAT => Some("STORAGE_WRITE_WITHOUT_FORMAT"),
                Self::SAMPLED_IMAGE_DEPTH_COMPARISON => Some("SAMPLED_IMAGE_DEPTH_COMPARISON"),
                Self::TENSOR_DATA_GRAPH_ARM => Some("TENSOR_DATA_GRAPH_ARM"),
                Self::TENSOR_SHADER_ARM => Some("TENSOR_SHADER_ARM"),
                Self::TENSOR_IMAGE_ALIASING_ARM => Some("TENSOR_IMAGE_ALIASING_ARM"),
                Self::FRAGMENT_DENSITY_MAP_EXT => Some("FRAGMENT_DENSITY_MAP_EXT"),
                Self::ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR => {
                    Some("ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR")
                }
                Self::COPY_IMAGE_INDIRECT_DST_KHR => Some("COPY_IMAGE_INDIRECT_DST_KHR"),
                Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR => {
                    Some("FRAGMENT_SHADING_RATE_ATTACHMENT_KHR")
                }
                Self::DEPTH_COPY_ON_COMPUTE_QUEUE_KHR => Some("DEPTH_COPY_ON_COMPUTE_QUEUE_KHR"),
                Self::DEPTH_COPY_ON_TRANSFER_QUEUE_KHR => Some("DEPTH_COPY_ON_TRANSFER_QUEUE_KHR"),
                Self::STENCIL_COPY_ON_COMPUTE_QUEUE_KHR => {
                    Some("STENCIL_COPY_ON_COMPUTE_QUEUE_KHR")
                }
                Self::STENCIL_COPY_ON_TRANSFER_QUEUE_KHR => {
                    Some("STENCIL_COPY_ON_TRANSFER_QUEUE_KHR")
                }
                Self::VIDEO_DECODE_OUTPUT_KHR => Some("VIDEO_DECODE_OUTPUT_KHR"),
                Self::VIDEO_DECODE_DPB_KHR => Some("VIDEO_DECODE_DPB_KHR"),
                Self::VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR => {
                    Some("VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR")
                }
                Self::VIDEO_ENCODE_EMPHASIS_MAP_KHR => Some("VIDEO_ENCODE_EMPHASIS_MAP_KHR"),
                Self::VIDEO_ENCODE_INPUT_KHR => Some("VIDEO_ENCODE_INPUT_KHR"),
                Self::VIDEO_ENCODE_DPB_KHR => Some("VIDEO_ENCODE_DPB_KHR"),
                Self::LINEAR_COLOR_ATTACHMENT_NV => Some("LINEAR_COLOR_ATTACHMENT_NV"),
                Self::OPTICAL_FLOW_IMAGE_NV => Some("OPTICAL_FLOW_IMAGE_NV"),
                Self::OPTICAL_FLOW_VECTOR_NV => Some("OPTICAL_FLOW_VECTOR_NV"),
                Self::OPTICAL_FLOW_COST_NV => Some("OPTICAL_FLOW_COST_NV"),
                Self::ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV => {
                    Some("ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV")
                }
                Self::WEIGHT_IMAGE_QCOM => Some("WEIGHT_IMAGE_QCOM"),
                Self::WEIGHT_SAMPLED_IMAGE_QCOM => Some("WEIGHT_SAMPLED_IMAGE_QCOM"),
                Self::BLOCK_MATCHING_QCOM => Some("BLOCK_MATCHING_QCOM"),
                Self::BOX_FILTER_SAMPLED_QCOM => Some("BOX_FILTER_SAMPLED_QCOM"),
                Self::SAMPLED_IMAGE_FILTER_CUBIC => Some("SAMPLED_IMAGE_FILTER_CUBIC"),
                Self::HOST_IMAGE_TRANSFER => Some("HOST_IMAGE_TRANSFER"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct RenderingFlags(Flags);
    vk_bitflags_wrapped!(RenderingFlags, Flags, RenderingFlagBits);

    impl fmt::Debug for RenderingFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    RenderingFlagBits::CONTENTS_SECONDARY_COMMAND_BUFFERS.0,
                    "CONTENTS_SECONDARY_COMMAND_BUFFERS",
                ),
                (RenderingFlagBits::SUSPENDING.0, "SUSPENDING"),
                (RenderingFlagBits::RESUMING.0, "RESUMING"),
                (
                    RenderingFlagBits::FRAGMENT_REGION_EXT.0,
                    "FRAGMENT_REGION_EXT",
                ),
                (
                    RenderingFlagBits::CUSTOM_RESOLVE_EXT.0,
                    "CUSTOM_RESOLVE_EXT",
                ),
                (
                    RenderingFlagBits::ENABLE_LEGACY_DITHERING_EXT.0,
                    "ENABLE_LEGACY_DITHERING_EXT",
                ),
                (
                    RenderingFlagBits::LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR.0,
                    "LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR",
                ),
                (
                    RenderingFlagBits::CONTENTS_INLINE_KHR.0,
                    "CONTENTS_INLINE_KHR",
                ),
                (
                    RenderingFlagBits::PER_LAYER_FRAGMENT_DENSITY_VALVE.0,
                    "PER_LAYER_FRAGMENT_DENSITY_VALVE",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct RenderingFlagBits(u32);

    impl RenderingFlagBits {
        pub const CONTENTS_SECONDARY_COMMAND_BUFFERS: Self = Self(1 << 0);
        pub const SUSPENDING: Self = Self(1 << 1);
        pub const RESUMING: Self = Self(1 << 2);
        // VK_EXT_custom_resolve
        pub const FRAGMENT_REGION_EXT: Self = Self(1 << 6);
        pub const CUSTOM_RESOLVE_EXT: Self = Self(1 << 7);

        // VK_EXT_legacy_dithering
        pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(1 << 3);

        // VK_EXT_nested_command_buffer
        pub const CONTENTS_INLINE_EXT: Self = Self::CONTENTS_INLINE_KHR;

        // VK_KHR_dynamic_rendering
        pub const CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR: Self =
            Self::CONTENTS_SECONDARY_COMMAND_BUFFERS;
        pub const SUSPENDING_KHR: Self = Self::SUSPENDING;
        pub const RESUMING_KHR: Self = Self::RESUMING;

        // VK_KHR_maintenance10
        pub const LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR: Self = Self(1 << 8);

        // VK_KHR_maintenance7
        /// Promoted from extension 452
        pub const CONTENTS_INLINE_KHR: Self = Self(1 << 4);

        // VK_VALVE_fragment_density_map_layered
        pub const PER_LAYER_FRAGMENT_DENSITY_VALVE: Self = Self(1 << 5);
    }

    impl fmt::Debug for RenderingFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::CONTENTS_SECONDARY_COMMAND_BUFFERS => {
                    Some("CONTENTS_SECONDARY_COMMAND_BUFFERS")
                }
                Self::SUSPENDING => Some("SUSPENDING"),
                Self::RESUMING => Some("RESUMING"),
                Self::FRAGMENT_REGION_EXT => Some("FRAGMENT_REGION_EXT"),
                Self::CUSTOM_RESOLVE_EXT => Some("CUSTOM_RESOLVE_EXT"),
                Self::ENABLE_LEGACY_DITHERING_EXT => Some("ENABLE_LEGACY_DITHERING_EXT"),
                Self::LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR => {
                    Some("LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR")
                }
                Self::CONTENTS_INLINE_KHR => Some("CONTENTS_INLINE_KHR"),
                Self::PER_LAYER_FRAGMENT_DENSITY_VALVE => Some("PER_LAYER_FRAGMENT_DENSITY_VALVE"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkToolPurposeFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ToolPurposeFlags(Flags);
    vk_bitflags_wrapped!(ToolPurposeFlags, Flags, ToolPurposeFlagBits);

    impl fmt::Debug for ToolPurposeFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (ToolPurposeFlagBits::VALIDATION.0, "VALIDATION"),
                (ToolPurposeFlagBits::PROFILING.0, "PROFILING"),
                (ToolPurposeFlagBits::TRACING.0, "TRACING"),
                (
                    ToolPurposeFlagBits::ADDITIONAL_FEATURES.0,
                    "ADDITIONAL_FEATURES",
                ),
                (
                    ToolPurposeFlagBits::MODIFYING_FEATURES.0,
                    "MODIFYING_FEATURES",
                ),
                (
                    ToolPurposeFlagBits::DEBUG_REPORTING_EXT.0,
                    "DEBUG_REPORTING_EXT",
                ),
                (
                    ToolPurposeFlagBits::DEBUG_MARKERS_EXT.0,
                    "DEBUG_MARKERS_EXT",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkToolPurposeFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ToolPurposeFlagBits(u32);

    impl ToolPurposeFlagBits {
        pub const VALIDATION: Self = Self(1 << 0);
        pub const PROFILING: Self = Self(1 << 1);
        pub const TRACING: Self = Self(1 << 2);
        pub const ADDITIONAL_FEATURES: Self = Self(1 << 3);
        pub const MODIFYING_FEATURES: Self = Self(1 << 4);
        // VK_EXT_tooling_info
        pub const DEBUG_REPORTING_EXT: Self = Self(1 << 5);
        pub const DEBUG_MARKERS_EXT: Self = Self(1 << 6);
        pub const VALIDATION_EXT: Self = Self::VALIDATION;
        pub const PROFILING_EXT: Self = Self::PROFILING;
        pub const TRACING_EXT: Self = Self::TRACING;
        pub const ADDITIONAL_FEATURES_EXT: Self = Self::ADDITIONAL_FEATURES;
        pub const MODIFYING_FEATURES_EXT: Self = Self::MODIFYING_FEATURES;
    }

    impl fmt::Debug for ToolPurposeFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::VALIDATION => Some("VALIDATION"),
                Self::PROFILING => Some("PROFILING"),
                Self::TRACING => Some("TRACING"),
                Self::ADDITIONAL_FEATURES => Some("ADDITIONAL_FEATURES"),
                Self::MODIFYING_FEATURES => Some("MODIFYING_FEATURES"),
                Self::DEBUG_REPORTING_EXT => Some("DEBUG_REPORTING_EXT"),
                Self::DEBUG_MARKERS_EXT => Some("DEBUG_MARKERS_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubmitFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SubmitFlags(Flags);
    vk_bitflags_wrapped!(SubmitFlags, Flags, SubmitFlagBits);

    impl fmt::Debug for SubmitFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[(SubmitFlagBits::PROTECTED.0, "PROTECTED")];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubmitFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct SubmitFlagBits(u32);

    impl SubmitFlagBits {
        pub const PROTECTED: Self = Self(1 << 0);
        // VK_KHR_synchronization2
        pub const PROTECTED_KHR: Self = Self::PROTECTED;
    }

    impl fmt::Debug for SubmitFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::PROTECTED => Some("PROTECTED"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceBufferMemoryRequirements.html>
    pub type PFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceBufferMemoryRequirements<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceImageMemoryRequirements.html>
    pub type PFN_vkGetDeviceImageMemoryRequirements = unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceImageMemoryRequirements<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceImageSparseMemoryRequirements.html>
    pub type PFN_vkGetDeviceImageSparseMemoryRequirements = unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceImageMemoryRequirements<'_>,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceToolProperties.html>
    pub type PFN_vkGetPhysicalDeviceToolProperties = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_tool_count: *mut u32,
        p_tool_properties: *mut PhysicalDeviceToolProperties<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCullMode.html>
    pub type PFN_vkCmdSetCullMode =
        unsafe extern "system" fn(command_buffer: CommandBuffer, cull_mode: CullModeFlags);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetFrontFace.html>
    pub type PFN_vkCmdSetFrontFace =
        unsafe extern "system" fn(command_buffer: CommandBuffer, front_face: FrontFace);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPrimitiveTopology.html>
    pub type PFN_vkCmdSetPrimitiveTopology = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        primitive_topology: PrimitiveTopology,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetViewportWithCount.html>
    pub type PFN_vkCmdSetViewportWithCount = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        viewport_count: u32,
        p_viewports: *const Viewport,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetScissorWithCount.html>
    pub type PFN_vkCmdSetScissorWithCount = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        scissor_count: u32,
        p_scissors: *const Rect2D,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindVertexBuffers2.html>
    pub type PFN_vkCmdBindVertexBuffers2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
        p_sizes: *const DeviceSize,
        p_strides: *const DeviceSize,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthTestEnable.html>
    pub type PFN_vkCmdSetDepthTestEnable =
        unsafe extern "system" fn(command_buffer: CommandBuffer, depth_test_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthWriteEnable.html>
    pub type PFN_vkCmdSetDepthWriteEnable =
        unsafe extern "system" fn(command_buffer: CommandBuffer, depth_write_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthCompareOp.html>
    pub type PFN_vkCmdSetDepthCompareOp =
        unsafe extern "system" fn(command_buffer: CommandBuffer, depth_compare_op: CompareOp);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthBoundsTestEnable.html>
    pub type PFN_vkCmdSetDepthBoundsTestEnable =
        unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bounds_test_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetStencilTestEnable.html>
    pub type PFN_vkCmdSetStencilTestEnable =
        unsafe extern "system" fn(command_buffer: CommandBuffer, stencil_test_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetStencilOp.html>
    pub type PFN_vkCmdSetStencilOp = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRasterizerDiscardEnable.html>
    pub type PFN_vkCmdSetRasterizerDiscardEnable =
        unsafe extern "system" fn(command_buffer: CommandBuffer, rasterizer_discard_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthBiasEnable.html>
    pub type PFN_vkCmdSetDepthBiasEnable =
        unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bias_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPrimitiveRestartEnable.html>
    pub type PFN_vkCmdSetPrimitiveRestartEnable =
        unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_restart_enable: Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreatePrivateDataSlot.html>
    pub type PFN_vkCreatePrivateDataSlot = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const PrivateDataSlotCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_private_data_slot: *mut PrivateDataSlot,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyPrivateDataSlot.html>
    pub type PFN_vkDestroyPrivateDataSlot = unsafe extern "system" fn(
        device: Device,
        private_data_slot: PrivateDataSlot,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetPrivateData.html>
    pub type PFN_vkSetPrivateData = unsafe extern "system" fn(
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPrivateData.html>
    pub type PFN_vkGetPrivateData = unsafe extern "system" fn(
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        p_data: *mut u64,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyBuffer2.html>
    pub type PFN_vkCmdCopyBuffer2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_buffer_info: *const CopyBufferInfo2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyImage2.html>
    pub type PFN_vkCmdCopyImage2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_image_info: *const CopyImageInfo2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBlitImage2.html>
    pub type PFN_vkCmdBlitImage2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_blit_image_info: *const BlitImageInfo2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyBufferToImage2.html>
    pub type PFN_vkCmdCopyBufferToImage2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyImageToBuffer2.html>
    pub type PFN_vkCmdCopyImageToBuffer2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdResolveImage2.html>
    pub type PFN_vkCmdResolveImage2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_resolve_image_info: *const ResolveImageInfo2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetEvent2.html>
    pub type PFN_vkCmdSetEvent2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event: Event,
        p_dependency_info: *const DependencyInfo<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdResetEvent2.html>
    pub type PFN_vkCmdResetEvent2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags2,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWaitEvents2.html>
    pub type PFN_vkCmdWaitEvents2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event_count: u32,
        p_events: *const Event,
        p_dependency_infos: *const DependencyInfo<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPipelineBarrier2.html>
    pub type PFN_vkCmdPipelineBarrier2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_dependency_info: *const DependencyInfo<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueSubmit2.html>
    pub type PFN_vkQueueSubmit2 = unsafe extern "system" fn(
        queue: Queue,
        submit_count: u32,
        p_submits: *const SubmitInfo2<'_>,
        fence: Fence,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWriteTimestamp2.html>
    pub type PFN_vkCmdWriteTimestamp2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginRendering.html>
    pub type PFN_vkCmdBeginRendering = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_rendering_info: *const RenderingInfo<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndRendering.html>
    pub type PFN_vkCmdEndRendering = unsafe extern "system" fn(command_buffer: CommandBuffer);
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkFlags64 = Flags64;
    pub type VkPrivateDataSlot = PrivateDataSlot;
    pub type VkDevicePrivateDataCreateInfo = DevicePrivateDataCreateInfo<'static>;
    pub type VkPrivateDataSlotCreateInfo = PrivateDataSlotCreateInfo<'static>;
    pub type VkPhysicalDevicePrivateDataFeatures = PhysicalDevicePrivateDataFeatures<'static>;
    pub type VkDeviceBufferMemoryRequirements = DeviceBufferMemoryRequirements<'static>;
    pub type VkDeviceImageMemoryRequirements = DeviceImageMemoryRequirements<'static>;
    pub type VkPhysicalDeviceInlineUniformBlockFeatures =
        PhysicalDeviceInlineUniformBlockFeatures<'static>;
    pub type VkPhysicalDeviceInlineUniformBlockProperties =
        PhysicalDeviceInlineUniformBlockProperties<'static>;
    pub type VkWriteDescriptorSetInlineUniformBlock = WriteDescriptorSetInlineUniformBlock<'static>;
    pub type VkDescriptorPoolInlineUniformBlockCreateInfo =
        DescriptorPoolInlineUniformBlockCreateInfo<'static>;
    pub type VkPhysicalDeviceMaintenance4Features = PhysicalDeviceMaintenance4Features<'static>;
    pub type VkPhysicalDeviceMaintenance4Properties = PhysicalDeviceMaintenance4Properties<'static>;
    pub type VkPhysicalDeviceTextureCompressionASTCHDRFeatures =
        PhysicalDeviceTextureCompressionASTCHDRFeatures<'static>;
    pub type VkPipelineCreationFeedback = PipelineCreationFeedback;
    pub type VkPipelineCreationFeedbackCreateInfo = PipelineCreationFeedbackCreateInfo<'static>;
    pub type VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures =
        PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'static>;
    pub type VkPhysicalDeviceTexelBufferAlignmentProperties =
        PhysicalDeviceTexelBufferAlignmentProperties<'static>;
    pub type VkPhysicalDeviceSubgroupSizeControlFeatures =
        PhysicalDeviceSubgroupSizeControlFeatures<'static>;
    pub type VkPhysicalDeviceSubgroupSizeControlProperties =
        PhysicalDeviceSubgroupSizeControlProperties<'static>;
    pub type VkPipelineShaderStageRequiredSubgroupSizeCreateInfo =
        PipelineShaderStageRequiredSubgroupSizeCreateInfo<'static>;
    pub type VkPhysicalDevicePipelineCreationCacheControlFeatures =
        PhysicalDevicePipelineCreationCacheControlFeatures<'static>;
    pub type VkPhysicalDeviceVulkan13Features = PhysicalDeviceVulkan13Features<'static>;
    pub type VkPhysicalDeviceVulkan13Properties = PhysicalDeviceVulkan13Properties<'static>;
    pub type VkPhysicalDeviceToolProperties = PhysicalDeviceToolProperties<'static>;
    pub type VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures =
        PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'static>;
    pub type VkPhysicalDeviceImageRobustnessFeatures =
        PhysicalDeviceImageRobustnessFeatures<'static>;
    pub type VkBufferCopy2 = BufferCopy2<'static>;
    pub type VkImageCopy2 = ImageCopy2<'static>;
    pub type VkImageBlit2 = ImageBlit2<'static>;
    pub type VkBufferImageCopy2 = BufferImageCopy2<'static>;
    pub type VkImageResolve2 = ImageResolve2<'static>;
    pub type VkCopyBufferInfo2 = CopyBufferInfo2<'static>;
    pub type VkCopyImageInfo2 = CopyImageInfo2<'static>;
    pub type VkBlitImageInfo2 = BlitImageInfo2<'static>;
    pub type VkCopyBufferToImageInfo2 = CopyBufferToImageInfo2<'static>;
    pub type VkCopyImageToBufferInfo2 = CopyImageToBufferInfo2<'static>;
    pub type VkResolveImageInfo2 = ResolveImageInfo2<'static>;
    pub type VkPhysicalDeviceShaderTerminateInvocationFeatures =
        PhysicalDeviceShaderTerminateInvocationFeatures<'static>;
    pub type VkMemoryBarrier2 = MemoryBarrier2<'static>;
    pub type VkImageMemoryBarrier2 = ImageMemoryBarrier2<'static>;
    pub type VkBufferMemoryBarrier2 = BufferMemoryBarrier2<'static>;
    pub type VkDependencyInfo = DependencyInfo<'static>;
    pub type VkSemaphoreSubmitInfo = SemaphoreSubmitInfo<'static>;
    pub type VkCommandBufferSubmitInfo = CommandBufferSubmitInfo<'static>;
    pub type VkSubmitInfo2 = SubmitInfo2<'static>;
    pub type VkPhysicalDeviceSynchronization2Features =
        PhysicalDeviceSynchronization2Features<'static>;
    pub type VkPhysicalDeviceShaderIntegerDotProductFeatures =
        PhysicalDeviceShaderIntegerDotProductFeatures<'static>;
    pub type VkPhysicalDeviceShaderIntegerDotProductProperties =
        PhysicalDeviceShaderIntegerDotProductProperties<'static>;
    pub type VkFormatProperties3 = FormatProperties3<'static>;
    pub type VkPipelineRenderingCreateInfo = PipelineRenderingCreateInfo<'static>;
    pub type VkRenderingInfo = RenderingInfo<'static>;
    pub type VkRenderingAttachmentInfo = RenderingAttachmentInfo<'static>;
    pub type VkPhysicalDeviceDynamicRenderingFeatures =
        PhysicalDeviceDynamicRenderingFeatures<'static>;
    pub type VkCommandBufferInheritanceRenderingInfo =
        CommandBufferInheritanceRenderingInfo<'static>;
    pub type VkPrivateDataSlotCreateFlags = PrivateDataSlotCreateFlags;
    pub type VkPipelineCreationFeedbackFlags = PipelineCreationFeedbackFlags;
    pub type VkPipelineCreationFeedbackFlagBits = PipelineCreationFeedbackFlagBits;
    pub type VkAccessFlags2 = AccessFlags2;
    pub type VkAccessFlagBits2 = AccessFlagBits2;
    pub type VkPipelineStageFlags2 = PipelineStageFlags2;
    pub type VkPipelineStageFlagBits2 = PipelineStageFlagBits2;
    pub type VkFormatFeatureFlags2 = FormatFeatureFlags2;
    pub type VkFormatFeatureFlagBits2 = FormatFeatureFlagBits2;
    pub type VkRenderingFlags = RenderingFlags;
    pub type VkRenderingFlagBits = RenderingFlagBits;
    pub type VkToolPurposeFlags = ToolPurposeFlags;
    pub type VkToolPurposeFlagBits = ToolPurposeFlagBits;
    pub type VkSubmitFlags = SubmitFlags;
    pub type VkSubmitFlagBits = SubmitFlagBits;
    impl DevicePrivateDataCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDevicePrivateDataCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PrivateDataSlotCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPrivateDataSlotCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDevicePrivateDataFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDevicePrivateDataFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceBufferMemoryRequirements<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceBufferMemoryRequirements {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceImageMemoryRequirements<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceImageMemoryRequirements {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceInlineUniformBlockFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceInlineUniformBlockFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceInlineUniformBlockProperties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceInlineUniformBlockProperties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl WriteDescriptorSetInlineUniformBlock<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkWriteDescriptorSetInlineUniformBlock {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DescriptorPoolInlineUniformBlockCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkDescriptorPoolInlineUniformBlockCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceMaintenance4Features<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceMaintenance4Features {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceMaintenance4Properties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceMaintenance4Properties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceTextureCompressionASTCHDRFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceTextureCompressionASTCHDRFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineCreationFeedbackCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineCreationFeedbackCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceTexelBufferAlignmentProperties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceTexelBufferAlignmentProperties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceSubgroupSizeControlFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceSubgroupSizeControlFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceSubgroupSizeControlProperties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceSubgroupSizeControlProperties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineShaderStageRequiredSubgroupSizeCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDevicePipelineCreationCacheControlFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDevicePipelineCreationCacheControlFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceVulkan13Features<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceVulkan13Features {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceVulkan13Properties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceVulkan13Properties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceToolProperties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceToolProperties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceImageRobustnessFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceImageRobustnessFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BufferCopy2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBufferCopy2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageCopy2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageCopy2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageBlit2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageBlit2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BufferImageCopy2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBufferImageCopy2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageResolve2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageResolve2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CopyBufferInfo2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCopyBufferInfo2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CopyImageInfo2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCopyImageInfo2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BlitImageInfo2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBlitImageInfo2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CopyBufferToImageInfo2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCopyBufferToImageInfo2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CopyImageToBufferInfo2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCopyImageToBufferInfo2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ResolveImageInfo2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkResolveImageInfo2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceShaderTerminateInvocationFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderTerminateInvocationFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MemoryBarrier2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMemoryBarrier2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageMemoryBarrier2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageMemoryBarrier2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BufferMemoryBarrier2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBufferMemoryBarrier2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DependencyInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDependencyInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SemaphoreSubmitInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSemaphoreSubmitInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CommandBufferSubmitInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCommandBufferSubmitInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SubmitInfo2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSubmitInfo2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceSynchronization2Features<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceSynchronization2Features {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceShaderIntegerDotProductFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderIntegerDotProductFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceShaderIntegerDotProductProperties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderIntegerDotProductProperties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl FormatProperties3<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkFormatProperties3 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineRenderingCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineRenderingCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl RenderingInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkRenderingInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl RenderingAttachmentInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkRenderingAttachmentInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceDynamicRenderingFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceDynamicRenderingFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CommandBufferInheritanceRenderingInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCommandBufferInheritanceRenderingInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    get_physical_device_tool_properties: PFN_vkGetPhysicalDeviceToolProperties,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_tool_properties: transmute(
                    load(c"vkGetPhysicalDeviceToolProperties").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceToolProperties.html>
    #[inline]
    pub unsafe fn get_physical_device_tool_properties<'a>(
        &self,
        physical_device: PhysicalDevice,
        mut tool_properties: impl ExtendUninit<PhysicalDeviceToolProperties<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |tool_count, tool_properties| {
                let result = (self.get_physical_device_tool_properties)(
                    physical_device,
                    tool_count,
                    tool_properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let tool_properties_buf = tool_properties.reserve(capacity);
            len = tool_properties_buf.len().try_into().unwrap();
            let result = call(&mut len, tool_properties_buf.as_mut_ptr() as *mut _)?;
            tool_properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}

pub struct DeviceFn {
    create_private_data_slot: PFN_vkCreatePrivateDataSlot,
    destroy_private_data_slot: PFN_vkDestroyPrivateDataSlot,
    set_private_data: PFN_vkSetPrivateData,
    get_private_data: PFN_vkGetPrivateData,
    cmd_pipeline_barrier2: PFN_vkCmdPipelineBarrier2,
    cmd_write_timestamp2: PFN_vkCmdWriteTimestamp2,
    queue_submit2: PFN_vkQueueSubmit2,
    cmd_copy_buffer2: PFN_vkCmdCopyBuffer2,
    cmd_copy_image2: PFN_vkCmdCopyImage2,
    cmd_copy_buffer_to_image2: PFN_vkCmdCopyBufferToImage2,
    cmd_copy_image_to_buffer2: PFN_vkCmdCopyImageToBuffer2,
    get_device_buffer_memory_requirements: PFN_vkGetDeviceBufferMemoryRequirements,
    get_device_image_memory_requirements: PFN_vkGetDeviceImageMemoryRequirements,
    get_device_image_sparse_memory_requirements: PFN_vkGetDeviceImageSparseMemoryRequirements,
    cmd_set_event2: PFN_vkCmdSetEvent2,
    cmd_reset_event2: PFN_vkCmdResetEvent2,
    cmd_wait_events2: PFN_vkCmdWaitEvents2,
    cmd_blit_image2: PFN_vkCmdBlitImage2,
    cmd_resolve_image2: PFN_vkCmdResolveImage2,
    cmd_begin_rendering: PFN_vkCmdBeginRendering,
    cmd_end_rendering: PFN_vkCmdEndRendering,
    cmd_set_cull_mode: PFN_vkCmdSetCullMode,
    cmd_set_front_face: PFN_vkCmdSetFrontFace,
    cmd_set_primitive_topology: PFN_vkCmdSetPrimitiveTopology,
    cmd_set_viewport_with_count: PFN_vkCmdSetViewportWithCount,
    cmd_set_scissor_with_count: PFN_vkCmdSetScissorWithCount,
    cmd_bind_vertex_buffers2: PFN_vkCmdBindVertexBuffers2,
    cmd_set_depth_test_enable: PFN_vkCmdSetDepthTestEnable,
    cmd_set_depth_write_enable: PFN_vkCmdSetDepthWriteEnable,
    cmd_set_depth_compare_op: PFN_vkCmdSetDepthCompareOp,
    cmd_set_depth_bounds_test_enable: PFN_vkCmdSetDepthBoundsTestEnable,
    cmd_set_stencil_test_enable: PFN_vkCmdSetStencilTestEnable,
    cmd_set_stencil_op: PFN_vkCmdSetStencilOp,
    cmd_set_rasterizer_discard_enable: PFN_vkCmdSetRasterizerDiscardEnable,
    cmd_set_depth_bias_enable: PFN_vkCmdSetDepthBiasEnable,
    cmd_set_primitive_restart_enable: PFN_vkCmdSetPrimitiveRestartEnable,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_private_data_slot: transmute(
                    load(c"vkCreatePrivateDataSlot").ok_or(MissingEntryPointError)?,
                ),
                destroy_private_data_slot: transmute(
                    load(c"vkDestroyPrivateDataSlot").ok_or(MissingEntryPointError)?,
                ),
                set_private_data: transmute(
                    load(c"vkSetPrivateData").ok_or(MissingEntryPointError)?,
                ),
                get_private_data: transmute(
                    load(c"vkGetPrivateData").ok_or(MissingEntryPointError)?,
                ),
                cmd_pipeline_barrier2: transmute(
                    load(c"vkCmdPipelineBarrier2").ok_or(MissingEntryPointError)?,
                ),
                cmd_write_timestamp2: transmute(
                    load(c"vkCmdWriteTimestamp2").ok_or(MissingEntryPointError)?,
                ),
                queue_submit2: transmute(load(c"vkQueueSubmit2").ok_or(MissingEntryPointError)?),
                cmd_copy_buffer2: transmute(
                    load(c"vkCmdCopyBuffer2").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_image2: transmute(load(c"vkCmdCopyImage2").ok_or(MissingEntryPointError)?),
                cmd_copy_buffer_to_image2: transmute(
                    load(c"vkCmdCopyBufferToImage2").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_image_to_buffer2: transmute(
                    load(c"vkCmdCopyImageToBuffer2").ok_or(MissingEntryPointError)?,
                ),
                get_device_buffer_memory_requirements: transmute(
                    load(c"vkGetDeviceBufferMemoryRequirements").ok_or(MissingEntryPointError)?,
                ),
                get_device_image_memory_requirements: transmute(
                    load(c"vkGetDeviceImageMemoryRequirements").ok_or(MissingEntryPointError)?,
                ),
                get_device_image_sparse_memory_requirements: transmute(
                    load(c"vkGetDeviceImageSparseMemoryRequirements")
                        .ok_or(MissingEntryPointError)?,
                ),
                cmd_set_event2: transmute(load(c"vkCmdSetEvent2").ok_or(MissingEntryPointError)?),
                cmd_reset_event2: transmute(
                    load(c"vkCmdResetEvent2").ok_or(MissingEntryPointError)?,
                ),
                cmd_wait_events2: transmute(
                    load(c"vkCmdWaitEvents2").ok_or(MissingEntryPointError)?,
                ),
                cmd_blit_image2: transmute(load(c"vkCmdBlitImage2").ok_or(MissingEntryPointError)?),
                cmd_resolve_image2: transmute(
                    load(c"vkCmdResolveImage2").ok_or(MissingEntryPointError)?,
                ),
                cmd_begin_rendering: transmute(
                    load(c"vkCmdBeginRendering").ok_or(MissingEntryPointError)?,
                ),
                cmd_end_rendering: transmute(
                    load(c"vkCmdEndRendering").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_cull_mode: transmute(
                    load(c"vkCmdSetCullMode").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_front_face: transmute(
                    load(c"vkCmdSetFrontFace").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_primitive_topology: transmute(
                    load(c"vkCmdSetPrimitiveTopology").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_viewport_with_count: transmute(
                    load(c"vkCmdSetViewportWithCount").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_scissor_with_count: transmute(
                    load(c"vkCmdSetScissorWithCount").ok_or(MissingEntryPointError)?,
                ),
                cmd_bind_vertex_buffers2: transmute(
                    load(c"vkCmdBindVertexBuffers2").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_test_enable: transmute(
                    load(c"vkCmdSetDepthTestEnable").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_write_enable: transmute(
                    load(c"vkCmdSetDepthWriteEnable").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_compare_op: transmute(
                    load(c"vkCmdSetDepthCompareOp").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_bounds_test_enable: transmute(
                    load(c"vkCmdSetDepthBoundsTestEnable").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_stencil_test_enable: transmute(
                    load(c"vkCmdSetStencilTestEnable").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_stencil_op: transmute(
                    load(c"vkCmdSetStencilOp").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_rasterizer_discard_enable: transmute(
                    load(c"vkCmdSetRasterizerDiscardEnable").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_bias_enable: transmute(
                    load(c"vkCmdSetDepthBiasEnable").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_primitive_restart_enable: transmute(
                    load(c"vkCmdSetPrimitiveRestartEnable").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreatePrivateDataSlot.html>
    #[inline]
    pub unsafe fn create_private_data_slot(
        &self,
        device: Device,
        create_info: &PrivateDataSlotCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<PrivateDataSlot> {
        unsafe {
            let mut private_data_slot = core::mem::MaybeUninit::uninit();
            let result = (self.create_private_data_slot)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                private_data_slot.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(private_data_slot.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyPrivateDataSlot.html>
    #[inline]
    pub unsafe fn destroy_private_data_slot(
        &self,
        device: Device,
        private_data_slot: PrivateDataSlot,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_private_data_slot)(device, private_data_slot, allocator.to_raw_ptr())
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetPrivateData.html>
    #[inline]
    pub unsafe fn set_private_data(
        &self,
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.set_private_data)(
                device,
                object_type,
                object_handle,
                private_data_slot,
                data,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPrivateData.html>
    #[inline]
    pub unsafe fn get_private_data(
        &self,
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
    ) -> u64 {
        unsafe {
            let mut data = core::mem::MaybeUninit::uninit();
            (self.get_private_data)(
                device,
                object_type,
                object_handle,
                private_data_slot,
                data.as_mut_ptr(),
            );
            data.assume_init()
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPipelineBarrier2.html>
    #[inline]
    pub unsafe fn cmd_pipeline_barrier2(
        &self,
        command_buffer: CommandBuffer,
        dependency_info: &DependencyInfo<'_>,
    ) {
        unsafe { (self.cmd_pipeline_barrier2)(command_buffer, dependency_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWriteTimestamp2.html>
    #[inline]
    pub unsafe fn cmd_write_timestamp2(
        &self,
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ) {
        unsafe { (self.cmd_write_timestamp2)(command_buffer, stage, query_pool, query) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueSubmit2.html>
    #[inline]
    pub unsafe fn queue_submit2(
        &self,
        queue: Queue,
        submits: &[SubmitInfo2<'_>],
        fence: Fence,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.queue_submit2)(
                queue,
                submits.len().try_into().unwrap(),
                submits.as_ptr() as _,
                fence,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyBuffer2.html>
    #[inline]
    pub unsafe fn cmd_copy_buffer2(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_info: &CopyBufferInfo2<'_>,
    ) {
        unsafe { (self.cmd_copy_buffer2)(command_buffer, copy_buffer_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyImage2.html>
    #[inline]
    pub unsafe fn cmd_copy_image2(
        &self,
        command_buffer: CommandBuffer,
        copy_image_info: &CopyImageInfo2<'_>,
    ) {
        unsafe { (self.cmd_copy_image2)(command_buffer, copy_image_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyBufferToImage2.html>
    #[inline]
    pub unsafe fn cmd_copy_buffer_to_image2(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_to_image_info: &CopyBufferToImageInfo2<'_>,
    ) {
        unsafe { (self.cmd_copy_buffer_to_image2)(command_buffer, copy_buffer_to_image_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyImageToBuffer2.html>
    #[inline]
    pub unsafe fn cmd_copy_image_to_buffer2(
        &self,
        command_buffer: CommandBuffer,
        copy_image_to_buffer_info: &CopyImageToBufferInfo2<'_>,
    ) {
        unsafe { (self.cmd_copy_image_to_buffer2)(command_buffer, copy_image_to_buffer_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceBufferMemoryRequirements.html>
    #[inline]
    pub unsafe fn get_device_buffer_memory_requirements(
        &self,
        device: Device,
        info: &DeviceBufferMemoryRequirements<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe { (self.get_device_buffer_memory_requirements)(device, info, memory_requirements) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceImageMemoryRequirements.html>
    #[inline]
    pub unsafe fn get_device_image_memory_requirements(
        &self,
        device: Device,
        info: &DeviceImageMemoryRequirements<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe { (self.get_device_image_memory_requirements)(device, info, memory_requirements) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceImageSparseMemoryRequirements.html>
    #[inline]
    pub unsafe fn get_device_image_sparse_memory_requirements<'a>(
        &self,
        device: Device,
        info: &DeviceImageMemoryRequirements<'a>,
        mut sparse_memory_requirements: impl ExtendUninit<SparseImageMemoryRequirements2<'a>>,
    ) {
        unsafe {
            let call = |sparse_memory_requirement_count, sparse_memory_requirements| {
                (self.get_device_image_sparse_memory_requirements)(
                    device,
                    info,
                    sparse_memory_requirement_count,
                    sparse_memory_requirements as _,
                )
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut());
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let sparse_memory_requirements_buf = sparse_memory_requirements.reserve(capacity);
            len = sparse_memory_requirements_buf.len().try_into().unwrap();
            call(
                &mut len,
                sparse_memory_requirements_buf.as_mut_ptr() as *mut _,
            );
            sparse_memory_requirements.set_len(len.try_into().unwrap());
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetEvent2.html>
    #[inline]
    pub unsafe fn cmd_set_event2(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        dependency_info: &DependencyInfo<'_>,
    ) {
        unsafe { (self.cmd_set_event2)(command_buffer, event, dependency_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdResetEvent2.html>
    #[inline]
    pub unsafe fn cmd_reset_event2(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags2,
    ) {
        unsafe { (self.cmd_reset_event2)(command_buffer, event, stage_mask) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWaitEvents2.html>
    #[inline]
    pub unsafe fn cmd_wait_events2(
        &self,
        command_buffer: CommandBuffer,
        events: &[Event],
        dependency_infos: &[DependencyInfo<'_>],
    ) {
        unsafe {
            (self.cmd_wait_events2)(
                command_buffer,
                events.len().try_into().unwrap(),
                events.as_ptr() as _,
                dependency_infos.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBlitImage2.html>
    #[inline]
    pub unsafe fn cmd_blit_image2(
        &self,
        command_buffer: CommandBuffer,
        blit_image_info: &BlitImageInfo2<'_>,
    ) {
        unsafe { (self.cmd_blit_image2)(command_buffer, blit_image_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdResolveImage2.html>
    #[inline]
    pub unsafe fn cmd_resolve_image2(
        &self,
        command_buffer: CommandBuffer,
        resolve_image_info: &ResolveImageInfo2<'_>,
    ) {
        unsafe { (self.cmd_resolve_image2)(command_buffer, resolve_image_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginRendering.html>
    #[inline]
    pub unsafe fn cmd_begin_rendering(
        &self,
        command_buffer: CommandBuffer,
        rendering_info: &RenderingInfo<'_>,
    ) {
        unsafe { (self.cmd_begin_rendering)(command_buffer, rendering_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndRendering.html>
    #[inline]
    pub unsafe fn cmd_end_rendering(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_end_rendering)(command_buffer) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCullMode.html>
    #[inline]
    pub unsafe fn cmd_set_cull_mode(
        &self,
        command_buffer: CommandBuffer,
        cull_mode: CullModeFlags,
    ) {
        unsafe { (self.cmd_set_cull_mode)(command_buffer, cull_mode) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetFrontFace.html>
    #[inline]
    pub unsafe fn cmd_set_front_face(&self, command_buffer: CommandBuffer, front_face: FrontFace) {
        unsafe { (self.cmd_set_front_face)(command_buffer, front_face) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPrimitiveTopology.html>
    #[inline]
    pub unsafe fn cmd_set_primitive_topology(
        &self,
        command_buffer: CommandBuffer,
        primitive_topology: PrimitiveTopology,
    ) {
        unsafe { (self.cmd_set_primitive_topology)(command_buffer, primitive_topology) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetViewportWithCount.html>
    #[inline]
    pub unsafe fn cmd_set_viewport_with_count(
        &self,
        command_buffer: CommandBuffer,
        viewports: &[Viewport],
    ) {
        unsafe {
            (self.cmd_set_viewport_with_count)(
                command_buffer,
                viewports.len().try_into().unwrap(),
                viewports.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetScissorWithCount.html>
    #[inline]
    pub unsafe fn cmd_set_scissor_with_count(
        &self,
        command_buffer: CommandBuffer,
        scissors: &[Rect2D],
    ) {
        unsafe {
            (self.cmd_set_scissor_with_count)(
                command_buffer,
                scissors.len().try_into().unwrap(),
                scissors.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindVertexBuffers2.html>
    #[inline]
    pub unsafe fn cmd_bind_vertex_buffers2(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
        sizes: Option<&[DeviceSize]>,
        strides: Option<&[DeviceSize]>,
    ) {
        unsafe {
            (self.cmd_bind_vertex_buffers2)(
                command_buffer,
                first_binding,
                buffers.len().try_into().unwrap(),
                buffers.as_ptr() as _,
                offsets.as_ptr() as _,
                sizes.to_raw_ptr(),
                strides.to_raw_ptr(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthTestEnable.html>
    #[inline]
    pub unsafe fn cmd_set_depth_test_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_test_enable: bool,
    ) {
        unsafe { (self.cmd_set_depth_test_enable)(command_buffer, depth_test_enable.into()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthWriteEnable.html>
    #[inline]
    pub unsafe fn cmd_set_depth_write_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_write_enable: bool,
    ) {
        unsafe { (self.cmd_set_depth_write_enable)(command_buffer, depth_write_enable.into()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthCompareOp.html>
    #[inline]
    pub unsafe fn cmd_set_depth_compare_op(
        &self,
        command_buffer: CommandBuffer,
        depth_compare_op: CompareOp,
    ) {
        unsafe { (self.cmd_set_depth_compare_op)(command_buffer, depth_compare_op) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthBoundsTestEnable.html>
    #[inline]
    pub unsafe fn cmd_set_depth_bounds_test_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_bounds_test_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_depth_bounds_test_enable)(command_buffer, depth_bounds_test_enable.into())
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetStencilTestEnable.html>
    #[inline]
    pub unsafe fn cmd_set_stencil_test_enable(
        &self,
        command_buffer: CommandBuffer,
        stencil_test_enable: bool,
    ) {
        unsafe { (self.cmd_set_stencil_test_enable)(command_buffer, stencil_test_enable.into()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetStencilOp.html>
    #[inline]
    pub unsafe fn cmd_set_stencil_op(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) {
        unsafe {
            (self.cmd_set_stencil_op)(
                command_buffer,
                face_mask,
                fail_op,
                pass_op,
                depth_fail_op,
                compare_op,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRasterizerDiscardEnable.html>
    #[inline]
    pub unsafe fn cmd_set_rasterizer_discard_enable(
        &self,
        command_buffer: CommandBuffer,
        rasterizer_discard_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_rasterizer_discard_enable)(
                command_buffer,
                rasterizer_discard_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthBiasEnable.html>
    #[inline]
    pub unsafe fn cmd_set_depth_bias_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_enable: bool,
    ) {
        unsafe { (self.cmd_set_depth_bias_enable)(command_buffer, depth_bias_enable.into()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPrimitiveRestartEnable.html>
    #[inline]
    pub unsafe fn cmd_set_primitive_restart_enable(
        &self,
        command_buffer: CommandBuffer,
        primitive_restart_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_primitive_restart_enable)(command_buffer, primitive_restart_enable.into())
        }
    }
}
