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
    pub const FALSE: u32 = 0;
    pub const LOD_CLAMP_NONE: f32 = 1000.0;
    pub const QUEUE_FAMILY_IGNORED: u32 = !0;
    pub const REMAINING_ARRAY_LAYERS: u32 = !0;
    pub const REMAINING_MIP_LEVELS: u32 = !0;
    pub const TRUE: u32 = 1;
    pub const WHOLE_SIZE: u64 = !0;
    pub const MAX_MEMORY_TYPES: u32 = 32;
    pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
    pub const UUID_SIZE: u32 = 16;
    pub const MAX_EXTENSION_NAME_SIZE: u32 = 256;
    pub const MAX_DESCRIPTION_SIZE: u32 = 256;
    pub const MAX_MEMORY_HEAPS: u32 = 16;
    pub const ATTACHMENT_UNUSED: u32 = !0;
    pub const SUBPASS_EXTERNAL: u32 = !0;
    pub type SampleMask = u32;
    pub type Bool32 = u32;
    pub type Flags = u32;
    pub type DeviceSize = u64;
    pub type DeviceAddress = u64;
    define_handle!(Instance, INSTANCE, doc = "");
    define_handle!(PhysicalDevice, PHYSICAL_DEVICE, doc = "");
    define_handle!(Device, DEVICE, doc = "");
    define_handle!(Queue, QUEUE, doc = "");
    define_handle!(CommandBuffer, COMMAND_BUFFER, doc = "");
    handle_nondispatchable!(DeviceMemory, DEVICE_MEMORY, doc = "");
    handle_nondispatchable!(CommandPool, COMMAND_POOL, doc = "");
    handle_nondispatchable!(Buffer, BUFFER, doc = "");
    handle_nondispatchable!(BufferView, BUFFER_VIEW, doc = "");
    handle_nondispatchable!(Image, IMAGE, doc = "");
    handle_nondispatchable!(ImageView, IMAGE_VIEW, doc = "");
    handle_nondispatchable!(ShaderModule, SHADER_MODULE, doc = "");
    handle_nondispatchable!(Pipeline, PIPELINE, doc = "");
    handle_nondispatchable!(PipelineLayout, PIPELINE_LAYOUT, doc = "");
    handle_nondispatchable!(Sampler, SAMPLER, doc = "");
    handle_nondispatchable!(DescriptorSet, DESCRIPTOR_SET, doc = "");
    handle_nondispatchable!(DescriptorSetLayout, DESCRIPTOR_SET_LAYOUT, doc = "");
    handle_nondispatchable!(DescriptorPool, DESCRIPTOR_POOL, doc = "");
    handle_nondispatchable!(Fence, FENCE, doc = "");
    handle_nondispatchable!(Semaphore, SEMAPHORE, doc = "");
    handle_nondispatchable!(Event, EVENT, doc = "");
    handle_nondispatchable!(QueryPool, QUERY_POOL, doc = "");
    handle_nondispatchable!(Framebuffer, FRAMEBUFFER, doc = "");
    handle_nondispatchable!(RenderPass, RENDER_PASS, doc = "");
    handle_nondispatchable!(PipelineCache, PIPELINE_CACHE, doc = "");
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BaseOutStructure<'a> {
        pub s_type: StructureType,
        pub p_next: *mut BaseOutStructure<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    impl<'a> BaseOutStructure<'a> {}
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BaseInStructure<'a> {
        pub s_type: StructureType,
        pub p_next: *const BaseInStructure<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    impl<'a> BaseInStructure<'a> {}
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct Offset2D {
        pub x: i32,
        pub y: i32,
    }
    impl Offset2D {
        pub fn x(mut self, x: i32) -> Self {
            self.x = x;
            self
        }
        pub fn y(mut self, y: i32) -> Self {
            self.y = y;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct Offset3D {
        pub x: i32,
        pub y: i32,
        pub z: i32,
    }
    impl Offset3D {
        pub fn x(mut self, x: i32) -> Self {
            self.x = x;
            self
        }
        pub fn y(mut self, y: i32) -> Self {
            self.y = y;
            self
        }
        pub fn z(mut self, z: i32) -> Self {
            self.z = z;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct Extent2D {
        pub width: u32,
        pub height: u32,
    }
    impl Extent2D {
        pub fn width(mut self, width: u32) -> Self {
            self.width = width;
            self
        }
        pub fn height(mut self, height: u32) -> Self {
            self.height = height;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct Extent3D {
        pub width: u32,
        pub height: u32,
        pub depth: u32,
    }
    impl Extent3D {
        pub fn width(mut self, width: u32) -> Self {
            self.width = width;
            self
        }
        pub fn height(mut self, height: u32) -> Self {
            self.height = height;
            self
        }
        pub fn depth(mut self, depth: u32) -> Self {
            self.depth = depth;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct Viewport {
        pub x: f32,
        pub y: f32,
        pub width: f32,
        pub height: f32,
        pub min_depth: f32,
        pub max_depth: f32,
    }
    impl Viewport {
        pub fn x(mut self, x: f32) -> Self {
            self.x = x;
            self
        }
        pub fn y(mut self, y: f32) -> Self {
            self.y = y;
            self
        }
        pub fn width(mut self, width: f32) -> Self {
            self.width = width;
            self
        }
        pub fn height(mut self, height: f32) -> Self {
            self.height = height;
            self
        }
        pub fn min_depth(mut self, min_depth: f32) -> Self {
            self.min_depth = min_depth;
            self
        }
        pub fn max_depth(mut self, max_depth: f32) -> Self {
            self.max_depth = max_depth;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct Rect2D {
        pub offset: Offset2D,
        pub extent: Extent2D,
    }
    impl Rect2D {
        pub fn offset(mut self, offset: Offset2D) -> Self {
            self.offset = offset;
            self
        }
        pub fn extent(mut self, extent: Extent2D) -> Self {
            self.extent = extent;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct ClearRect {
        pub rect: Rect2D,
        pub base_array_layer: u32,
        pub layer_count: u32,
    }
    impl ClearRect {
        pub fn rect(mut self, rect: Rect2D) -> Self {
            self.rect = rect;
            self
        }
        pub fn base_array_layer(mut self, base_array_layer: u32) -> Self {
            self.base_array_layer = base_array_layer;
            self
        }
        pub fn layer_count(mut self, layer_count: u32) -> Self {
            self.layer_count = layer_count;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct ComponentMapping {
        pub r: ComponentSwizzle,
        pub g: ComponentSwizzle,
        pub b: ComponentSwizzle,
        pub a: ComponentSwizzle,
    }
    impl ComponentMapping {
        pub fn r(mut self, r: ComponentSwizzle) -> Self {
            self.r = r;
            self
        }
        pub fn g(mut self, g: ComponentSwizzle) -> Self {
            self.g = g;
            self
        }
        pub fn b(mut self, b: ComponentSwizzle) -> Self {
            self.b = b;
            self
        }
        pub fn a(mut self, a: ComponentSwizzle) -> Self {
            self.a = a;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceProperties {
        pub api_version: u32,
        pub driver_version: u32,
        pub vendor_id: u32,
        pub device_id: u32,
        pub device_type: PhysicalDeviceType,
        pub device_name: [c_char; MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
        pub pipeline_cache_uuid: [u8; UUID_SIZE as usize],
        pub limits: PhysicalDeviceLimits,
        pub sparse_properties: PhysicalDeviceSparseProperties,
    }
    impl Default for PhysicalDeviceProperties {
        fn default() -> Self {
            Self {
                api_version: Default::default(),
                driver_version: Default::default(),
                vendor_id: Default::default(),
                device_id: Default::default(),
                device_type: Default::default(),
                device_name: [Default::default(); _],
                pipeline_cache_uuid: [Default::default(); _],
                limits: Default::default(),
                sparse_properties: Default::default(),
            }
        }
    }
    impl PhysicalDeviceProperties {
        pub fn api_version(mut self, api_version: u32) -> Self {
            self.api_version = api_version;
            self
        }
        pub fn driver_version(mut self, driver_version: u32) -> Self {
            self.driver_version = driver_version;
            self
        }
        pub fn vendor_id(mut self, vendor_id: u32) -> Self {
            self.vendor_id = vendor_id;
            self
        }
        pub fn device_id(mut self, device_id: u32) -> Self {
            self.device_id = device_id;
            self
        }
        pub fn device_type(mut self, device_type: PhysicalDeviceType) -> Self {
            self.device_type = device_type;
            self
        }
        pub fn pipeline_cache_uuid(
            mut self,
            pipeline_cache_uuid: [u8; UUID_SIZE as usize],
        ) -> Self {
            self.pipeline_cache_uuid = pipeline_cache_uuid;
            self
        }
        pub fn limits(mut self, limits: PhysicalDeviceLimits) -> Self {
            self.limits = limits;
            self
        }
        pub fn sparse_properties(
            mut self,
            sparse_properties: PhysicalDeviceSparseProperties,
        ) -> Self {
            self.sparse_properties = sparse_properties;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExtensionProperties {
        pub extension_name: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
        pub spec_version: u32,
    }
    impl Default for ExtensionProperties {
        fn default() -> Self {
            Self {
                extension_name: [Default::default(); _],
                spec_version: Default::default(),
            }
        }
    }
    impl ExtensionProperties {
        pub fn spec_version(mut self, spec_version: u32) -> Self {
            self.spec_version = spec_version;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct LayerProperties {
        pub layer_name: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
        pub spec_version: u32,
        pub implementation_version: u32,
        pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    }
    impl Default for LayerProperties {
        fn default() -> Self {
            Self {
                layer_name: [Default::default(); _],
                spec_version: Default::default(),
                implementation_version: Default::default(),
                description: [Default::default(); _],
            }
        }
    }
    impl LayerProperties {
        pub fn spec_version(mut self, spec_version: u32) -> Self {
            self.spec_version = spec_version;
            self
        }
        pub fn implementation_version(mut self, implementation_version: u32) -> Self {
            self.implementation_version = implementation_version;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ApplicationInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_application_name: *const c_char,
        pub application_version: u32,
        pub p_engine_name: *const c_char,
        pub engine_version: u32,
        pub api_version: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ApplicationInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::APPLICATION_INFO;
    }
    impl Default for ApplicationInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_application_name: core::ptr::null(),
                application_version: Default::default(),
                p_engine_name: core::ptr::null(),
                engine_version: Default::default(),
                api_version: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ApplicationInfo<'a> {
        pub fn application_version(mut self, application_version: u32) -> Self {
            self.application_version = application_version;
            self
        }
        pub fn engine_version(mut self, engine_version: u32) -> Self {
            self.engine_version = engine_version;
            self
        }
        pub fn api_version(mut self, api_version: u32) -> Self {
            self.api_version = api_version;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AllocationCallbacks<'a> {
        pub p_user_data: *mut c_void,
        pub pfn_allocation: Option<PFN_vkAllocationFunction>,
        pub pfn_reallocation: Option<PFN_vkReallocationFunction>,
        pub pfn_free: Option<PFN_vkFreeFunction>,
        pub pfn_internal_allocation: Option<PFN_vkInternalAllocationNotification>,
        pub pfn_internal_free: Option<PFN_vkInternalFreeNotification>,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for AllocationCallbacks<'_> {
        fn default() -> Self {
            Self {
                p_user_data: core::ptr::null_mut(),
                pfn_allocation: Default::default(),
                pfn_reallocation: Default::default(),
                pfn_free: Default::default(),
                pfn_internal_allocation: Default::default(),
                pfn_internal_free: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> AllocationCallbacks<'a> {
        pub fn user_data(mut self, user_data: *mut c_void) -> Self {
            self.p_user_data = user_data;
            self
        }
        pub fn pfn_allocation(mut self, pfn_allocation: PFN_vkAllocationFunction) -> Self {
            self.pfn_allocation = Some(pfn_allocation);
            self
        }
        pub fn pfn_reallocation(mut self, pfn_reallocation: PFN_vkReallocationFunction) -> Self {
            self.pfn_reallocation = Some(pfn_reallocation);
            self
        }
        pub fn pfn_free(mut self, pfn_free: PFN_vkFreeFunction) -> Self {
            self.pfn_free = Some(pfn_free);
            self
        }
        pub fn pfn_internal_allocation(
            mut self,
            pfn_internal_allocation: PFN_vkInternalAllocationNotification,
        ) -> Self {
            self.pfn_internal_allocation = Some(pfn_internal_allocation);
            self
        }
        pub fn pfn_internal_free(
            mut self,
            pfn_internal_free: PFN_vkInternalFreeNotification,
        ) -> Self {
            self.pfn_internal_free = Some(pfn_internal_free);
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceQueueCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DeviceQueueCreateFlags,
        pub queue_family_index: u32,
        pub queue_count: u32,
        pub p_queue_priorities: *const f32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DeviceQueueCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_QUEUE_CREATE_INFO;
    }
    impl Default for DeviceQueueCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                queue_family_index: Default::default(),
                queue_count: Default::default(),
                p_queue_priorities: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceQueueCreateInfo<'a> {
        pub fn flags(mut self, flags: DeviceQueueCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
            self.queue_family_index = queue_family_index;
            self
        }
        pub fn queue_priorities(mut self, queue_priorities: &'a [f32]) -> Self {
            self.queue_count = queue_priorities.len().try_into().unwrap();
            self.p_queue_priorities = queue_priorities.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DeviceCreateFlags,
        pub queue_create_info_count: u32,
        pub p_queue_create_infos: *const DeviceQueueCreateInfo<'a>,
        pub enabled_layer_count: u32,
        pub pp_enabled_layer_names: *const *const c_char,
        pub enabled_extension_count: u32,
        pub pp_enabled_extension_names: *const *const c_char,
        pub p_enabled_features: *const PhysicalDeviceFeatures,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DeviceCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_CREATE_INFO;
    }
    impl Default for DeviceCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                queue_create_info_count: Default::default(),
                p_queue_create_infos: core::ptr::null(),
                enabled_layer_count: Default::default(),
                pp_enabled_layer_names: core::ptr::null(),
                enabled_extension_count: Default::default(),
                pp_enabled_extension_names: core::ptr::null(),
                p_enabled_features: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceCreateInfo<'a> {
        pub fn flags(mut self, flags: DeviceCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn queue_create_infos(
            mut self,
            queue_create_infos: &'a [DeviceQueueCreateInfo<'a>],
        ) -> Self {
            self.queue_create_info_count = queue_create_infos.len().try_into().unwrap();
            self.p_queue_create_infos = queue_create_infos.as_ptr();
            self
        }
        pub fn enabled_layer_names_ptrs(
            mut self,
            enabled_layer_names_ptrs: &'a [*const c_char],
        ) -> Self {
            self.enabled_layer_count = enabled_layer_names_ptrs.len().try_into().unwrap();
            self.pp_enabled_layer_names = enabled_layer_names_ptrs.as_ptr() as _;
            self
        }
        pub fn enabled_extension_names_ptrs(
            mut self,
            enabled_extension_names_ptrs: &'a [*const c_char],
        ) -> Self {
            self.enabled_extension_count = enabled_extension_names_ptrs.len().try_into().unwrap();
            self.pp_enabled_extension_names = enabled_extension_names_ptrs.as_ptr() as _;
            self
        }
        pub fn enabled_features(mut self, enabled_features: &'a PhysicalDeviceFeatures) -> Self {
            self.p_enabled_features = enabled_features;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct InstanceCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: InstanceCreateFlags,
        pub p_application_info: *const ApplicationInfo<'a>,
        pub enabled_layer_count: u32,
        pub pp_enabled_layer_names: *const *const c_char,
        pub enabled_extension_count: u32,
        pub pp_enabled_extension_names: *const *const c_char,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for InstanceCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::INSTANCE_CREATE_INFO;
    }
    impl Default for InstanceCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                p_application_info: core::ptr::null(),
                enabled_layer_count: Default::default(),
                pp_enabled_layer_names: core::ptr::null(),
                enabled_extension_count: Default::default(),
                pp_enabled_extension_names: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> InstanceCreateInfo<'a> {
        pub fn flags(mut self, flags: InstanceCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn application_info(mut self, application_info: &'a ApplicationInfo<'a>) -> Self {
            self.p_application_info = application_info;
            self
        }
        pub fn enabled_layer_names_ptrs(
            mut self,
            enabled_layer_names_ptrs: &'a [*const c_char],
        ) -> Self {
            self.enabled_layer_count = enabled_layer_names_ptrs.len().try_into().unwrap();
            self.pp_enabled_layer_names = enabled_layer_names_ptrs.as_ptr() as _;
            self
        }
        pub fn enabled_extension_names_ptrs(
            mut self,
            enabled_extension_names_ptrs: &'a [*const c_char],
        ) -> Self {
            self.enabled_extension_count = enabled_extension_names_ptrs.len().try_into().unwrap();
            self.pp_enabled_extension_names = enabled_extension_names_ptrs.as_ptr() as _;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct QueueFamilyProperties {
        pub queue_flags: QueueFlags,
        pub queue_count: u32,
        pub timestamp_valid_bits: u32,
        pub min_image_transfer_granularity: Extent3D,
    }
    impl QueueFamilyProperties {
        pub fn queue_flags(mut self, queue_flags: QueueFlags) -> Self {
            self.queue_flags = queue_flags;
            self
        }
        pub fn queue_count(mut self, queue_count: u32) -> Self {
            self.queue_count = queue_count;
            self
        }
        pub fn timestamp_valid_bits(mut self, timestamp_valid_bits: u32) -> Self {
            self.timestamp_valid_bits = timestamp_valid_bits;
            self
        }
        pub fn min_image_transfer_granularity(
            mut self,
            min_image_transfer_granularity: Extent3D,
        ) -> Self {
            self.min_image_transfer_granularity = min_image_transfer_granularity;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMemoryProperties {
        pub memory_type_count: u32,
        pub memory_types: [MemoryType; MAX_MEMORY_TYPES as usize],
        pub memory_heap_count: u32,
        pub memory_heaps: [MemoryHeap; MAX_MEMORY_HEAPS as usize],
    }
    impl Default for PhysicalDeviceMemoryProperties {
        fn default() -> Self {
            Self {
                memory_type_count: Default::default(),
                memory_types: [Default::default(); _],
                memory_heap_count: Default::default(),
                memory_heaps: [Default::default(); _],
            }
        }
    }
    impl PhysicalDeviceMemoryProperties {
        pub fn memory_types(mut self, memory_types: &[MemoryType]) -> Self {
            self.memory_type_count = memory_types.len().try_into().unwrap();
            self.memory_types[..memory_types.len()].copy_from_slice(memory_types);
            self
        }
        pub fn memory_heaps(mut self, memory_heaps: &[MemoryHeap]) -> Self {
            self.memory_heap_count = memory_heaps.len().try_into().unwrap();
            self.memory_heaps[..memory_heaps.len()].copy_from_slice(memory_heaps);
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryAllocateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub allocation_size: DeviceSize,
        pub memory_type_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryAllocateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_ALLOCATE_INFO;
    }
    impl Default for MemoryAllocateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                allocation_size: Default::default(),
                memory_type_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryAllocateInfo<'a> {
        pub fn allocation_size(mut self, allocation_size: DeviceSize) -> Self {
            self.allocation_size = allocation_size;
            self
        }
        pub fn memory_type_index(mut self, memory_type_index: u32) -> Self {
            self.memory_type_index = memory_type_index;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct MemoryRequirements {
        pub size: DeviceSize,
        pub alignment: DeviceSize,
        pub memory_type_bits: u32,
    }
    impl MemoryRequirements {
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
        pub fn alignment(mut self, alignment: DeviceSize) -> Self {
            self.alignment = alignment;
            self
        }
        pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
            self.memory_type_bits = memory_type_bits;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct SparseImageFormatProperties {
        pub aspect_mask: ImageAspectFlags,
        pub image_granularity: Extent3D,
        pub flags: SparseImageFormatFlags,
    }
    impl SparseImageFormatProperties {
        pub fn aspect_mask(mut self, aspect_mask: ImageAspectFlags) -> Self {
            self.aspect_mask = aspect_mask;
            self
        }
        pub fn image_granularity(mut self, image_granularity: Extent3D) -> Self {
            self.image_granularity = image_granularity;
            self
        }
        pub fn flags(mut self, flags: SparseImageFormatFlags) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct SparseImageMemoryRequirements {
        pub format_properties: SparseImageFormatProperties,
        pub image_mip_tail_first_lod: u32,
        pub image_mip_tail_size: DeviceSize,
        pub image_mip_tail_offset: DeviceSize,
        pub image_mip_tail_stride: DeviceSize,
    }
    impl SparseImageMemoryRequirements {
        pub fn format_properties(mut self, format_properties: SparseImageFormatProperties) -> Self {
            self.format_properties = format_properties;
            self
        }
        pub fn image_mip_tail_first_lod(mut self, image_mip_tail_first_lod: u32) -> Self {
            self.image_mip_tail_first_lod = image_mip_tail_first_lod;
            self
        }
        pub fn image_mip_tail_size(mut self, image_mip_tail_size: DeviceSize) -> Self {
            self.image_mip_tail_size = image_mip_tail_size;
            self
        }
        pub fn image_mip_tail_offset(mut self, image_mip_tail_offset: DeviceSize) -> Self {
            self.image_mip_tail_offset = image_mip_tail_offset;
            self
        }
        pub fn image_mip_tail_stride(mut self, image_mip_tail_stride: DeviceSize) -> Self {
            self.image_mip_tail_stride = image_mip_tail_stride;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct MemoryType {
        pub property_flags: MemoryPropertyFlags,
        pub heap_index: u32,
    }
    impl MemoryType {
        pub fn property_flags(mut self, property_flags: MemoryPropertyFlags) -> Self {
            self.property_flags = property_flags;
            self
        }
        pub fn heap_index(mut self, heap_index: u32) -> Self {
            self.heap_index = heap_index;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct MemoryHeap {
        pub size: DeviceSize,
        pub flags: MemoryHeapFlags,
    }
    impl MemoryHeap {
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
        pub fn flags(mut self, flags: MemoryHeapFlags) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MappedMemoryRange<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub offset: DeviceSize,
        pub size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MappedMemoryRange<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MAPPED_MEMORY_RANGE;
    }
    impl Default for MappedMemoryRange<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                memory: Default::default(),
                offset: Default::default(),
                size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MappedMemoryRange<'a> {
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
        pub fn offset(mut self, offset: DeviceSize) -> Self {
            self.offset = offset;
            self
        }
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct FormatProperties {
        pub linear_tiling_features: FormatFeatureFlags,
        pub optimal_tiling_features: FormatFeatureFlags,
        pub buffer_features: FormatFeatureFlags,
    }
    impl FormatProperties {
        pub fn linear_tiling_features(
            mut self,
            linear_tiling_features: FormatFeatureFlags,
        ) -> Self {
            self.linear_tiling_features = linear_tiling_features;
            self
        }
        pub fn optimal_tiling_features(
            mut self,
            optimal_tiling_features: FormatFeatureFlags,
        ) -> Self {
            self.optimal_tiling_features = optimal_tiling_features;
            self
        }
        pub fn buffer_features(mut self, buffer_features: FormatFeatureFlags) -> Self {
            self.buffer_features = buffer_features;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct ImageFormatProperties {
        pub max_extent: Extent3D,
        pub max_mip_levels: u32,
        pub max_array_layers: u32,
        pub sample_counts: SampleCountFlags,
        pub max_resource_size: DeviceSize,
    }
    impl ImageFormatProperties {
        pub fn max_extent(mut self, max_extent: Extent3D) -> Self {
            self.max_extent = max_extent;
            self
        }
        pub fn max_mip_levels(mut self, max_mip_levels: u32) -> Self {
            self.max_mip_levels = max_mip_levels;
            self
        }
        pub fn max_array_layers(mut self, max_array_layers: u32) -> Self {
            self.max_array_layers = max_array_layers;
            self
        }
        pub fn sample_counts(mut self, sample_counts: SampleCountFlags) -> Self {
            self.sample_counts = sample_counts;
            self
        }
        pub fn max_resource_size(mut self, max_resource_size: DeviceSize) -> Self {
            self.max_resource_size = max_resource_size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct DescriptorBufferInfo {
        pub buffer: Buffer,
        pub offset: DeviceSize,
        pub range: DeviceSize,
    }
    impl DescriptorBufferInfo {
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
        pub fn offset(mut self, offset: DeviceSize) -> Self {
            self.offset = offset;
            self
        }
        pub fn range(mut self, range: DeviceSize) -> Self {
            self.range = range;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct DescriptorImageInfo {
        pub sampler: Sampler,
        pub image_view: ImageView,
        pub image_layout: ImageLayout,
    }
    impl DescriptorImageInfo {
        pub fn sampler(mut self, sampler: Sampler) -> Self {
            self.sampler = sampler;
            self
        }
        pub fn image_view(mut self, image_view: ImageView) -> Self {
            self.image_view = image_view;
            self
        }
        pub fn image_layout(mut self, image_layout: ImageLayout) -> Self {
            self.image_layout = image_layout;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct WriteDescriptorSet<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub dst_set: DescriptorSet,
        pub dst_binding: u32,
        pub dst_array_element: u32,
        pub descriptor_count: u32,
        pub descriptor_type: DescriptorType,
        pub p_image_info: *const DescriptorImageInfo,
        pub p_buffer_info: *const DescriptorBufferInfo,
        pub p_texel_buffer_view: *const BufferView,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for WriteDescriptorSet<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::WRITE_DESCRIPTOR_SET;
    }
    impl Default for WriteDescriptorSet<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                dst_set: Default::default(),
                dst_binding: Default::default(),
                dst_array_element: Default::default(),
                descriptor_count: Default::default(),
                descriptor_type: Default::default(),
                p_image_info: core::ptr::null(),
                p_buffer_info: core::ptr::null(),
                p_texel_buffer_view: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> WriteDescriptorSet<'a> {
        pub fn dst_set(mut self, dst_set: DescriptorSet) -> Self {
            self.dst_set = dst_set;
            self
        }
        pub fn dst_binding(mut self, dst_binding: u32) -> Self {
            self.dst_binding = dst_binding;
            self
        }
        pub fn dst_array_element(mut self, dst_array_element: u32) -> Self {
            self.dst_array_element = dst_array_element;
            self
        }
        pub fn image_info(mut self, image_info: &'a [DescriptorImageInfo]) -> Self {
            self.descriptor_count = image_info.len().try_into().unwrap();
            self.p_image_info = image_info.as_ptr();
            self
        }
        pub fn buffer_info(mut self, buffer_info: &'a [DescriptorBufferInfo]) -> Self {
            self.descriptor_count = buffer_info.len().try_into().unwrap();
            self.p_buffer_info = buffer_info.as_ptr();
            self
        }
        pub fn texel_buffer_view(mut self, texel_buffer_view: &'a [BufferView]) -> Self {
            self.descriptor_count = texel_buffer_view.len().try_into().unwrap();
            self.p_texel_buffer_view = texel_buffer_view.as_ptr();
            self
        }
        pub fn descriptor_type(mut self, descriptor_type: DescriptorType) -> Self {
            self.descriptor_type = descriptor_type;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CopyDescriptorSet<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_set: DescriptorSet,
        pub src_binding: u32,
        pub src_array_element: u32,
        pub dst_set: DescriptorSet,
        pub dst_binding: u32,
        pub dst_array_element: u32,
        pub descriptor_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CopyDescriptorSet<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_DESCRIPTOR_SET;
    }
    impl Default for CopyDescriptorSet<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_set: Default::default(),
                src_binding: Default::default(),
                src_array_element: Default::default(),
                dst_set: Default::default(),
                dst_binding: Default::default(),
                dst_array_element: Default::default(),
                descriptor_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CopyDescriptorSet<'a> {
        pub fn src_set(mut self, src_set: DescriptorSet) -> Self {
            self.src_set = src_set;
            self
        }
        pub fn src_binding(mut self, src_binding: u32) -> Self {
            self.src_binding = src_binding;
            self
        }
        pub fn src_array_element(mut self, src_array_element: u32) -> Self {
            self.src_array_element = src_array_element;
            self
        }
        pub fn dst_set(mut self, dst_set: DescriptorSet) -> Self {
            self.dst_set = dst_set;
            self
        }
        pub fn dst_binding(mut self, dst_binding: u32) -> Self {
            self.dst_binding = dst_binding;
            self
        }
        pub fn dst_array_element(mut self, dst_array_element: u32) -> Self {
            self.dst_array_element = dst_array_element;
            self
        }
        pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
            self.descriptor_count = descriptor_count;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BufferCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: BufferCreateFlags,
        pub size: DeviceSize,
        pub usage: BufferUsageFlags,
        pub sharing_mode: SharingMode,
        pub queue_family_index_count: u32,
        pub p_queue_family_indices: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BufferCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_CREATE_INFO;
    }
    impl Default for BufferCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                size: Default::default(),
                usage: Default::default(),
                sharing_mode: Default::default(),
                queue_family_index_count: Default::default(),
                p_queue_family_indices: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BufferCreateInfo<'a> {
        pub fn flags(mut self, flags: BufferCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
        pub fn usage(mut self, usage: BufferUsageFlags) -> Self {
            self.usage = usage;
            self
        }
        pub fn sharing_mode(mut self, sharing_mode: SharingMode) -> Self {
            self.sharing_mode = sharing_mode;
            self
        }
        pub fn queue_family_indices(mut self, queue_family_indices: &'a [u32]) -> Self {
            self.queue_family_index_count = queue_family_indices.len().try_into().unwrap();
            self.p_queue_family_indices = queue_family_indices.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BufferViewCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: BufferViewCreateFlags,
        pub buffer: Buffer,
        pub format: Format,
        pub offset: DeviceSize,
        pub range: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BufferViewCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_VIEW_CREATE_INFO;
    }
    impl Default for BufferViewCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                buffer: Default::default(),
                format: Default::default(),
                offset: Default::default(),
                range: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BufferViewCreateInfo<'a> {
        pub fn flags(mut self, flags: BufferViewCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
        pub fn offset(mut self, offset: DeviceSize) -> Self {
            self.offset = offset;
            self
        }
        pub fn range(mut self, range: DeviceSize) -> Self {
            self.range = range;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct ImageSubresource {
        pub aspect_mask: ImageAspectFlags,
        pub mip_level: u32,
        pub array_layer: u32,
    }
    impl ImageSubresource {
        pub fn aspect_mask(mut self, aspect_mask: ImageAspectFlags) -> Self {
            self.aspect_mask = aspect_mask;
            self
        }
        pub fn mip_level(mut self, mip_level: u32) -> Self {
            self.mip_level = mip_level;
            self
        }
        pub fn array_layer(mut self, array_layer: u32) -> Self {
            self.array_layer = array_layer;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct ImageSubresourceLayers {
        pub aspect_mask: ImageAspectFlags,
        pub mip_level: u32,
        pub base_array_layer: u32,
        pub layer_count: u32,
    }
    impl ImageSubresourceLayers {
        pub fn aspect_mask(mut self, aspect_mask: ImageAspectFlags) -> Self {
            self.aspect_mask = aspect_mask;
            self
        }
        pub fn mip_level(mut self, mip_level: u32) -> Self {
            self.mip_level = mip_level;
            self
        }
        pub fn base_array_layer(mut self, base_array_layer: u32) -> Self {
            self.base_array_layer = base_array_layer;
            self
        }
        pub fn layer_count(mut self, layer_count: u32) -> Self {
            self.layer_count = layer_count;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct ImageSubresourceRange {
        pub aspect_mask: ImageAspectFlags,
        pub base_mip_level: u32,
        pub level_count: u32,
        pub base_array_layer: u32,
        pub layer_count: u32,
    }
    impl ImageSubresourceRange {
        pub fn aspect_mask(mut self, aspect_mask: ImageAspectFlags) -> Self {
            self.aspect_mask = aspect_mask;
            self
        }
        pub fn base_mip_level(mut self, base_mip_level: u32) -> Self {
            self.base_mip_level = base_mip_level;
            self
        }
        pub fn level_count(mut self, level_count: u32) -> Self {
            self.level_count = level_count;
            self
        }
        pub fn base_array_layer(mut self, base_array_layer: u32) -> Self {
            self.base_array_layer = base_array_layer;
            self
        }
        pub fn layer_count(mut self, layer_count: u32) -> Self {
            self.layer_count = layer_count;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryBarrier<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_access_mask: AccessFlags,
        pub dst_access_mask: AccessFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryBarrier<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_BARRIER;
    }
    impl Default for MemoryBarrier<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_access_mask: Default::default(),
                dst_access_mask: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryBarrier<'a> {
        pub fn src_access_mask(mut self, src_access_mask: AccessFlags) -> Self {
            self.src_access_mask = src_access_mask;
            self
        }
        pub fn dst_access_mask(mut self, dst_access_mask: AccessFlags) -> Self {
            self.dst_access_mask = dst_access_mask;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BufferMemoryBarrier<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_access_mask: AccessFlags,
        pub dst_access_mask: AccessFlags,
        pub src_queue_family_index: u32,
        pub dst_queue_family_index: u32,
        pub buffer: Buffer,
        pub offset: DeviceSize,
        pub size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BufferMemoryBarrier<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_MEMORY_BARRIER;
    }
    impl Default for BufferMemoryBarrier<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_access_mask: Default::default(),
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
    impl<'a> BufferMemoryBarrier<'a> {
        pub fn src_access_mask(mut self, src_access_mask: AccessFlags) -> Self {
            self.src_access_mask = src_access_mask;
            self
        }
        pub fn dst_access_mask(mut self, dst_access_mask: AccessFlags) -> Self {
            self.dst_access_mask = dst_access_mask;
            self
        }
        pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
            self.src_queue_family_index = src_queue_family_index;
            self
        }
        pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
            self.dst_queue_family_index = dst_queue_family_index;
            self
        }
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
        pub fn offset(mut self, offset: DeviceSize) -> Self {
            self.offset = offset;
            self
        }
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageMemoryBarrier<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_access_mask: AccessFlags,
        pub dst_access_mask: AccessFlags,
        pub old_layout: ImageLayout,
        pub new_layout: ImageLayout,
        pub src_queue_family_index: u32,
        pub dst_queue_family_index: u32,
        pub image: Image,
        pub subresource_range: ImageSubresourceRange,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageMemoryBarrier<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_MEMORY_BARRIER;
    }
    impl Default for ImageMemoryBarrier<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_access_mask: Default::default(),
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
    impl<'a> ImageMemoryBarrier<'a> {
        pub fn src_access_mask(mut self, src_access_mask: AccessFlags) -> Self {
            self.src_access_mask = src_access_mask;
            self
        }
        pub fn dst_access_mask(mut self, dst_access_mask: AccessFlags) -> Self {
            self.dst_access_mask = dst_access_mask;
            self
        }
        pub fn old_layout(mut self, old_layout: ImageLayout) -> Self {
            self.old_layout = old_layout;
            self
        }
        pub fn new_layout(mut self, new_layout: ImageLayout) -> Self {
            self.new_layout = new_layout;
            self
        }
        pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
            self.src_queue_family_index = src_queue_family_index;
            self
        }
        pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
            self.dst_queue_family_index = dst_queue_family_index;
            self
        }
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }
        pub fn subresource_range(mut self, subresource_range: ImageSubresourceRange) -> Self {
            self.subresource_range = subresource_range;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: ImageCreateFlags,
        pub image_type: ImageType,
        pub format: Format,
        pub extent: Extent3D,
        pub mip_levels: u32,
        pub array_layers: u32,
        pub samples: SampleCountFlagBits,
        pub tiling: ImageTiling,
        pub usage: ImageUsageFlags,
        pub sharing_mode: SharingMode,
        pub queue_family_index_count: u32,
        pub p_queue_family_indices: *const u32,
        pub initial_layout: ImageLayout,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_CREATE_INFO;
    }
    impl Default for ImageCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                image_type: Default::default(),
                format: Default::default(),
                extent: Default::default(),
                mip_levels: Default::default(),
                array_layers: Default::default(),
                samples: Default::default(),
                tiling: Default::default(),
                usage: Default::default(),
                sharing_mode: Default::default(),
                queue_family_index_count: Default::default(),
                p_queue_family_indices: core::ptr::null(),
                initial_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageCreateInfo<'a> {
        pub fn flags(mut self, flags: ImageCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn image_type(mut self, image_type: ImageType) -> Self {
            self.image_type = image_type;
            self
        }
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
        pub fn extent(mut self, extent: Extent3D) -> Self {
            self.extent = extent;
            self
        }
        pub fn mip_levels(mut self, mip_levels: u32) -> Self {
            self.mip_levels = mip_levels;
            self
        }
        pub fn array_layers(mut self, array_layers: u32) -> Self {
            self.array_layers = array_layers;
            self
        }
        pub fn samples(mut self, samples: SampleCountFlagBits) -> Self {
            self.samples = samples;
            self
        }
        pub fn tiling(mut self, tiling: ImageTiling) -> Self {
            self.tiling = tiling;
            self
        }
        pub fn usage(mut self, usage: ImageUsageFlags) -> Self {
            self.usage = usage;
            self
        }
        pub fn sharing_mode(mut self, sharing_mode: SharingMode) -> Self {
            self.sharing_mode = sharing_mode;
            self
        }
        pub fn queue_family_indices(mut self, queue_family_indices: &'a [u32]) -> Self {
            self.queue_family_index_count = queue_family_indices.len().try_into().unwrap();
            self.p_queue_family_indices = queue_family_indices.as_ptr();
            self
        }
        pub fn initial_layout(mut self, initial_layout: ImageLayout) -> Self {
            self.initial_layout = initial_layout;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct SubresourceLayout {
        pub offset: DeviceSize,
        pub size: DeviceSize,
        pub row_pitch: DeviceSize,
        pub array_pitch: DeviceSize,
        pub depth_pitch: DeviceSize,
    }
    impl SubresourceLayout {
        pub fn offset(mut self, offset: DeviceSize) -> Self {
            self.offset = offset;
            self
        }
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
        pub fn row_pitch(mut self, row_pitch: DeviceSize) -> Self {
            self.row_pitch = row_pitch;
            self
        }
        pub fn array_pitch(mut self, array_pitch: DeviceSize) -> Self {
            self.array_pitch = array_pitch;
            self
        }
        pub fn depth_pitch(mut self, depth_pitch: DeviceSize) -> Self {
            self.depth_pitch = depth_pitch;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageViewCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: ImageViewCreateFlags,
        pub image: Image,
        pub view_type: ImageViewType,
        pub format: Format,
        pub components: ComponentMapping,
        pub subresource_range: ImageSubresourceRange,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageViewCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_VIEW_CREATE_INFO;
    }
    impl Default for ImageViewCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                image: Default::default(),
                view_type: Default::default(),
                format: Default::default(),
                components: Default::default(),
                subresource_range: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageViewCreateInfo<'a> {
        pub fn flags(mut self, flags: ImageViewCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }
        pub fn view_type(mut self, view_type: ImageViewType) -> Self {
            self.view_type = view_type;
            self
        }
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
        pub fn components(mut self, components: ComponentMapping) -> Self {
            self.components = components;
            self
        }
        pub fn subresource_range(mut self, subresource_range: ImageSubresourceRange) -> Self {
            self.subresource_range = subresource_range;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct BufferCopy {
        pub src_offset: DeviceSize,
        pub dst_offset: DeviceSize,
        pub size: DeviceSize,
    }
    impl BufferCopy {
        pub fn src_offset(mut self, src_offset: DeviceSize) -> Self {
            self.src_offset = src_offset;
            self
        }
        pub fn dst_offset(mut self, dst_offset: DeviceSize) -> Self {
            self.dst_offset = dst_offset;
            self
        }
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct SparseMemoryBind {
        pub resource_offset: DeviceSize,
        pub size: DeviceSize,
        pub memory: DeviceMemory,
        pub memory_offset: DeviceSize,
        pub flags: SparseMemoryBindFlags,
    }
    impl SparseMemoryBind {
        pub fn resource_offset(mut self, resource_offset: DeviceSize) -> Self {
            self.resource_offset = resource_offset;
            self
        }
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
        pub fn memory_offset(mut self, memory_offset: DeviceSize) -> Self {
            self.memory_offset = memory_offset;
            self
        }
        pub fn flags(mut self, flags: SparseMemoryBindFlags) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct SparseImageMemoryBind {
        pub subresource: ImageSubresource,
        pub offset: Offset3D,
        pub extent: Extent3D,
        pub memory: DeviceMemory,
        pub memory_offset: DeviceSize,
        pub flags: SparseMemoryBindFlags,
    }
    impl SparseImageMemoryBind {
        pub fn subresource(mut self, subresource: ImageSubresource) -> Self {
            self.subresource = subresource;
            self
        }
        pub fn offset(mut self, offset: Offset3D) -> Self {
            self.offset = offset;
            self
        }
        pub fn extent(mut self, extent: Extent3D) -> Self {
            self.extent = extent;
            self
        }
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
        pub fn memory_offset(mut self, memory_offset: DeviceSize) -> Self {
            self.memory_offset = memory_offset;
            self
        }
        pub fn flags(mut self, flags: SparseMemoryBindFlags) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SparseBufferMemoryBindInfo<'a> {
        pub buffer: Buffer,
        pub bind_count: u32,
        pub p_binds: *const SparseMemoryBind,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for SparseBufferMemoryBindInfo<'_> {
        fn default() -> Self {
            Self {
                buffer: Default::default(),
                bind_count: Default::default(),
                p_binds: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SparseBufferMemoryBindInfo<'a> {
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
        pub fn binds(mut self, binds: &'a [SparseMemoryBind]) -> Self {
            self.bind_count = binds.len().try_into().unwrap();
            self.p_binds = binds.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SparseImageOpaqueMemoryBindInfo<'a> {
        pub image: Image,
        pub bind_count: u32,
        pub p_binds: *const SparseMemoryBind,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for SparseImageOpaqueMemoryBindInfo<'_> {
        fn default() -> Self {
            Self {
                image: Default::default(),
                bind_count: Default::default(),
                p_binds: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SparseImageOpaqueMemoryBindInfo<'a> {
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }
        pub fn binds(mut self, binds: &'a [SparseMemoryBind]) -> Self {
            self.bind_count = binds.len().try_into().unwrap();
            self.p_binds = binds.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SparseImageMemoryBindInfo<'a> {
        pub image: Image,
        pub bind_count: u32,
        pub p_binds: *const SparseImageMemoryBind,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for SparseImageMemoryBindInfo<'_> {
        fn default() -> Self {
            Self {
                image: Default::default(),
                bind_count: Default::default(),
                p_binds: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SparseImageMemoryBindInfo<'a> {
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }
        pub fn binds(mut self, binds: &'a [SparseImageMemoryBind]) -> Self {
            self.bind_count = binds.len().try_into().unwrap();
            self.p_binds = binds.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BindSparseInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub wait_semaphore_count: u32,
        pub p_wait_semaphores: *const Semaphore,
        pub buffer_bind_count: u32,
        pub p_buffer_binds: *const SparseBufferMemoryBindInfo<'a>,
        pub image_opaque_bind_count: u32,
        pub p_image_opaque_binds: *const SparseImageOpaqueMemoryBindInfo<'a>,
        pub image_bind_count: u32,
        pub p_image_binds: *const SparseImageMemoryBindInfo<'a>,
        pub signal_semaphore_count: u32,
        pub p_signal_semaphores: *const Semaphore,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BindSparseInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_SPARSE_INFO;
    }
    impl Default for BindSparseInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                wait_semaphore_count: Default::default(),
                p_wait_semaphores: core::ptr::null(),
                buffer_bind_count: Default::default(),
                p_buffer_binds: core::ptr::null(),
                image_opaque_bind_count: Default::default(),
                p_image_opaque_binds: core::ptr::null(),
                image_bind_count: Default::default(),
                p_image_binds: core::ptr::null(),
                signal_semaphore_count: Default::default(),
                p_signal_semaphores: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BindSparseInfo<'a> {
        pub fn wait_semaphores(mut self, wait_semaphores: &'a [Semaphore]) -> Self {
            self.wait_semaphore_count = wait_semaphores.len().try_into().unwrap();
            self.p_wait_semaphores = wait_semaphores.as_ptr();
            self
        }
        pub fn buffer_binds(mut self, buffer_binds: &'a [SparseBufferMemoryBindInfo<'a>]) -> Self {
            self.buffer_bind_count = buffer_binds.len().try_into().unwrap();
            self.p_buffer_binds = buffer_binds.as_ptr();
            self
        }
        pub fn image_opaque_binds(
            mut self,
            image_opaque_binds: &'a [SparseImageOpaqueMemoryBindInfo<'a>],
        ) -> Self {
            self.image_opaque_bind_count = image_opaque_binds.len().try_into().unwrap();
            self.p_image_opaque_binds = image_opaque_binds.as_ptr();
            self
        }
        pub fn image_binds(mut self, image_binds: &'a [SparseImageMemoryBindInfo<'a>]) -> Self {
            self.image_bind_count = image_binds.len().try_into().unwrap();
            self.p_image_binds = image_binds.as_ptr();
            self
        }
        pub fn signal_semaphores(mut self, signal_semaphores: &'a [Semaphore]) -> Self {
            self.signal_semaphore_count = signal_semaphores.len().try_into().unwrap();
            self.p_signal_semaphores = signal_semaphores.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct ImageCopy {
        pub src_subresource: ImageSubresourceLayers,
        pub src_offset: Offset3D,
        pub dst_subresource: ImageSubresourceLayers,
        pub dst_offset: Offset3D,
        pub extent: Extent3D,
    }
    impl ImageCopy {
        pub fn src_subresource(mut self, src_subresource: ImageSubresourceLayers) -> Self {
            self.src_subresource = src_subresource;
            self
        }
        pub fn src_offset(mut self, src_offset: Offset3D) -> Self {
            self.src_offset = src_offset;
            self
        }
        pub fn dst_subresource(mut self, dst_subresource: ImageSubresourceLayers) -> Self {
            self.dst_subresource = dst_subresource;
            self
        }
        pub fn dst_offset(mut self, dst_offset: Offset3D) -> Self {
            self.dst_offset = dst_offset;
            self
        }
        pub fn extent(mut self, extent: Extent3D) -> Self {
            self.extent = extent;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageBlit {
        pub src_subresource: ImageSubresourceLayers,
        pub src_offsets: [Offset3D; 2],
        pub dst_subresource: ImageSubresourceLayers,
        pub dst_offsets: [Offset3D; 2],
    }
    impl Default for ImageBlit {
        fn default() -> Self {
            Self {
                src_subresource: Default::default(),
                src_offsets: [Default::default(); _],
                dst_subresource: Default::default(),
                dst_offsets: [Default::default(); _],
            }
        }
    }
    impl ImageBlit {
        pub fn src_subresource(mut self, src_subresource: ImageSubresourceLayers) -> Self {
            self.src_subresource = src_subresource;
            self
        }
        pub fn src_offsets(mut self, src_offsets: [Offset3D; 2]) -> Self {
            self.src_offsets = src_offsets;
            self
        }
        pub fn dst_subresource(mut self, dst_subresource: ImageSubresourceLayers) -> Self {
            self.dst_subresource = dst_subresource;
            self
        }
        pub fn dst_offsets(mut self, dst_offsets: [Offset3D; 2]) -> Self {
            self.dst_offsets = dst_offsets;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct BufferImageCopy {
        pub buffer_offset: DeviceSize,
        pub buffer_row_length: u32,
        pub buffer_image_height: u32,
        pub image_subresource: ImageSubresourceLayers,
        pub image_offset: Offset3D,
        pub image_extent: Extent3D,
    }
    impl BufferImageCopy {
        pub fn buffer_offset(mut self, buffer_offset: DeviceSize) -> Self {
            self.buffer_offset = buffer_offset;
            self
        }
        pub fn buffer_row_length(mut self, buffer_row_length: u32) -> Self {
            self.buffer_row_length = buffer_row_length;
            self
        }
        pub fn buffer_image_height(mut self, buffer_image_height: u32) -> Self {
            self.buffer_image_height = buffer_image_height;
            self
        }
        pub fn image_subresource(mut self, image_subresource: ImageSubresourceLayers) -> Self {
            self.image_subresource = image_subresource;
            self
        }
        pub fn image_offset(mut self, image_offset: Offset3D) -> Self {
            self.image_offset = image_offset;
            self
        }
        pub fn image_extent(mut self, image_extent: Extent3D) -> Self {
            self.image_extent = image_extent;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct ImageResolve {
        pub src_subresource: ImageSubresourceLayers,
        pub src_offset: Offset3D,
        pub dst_subresource: ImageSubresourceLayers,
        pub dst_offset: Offset3D,
        pub extent: Extent3D,
    }
    impl ImageResolve {
        pub fn src_subresource(mut self, src_subresource: ImageSubresourceLayers) -> Self {
            self.src_subresource = src_subresource;
            self
        }
        pub fn src_offset(mut self, src_offset: Offset3D) -> Self {
            self.src_offset = src_offset;
            self
        }
        pub fn dst_subresource(mut self, dst_subresource: ImageSubresourceLayers) -> Self {
            self.dst_subresource = dst_subresource;
            self
        }
        pub fn dst_offset(mut self, dst_offset: Offset3D) -> Self {
            self.dst_offset = dst_offset;
            self
        }
        pub fn extent(mut self, extent: Extent3D) -> Self {
            self.extent = extent;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ShaderModuleCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: ShaderModuleCreateFlags,
        pub code_size: usize,
        pub p_code: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ShaderModuleCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SHADER_MODULE_CREATE_INFO;
    }
    unsafe impl<'a> Extends<PipelineShaderStageCreateInfo<'a>> for ShaderModuleCreateInfo<'a> {}
    unsafe impl<'a> Extends<DataGraphPipelineCreateInfoARM<'a>> for ShaderModuleCreateInfo<'a> {}
    impl Default for ShaderModuleCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                code_size: Default::default(),
                p_code: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ShaderModuleCreateInfo<'a> {
        pub fn flags(mut self, flags: ShaderModuleCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn code_size(mut self, code_size: usize) -> Self {
            self.code_size = code_size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DescriptorSetLayoutBinding<'a> {
        pub binding: u32,
        pub descriptor_type: DescriptorType,
        pub descriptor_count: u32,
        pub stage_flags: ShaderStageFlags,
        pub p_immutable_samplers: *const Sampler,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for DescriptorSetLayoutBinding<'_> {
        fn default() -> Self {
            Self {
                binding: Default::default(),
                descriptor_type: Default::default(),
                descriptor_count: Default::default(),
                stage_flags: Default::default(),
                p_immutable_samplers: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DescriptorSetLayoutBinding<'a> {
        pub fn binding(mut self, binding: u32) -> Self {
            self.binding = binding;
            self
        }
        pub fn descriptor_type(mut self, descriptor_type: DescriptorType) -> Self {
            self.descriptor_type = descriptor_type;
            self
        }
        pub fn immutable_samplers(mut self, immutable_samplers: &'a [Sampler]) -> Self {
            self.descriptor_count = immutable_samplers.len().try_into().unwrap();
            self.p_immutable_samplers = immutable_samplers.as_ptr();
            self
        }
        pub fn stage_flags(mut self, stage_flags: ShaderStageFlags) -> Self {
            self.stage_flags = stage_flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DescriptorSetLayoutCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DescriptorSetLayoutCreateFlags,
        pub binding_count: u32,
        pub p_bindings: *const DescriptorSetLayoutBinding<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DescriptorSetLayoutCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO;
    }
    impl Default for DescriptorSetLayoutCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                binding_count: Default::default(),
                p_bindings: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DescriptorSetLayoutCreateInfo<'a> {
        pub fn flags(mut self, flags: DescriptorSetLayoutCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn bindings(mut self, bindings: &'a [DescriptorSetLayoutBinding<'a>]) -> Self {
            self.binding_count = bindings.len().try_into().unwrap();
            self.p_bindings = bindings.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct DescriptorPoolSize {
        pub ty: DescriptorType,
        pub descriptor_count: u32,
    }
    impl DescriptorPoolSize {
        pub fn ty(mut self, ty: DescriptorType) -> Self {
            self.ty = ty;
            self
        }
        pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
            self.descriptor_count = descriptor_count;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DescriptorPoolCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DescriptorPoolCreateFlags,
        pub max_sets: u32,
        pub pool_size_count: u32,
        pub p_pool_sizes: *const DescriptorPoolSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DescriptorPoolCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DESCRIPTOR_POOL_CREATE_INFO;
    }
    impl Default for DescriptorPoolCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                max_sets: Default::default(),
                pool_size_count: Default::default(),
                p_pool_sizes: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DescriptorPoolCreateInfo<'a> {
        pub fn flags(mut self, flags: DescriptorPoolCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn max_sets(mut self, max_sets: u32) -> Self {
            self.max_sets = max_sets;
            self
        }
        pub fn pool_sizes(mut self, pool_sizes: &'a [DescriptorPoolSize]) -> Self {
            self.pool_size_count = pool_sizes.len().try_into().unwrap();
            self.p_pool_sizes = pool_sizes.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DescriptorSetAllocateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub descriptor_pool: DescriptorPool,
        pub descriptor_set_count: u32,
        pub p_set_layouts: *const DescriptorSetLayout,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DescriptorSetAllocateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DESCRIPTOR_SET_ALLOCATE_INFO;
    }
    impl Default for DescriptorSetAllocateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                descriptor_pool: Default::default(),
                descriptor_set_count: Default::default(),
                p_set_layouts: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DescriptorSetAllocateInfo<'a> {
        pub fn descriptor_pool(mut self, descriptor_pool: DescriptorPool) -> Self {
            self.descriptor_pool = descriptor_pool;
            self
        }
        pub fn set_layouts(mut self, set_layouts: &'a [DescriptorSetLayout]) -> Self {
            self.descriptor_set_count = set_layouts.len().try_into().unwrap();
            self.p_set_layouts = set_layouts.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct SpecializationMapEntry {
        pub constant_id: u32,
        pub offset: u32,
        pub size: usize,
    }
    impl SpecializationMapEntry {
        pub fn constant_id(mut self, constant_id: u32) -> Self {
            self.constant_id = constant_id;
            self
        }
        pub fn offset(mut self, offset: u32) -> Self {
            self.offset = offset;
            self
        }
        pub fn size(mut self, size: usize) -> Self {
            self.size = size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SpecializationInfo<'a> {
        pub map_entry_count: u32,
        pub p_map_entries: *const SpecializationMapEntry,
        pub data_size: usize,
        pub p_data: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for SpecializationInfo<'_> {
        fn default() -> Self {
            Self {
                map_entry_count: Default::default(),
                p_map_entries: core::ptr::null(),
                data_size: Default::default(),
                p_data: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SpecializationInfo<'a> {
        pub fn map_entries(mut self, map_entries: &'a [SpecializationMapEntry]) -> Self {
            self.map_entry_count = map_entries.len().try_into().unwrap();
            self.p_map_entries = map_entries.as_ptr();
            self
        }
        pub fn data(mut self, data: &'a [u8]) -> Self {
            self.data_size = data.len().try_into().unwrap();
            self.p_data = data.as_ptr() as _;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineShaderStageCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineShaderStageCreateFlags,
        pub stage: ShaderStageFlagBits,
        pub module: ShaderModule,
        pub p_name: *const c_char,
        pub p_specialization_info: *const SpecializationInfo<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineShaderStageCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO;
    }
    impl Default for PipelineShaderStageCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                stage: Default::default(),
                module: Default::default(),
                p_name: core::ptr::null(),
                p_specialization_info: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineShaderStageCreateInfo<'a> {
        pub fn flags(mut self, flags: PipelineShaderStageCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn stage(mut self, stage: ShaderStageFlagBits) -> Self {
            self.stage = stage;
            self
        }
        pub fn module(mut self, module: ShaderModule) -> Self {
            self.module = module;
            self
        }
        pub fn specialization_info(
            mut self,
            specialization_info: &'a SpecializationInfo<'a>,
        ) -> Self {
            self.p_specialization_info = specialization_info;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ComputePipelineCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineCreateFlags,
        pub stage: PipelineShaderStageCreateInfo<'a>,
        pub layout: PipelineLayout,
        pub base_pipeline_handle: Pipeline,
        pub base_pipeline_index: i32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ComputePipelineCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COMPUTE_PIPELINE_CREATE_INFO;
    }
    impl Default for ComputePipelineCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                stage: Default::default(),
                layout: Default::default(),
                base_pipeline_handle: Default::default(),
                base_pipeline_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ComputePipelineCreateInfo<'a> {
        pub fn flags(mut self, flags: PipelineCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn stage(mut self, stage: PipelineShaderStageCreateInfo<'a>) -> Self {
            self.stage = stage;
            self
        }
        pub fn layout(mut self, layout: PipelineLayout) -> Self {
            self.layout = layout;
            self
        }
        pub fn base_pipeline_handle(mut self, base_pipeline_handle: Pipeline) -> Self {
            self.base_pipeline_handle = base_pipeline_handle;
            self
        }
        pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
            self.base_pipeline_index = base_pipeline_index;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct VertexInputBindingDescription {
        pub binding: u32,
        pub stride: u32,
        pub input_rate: VertexInputRate,
    }
    impl VertexInputBindingDescription {
        pub fn binding(mut self, binding: u32) -> Self {
            self.binding = binding;
            self
        }
        pub fn stride(mut self, stride: u32) -> Self {
            self.stride = stride;
            self
        }
        pub fn input_rate(mut self, input_rate: VertexInputRate) -> Self {
            self.input_rate = input_rate;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct VertexInputAttributeDescription {
        pub location: u32,
        pub binding: u32,
        pub format: Format,
        pub offset: u32,
    }
    impl VertexInputAttributeDescription {
        pub fn location(mut self, location: u32) -> Self {
            self.location = location;
            self
        }
        pub fn binding(mut self, binding: u32) -> Self {
            self.binding = binding;
            self
        }
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
        pub fn offset(mut self, offset: u32) -> Self {
            self.offset = offset;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineVertexInputStateCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineVertexInputStateCreateFlags,
        pub vertex_binding_description_count: u32,
        pub p_vertex_binding_descriptions: *const VertexInputBindingDescription,
        pub vertex_attribute_description_count: u32,
        pub p_vertex_attribute_descriptions: *const VertexInputAttributeDescription,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineVertexInputStateCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
    }
    impl Default for PipelineVertexInputStateCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                vertex_binding_description_count: Default::default(),
                p_vertex_binding_descriptions: core::ptr::null(),
                vertex_attribute_description_count: Default::default(),
                p_vertex_attribute_descriptions: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineVertexInputStateCreateInfo<'a> {
        pub fn flags(mut self, flags: PipelineVertexInputStateCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn vertex_binding_descriptions(
            mut self,
            vertex_binding_descriptions: &'a [VertexInputBindingDescription],
        ) -> Self {
            self.vertex_binding_description_count =
                vertex_binding_descriptions.len().try_into().unwrap();
            self.p_vertex_binding_descriptions = vertex_binding_descriptions.as_ptr();
            self
        }
        pub fn vertex_attribute_descriptions(
            mut self,
            vertex_attribute_descriptions: &'a [VertexInputAttributeDescription],
        ) -> Self {
            self.vertex_attribute_description_count =
                vertex_attribute_descriptions.len().try_into().unwrap();
            self.p_vertex_attribute_descriptions = vertex_attribute_descriptions.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineInputAssemblyStateCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineInputAssemblyStateCreateFlags,
        pub topology: PrimitiveTopology,
        pub primitive_restart_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineInputAssemblyStateCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;
    }
    impl Default for PipelineInputAssemblyStateCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                topology: Default::default(),
                primitive_restart_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineInputAssemblyStateCreateInfo<'a> {
        pub fn flags(mut self, flags: PipelineInputAssemblyStateCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn topology(mut self, topology: PrimitiveTopology) -> Self {
            self.topology = topology;
            self
        }
        pub fn primitive_restart_enable(mut self, primitive_restart_enable: Bool32) -> Self {
            self.primitive_restart_enable = primitive_restart_enable;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineTessellationStateCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineTessellationStateCreateFlags,
        pub patch_control_points: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineTessellationStateCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_TESSELLATION_STATE_CREATE_INFO;
    }
    impl Default for PipelineTessellationStateCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                patch_control_points: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineTessellationStateCreateInfo<'a> {
        pub fn flags(mut self, flags: PipelineTessellationStateCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn patch_control_points(mut self, patch_control_points: u32) -> Self {
            self.patch_control_points = patch_control_points;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineViewportStateCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineViewportStateCreateFlags,
        pub viewport_count: u32,
        pub p_viewports: *const Viewport,
        pub scissor_count: u32,
        pub p_scissors: *const Rect2D,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineViewportStateCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO;
    }
    impl Default for PipelineViewportStateCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                viewport_count: Default::default(),
                p_viewports: core::ptr::null(),
                scissor_count: Default::default(),
                p_scissors: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineViewportStateCreateInfo<'a> {
        pub fn flags(mut self, flags: PipelineViewportStateCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn viewports(mut self, viewports: &'a [Viewport]) -> Self {
            self.viewport_count = viewports.len().try_into().unwrap();
            self.p_viewports = viewports.as_ptr();
            self
        }
        pub fn scissors(mut self, scissors: &'a [Rect2D]) -> Self {
            self.scissor_count = scissors.len().try_into().unwrap();
            self.p_scissors = scissors.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineRasterizationStateCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineRasterizationStateCreateFlags,
        pub depth_clamp_enable: Bool32,
        pub rasterizer_discard_enable: Bool32,
        pub polygon_mode: PolygonMode,
        pub cull_mode: CullModeFlags,
        pub front_face: FrontFace,
        pub depth_bias_enable: Bool32,
        pub depth_bias_constant_factor: f32,
        pub depth_bias_clamp: f32,
        pub depth_bias_slope_factor: f32,
        pub line_width: f32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineRasterizationStateCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO;
    }
    impl Default for PipelineRasterizationStateCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                depth_clamp_enable: Default::default(),
                rasterizer_discard_enable: Default::default(),
                polygon_mode: Default::default(),
                cull_mode: Default::default(),
                front_face: Default::default(),
                depth_bias_enable: Default::default(),
                depth_bias_constant_factor: Default::default(),
                depth_bias_clamp: Default::default(),
                depth_bias_slope_factor: Default::default(),
                line_width: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineRasterizationStateCreateInfo<'a> {
        pub fn flags(mut self, flags: PipelineRasterizationStateCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn depth_clamp_enable(mut self, depth_clamp_enable: Bool32) -> Self {
            self.depth_clamp_enable = depth_clamp_enable;
            self
        }
        pub fn rasterizer_discard_enable(mut self, rasterizer_discard_enable: Bool32) -> Self {
            self.rasterizer_discard_enable = rasterizer_discard_enable;
            self
        }
        pub fn polygon_mode(mut self, polygon_mode: PolygonMode) -> Self {
            self.polygon_mode = polygon_mode;
            self
        }
        pub fn cull_mode(mut self, cull_mode: CullModeFlags) -> Self {
            self.cull_mode = cull_mode;
            self
        }
        pub fn front_face(mut self, front_face: FrontFace) -> Self {
            self.front_face = front_face;
            self
        }
        pub fn depth_bias_enable(mut self, depth_bias_enable: Bool32) -> Self {
            self.depth_bias_enable = depth_bias_enable;
            self
        }
        pub fn depth_bias_constant_factor(mut self, depth_bias_constant_factor: f32) -> Self {
            self.depth_bias_constant_factor = depth_bias_constant_factor;
            self
        }
        pub fn depth_bias_clamp(mut self, depth_bias_clamp: f32) -> Self {
            self.depth_bias_clamp = depth_bias_clamp;
            self
        }
        pub fn depth_bias_slope_factor(mut self, depth_bias_slope_factor: f32) -> Self {
            self.depth_bias_slope_factor = depth_bias_slope_factor;
            self
        }
        pub fn line_width(mut self, line_width: f32) -> Self {
            self.line_width = line_width;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineMultisampleStateCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineMultisampleStateCreateFlags,
        pub rasterization_samples: SampleCountFlagBits,
        pub sample_shading_enable: Bool32,
        pub min_sample_shading: f32,
        pub p_sample_mask: *const SampleMask,
        pub alpha_to_coverage_enable: Bool32,
        pub alpha_to_one_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineMultisampleStateCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO;
    }
    impl Default for PipelineMultisampleStateCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                rasterization_samples: Default::default(),
                sample_shading_enable: Default::default(),
                min_sample_shading: Default::default(),
                p_sample_mask: core::ptr::null(),
                alpha_to_coverage_enable: Default::default(),
                alpha_to_one_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineMultisampleStateCreateInfo<'a> {
        pub fn flags(mut self, flags: PipelineMultisampleStateCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn rasterization_samples(mut self, rasterization_samples: SampleCountFlagBits) -> Self {
            self.rasterization_samples = rasterization_samples;
            self
        }
        pub fn sample_shading_enable(mut self, sample_shading_enable: Bool32) -> Self {
            self.sample_shading_enable = sample_shading_enable;
            self
        }
        pub fn min_sample_shading(mut self, min_sample_shading: f32) -> Self {
            self.min_sample_shading = min_sample_shading;
            self
        }
        pub fn alpha_to_coverage_enable(mut self, alpha_to_coverage_enable: Bool32) -> Self {
            self.alpha_to_coverage_enable = alpha_to_coverage_enable;
            self
        }
        pub fn alpha_to_one_enable(mut self, alpha_to_one_enable: Bool32) -> Self {
            self.alpha_to_one_enable = alpha_to_one_enable;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct PipelineColorBlendAttachmentState {
        pub blend_enable: Bool32,
        pub src_color_blend_factor: BlendFactor,
        pub dst_color_blend_factor: BlendFactor,
        pub color_blend_op: BlendOp,
        pub src_alpha_blend_factor: BlendFactor,
        pub dst_alpha_blend_factor: BlendFactor,
        pub alpha_blend_op: BlendOp,
        pub color_write_mask: ColorComponentFlags,
    }
    impl PipelineColorBlendAttachmentState {
        pub fn blend_enable(mut self, blend_enable: Bool32) -> Self {
            self.blend_enable = blend_enable;
            self
        }
        pub fn src_color_blend_factor(mut self, src_color_blend_factor: BlendFactor) -> Self {
            self.src_color_blend_factor = src_color_blend_factor;
            self
        }
        pub fn dst_color_blend_factor(mut self, dst_color_blend_factor: BlendFactor) -> Self {
            self.dst_color_blend_factor = dst_color_blend_factor;
            self
        }
        pub fn color_blend_op(mut self, color_blend_op: BlendOp) -> Self {
            self.color_blend_op = color_blend_op;
            self
        }
        pub fn src_alpha_blend_factor(mut self, src_alpha_blend_factor: BlendFactor) -> Self {
            self.src_alpha_blend_factor = src_alpha_blend_factor;
            self
        }
        pub fn dst_alpha_blend_factor(mut self, dst_alpha_blend_factor: BlendFactor) -> Self {
            self.dst_alpha_blend_factor = dst_alpha_blend_factor;
            self
        }
        pub fn alpha_blend_op(mut self, alpha_blend_op: BlendOp) -> Self {
            self.alpha_blend_op = alpha_blend_op;
            self
        }
        pub fn color_write_mask(mut self, color_write_mask: ColorComponentFlags) -> Self {
            self.color_write_mask = color_write_mask;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineColorBlendStateCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineColorBlendStateCreateFlags,
        pub logic_op_enable: Bool32,
        pub logic_op: LogicOp,
        pub attachment_count: u32,
        pub p_attachments: *const PipelineColorBlendAttachmentState,
        pub blend_constants: [f32; 4],
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineColorBlendStateCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO;
    }
    impl Default for PipelineColorBlendStateCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                logic_op_enable: Default::default(),
                logic_op: Default::default(),
                attachment_count: Default::default(),
                p_attachments: core::ptr::null(),
                blend_constants: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineColorBlendStateCreateInfo<'a> {
        pub fn flags(mut self, flags: PipelineColorBlendStateCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn logic_op_enable(mut self, logic_op_enable: Bool32) -> Self {
            self.logic_op_enable = logic_op_enable;
            self
        }
        pub fn logic_op(mut self, logic_op: LogicOp) -> Self {
            self.logic_op = logic_op;
            self
        }
        pub fn attachments(mut self, attachments: &'a [PipelineColorBlendAttachmentState]) -> Self {
            self.attachment_count = attachments.len().try_into().unwrap();
            self.p_attachments = attachments.as_ptr();
            self
        }
        pub fn blend_constants(mut self, blend_constants: [f32; 4]) -> Self {
            self.blend_constants = blend_constants;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineDynamicStateCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineDynamicStateCreateFlags,
        pub dynamic_state_count: u32,
        pub p_dynamic_states: *const DynamicState,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineDynamicStateCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO;
    }
    impl Default for PipelineDynamicStateCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                dynamic_state_count: Default::default(),
                p_dynamic_states: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineDynamicStateCreateInfo<'a> {
        pub fn flags(mut self, flags: PipelineDynamicStateCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn dynamic_states(mut self, dynamic_states: &'a [DynamicState]) -> Self {
            self.dynamic_state_count = dynamic_states.len().try_into().unwrap();
            self.p_dynamic_states = dynamic_states.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct StencilOpState {
        pub fail_op: StencilOp,
        pub pass_op: StencilOp,
        pub depth_fail_op: StencilOp,
        pub compare_op: CompareOp,
        pub compare_mask: u32,
        pub write_mask: u32,
        pub reference: u32,
    }
    impl StencilOpState {
        pub fn fail_op(mut self, fail_op: StencilOp) -> Self {
            self.fail_op = fail_op;
            self
        }
        pub fn pass_op(mut self, pass_op: StencilOp) -> Self {
            self.pass_op = pass_op;
            self
        }
        pub fn depth_fail_op(mut self, depth_fail_op: StencilOp) -> Self {
            self.depth_fail_op = depth_fail_op;
            self
        }
        pub fn compare_op(mut self, compare_op: CompareOp) -> Self {
            self.compare_op = compare_op;
            self
        }
        pub fn compare_mask(mut self, compare_mask: u32) -> Self {
            self.compare_mask = compare_mask;
            self
        }
        pub fn write_mask(mut self, write_mask: u32) -> Self {
            self.write_mask = write_mask;
            self
        }
        pub fn reference(mut self, reference: u32) -> Self {
            self.reference = reference;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineDepthStencilStateCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineDepthStencilStateCreateFlags,
        pub depth_test_enable: Bool32,
        pub depth_write_enable: Bool32,
        pub depth_compare_op: CompareOp,
        pub depth_bounds_test_enable: Bool32,
        pub stencil_test_enable: Bool32,
        pub front: StencilOpState,
        pub back: StencilOpState,
        pub min_depth_bounds: f32,
        pub max_depth_bounds: f32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineDepthStencilStateCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO;
    }
    impl Default for PipelineDepthStencilStateCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                depth_test_enable: Default::default(),
                depth_write_enable: Default::default(),
                depth_compare_op: Default::default(),
                depth_bounds_test_enable: Default::default(),
                stencil_test_enable: Default::default(),
                front: Default::default(),
                back: Default::default(),
                min_depth_bounds: Default::default(),
                max_depth_bounds: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineDepthStencilStateCreateInfo<'a> {
        pub fn flags(mut self, flags: PipelineDepthStencilStateCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn depth_test_enable(mut self, depth_test_enable: Bool32) -> Self {
            self.depth_test_enable = depth_test_enable;
            self
        }
        pub fn depth_write_enable(mut self, depth_write_enable: Bool32) -> Self {
            self.depth_write_enable = depth_write_enable;
            self
        }
        pub fn depth_compare_op(mut self, depth_compare_op: CompareOp) -> Self {
            self.depth_compare_op = depth_compare_op;
            self
        }
        pub fn depth_bounds_test_enable(mut self, depth_bounds_test_enable: Bool32) -> Self {
            self.depth_bounds_test_enable = depth_bounds_test_enable;
            self
        }
        pub fn stencil_test_enable(mut self, stencil_test_enable: Bool32) -> Self {
            self.stencil_test_enable = stencil_test_enable;
            self
        }
        pub fn front(mut self, front: StencilOpState) -> Self {
            self.front = front;
            self
        }
        pub fn back(mut self, back: StencilOpState) -> Self {
            self.back = back;
            self
        }
        pub fn min_depth_bounds(mut self, min_depth_bounds: f32) -> Self {
            self.min_depth_bounds = min_depth_bounds;
            self
        }
        pub fn max_depth_bounds(mut self, max_depth_bounds: f32) -> Self {
            self.max_depth_bounds = max_depth_bounds;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GraphicsPipelineCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineCreateFlags,
        pub stage_count: u32,
        pub p_stages: *const PipelineShaderStageCreateInfo<'a>,
        pub p_vertex_input_state: *const PipelineVertexInputStateCreateInfo<'a>,
        pub p_input_assembly_state: *const PipelineInputAssemblyStateCreateInfo<'a>,
        pub p_tessellation_state: *const PipelineTessellationStateCreateInfo<'a>,
        pub p_viewport_state: *const PipelineViewportStateCreateInfo<'a>,
        pub p_rasterization_state: *const PipelineRasterizationStateCreateInfo<'a>,
        pub p_multisample_state: *const PipelineMultisampleStateCreateInfo<'a>,
        pub p_depth_stencil_state: *const PipelineDepthStencilStateCreateInfo<'a>,
        pub p_color_blend_state: *const PipelineColorBlendStateCreateInfo<'a>,
        pub p_dynamic_state: *const PipelineDynamicStateCreateInfo<'a>,
        pub layout: PipelineLayout,
        pub render_pass: RenderPass,
        pub subpass: u32,
        pub base_pipeline_handle: Pipeline,
        pub base_pipeline_index: i32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for GraphicsPipelineCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::GRAPHICS_PIPELINE_CREATE_INFO;
    }
    impl Default for GraphicsPipelineCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                stage_count: Default::default(),
                p_stages: core::ptr::null(),
                p_vertex_input_state: core::ptr::null(),
                p_input_assembly_state: core::ptr::null(),
                p_tessellation_state: core::ptr::null(),
                p_viewport_state: core::ptr::null(),
                p_rasterization_state: core::ptr::null(),
                p_multisample_state: core::ptr::null(),
                p_depth_stencil_state: core::ptr::null(),
                p_color_blend_state: core::ptr::null(),
                p_dynamic_state: core::ptr::null(),
                layout: Default::default(),
                render_pass: Default::default(),
                subpass: Default::default(),
                base_pipeline_handle: Default::default(),
                base_pipeline_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> GraphicsPipelineCreateInfo<'a> {
        pub fn flags(mut self, flags: PipelineCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn stages(mut self, stages: &'a [PipelineShaderStageCreateInfo<'a>]) -> Self {
            self.stage_count = stages.len().try_into().unwrap();
            self.p_stages = stages.as_ptr();
            self
        }
        pub fn vertex_input_state(
            mut self,
            vertex_input_state: &'a PipelineVertexInputStateCreateInfo<'a>,
        ) -> Self {
            self.p_vertex_input_state = vertex_input_state;
            self
        }
        pub fn input_assembly_state(
            mut self,
            input_assembly_state: &'a PipelineInputAssemblyStateCreateInfo<'a>,
        ) -> Self {
            self.p_input_assembly_state = input_assembly_state;
            self
        }
        pub fn tessellation_state(
            mut self,
            tessellation_state: &'a PipelineTessellationStateCreateInfo<'a>,
        ) -> Self {
            self.p_tessellation_state = tessellation_state;
            self
        }
        pub fn viewport_state(
            mut self,
            viewport_state: &'a PipelineViewportStateCreateInfo<'a>,
        ) -> Self {
            self.p_viewport_state = viewport_state;
            self
        }
        pub fn rasterization_state(
            mut self,
            rasterization_state: &'a PipelineRasterizationStateCreateInfo<'a>,
        ) -> Self {
            self.p_rasterization_state = rasterization_state;
            self
        }
        pub fn multisample_state(
            mut self,
            multisample_state: &'a PipelineMultisampleStateCreateInfo<'a>,
        ) -> Self {
            self.p_multisample_state = multisample_state;
            self
        }
        pub fn depth_stencil_state(
            mut self,
            depth_stencil_state: &'a PipelineDepthStencilStateCreateInfo<'a>,
        ) -> Self {
            self.p_depth_stencil_state = depth_stencil_state;
            self
        }
        pub fn color_blend_state(
            mut self,
            color_blend_state: &'a PipelineColorBlendStateCreateInfo<'a>,
        ) -> Self {
            self.p_color_blend_state = color_blend_state;
            self
        }
        pub fn dynamic_state(
            mut self,
            dynamic_state: &'a PipelineDynamicStateCreateInfo<'a>,
        ) -> Self {
            self.p_dynamic_state = dynamic_state;
            self
        }
        pub fn layout(mut self, layout: PipelineLayout) -> Self {
            self.layout = layout;
            self
        }
        pub fn render_pass(mut self, render_pass: RenderPass) -> Self {
            self.render_pass = render_pass;
            self
        }
        pub fn subpass(mut self, subpass: u32) -> Self {
            self.subpass = subpass;
            self
        }
        pub fn base_pipeline_handle(mut self, base_pipeline_handle: Pipeline) -> Self {
            self.base_pipeline_handle = base_pipeline_handle;
            self
        }
        pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
            self.base_pipeline_index = base_pipeline_index;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineCacheCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineCacheCreateFlags,
        pub initial_data_size: usize,
        pub p_initial_data: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineCacheCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_CACHE_CREATE_INFO;
    }
    impl Default for PipelineCacheCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                initial_data_size: Default::default(),
                p_initial_data: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineCacheCreateInfo<'a> {
        pub fn flags(mut self, flags: PipelineCacheCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn initial_data(mut self, initial_data: &'a [u8]) -> Self {
            self.initial_data_size = initial_data.len().try_into().unwrap();
            self.p_initial_data = initial_data.as_ptr() as _;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineCacheHeaderVersionOne {
        pub header_size: u32,
        pub header_version: PipelineCacheHeaderVersion,
        pub vendor_id: u32,
        pub device_id: u32,
        pub pipeline_cache_uuid: [u8; UUID_SIZE as usize],
    }
    impl Default for PipelineCacheHeaderVersionOne {
        fn default() -> Self {
            Self {
                header_size: Default::default(),
                header_version: Default::default(),
                vendor_id: Default::default(),
                device_id: Default::default(),
                pipeline_cache_uuid: [Default::default(); _],
            }
        }
    }
    impl PipelineCacheHeaderVersionOne {
        pub fn header_size(mut self, header_size: u32) -> Self {
            self.header_size = header_size;
            self
        }
        pub fn header_version(mut self, header_version: PipelineCacheHeaderVersion) -> Self {
            self.header_version = header_version;
            self
        }
        pub fn vendor_id(mut self, vendor_id: u32) -> Self {
            self.vendor_id = vendor_id;
            self
        }
        pub fn device_id(mut self, device_id: u32) -> Self {
            self.device_id = device_id;
            self
        }
        pub fn pipeline_cache_uuid(
            mut self,
            pipeline_cache_uuid: [u8; UUID_SIZE as usize],
        ) -> Self {
            self.pipeline_cache_uuid = pipeline_cache_uuid;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct PushConstantRange {
        pub stage_flags: ShaderStageFlags,
        pub offset: u32,
        pub size: u32,
    }
    impl PushConstantRange {
        pub fn stage_flags(mut self, stage_flags: ShaderStageFlags) -> Self {
            self.stage_flags = stage_flags;
            self
        }
        pub fn offset(mut self, offset: u32) -> Self {
            self.offset = offset;
            self
        }
        pub fn size(mut self, size: u32) -> Self {
            self.size = size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineLayoutCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineLayoutCreateFlags,
        pub set_layout_count: u32,
        pub p_set_layouts: *const DescriptorSetLayout,
        pub push_constant_range_count: u32,
        pub p_push_constant_ranges: *const PushConstantRange,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineLayoutCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_LAYOUT_CREATE_INFO;
    }
    unsafe impl<'a> Extends<BindDescriptorSetsInfo<'a>> for PipelineLayoutCreateInfo<'a> {}
    unsafe impl<'a> Extends<PushConstantsInfo<'a>> for PipelineLayoutCreateInfo<'a> {}
    unsafe impl<'a> Extends<PushDescriptorSetInfo<'a>> for PipelineLayoutCreateInfo<'a> {}
    unsafe impl<'a> Extends<PushDescriptorSetWithTemplateInfo<'a>> for PipelineLayoutCreateInfo<'a> {}
    unsafe impl<'a> Extends<SetDescriptorBufferOffsetsInfoEXT<'a>> for PipelineLayoutCreateInfo<'a> {}
    unsafe impl<'a> Extends<BindDescriptorBufferEmbeddedSamplersInfoEXT<'a>>
        for PipelineLayoutCreateInfo<'a>
    {
    }
    unsafe impl<'a> Extends<IndirectCommandsLayoutCreateInfoEXT<'a>> for PipelineLayoutCreateInfo<'a> {}
    impl Default for PipelineLayoutCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                set_layout_count: Default::default(),
                p_set_layouts: core::ptr::null(),
                push_constant_range_count: Default::default(),
                p_push_constant_ranges: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineLayoutCreateInfo<'a> {
        pub fn flags(mut self, flags: PipelineLayoutCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn set_layouts(mut self, set_layouts: &'a [DescriptorSetLayout]) -> Self {
            self.set_layout_count = set_layouts.len().try_into().unwrap();
            self.p_set_layouts = set_layouts.as_ptr();
            self
        }
        pub fn push_constant_ranges(
            mut self,
            push_constant_ranges: &'a [PushConstantRange],
        ) -> Self {
            self.push_constant_range_count = push_constant_ranges.len().try_into().unwrap();
            self.p_push_constant_ranges = push_constant_ranges.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SamplerCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: SamplerCreateFlags,
        pub mag_filter: Filter,
        pub min_filter: Filter,
        pub mipmap_mode: SamplerMipmapMode,
        pub address_mode_u: SamplerAddressMode,
        pub address_mode_v: SamplerAddressMode,
        pub address_mode_w: SamplerAddressMode,
        pub mip_lod_bias: f32,
        pub anisotropy_enable: Bool32,
        pub max_anisotropy: f32,
        pub compare_enable: Bool32,
        pub compare_op: CompareOp,
        pub min_lod: f32,
        pub max_lod: f32,
        pub border_color: BorderColor,
        pub unnormalized_coordinates: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SamplerCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SAMPLER_CREATE_INFO;
    }
    impl Default for SamplerCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                mag_filter: Default::default(),
                min_filter: Default::default(),
                mipmap_mode: Default::default(),
                address_mode_u: Default::default(),
                address_mode_v: Default::default(),
                address_mode_w: Default::default(),
                mip_lod_bias: Default::default(),
                anisotropy_enable: Default::default(),
                max_anisotropy: Default::default(),
                compare_enable: Default::default(),
                compare_op: Default::default(),
                min_lod: Default::default(),
                max_lod: Default::default(),
                border_color: Default::default(),
                unnormalized_coordinates: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SamplerCreateInfo<'a> {
        pub fn flags(mut self, flags: SamplerCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn mag_filter(mut self, mag_filter: Filter) -> Self {
            self.mag_filter = mag_filter;
            self
        }
        pub fn min_filter(mut self, min_filter: Filter) -> Self {
            self.min_filter = min_filter;
            self
        }
        pub fn mipmap_mode(mut self, mipmap_mode: SamplerMipmapMode) -> Self {
            self.mipmap_mode = mipmap_mode;
            self
        }
        pub fn address_mode_u(mut self, address_mode_u: SamplerAddressMode) -> Self {
            self.address_mode_u = address_mode_u;
            self
        }
        pub fn address_mode_v(mut self, address_mode_v: SamplerAddressMode) -> Self {
            self.address_mode_v = address_mode_v;
            self
        }
        pub fn address_mode_w(mut self, address_mode_w: SamplerAddressMode) -> Self {
            self.address_mode_w = address_mode_w;
            self
        }
        pub fn mip_lod_bias(mut self, mip_lod_bias: f32) -> Self {
            self.mip_lod_bias = mip_lod_bias;
            self
        }
        pub fn anisotropy_enable(mut self, anisotropy_enable: Bool32) -> Self {
            self.anisotropy_enable = anisotropy_enable;
            self
        }
        pub fn max_anisotropy(mut self, max_anisotropy: f32) -> Self {
            self.max_anisotropy = max_anisotropy;
            self
        }
        pub fn compare_enable(mut self, compare_enable: Bool32) -> Self {
            self.compare_enable = compare_enable;
            self
        }
        pub fn compare_op(mut self, compare_op: CompareOp) -> Self {
            self.compare_op = compare_op;
            self
        }
        pub fn min_lod(mut self, min_lod: f32) -> Self {
            self.min_lod = min_lod;
            self
        }
        pub fn max_lod(mut self, max_lod: f32) -> Self {
            self.max_lod = max_lod;
            self
        }
        pub fn border_color(mut self, border_color: BorderColor) -> Self {
            self.border_color = border_color;
            self
        }
        pub fn unnormalized_coordinates(mut self, unnormalized_coordinates: Bool32) -> Self {
            self.unnormalized_coordinates = unnormalized_coordinates;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CommandPoolCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: CommandPoolCreateFlags,
        pub queue_family_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CommandPoolCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COMMAND_POOL_CREATE_INFO;
    }
    impl Default for CommandPoolCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                queue_family_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CommandPoolCreateInfo<'a> {
        pub fn flags(mut self, flags: CommandPoolCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
            self.queue_family_index = queue_family_index;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CommandBufferAllocateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub command_pool: CommandPool,
        pub level: CommandBufferLevel,
        pub command_buffer_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CommandBufferAllocateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COMMAND_BUFFER_ALLOCATE_INFO;
    }
    impl Default for CommandBufferAllocateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                command_pool: Default::default(),
                level: Default::default(),
                command_buffer_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CommandBufferAllocateInfo<'a> {
        pub fn command_pool(mut self, command_pool: CommandPool) -> Self {
            self.command_pool = command_pool;
            self
        }
        pub fn level(mut self, level: CommandBufferLevel) -> Self {
            self.level = level;
            self
        }
        pub fn command_buffer_count(mut self, command_buffer_count: u32) -> Self {
            self.command_buffer_count = command_buffer_count;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CommandBufferInheritanceInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub render_pass: RenderPass,
        pub subpass: u32,
        pub framebuffer: Framebuffer,
        pub occlusion_query_enable: Bool32,
        pub query_flags: QueryControlFlags,
        pub pipeline_statistics: QueryPipelineStatisticFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CommandBufferInheritanceInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COMMAND_BUFFER_INHERITANCE_INFO;
    }
    impl Default for CommandBufferInheritanceInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                render_pass: Default::default(),
                subpass: Default::default(),
                framebuffer: Default::default(),
                occlusion_query_enable: Default::default(),
                query_flags: Default::default(),
                pipeline_statistics: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CommandBufferInheritanceInfo<'a> {
        pub fn render_pass(mut self, render_pass: RenderPass) -> Self {
            self.render_pass = render_pass;
            self
        }
        pub fn subpass(mut self, subpass: u32) -> Self {
            self.subpass = subpass;
            self
        }
        pub fn framebuffer(mut self, framebuffer: Framebuffer) -> Self {
            self.framebuffer = framebuffer;
            self
        }
        pub fn occlusion_query_enable(mut self, occlusion_query_enable: Bool32) -> Self {
            self.occlusion_query_enable = occlusion_query_enable;
            self
        }
        pub fn query_flags(mut self, query_flags: QueryControlFlags) -> Self {
            self.query_flags = query_flags;
            self
        }
        pub fn pipeline_statistics(
            mut self,
            pipeline_statistics: QueryPipelineStatisticFlags,
        ) -> Self {
            self.pipeline_statistics = pipeline_statistics;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CommandBufferBeginInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: CommandBufferUsageFlags,
        pub p_inheritance_info: *const CommandBufferInheritanceInfo<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CommandBufferBeginInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COMMAND_BUFFER_BEGIN_INFO;
    }
    impl Default for CommandBufferBeginInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                p_inheritance_info: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CommandBufferBeginInfo<'a> {
        pub fn flags(mut self, flags: CommandBufferUsageFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn inheritance_info(
            mut self,
            inheritance_info: &'a CommandBufferInheritanceInfo<'a>,
        ) -> Self {
            self.p_inheritance_info = inheritance_info;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderPassBeginInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub render_pass: RenderPass,
        pub framebuffer: Framebuffer,
        pub render_area: Rect2D,
        pub clear_value_count: u32,
        pub p_clear_values: *const ClearValue,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for RenderPassBeginInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDER_PASS_BEGIN_INFO;
    }
    impl Default for RenderPassBeginInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                render_pass: Default::default(),
                framebuffer: Default::default(),
                render_area: Default::default(),
                clear_value_count: Default::default(),
                p_clear_values: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderPassBeginInfo<'a> {
        pub fn render_pass(mut self, render_pass: RenderPass) -> Self {
            self.render_pass = render_pass;
            self
        }
        pub fn framebuffer(mut self, framebuffer: Framebuffer) -> Self {
            self.framebuffer = framebuffer;
            self
        }
        pub fn render_area(mut self, render_area: Rect2D) -> Self {
            self.render_area = render_area;
            self
        }
        pub fn clear_values(mut self, clear_values: &'a [ClearValue]) -> Self {
            self.clear_value_count = clear_values.len().try_into().unwrap();
            self.p_clear_values = clear_values.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct ClearDepthStencilValue {
        pub depth: f32,
        pub stencil: u32,
    }
    impl ClearDepthStencilValue {
        pub fn depth(mut self, depth: f32) -> Self {
            self.depth = depth;
            self
        }
        pub fn stencil(mut self, stencil: u32) -> Self {
            self.stencil = stencil;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ClearAttachment {
        pub aspect_mask: ImageAspectFlags,
        pub color_attachment: u32,
        pub clear_value: ClearValue,
    }
    impl Default for ClearAttachment {
        fn default() -> Self {
            Self {
                aspect_mask: Default::default(),
                color_attachment: Default::default(),
                clear_value: Default::default(),
            }
        }
    }
    impl ClearAttachment {
        pub fn aspect_mask(mut self, aspect_mask: ImageAspectFlags) -> Self {
            self.aspect_mask = aspect_mask;
            self
        }
        pub fn color_attachment(mut self, color_attachment: u32) -> Self {
            self.color_attachment = color_attachment;
            self
        }
        pub fn clear_value(mut self, clear_value: ClearValue) -> Self {
            self.clear_value = clear_value;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct AttachmentDescription {
        pub flags: AttachmentDescriptionFlags,
        pub format: Format,
        pub samples: SampleCountFlagBits,
        pub load_op: AttachmentLoadOp,
        pub store_op: AttachmentStoreOp,
        pub stencil_load_op: AttachmentLoadOp,
        pub stencil_store_op: AttachmentStoreOp,
        pub initial_layout: ImageLayout,
        pub final_layout: ImageLayout,
    }
    impl AttachmentDescription {
        pub fn flags(mut self, flags: AttachmentDescriptionFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
        pub fn samples(mut self, samples: SampleCountFlagBits) -> Self {
            self.samples = samples;
            self
        }
        pub fn load_op(mut self, load_op: AttachmentLoadOp) -> Self {
            self.load_op = load_op;
            self
        }
        pub fn store_op(mut self, store_op: AttachmentStoreOp) -> Self {
            self.store_op = store_op;
            self
        }
        pub fn stencil_load_op(mut self, stencil_load_op: AttachmentLoadOp) -> Self {
            self.stencil_load_op = stencil_load_op;
            self
        }
        pub fn stencil_store_op(mut self, stencil_store_op: AttachmentStoreOp) -> Self {
            self.stencil_store_op = stencil_store_op;
            self
        }
        pub fn initial_layout(mut self, initial_layout: ImageLayout) -> Self {
            self.initial_layout = initial_layout;
            self
        }
        pub fn final_layout(mut self, final_layout: ImageLayout) -> Self {
            self.final_layout = final_layout;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct AttachmentReference {
        pub attachment: u32,
        pub layout: ImageLayout,
    }
    impl AttachmentReference {
        pub fn attachment(mut self, attachment: u32) -> Self {
            self.attachment = attachment;
            self
        }
        pub fn layout(mut self, layout: ImageLayout) -> Self {
            self.layout = layout;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SubpassDescription<'a> {
        pub flags: SubpassDescriptionFlags,
        pub pipeline_bind_point: PipelineBindPoint,
        pub input_attachment_count: u32,
        pub p_input_attachments: *const AttachmentReference,
        pub color_attachment_count: u32,
        pub p_color_attachments: *const AttachmentReference,
        pub p_resolve_attachments: *const AttachmentReference,
        pub p_depth_stencil_attachment: *const AttachmentReference,
        pub preserve_attachment_count: u32,
        pub p_preserve_attachments: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for SubpassDescription<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                pipeline_bind_point: Default::default(),
                input_attachment_count: Default::default(),
                p_input_attachments: core::ptr::null(),
                color_attachment_count: Default::default(),
                p_color_attachments: core::ptr::null(),
                p_resolve_attachments: core::ptr::null(),
                p_depth_stencil_attachment: core::ptr::null(),
                preserve_attachment_count: Default::default(),
                p_preserve_attachments: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SubpassDescription<'a> {
        pub fn flags(mut self, flags: SubpassDescriptionFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn pipeline_bind_point(mut self, pipeline_bind_point: PipelineBindPoint) -> Self {
            self.pipeline_bind_point = pipeline_bind_point;
            self
        }
        pub fn input_attachments(mut self, input_attachments: &'a [AttachmentReference]) -> Self {
            self.input_attachment_count = input_attachments.len().try_into().unwrap();
            self.p_input_attachments = input_attachments.as_ptr();
            self
        }
        pub fn color_attachments(mut self, color_attachments: &'a [AttachmentReference]) -> Self {
            self.color_attachment_count = color_attachments.len().try_into().unwrap();
            self.p_color_attachments = color_attachments.as_ptr();
            self
        }
        pub fn resolve_attachments(
            mut self,
            resolve_attachments: &'a [AttachmentReference],
        ) -> Self {
            self.color_attachment_count = resolve_attachments.len().try_into().unwrap();
            self.p_resolve_attachments = resolve_attachments.as_ptr();
            self
        }
        pub fn depth_stencil_attachment(
            mut self,
            depth_stencil_attachment: &'a AttachmentReference,
        ) -> Self {
            self.p_depth_stencil_attachment = depth_stencil_attachment;
            self
        }
        pub fn preserve_attachments(mut self, preserve_attachments: &'a [u32]) -> Self {
            self.preserve_attachment_count = preserve_attachments.len().try_into().unwrap();
            self.p_preserve_attachments = preserve_attachments.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct SubpassDependency {
        pub src_subpass: u32,
        pub dst_subpass: u32,
        pub src_stage_mask: PipelineStageFlags,
        pub dst_stage_mask: PipelineStageFlags,
        pub src_access_mask: AccessFlags,
        pub dst_access_mask: AccessFlags,
        pub dependency_flags: DependencyFlags,
    }
    impl SubpassDependency {
        pub fn src_subpass(mut self, src_subpass: u32) -> Self {
            self.src_subpass = src_subpass;
            self
        }
        pub fn dst_subpass(mut self, dst_subpass: u32) -> Self {
            self.dst_subpass = dst_subpass;
            self
        }
        pub fn src_stage_mask(mut self, src_stage_mask: PipelineStageFlags) -> Self {
            self.src_stage_mask = src_stage_mask;
            self
        }
        pub fn dst_stage_mask(mut self, dst_stage_mask: PipelineStageFlags) -> Self {
            self.dst_stage_mask = dst_stage_mask;
            self
        }
        pub fn src_access_mask(mut self, src_access_mask: AccessFlags) -> Self {
            self.src_access_mask = src_access_mask;
            self
        }
        pub fn dst_access_mask(mut self, dst_access_mask: AccessFlags) -> Self {
            self.dst_access_mask = dst_access_mask;
            self
        }
        pub fn dependency_flags(mut self, dependency_flags: DependencyFlags) -> Self {
            self.dependency_flags = dependency_flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderPassCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: RenderPassCreateFlags,
        pub attachment_count: u32,
        pub p_attachments: *const AttachmentDescription,
        pub subpass_count: u32,
        pub p_subpasses: *const SubpassDescription<'a>,
        pub dependency_count: u32,
        pub p_dependencies: *const SubpassDependency,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for RenderPassCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDER_PASS_CREATE_INFO;
    }
    impl Default for RenderPassCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                attachment_count: Default::default(),
                p_attachments: core::ptr::null(),
                subpass_count: Default::default(),
                p_subpasses: core::ptr::null(),
                dependency_count: Default::default(),
                p_dependencies: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderPassCreateInfo<'a> {
        pub fn flags(mut self, flags: RenderPassCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn attachments(mut self, attachments: &'a [AttachmentDescription]) -> Self {
            self.attachment_count = attachments.len().try_into().unwrap();
            self.p_attachments = attachments.as_ptr();
            self
        }
        pub fn subpasses(mut self, subpasses: &'a [SubpassDescription<'a>]) -> Self {
            self.subpass_count = subpasses.len().try_into().unwrap();
            self.p_subpasses = subpasses.as_ptr();
            self
        }
        pub fn dependencies(mut self, dependencies: &'a [SubpassDependency]) -> Self {
            self.dependency_count = dependencies.len().try_into().unwrap();
            self.p_dependencies = dependencies.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct EventCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: EventCreateFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for EventCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EVENT_CREATE_INFO;
    }
    impl Default for EventCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> EventCreateInfo<'a> {
        pub fn flags(mut self, flags: EventCreateFlags) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct FenceCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: FenceCreateFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for FenceCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::FENCE_CREATE_INFO;
    }
    impl Default for FenceCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> FenceCreateInfo<'a> {
        pub fn flags(mut self, flags: FenceCreateFlags) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct PhysicalDeviceFeatures {
        pub robust_buffer_access: Bool32,
        pub full_draw_index_uint32: Bool32,
        pub image_cube_array: Bool32,
        pub independent_blend: Bool32,
        pub geometry_shader: Bool32,
        pub tessellation_shader: Bool32,
        pub sample_rate_shading: Bool32,
        pub dual_src_blend: Bool32,
        pub logic_op: Bool32,
        pub multi_draw_indirect: Bool32,
        pub draw_indirect_first_instance: Bool32,
        pub depth_clamp: Bool32,
        pub depth_bias_clamp: Bool32,
        pub fill_mode_non_solid: Bool32,
        pub depth_bounds: Bool32,
        pub wide_lines: Bool32,
        pub large_points: Bool32,
        pub alpha_to_one: Bool32,
        pub multi_viewport: Bool32,
        pub sampler_anisotropy: Bool32,
        pub texture_compression_etc2: Bool32,
        pub texture_compression_astc_ldr: Bool32,
        pub texture_compression_bc: Bool32,
        pub occlusion_query_precise: Bool32,
        pub pipeline_statistics_query: Bool32,
        pub vertex_pipeline_stores_and_atomics: Bool32,
        pub fragment_stores_and_atomics: Bool32,
        pub shader_tessellation_and_geometry_point_size: Bool32,
        pub shader_image_gather_extended: Bool32,
        pub shader_storage_image_extended_formats: Bool32,
        pub shader_storage_image_multisample: Bool32,
        pub shader_storage_image_read_without_format: Bool32,
        pub shader_storage_image_write_without_format: Bool32,
        pub shader_uniform_buffer_array_dynamic_indexing: Bool32,
        pub shader_sampled_image_array_dynamic_indexing: Bool32,
        pub shader_storage_buffer_array_dynamic_indexing: Bool32,
        pub shader_storage_image_array_dynamic_indexing: Bool32,
        pub shader_clip_distance: Bool32,
        pub shader_cull_distance: Bool32,
        pub shader_float64: Bool32,
        pub shader_int64: Bool32,
        pub shader_int16: Bool32,
        pub shader_resource_residency: Bool32,
        pub shader_resource_min_lod: Bool32,
        pub sparse_binding: Bool32,
        pub sparse_residency_buffer: Bool32,
        pub sparse_residency_image2_d: Bool32,
        pub sparse_residency_image3_d: Bool32,
        pub sparse_residency2_samples: Bool32,
        pub sparse_residency4_samples: Bool32,
        pub sparse_residency8_samples: Bool32,
        pub sparse_residency16_samples: Bool32,
        pub sparse_residency_aliased: Bool32,
        pub variable_multisample_rate: Bool32,
        pub inherited_queries: Bool32,
    }
    impl PhysicalDeviceFeatures {
        pub fn robust_buffer_access(mut self, robust_buffer_access: Bool32) -> Self {
            self.robust_buffer_access = robust_buffer_access;
            self
        }
        pub fn full_draw_index_uint32(mut self, full_draw_index_uint32: Bool32) -> Self {
            self.full_draw_index_uint32 = full_draw_index_uint32;
            self
        }
        pub fn image_cube_array(mut self, image_cube_array: Bool32) -> Self {
            self.image_cube_array = image_cube_array;
            self
        }
        pub fn independent_blend(mut self, independent_blend: Bool32) -> Self {
            self.independent_blend = independent_blend;
            self
        }
        pub fn geometry_shader(mut self, geometry_shader: Bool32) -> Self {
            self.geometry_shader = geometry_shader;
            self
        }
        pub fn tessellation_shader(mut self, tessellation_shader: Bool32) -> Self {
            self.tessellation_shader = tessellation_shader;
            self
        }
        pub fn sample_rate_shading(mut self, sample_rate_shading: Bool32) -> Self {
            self.sample_rate_shading = sample_rate_shading;
            self
        }
        pub fn dual_src_blend(mut self, dual_src_blend: Bool32) -> Self {
            self.dual_src_blend = dual_src_blend;
            self
        }
        pub fn logic_op(mut self, logic_op: Bool32) -> Self {
            self.logic_op = logic_op;
            self
        }
        pub fn multi_draw_indirect(mut self, multi_draw_indirect: Bool32) -> Self {
            self.multi_draw_indirect = multi_draw_indirect;
            self
        }
        pub fn draw_indirect_first_instance(
            mut self,
            draw_indirect_first_instance: Bool32,
        ) -> Self {
            self.draw_indirect_first_instance = draw_indirect_first_instance;
            self
        }
        pub fn depth_clamp(mut self, depth_clamp: Bool32) -> Self {
            self.depth_clamp = depth_clamp;
            self
        }
        pub fn depth_bias_clamp(mut self, depth_bias_clamp: Bool32) -> Self {
            self.depth_bias_clamp = depth_bias_clamp;
            self
        }
        pub fn fill_mode_non_solid(mut self, fill_mode_non_solid: Bool32) -> Self {
            self.fill_mode_non_solid = fill_mode_non_solid;
            self
        }
        pub fn depth_bounds(mut self, depth_bounds: Bool32) -> Self {
            self.depth_bounds = depth_bounds;
            self
        }
        pub fn wide_lines(mut self, wide_lines: Bool32) -> Self {
            self.wide_lines = wide_lines;
            self
        }
        pub fn large_points(mut self, large_points: Bool32) -> Self {
            self.large_points = large_points;
            self
        }
        pub fn alpha_to_one(mut self, alpha_to_one: Bool32) -> Self {
            self.alpha_to_one = alpha_to_one;
            self
        }
        pub fn multi_viewport(mut self, multi_viewport: Bool32) -> Self {
            self.multi_viewport = multi_viewport;
            self
        }
        pub fn sampler_anisotropy(mut self, sampler_anisotropy: Bool32) -> Self {
            self.sampler_anisotropy = sampler_anisotropy;
            self
        }
        pub fn texture_compression_etc2(mut self, texture_compression_etc2: Bool32) -> Self {
            self.texture_compression_etc2 = texture_compression_etc2;
            self
        }
        pub fn texture_compression_astc_ldr(
            mut self,
            texture_compression_astc_ldr: Bool32,
        ) -> Self {
            self.texture_compression_astc_ldr = texture_compression_astc_ldr;
            self
        }
        pub fn texture_compression_bc(mut self, texture_compression_bc: Bool32) -> Self {
            self.texture_compression_bc = texture_compression_bc;
            self
        }
        pub fn occlusion_query_precise(mut self, occlusion_query_precise: Bool32) -> Self {
            self.occlusion_query_precise = occlusion_query_precise;
            self
        }
        pub fn pipeline_statistics_query(mut self, pipeline_statistics_query: Bool32) -> Self {
            self.pipeline_statistics_query = pipeline_statistics_query;
            self
        }
        pub fn vertex_pipeline_stores_and_atomics(
            mut self,
            vertex_pipeline_stores_and_atomics: Bool32,
        ) -> Self {
            self.vertex_pipeline_stores_and_atomics = vertex_pipeline_stores_and_atomics;
            self
        }
        pub fn fragment_stores_and_atomics(mut self, fragment_stores_and_atomics: Bool32) -> Self {
            self.fragment_stores_and_atomics = fragment_stores_and_atomics;
            self
        }
        pub fn shader_tessellation_and_geometry_point_size(
            mut self,
            shader_tessellation_and_geometry_point_size: Bool32,
        ) -> Self {
            self.shader_tessellation_and_geometry_point_size =
                shader_tessellation_and_geometry_point_size;
            self
        }
        pub fn shader_image_gather_extended(
            mut self,
            shader_image_gather_extended: Bool32,
        ) -> Self {
            self.shader_image_gather_extended = shader_image_gather_extended;
            self
        }
        pub fn shader_storage_image_extended_formats(
            mut self,
            shader_storage_image_extended_formats: Bool32,
        ) -> Self {
            self.shader_storage_image_extended_formats = shader_storage_image_extended_formats;
            self
        }
        pub fn shader_storage_image_multisample(
            mut self,
            shader_storage_image_multisample: Bool32,
        ) -> Self {
            self.shader_storage_image_multisample = shader_storage_image_multisample;
            self
        }
        pub fn shader_storage_image_read_without_format(
            mut self,
            shader_storage_image_read_without_format: Bool32,
        ) -> Self {
            self.shader_storage_image_read_without_format =
                shader_storage_image_read_without_format;
            self
        }
        pub fn shader_storage_image_write_without_format(
            mut self,
            shader_storage_image_write_without_format: Bool32,
        ) -> Self {
            self.shader_storage_image_write_without_format =
                shader_storage_image_write_without_format;
            self
        }
        pub fn shader_uniform_buffer_array_dynamic_indexing(
            mut self,
            shader_uniform_buffer_array_dynamic_indexing: Bool32,
        ) -> Self {
            self.shader_uniform_buffer_array_dynamic_indexing =
                shader_uniform_buffer_array_dynamic_indexing;
            self
        }
        pub fn shader_sampled_image_array_dynamic_indexing(
            mut self,
            shader_sampled_image_array_dynamic_indexing: Bool32,
        ) -> Self {
            self.shader_sampled_image_array_dynamic_indexing =
                shader_sampled_image_array_dynamic_indexing;
            self
        }
        pub fn shader_storage_buffer_array_dynamic_indexing(
            mut self,
            shader_storage_buffer_array_dynamic_indexing: Bool32,
        ) -> Self {
            self.shader_storage_buffer_array_dynamic_indexing =
                shader_storage_buffer_array_dynamic_indexing;
            self
        }
        pub fn shader_storage_image_array_dynamic_indexing(
            mut self,
            shader_storage_image_array_dynamic_indexing: Bool32,
        ) -> Self {
            self.shader_storage_image_array_dynamic_indexing =
                shader_storage_image_array_dynamic_indexing;
            self
        }
        pub fn shader_clip_distance(mut self, shader_clip_distance: Bool32) -> Self {
            self.shader_clip_distance = shader_clip_distance;
            self
        }
        pub fn shader_cull_distance(mut self, shader_cull_distance: Bool32) -> Self {
            self.shader_cull_distance = shader_cull_distance;
            self
        }
        pub fn shader_float64(mut self, shader_float64: Bool32) -> Self {
            self.shader_float64 = shader_float64;
            self
        }
        pub fn shader_int64(mut self, shader_int64: Bool32) -> Self {
            self.shader_int64 = shader_int64;
            self
        }
        pub fn shader_int16(mut self, shader_int16: Bool32) -> Self {
            self.shader_int16 = shader_int16;
            self
        }
        pub fn shader_resource_residency(mut self, shader_resource_residency: Bool32) -> Self {
            self.shader_resource_residency = shader_resource_residency;
            self
        }
        pub fn shader_resource_min_lod(mut self, shader_resource_min_lod: Bool32) -> Self {
            self.shader_resource_min_lod = shader_resource_min_lod;
            self
        }
        pub fn sparse_binding(mut self, sparse_binding: Bool32) -> Self {
            self.sparse_binding = sparse_binding;
            self
        }
        pub fn sparse_residency_buffer(mut self, sparse_residency_buffer: Bool32) -> Self {
            self.sparse_residency_buffer = sparse_residency_buffer;
            self
        }
        pub fn sparse_residency_image2_d(mut self, sparse_residency_image2_d: Bool32) -> Self {
            self.sparse_residency_image2_d = sparse_residency_image2_d;
            self
        }
        pub fn sparse_residency_image3_d(mut self, sparse_residency_image3_d: Bool32) -> Self {
            self.sparse_residency_image3_d = sparse_residency_image3_d;
            self
        }
        pub fn sparse_residency2_samples(mut self, sparse_residency2_samples: Bool32) -> Self {
            self.sparse_residency2_samples = sparse_residency2_samples;
            self
        }
        pub fn sparse_residency4_samples(mut self, sparse_residency4_samples: Bool32) -> Self {
            self.sparse_residency4_samples = sparse_residency4_samples;
            self
        }
        pub fn sparse_residency8_samples(mut self, sparse_residency8_samples: Bool32) -> Self {
            self.sparse_residency8_samples = sparse_residency8_samples;
            self
        }
        pub fn sparse_residency16_samples(mut self, sparse_residency16_samples: Bool32) -> Self {
            self.sparse_residency16_samples = sparse_residency16_samples;
            self
        }
        pub fn sparse_residency_aliased(mut self, sparse_residency_aliased: Bool32) -> Self {
            self.sparse_residency_aliased = sparse_residency_aliased;
            self
        }
        pub fn variable_multisample_rate(mut self, variable_multisample_rate: Bool32) -> Self {
            self.variable_multisample_rate = variable_multisample_rate;
            self
        }
        pub fn inherited_queries(mut self, inherited_queries: Bool32) -> Self {
            self.inherited_queries = inherited_queries;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct PhysicalDeviceSparseProperties {
        pub residency_standard2_d_block_shape: Bool32,
        pub residency_standard2_d_multisample_block_shape: Bool32,
        pub residency_standard3_d_block_shape: Bool32,
        pub residency_aligned_mip_size: Bool32,
        pub residency_non_resident_strict: Bool32,
    }
    impl PhysicalDeviceSparseProperties {
        pub fn residency_standard2_d_block_shape(
            mut self,
            residency_standard2_d_block_shape: Bool32,
        ) -> Self {
            self.residency_standard2_d_block_shape = residency_standard2_d_block_shape;
            self
        }
        pub fn residency_standard2_d_multisample_block_shape(
            mut self,
            residency_standard2_d_multisample_block_shape: Bool32,
        ) -> Self {
            self.residency_standard2_d_multisample_block_shape =
                residency_standard2_d_multisample_block_shape;
            self
        }
        pub fn residency_standard3_d_block_shape(
            mut self,
            residency_standard3_d_block_shape: Bool32,
        ) -> Self {
            self.residency_standard3_d_block_shape = residency_standard3_d_block_shape;
            self
        }
        pub fn residency_aligned_mip_size(mut self, residency_aligned_mip_size: Bool32) -> Self {
            self.residency_aligned_mip_size = residency_aligned_mip_size;
            self
        }
        pub fn residency_non_resident_strict(
            mut self,
            residency_non_resident_strict: Bool32,
        ) -> Self {
            self.residency_non_resident_strict = residency_non_resident_strict;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceLimits {
        pub max_image_dimension1_d: u32,
        pub max_image_dimension2_d: u32,
        pub max_image_dimension3_d: u32,
        pub max_image_dimension_cube: u32,
        pub max_image_array_layers: u32,
        pub max_texel_buffer_elements: u32,
        pub max_uniform_buffer_range: u32,
        pub max_storage_buffer_range: u32,
        pub max_push_constants_size: u32,
        pub max_memory_allocation_count: u32,
        pub max_sampler_allocation_count: u32,
        pub buffer_image_granularity: DeviceSize,
        pub sparse_address_space_size: DeviceSize,
        pub max_bound_descriptor_sets: u32,
        pub max_per_stage_descriptor_samplers: u32,
        pub max_per_stage_descriptor_uniform_buffers: u32,
        pub max_per_stage_descriptor_storage_buffers: u32,
        pub max_per_stage_descriptor_sampled_images: u32,
        pub max_per_stage_descriptor_storage_images: u32,
        pub max_per_stage_descriptor_input_attachments: u32,
        pub max_per_stage_resources: u32,
        pub max_descriptor_set_samplers: u32,
        pub max_descriptor_set_uniform_buffers: u32,
        pub max_descriptor_set_uniform_buffers_dynamic: u32,
        pub max_descriptor_set_storage_buffers: u32,
        pub max_descriptor_set_storage_buffers_dynamic: u32,
        pub max_descriptor_set_sampled_images: u32,
        pub max_descriptor_set_storage_images: u32,
        pub max_descriptor_set_input_attachments: u32,
        pub max_vertex_input_attributes: u32,
        pub max_vertex_input_bindings: u32,
        pub max_vertex_input_attribute_offset: u32,
        pub max_vertex_input_binding_stride: u32,
        pub max_vertex_output_components: u32,
        pub max_tessellation_generation_level: u32,
        pub max_tessellation_patch_size: u32,
        pub max_tessellation_control_per_vertex_input_components: u32,
        pub max_tessellation_control_per_vertex_output_components: u32,
        pub max_tessellation_control_per_patch_output_components: u32,
        pub max_tessellation_control_total_output_components: u32,
        pub max_tessellation_evaluation_input_components: u32,
        pub max_tessellation_evaluation_output_components: u32,
        pub max_geometry_shader_invocations: u32,
        pub max_geometry_input_components: u32,
        pub max_geometry_output_components: u32,
        pub max_geometry_output_vertices: u32,
        pub max_geometry_total_output_components: u32,
        pub max_fragment_input_components: u32,
        pub max_fragment_output_attachments: u32,
        pub max_fragment_dual_src_attachments: u32,
        pub max_fragment_combined_output_resources: u32,
        pub max_compute_shared_memory_size: u32,
        pub max_compute_work_group_count: [u32; 3],
        pub max_compute_work_group_invocations: u32,
        pub max_compute_work_group_size: [u32; 3],
        pub sub_pixel_precision_bits: u32,
        pub sub_texel_precision_bits: u32,
        pub mipmap_precision_bits: u32,
        pub max_draw_indexed_index_value: u32,
        pub max_draw_indirect_count: u32,
        pub max_sampler_lod_bias: f32,
        pub max_sampler_anisotropy: f32,
        pub max_viewports: u32,
        pub max_viewport_dimensions: [u32; 2],
        pub viewport_bounds_range: [f32; 2],
        pub viewport_sub_pixel_bits: u32,
        pub min_memory_map_alignment: usize,
        pub min_texel_buffer_offset_alignment: DeviceSize,
        pub min_uniform_buffer_offset_alignment: DeviceSize,
        pub min_storage_buffer_offset_alignment: DeviceSize,
        pub min_texel_offset: i32,
        pub max_texel_offset: u32,
        pub min_texel_gather_offset: i32,
        pub max_texel_gather_offset: u32,
        pub min_interpolation_offset: f32,
        pub max_interpolation_offset: f32,
        pub sub_pixel_interpolation_offset_bits: u32,
        pub max_framebuffer_width: u32,
        pub max_framebuffer_height: u32,
        pub max_framebuffer_layers: u32,
        pub framebuffer_color_sample_counts: SampleCountFlags,
        pub framebuffer_depth_sample_counts: SampleCountFlags,
        pub framebuffer_stencil_sample_counts: SampleCountFlags,
        pub framebuffer_no_attachments_sample_counts: SampleCountFlags,
        pub max_color_attachments: u32,
        pub sampled_image_color_sample_counts: SampleCountFlags,
        pub sampled_image_integer_sample_counts: SampleCountFlags,
        pub sampled_image_depth_sample_counts: SampleCountFlags,
        pub sampled_image_stencil_sample_counts: SampleCountFlags,
        pub storage_image_sample_counts: SampleCountFlags,
        pub max_sample_mask_words: u32,
        pub timestamp_compute_and_graphics: Bool32,
        pub timestamp_period: f32,
        pub max_clip_distances: u32,
        pub max_cull_distances: u32,
        pub max_combined_clip_and_cull_distances: u32,
        pub discrete_queue_priorities: u32,
        pub point_size_range: [f32; 2],
        pub line_width_range: [f32; 2],
        pub point_size_granularity: f32,
        pub line_width_granularity: f32,
        pub strict_lines: Bool32,
        pub standard_sample_locations: Bool32,
        pub optimal_buffer_copy_offset_alignment: DeviceSize,
        pub optimal_buffer_copy_row_pitch_alignment: DeviceSize,
        pub non_coherent_atom_size: DeviceSize,
    }
    impl Default for PhysicalDeviceLimits {
        fn default() -> Self {
            Self {
                max_image_dimension1_d: Default::default(),
                max_image_dimension2_d: Default::default(),
                max_image_dimension3_d: Default::default(),
                max_image_dimension_cube: Default::default(),
                max_image_array_layers: Default::default(),
                max_texel_buffer_elements: Default::default(),
                max_uniform_buffer_range: Default::default(),
                max_storage_buffer_range: Default::default(),
                max_push_constants_size: Default::default(),
                max_memory_allocation_count: Default::default(),
                max_sampler_allocation_count: Default::default(),
                buffer_image_granularity: Default::default(),
                sparse_address_space_size: Default::default(),
                max_bound_descriptor_sets: Default::default(),
                max_per_stage_descriptor_samplers: Default::default(),
                max_per_stage_descriptor_uniform_buffers: Default::default(),
                max_per_stage_descriptor_storage_buffers: Default::default(),
                max_per_stage_descriptor_sampled_images: Default::default(),
                max_per_stage_descriptor_storage_images: Default::default(),
                max_per_stage_descriptor_input_attachments: Default::default(),
                max_per_stage_resources: Default::default(),
                max_descriptor_set_samplers: Default::default(),
                max_descriptor_set_uniform_buffers: Default::default(),
                max_descriptor_set_uniform_buffers_dynamic: Default::default(),
                max_descriptor_set_storage_buffers: Default::default(),
                max_descriptor_set_storage_buffers_dynamic: Default::default(),
                max_descriptor_set_sampled_images: Default::default(),
                max_descriptor_set_storage_images: Default::default(),
                max_descriptor_set_input_attachments: Default::default(),
                max_vertex_input_attributes: Default::default(),
                max_vertex_input_bindings: Default::default(),
                max_vertex_input_attribute_offset: Default::default(),
                max_vertex_input_binding_stride: Default::default(),
                max_vertex_output_components: Default::default(),
                max_tessellation_generation_level: Default::default(),
                max_tessellation_patch_size: Default::default(),
                max_tessellation_control_per_vertex_input_components: Default::default(),
                max_tessellation_control_per_vertex_output_components: Default::default(),
                max_tessellation_control_per_patch_output_components: Default::default(),
                max_tessellation_control_total_output_components: Default::default(),
                max_tessellation_evaluation_input_components: Default::default(),
                max_tessellation_evaluation_output_components: Default::default(),
                max_geometry_shader_invocations: Default::default(),
                max_geometry_input_components: Default::default(),
                max_geometry_output_components: Default::default(),
                max_geometry_output_vertices: Default::default(),
                max_geometry_total_output_components: Default::default(),
                max_fragment_input_components: Default::default(),
                max_fragment_output_attachments: Default::default(),
                max_fragment_dual_src_attachments: Default::default(),
                max_fragment_combined_output_resources: Default::default(),
                max_compute_shared_memory_size: Default::default(),
                max_compute_work_group_count: [Default::default(); _],
                max_compute_work_group_invocations: Default::default(),
                max_compute_work_group_size: [Default::default(); _],
                sub_pixel_precision_bits: Default::default(),
                sub_texel_precision_bits: Default::default(),
                mipmap_precision_bits: Default::default(),
                max_draw_indexed_index_value: Default::default(),
                max_draw_indirect_count: Default::default(),
                max_sampler_lod_bias: Default::default(),
                max_sampler_anisotropy: Default::default(),
                max_viewports: Default::default(),
                max_viewport_dimensions: [Default::default(); _],
                viewport_bounds_range: [Default::default(); _],
                viewport_sub_pixel_bits: Default::default(),
                min_memory_map_alignment: Default::default(),
                min_texel_buffer_offset_alignment: Default::default(),
                min_uniform_buffer_offset_alignment: Default::default(),
                min_storage_buffer_offset_alignment: Default::default(),
                min_texel_offset: Default::default(),
                max_texel_offset: Default::default(),
                min_texel_gather_offset: Default::default(),
                max_texel_gather_offset: Default::default(),
                min_interpolation_offset: Default::default(),
                max_interpolation_offset: Default::default(),
                sub_pixel_interpolation_offset_bits: Default::default(),
                max_framebuffer_width: Default::default(),
                max_framebuffer_height: Default::default(),
                max_framebuffer_layers: Default::default(),
                framebuffer_color_sample_counts: Default::default(),
                framebuffer_depth_sample_counts: Default::default(),
                framebuffer_stencil_sample_counts: Default::default(),
                framebuffer_no_attachments_sample_counts: Default::default(),
                max_color_attachments: Default::default(),
                sampled_image_color_sample_counts: Default::default(),
                sampled_image_integer_sample_counts: Default::default(),
                sampled_image_depth_sample_counts: Default::default(),
                sampled_image_stencil_sample_counts: Default::default(),
                storage_image_sample_counts: Default::default(),
                max_sample_mask_words: Default::default(),
                timestamp_compute_and_graphics: Default::default(),
                timestamp_period: Default::default(),
                max_clip_distances: Default::default(),
                max_cull_distances: Default::default(),
                max_combined_clip_and_cull_distances: Default::default(),
                discrete_queue_priorities: Default::default(),
                point_size_range: [Default::default(); _],
                line_width_range: [Default::default(); _],
                point_size_granularity: Default::default(),
                line_width_granularity: Default::default(),
                strict_lines: Default::default(),
                standard_sample_locations: Default::default(),
                optimal_buffer_copy_offset_alignment: Default::default(),
                optimal_buffer_copy_row_pitch_alignment: Default::default(),
                non_coherent_atom_size: Default::default(),
            }
        }
    }
    impl PhysicalDeviceLimits {
        pub fn max_image_dimension1_d(mut self, max_image_dimension1_d: u32) -> Self {
            self.max_image_dimension1_d = max_image_dimension1_d;
            self
        }
        pub fn max_image_dimension2_d(mut self, max_image_dimension2_d: u32) -> Self {
            self.max_image_dimension2_d = max_image_dimension2_d;
            self
        }
        pub fn max_image_dimension3_d(mut self, max_image_dimension3_d: u32) -> Self {
            self.max_image_dimension3_d = max_image_dimension3_d;
            self
        }
        pub fn max_image_dimension_cube(mut self, max_image_dimension_cube: u32) -> Self {
            self.max_image_dimension_cube = max_image_dimension_cube;
            self
        }
        pub fn max_image_array_layers(mut self, max_image_array_layers: u32) -> Self {
            self.max_image_array_layers = max_image_array_layers;
            self
        }
        pub fn max_texel_buffer_elements(mut self, max_texel_buffer_elements: u32) -> Self {
            self.max_texel_buffer_elements = max_texel_buffer_elements;
            self
        }
        pub fn max_uniform_buffer_range(mut self, max_uniform_buffer_range: u32) -> Self {
            self.max_uniform_buffer_range = max_uniform_buffer_range;
            self
        }
        pub fn max_storage_buffer_range(mut self, max_storage_buffer_range: u32) -> Self {
            self.max_storage_buffer_range = max_storage_buffer_range;
            self
        }
        pub fn max_push_constants_size(mut self, max_push_constants_size: u32) -> Self {
            self.max_push_constants_size = max_push_constants_size;
            self
        }
        pub fn max_memory_allocation_count(mut self, max_memory_allocation_count: u32) -> Self {
            self.max_memory_allocation_count = max_memory_allocation_count;
            self
        }
        pub fn max_sampler_allocation_count(mut self, max_sampler_allocation_count: u32) -> Self {
            self.max_sampler_allocation_count = max_sampler_allocation_count;
            self
        }
        pub fn buffer_image_granularity(mut self, buffer_image_granularity: DeviceSize) -> Self {
            self.buffer_image_granularity = buffer_image_granularity;
            self
        }
        pub fn sparse_address_space_size(mut self, sparse_address_space_size: DeviceSize) -> Self {
            self.sparse_address_space_size = sparse_address_space_size;
            self
        }
        pub fn max_bound_descriptor_sets(mut self, max_bound_descriptor_sets: u32) -> Self {
            self.max_bound_descriptor_sets = max_bound_descriptor_sets;
            self
        }
        pub fn max_per_stage_descriptor_samplers(
            mut self,
            max_per_stage_descriptor_samplers: u32,
        ) -> Self {
            self.max_per_stage_descriptor_samplers = max_per_stage_descriptor_samplers;
            self
        }
        pub fn max_per_stage_descriptor_uniform_buffers(
            mut self,
            max_per_stage_descriptor_uniform_buffers: u32,
        ) -> Self {
            self.max_per_stage_descriptor_uniform_buffers =
                max_per_stage_descriptor_uniform_buffers;
            self
        }
        pub fn max_per_stage_descriptor_storage_buffers(
            mut self,
            max_per_stage_descriptor_storage_buffers: u32,
        ) -> Self {
            self.max_per_stage_descriptor_storage_buffers =
                max_per_stage_descriptor_storage_buffers;
            self
        }
        pub fn max_per_stage_descriptor_sampled_images(
            mut self,
            max_per_stage_descriptor_sampled_images: u32,
        ) -> Self {
            self.max_per_stage_descriptor_sampled_images = max_per_stage_descriptor_sampled_images;
            self
        }
        pub fn max_per_stage_descriptor_storage_images(
            mut self,
            max_per_stage_descriptor_storage_images: u32,
        ) -> Self {
            self.max_per_stage_descriptor_storage_images = max_per_stage_descriptor_storage_images;
            self
        }
        pub fn max_per_stage_descriptor_input_attachments(
            mut self,
            max_per_stage_descriptor_input_attachments: u32,
        ) -> Self {
            self.max_per_stage_descriptor_input_attachments =
                max_per_stage_descriptor_input_attachments;
            self
        }
        pub fn max_per_stage_resources(mut self, max_per_stage_resources: u32) -> Self {
            self.max_per_stage_resources = max_per_stage_resources;
            self
        }
        pub fn max_descriptor_set_samplers(mut self, max_descriptor_set_samplers: u32) -> Self {
            self.max_descriptor_set_samplers = max_descriptor_set_samplers;
            self
        }
        pub fn max_descriptor_set_uniform_buffers(
            mut self,
            max_descriptor_set_uniform_buffers: u32,
        ) -> Self {
            self.max_descriptor_set_uniform_buffers = max_descriptor_set_uniform_buffers;
            self
        }
        pub fn max_descriptor_set_uniform_buffers_dynamic(
            mut self,
            max_descriptor_set_uniform_buffers_dynamic: u32,
        ) -> Self {
            self.max_descriptor_set_uniform_buffers_dynamic =
                max_descriptor_set_uniform_buffers_dynamic;
            self
        }
        pub fn max_descriptor_set_storage_buffers(
            mut self,
            max_descriptor_set_storage_buffers: u32,
        ) -> Self {
            self.max_descriptor_set_storage_buffers = max_descriptor_set_storage_buffers;
            self
        }
        pub fn max_descriptor_set_storage_buffers_dynamic(
            mut self,
            max_descriptor_set_storage_buffers_dynamic: u32,
        ) -> Self {
            self.max_descriptor_set_storage_buffers_dynamic =
                max_descriptor_set_storage_buffers_dynamic;
            self
        }
        pub fn max_descriptor_set_sampled_images(
            mut self,
            max_descriptor_set_sampled_images: u32,
        ) -> Self {
            self.max_descriptor_set_sampled_images = max_descriptor_set_sampled_images;
            self
        }
        pub fn max_descriptor_set_storage_images(
            mut self,
            max_descriptor_set_storage_images: u32,
        ) -> Self {
            self.max_descriptor_set_storage_images = max_descriptor_set_storage_images;
            self
        }
        pub fn max_descriptor_set_input_attachments(
            mut self,
            max_descriptor_set_input_attachments: u32,
        ) -> Self {
            self.max_descriptor_set_input_attachments = max_descriptor_set_input_attachments;
            self
        }
        pub fn max_vertex_input_attributes(mut self, max_vertex_input_attributes: u32) -> Self {
            self.max_vertex_input_attributes = max_vertex_input_attributes;
            self
        }
        pub fn max_vertex_input_bindings(mut self, max_vertex_input_bindings: u32) -> Self {
            self.max_vertex_input_bindings = max_vertex_input_bindings;
            self
        }
        pub fn max_vertex_input_attribute_offset(
            mut self,
            max_vertex_input_attribute_offset: u32,
        ) -> Self {
            self.max_vertex_input_attribute_offset = max_vertex_input_attribute_offset;
            self
        }
        pub fn max_vertex_input_binding_stride(
            mut self,
            max_vertex_input_binding_stride: u32,
        ) -> Self {
            self.max_vertex_input_binding_stride = max_vertex_input_binding_stride;
            self
        }
        pub fn max_vertex_output_components(mut self, max_vertex_output_components: u32) -> Self {
            self.max_vertex_output_components = max_vertex_output_components;
            self
        }
        pub fn max_tessellation_generation_level(
            mut self,
            max_tessellation_generation_level: u32,
        ) -> Self {
            self.max_tessellation_generation_level = max_tessellation_generation_level;
            self
        }
        pub fn max_tessellation_patch_size(mut self, max_tessellation_patch_size: u32) -> Self {
            self.max_tessellation_patch_size = max_tessellation_patch_size;
            self
        }
        pub fn max_tessellation_control_per_vertex_input_components(
            mut self,
            max_tessellation_control_per_vertex_input_components: u32,
        ) -> Self {
            self.max_tessellation_control_per_vertex_input_components =
                max_tessellation_control_per_vertex_input_components;
            self
        }
        pub fn max_tessellation_control_per_vertex_output_components(
            mut self,
            max_tessellation_control_per_vertex_output_components: u32,
        ) -> Self {
            self.max_tessellation_control_per_vertex_output_components =
                max_tessellation_control_per_vertex_output_components;
            self
        }
        pub fn max_tessellation_control_per_patch_output_components(
            mut self,
            max_tessellation_control_per_patch_output_components: u32,
        ) -> Self {
            self.max_tessellation_control_per_patch_output_components =
                max_tessellation_control_per_patch_output_components;
            self
        }
        pub fn max_tessellation_control_total_output_components(
            mut self,
            max_tessellation_control_total_output_components: u32,
        ) -> Self {
            self.max_tessellation_control_total_output_components =
                max_tessellation_control_total_output_components;
            self
        }
        pub fn max_tessellation_evaluation_input_components(
            mut self,
            max_tessellation_evaluation_input_components: u32,
        ) -> Self {
            self.max_tessellation_evaluation_input_components =
                max_tessellation_evaluation_input_components;
            self
        }
        pub fn max_tessellation_evaluation_output_components(
            mut self,
            max_tessellation_evaluation_output_components: u32,
        ) -> Self {
            self.max_tessellation_evaluation_output_components =
                max_tessellation_evaluation_output_components;
            self
        }
        pub fn max_geometry_shader_invocations(
            mut self,
            max_geometry_shader_invocations: u32,
        ) -> Self {
            self.max_geometry_shader_invocations = max_geometry_shader_invocations;
            self
        }
        pub fn max_geometry_input_components(mut self, max_geometry_input_components: u32) -> Self {
            self.max_geometry_input_components = max_geometry_input_components;
            self
        }
        pub fn max_geometry_output_components(
            mut self,
            max_geometry_output_components: u32,
        ) -> Self {
            self.max_geometry_output_components = max_geometry_output_components;
            self
        }
        pub fn max_geometry_output_vertices(mut self, max_geometry_output_vertices: u32) -> Self {
            self.max_geometry_output_vertices = max_geometry_output_vertices;
            self
        }
        pub fn max_geometry_total_output_components(
            mut self,
            max_geometry_total_output_components: u32,
        ) -> Self {
            self.max_geometry_total_output_components = max_geometry_total_output_components;
            self
        }
        pub fn max_fragment_input_components(mut self, max_fragment_input_components: u32) -> Self {
            self.max_fragment_input_components = max_fragment_input_components;
            self
        }
        pub fn max_fragment_output_attachments(
            mut self,
            max_fragment_output_attachments: u32,
        ) -> Self {
            self.max_fragment_output_attachments = max_fragment_output_attachments;
            self
        }
        pub fn max_fragment_dual_src_attachments(
            mut self,
            max_fragment_dual_src_attachments: u32,
        ) -> Self {
            self.max_fragment_dual_src_attachments = max_fragment_dual_src_attachments;
            self
        }
        pub fn max_fragment_combined_output_resources(
            mut self,
            max_fragment_combined_output_resources: u32,
        ) -> Self {
            self.max_fragment_combined_output_resources = max_fragment_combined_output_resources;
            self
        }
        pub fn max_compute_shared_memory_size(
            mut self,
            max_compute_shared_memory_size: u32,
        ) -> Self {
            self.max_compute_shared_memory_size = max_compute_shared_memory_size;
            self
        }
        pub fn max_compute_work_group_count(
            mut self,
            max_compute_work_group_count: [u32; 3],
        ) -> Self {
            self.max_compute_work_group_count = max_compute_work_group_count;
            self
        }
        pub fn max_compute_work_group_invocations(
            mut self,
            max_compute_work_group_invocations: u32,
        ) -> Self {
            self.max_compute_work_group_invocations = max_compute_work_group_invocations;
            self
        }
        pub fn max_compute_work_group_size(
            mut self,
            max_compute_work_group_size: [u32; 3],
        ) -> Self {
            self.max_compute_work_group_size = max_compute_work_group_size;
            self
        }
        pub fn sub_pixel_precision_bits(mut self, sub_pixel_precision_bits: u32) -> Self {
            self.sub_pixel_precision_bits = sub_pixel_precision_bits;
            self
        }
        pub fn sub_texel_precision_bits(mut self, sub_texel_precision_bits: u32) -> Self {
            self.sub_texel_precision_bits = sub_texel_precision_bits;
            self
        }
        pub fn mipmap_precision_bits(mut self, mipmap_precision_bits: u32) -> Self {
            self.mipmap_precision_bits = mipmap_precision_bits;
            self
        }
        pub fn max_draw_indexed_index_value(mut self, max_draw_indexed_index_value: u32) -> Self {
            self.max_draw_indexed_index_value = max_draw_indexed_index_value;
            self
        }
        pub fn max_draw_indirect_count(mut self, max_draw_indirect_count: u32) -> Self {
            self.max_draw_indirect_count = max_draw_indirect_count;
            self
        }
        pub fn max_sampler_lod_bias(mut self, max_sampler_lod_bias: f32) -> Self {
            self.max_sampler_lod_bias = max_sampler_lod_bias;
            self
        }
        pub fn max_sampler_anisotropy(mut self, max_sampler_anisotropy: f32) -> Self {
            self.max_sampler_anisotropy = max_sampler_anisotropy;
            self
        }
        pub fn max_viewports(mut self, max_viewports: u32) -> Self {
            self.max_viewports = max_viewports;
            self
        }
        pub fn max_viewport_dimensions(mut self, max_viewport_dimensions: [u32; 2]) -> Self {
            self.max_viewport_dimensions = max_viewport_dimensions;
            self
        }
        pub fn viewport_bounds_range(mut self, viewport_bounds_range: [f32; 2]) -> Self {
            self.viewport_bounds_range = viewport_bounds_range;
            self
        }
        pub fn viewport_sub_pixel_bits(mut self, viewport_sub_pixel_bits: u32) -> Self {
            self.viewport_sub_pixel_bits = viewport_sub_pixel_bits;
            self
        }
        pub fn min_memory_map_alignment(mut self, min_memory_map_alignment: usize) -> Self {
            self.min_memory_map_alignment = min_memory_map_alignment;
            self
        }
        pub fn min_texel_buffer_offset_alignment(
            mut self,
            min_texel_buffer_offset_alignment: DeviceSize,
        ) -> Self {
            self.min_texel_buffer_offset_alignment = min_texel_buffer_offset_alignment;
            self
        }
        pub fn min_uniform_buffer_offset_alignment(
            mut self,
            min_uniform_buffer_offset_alignment: DeviceSize,
        ) -> Self {
            self.min_uniform_buffer_offset_alignment = min_uniform_buffer_offset_alignment;
            self
        }
        pub fn min_storage_buffer_offset_alignment(
            mut self,
            min_storage_buffer_offset_alignment: DeviceSize,
        ) -> Self {
            self.min_storage_buffer_offset_alignment = min_storage_buffer_offset_alignment;
            self
        }
        pub fn min_texel_offset(mut self, min_texel_offset: i32) -> Self {
            self.min_texel_offset = min_texel_offset;
            self
        }
        pub fn max_texel_offset(mut self, max_texel_offset: u32) -> Self {
            self.max_texel_offset = max_texel_offset;
            self
        }
        pub fn min_texel_gather_offset(mut self, min_texel_gather_offset: i32) -> Self {
            self.min_texel_gather_offset = min_texel_gather_offset;
            self
        }
        pub fn max_texel_gather_offset(mut self, max_texel_gather_offset: u32) -> Self {
            self.max_texel_gather_offset = max_texel_gather_offset;
            self
        }
        pub fn min_interpolation_offset(mut self, min_interpolation_offset: f32) -> Self {
            self.min_interpolation_offset = min_interpolation_offset;
            self
        }
        pub fn max_interpolation_offset(mut self, max_interpolation_offset: f32) -> Self {
            self.max_interpolation_offset = max_interpolation_offset;
            self
        }
        pub fn sub_pixel_interpolation_offset_bits(
            mut self,
            sub_pixel_interpolation_offset_bits: u32,
        ) -> Self {
            self.sub_pixel_interpolation_offset_bits = sub_pixel_interpolation_offset_bits;
            self
        }
        pub fn max_framebuffer_width(mut self, max_framebuffer_width: u32) -> Self {
            self.max_framebuffer_width = max_framebuffer_width;
            self
        }
        pub fn max_framebuffer_height(mut self, max_framebuffer_height: u32) -> Self {
            self.max_framebuffer_height = max_framebuffer_height;
            self
        }
        pub fn max_framebuffer_layers(mut self, max_framebuffer_layers: u32) -> Self {
            self.max_framebuffer_layers = max_framebuffer_layers;
            self
        }
        pub fn framebuffer_color_sample_counts(
            mut self,
            framebuffer_color_sample_counts: SampleCountFlags,
        ) -> Self {
            self.framebuffer_color_sample_counts = framebuffer_color_sample_counts;
            self
        }
        pub fn framebuffer_depth_sample_counts(
            mut self,
            framebuffer_depth_sample_counts: SampleCountFlags,
        ) -> Self {
            self.framebuffer_depth_sample_counts = framebuffer_depth_sample_counts;
            self
        }
        pub fn framebuffer_stencil_sample_counts(
            mut self,
            framebuffer_stencil_sample_counts: SampleCountFlags,
        ) -> Self {
            self.framebuffer_stencil_sample_counts = framebuffer_stencil_sample_counts;
            self
        }
        pub fn framebuffer_no_attachments_sample_counts(
            mut self,
            framebuffer_no_attachments_sample_counts: SampleCountFlags,
        ) -> Self {
            self.framebuffer_no_attachments_sample_counts =
                framebuffer_no_attachments_sample_counts;
            self
        }
        pub fn max_color_attachments(mut self, max_color_attachments: u32) -> Self {
            self.max_color_attachments = max_color_attachments;
            self
        }
        pub fn sampled_image_color_sample_counts(
            mut self,
            sampled_image_color_sample_counts: SampleCountFlags,
        ) -> Self {
            self.sampled_image_color_sample_counts = sampled_image_color_sample_counts;
            self
        }
        pub fn sampled_image_integer_sample_counts(
            mut self,
            sampled_image_integer_sample_counts: SampleCountFlags,
        ) -> Self {
            self.sampled_image_integer_sample_counts = sampled_image_integer_sample_counts;
            self
        }
        pub fn sampled_image_depth_sample_counts(
            mut self,
            sampled_image_depth_sample_counts: SampleCountFlags,
        ) -> Self {
            self.sampled_image_depth_sample_counts = sampled_image_depth_sample_counts;
            self
        }
        pub fn sampled_image_stencil_sample_counts(
            mut self,
            sampled_image_stencil_sample_counts: SampleCountFlags,
        ) -> Self {
            self.sampled_image_stencil_sample_counts = sampled_image_stencil_sample_counts;
            self
        }
        pub fn storage_image_sample_counts(
            mut self,
            storage_image_sample_counts: SampleCountFlags,
        ) -> Self {
            self.storage_image_sample_counts = storage_image_sample_counts;
            self
        }
        pub fn max_sample_mask_words(mut self, max_sample_mask_words: u32) -> Self {
            self.max_sample_mask_words = max_sample_mask_words;
            self
        }
        pub fn timestamp_compute_and_graphics(
            mut self,
            timestamp_compute_and_graphics: Bool32,
        ) -> Self {
            self.timestamp_compute_and_graphics = timestamp_compute_and_graphics;
            self
        }
        pub fn timestamp_period(mut self, timestamp_period: f32) -> Self {
            self.timestamp_period = timestamp_period;
            self
        }
        pub fn max_clip_distances(mut self, max_clip_distances: u32) -> Self {
            self.max_clip_distances = max_clip_distances;
            self
        }
        pub fn max_cull_distances(mut self, max_cull_distances: u32) -> Self {
            self.max_cull_distances = max_cull_distances;
            self
        }
        pub fn max_combined_clip_and_cull_distances(
            mut self,
            max_combined_clip_and_cull_distances: u32,
        ) -> Self {
            self.max_combined_clip_and_cull_distances = max_combined_clip_and_cull_distances;
            self
        }
        pub fn discrete_queue_priorities(mut self, discrete_queue_priorities: u32) -> Self {
            self.discrete_queue_priorities = discrete_queue_priorities;
            self
        }
        pub fn point_size_range(mut self, point_size_range: [f32; 2]) -> Self {
            self.point_size_range = point_size_range;
            self
        }
        pub fn line_width_range(mut self, line_width_range: [f32; 2]) -> Self {
            self.line_width_range = line_width_range;
            self
        }
        pub fn point_size_granularity(mut self, point_size_granularity: f32) -> Self {
            self.point_size_granularity = point_size_granularity;
            self
        }
        pub fn line_width_granularity(mut self, line_width_granularity: f32) -> Self {
            self.line_width_granularity = line_width_granularity;
            self
        }
        pub fn strict_lines(mut self, strict_lines: Bool32) -> Self {
            self.strict_lines = strict_lines;
            self
        }
        pub fn standard_sample_locations(mut self, standard_sample_locations: Bool32) -> Self {
            self.standard_sample_locations = standard_sample_locations;
            self
        }
        pub fn optimal_buffer_copy_offset_alignment(
            mut self,
            optimal_buffer_copy_offset_alignment: DeviceSize,
        ) -> Self {
            self.optimal_buffer_copy_offset_alignment = optimal_buffer_copy_offset_alignment;
            self
        }
        pub fn optimal_buffer_copy_row_pitch_alignment(
            mut self,
            optimal_buffer_copy_row_pitch_alignment: DeviceSize,
        ) -> Self {
            self.optimal_buffer_copy_row_pitch_alignment = optimal_buffer_copy_row_pitch_alignment;
            self
        }
        pub fn non_coherent_atom_size(mut self, non_coherent_atom_size: DeviceSize) -> Self {
            self.non_coherent_atom_size = non_coherent_atom_size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SemaphoreCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: SemaphoreCreateFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SemaphoreCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SEMAPHORE_CREATE_INFO;
    }
    impl Default for SemaphoreCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SemaphoreCreateInfo<'a> {
        pub fn flags(mut self, flags: SemaphoreCreateFlags) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct QueryPoolCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: QueryPoolCreateFlags,
        pub query_type: QueryType,
        pub query_count: u32,
        pub pipeline_statistics: QueryPipelineStatisticFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for QueryPoolCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::QUERY_POOL_CREATE_INFO;
    }
    impl Default for QueryPoolCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                query_type: Default::default(),
                query_count: Default::default(),
                pipeline_statistics: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> QueryPoolCreateInfo<'a> {
        pub fn flags(mut self, flags: QueryPoolCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn query_type(mut self, query_type: QueryType) -> Self {
            self.query_type = query_type;
            self
        }
        pub fn query_count(mut self, query_count: u32) -> Self {
            self.query_count = query_count;
            self
        }
        pub fn pipeline_statistics(
            mut self,
            pipeline_statistics: QueryPipelineStatisticFlags,
        ) -> Self {
            self.pipeline_statistics = pipeline_statistics;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct FramebufferCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: FramebufferCreateFlags,
        pub render_pass: RenderPass,
        pub attachment_count: u32,
        pub p_attachments: *const ImageView,
        pub width: u32,
        pub height: u32,
        pub layers: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for FramebufferCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::FRAMEBUFFER_CREATE_INFO;
    }
    impl Default for FramebufferCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                render_pass: Default::default(),
                attachment_count: Default::default(),
                p_attachments: core::ptr::null(),
                width: Default::default(),
                height: Default::default(),
                layers: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> FramebufferCreateInfo<'a> {
        pub fn flags(mut self, flags: FramebufferCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn render_pass(mut self, render_pass: RenderPass) -> Self {
            self.render_pass = render_pass;
            self
        }
        pub fn attachments(mut self, attachments: &'a [ImageView]) -> Self {
            self.attachment_count = attachments.len().try_into().unwrap();
            self.p_attachments = attachments.as_ptr();
            self
        }
        pub fn width(mut self, width: u32) -> Self {
            self.width = width;
            self
        }
        pub fn height(mut self, height: u32) -> Self {
            self.height = height;
            self
        }
        pub fn layers(mut self, layers: u32) -> Self {
            self.layers = layers;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct DrawIndirectCommand {
        pub vertex_count: u32,
        pub instance_count: u32,
        pub first_vertex: u32,
        pub first_instance: u32,
    }
    impl DrawIndirectCommand {
        pub fn vertex_count(mut self, vertex_count: u32) -> Self {
            self.vertex_count = vertex_count;
            self
        }
        pub fn instance_count(mut self, instance_count: u32) -> Self {
            self.instance_count = instance_count;
            self
        }
        pub fn first_vertex(mut self, first_vertex: u32) -> Self {
            self.first_vertex = first_vertex;
            self
        }
        pub fn first_instance(mut self, first_instance: u32) -> Self {
            self.first_instance = first_instance;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct DrawIndexedIndirectCommand {
        pub index_count: u32,
        pub instance_count: u32,
        pub first_index: u32,
        pub vertex_offset: i32,
        pub first_instance: u32,
    }
    impl DrawIndexedIndirectCommand {
        pub fn index_count(mut self, index_count: u32) -> Self {
            self.index_count = index_count;
            self
        }
        pub fn instance_count(mut self, instance_count: u32) -> Self {
            self.instance_count = instance_count;
            self
        }
        pub fn first_index(mut self, first_index: u32) -> Self {
            self.first_index = first_index;
            self
        }
        pub fn vertex_offset(mut self, vertex_offset: i32) -> Self {
            self.vertex_offset = vertex_offset;
            self
        }
        pub fn first_instance(mut self, first_instance: u32) -> Self {
            self.first_instance = first_instance;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct DispatchIndirectCommand {
        pub x: u32,
        pub y: u32,
        pub z: u32,
    }
    impl DispatchIndirectCommand {
        pub fn x(mut self, x: u32) -> Self {
            self.x = x;
            self
        }
        pub fn y(mut self, y: u32) -> Self {
            self.y = y;
            self
        }
        pub fn z(mut self, z: u32) -> Self {
            self.z = z;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SubmitInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub wait_semaphore_count: u32,
        pub p_wait_semaphores: *const Semaphore,
        pub p_wait_dst_stage_mask: *const PipelineStageFlags,
        pub command_buffer_count: u32,
        pub p_command_buffers: *const CommandBuffer,
        pub signal_semaphore_count: u32,
        pub p_signal_semaphores: *const Semaphore,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SubmitInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SUBMIT_INFO;
    }
    impl Default for SubmitInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                wait_semaphore_count: Default::default(),
                p_wait_semaphores: core::ptr::null(),
                p_wait_dst_stage_mask: core::ptr::null(),
                command_buffer_count: Default::default(),
                p_command_buffers: core::ptr::null(),
                signal_semaphore_count: Default::default(),
                p_signal_semaphores: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SubmitInfo<'a> {
        pub fn wait_semaphores(mut self, wait_semaphores: &'a [Semaphore]) -> Self {
            self.wait_semaphore_count = wait_semaphores.len().try_into().unwrap();
            self.p_wait_semaphores = wait_semaphores.as_ptr();
            self
        }
        pub fn wait_dst_stage_mask(
            mut self,
            wait_dst_stage_mask: &'a [PipelineStageFlags],
        ) -> Self {
            self.wait_semaphore_count = wait_dst_stage_mask.len().try_into().unwrap();
            self.p_wait_dst_stage_mask = wait_dst_stage_mask.as_ptr();
            self
        }
        pub fn command_buffers(mut self, command_buffers: &'a [CommandBuffer]) -> Self {
            self.command_buffer_count = command_buffers.len().try_into().unwrap();
            self.p_command_buffers = command_buffers.as_ptr();
            self
        }
        pub fn signal_semaphores(mut self, signal_semaphores: &'a [Semaphore]) -> Self {
            self.signal_semaphore_count = signal_semaphores.len().try_into().unwrap();
            self.p_signal_semaphores = signal_semaphores.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union ClearColorValue {
        pub float32: [f32; 4],
        pub int32: [i32; 4],
        pub uint32: [u32; 4],
    }
    impl Default for ClearColorValue {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union ClearValue {
        pub color: ClearColorValue,
        pub depth_stencil: ClearDepthStencilValue,
    }
    impl Default for ClearValue {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ImageLayout(i32);
    impl ImageLayout {
        pub const UNDEFINED: Self = Self(0);
        pub const GENERAL: Self = Self(1);
        pub const COLOR_ATTACHMENT_OPTIMAL: Self = Self(2);
        pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(3);
        pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = Self(4);
        pub const SHADER_READ_ONLY_OPTIMAL: Self = Self(5);
        pub const TRANSFER_SRC_OPTIMAL: Self = Self(6);
        pub const TRANSFER_DST_OPTIMAL: Self = Self(7);
        pub const PREINITIALIZED: Self = Self(8);
        pub const ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT: Self = Self(1000339000);
        pub const ATTACHMENT_OPTIMAL: Self = Self(1000314001);
        pub const DEPTH_ATTACHMENT_OPTIMAL: Self = Self(1000241000);
        pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: Self = Self(1000117001);
        pub const DEPTH_READ_ONLY_OPTIMAL: Self = Self(1000241001);
        pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(1000117000);
        pub const FRAGMENT_DENSITY_MAP_OPTIMAL_EXT: Self = Self(1000218000);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR: Self = Self(1000164003);
        pub const PRESENT_SRC_KHR: Self = Self(1000001002);
        pub const READ_ONLY_OPTIMAL: Self = Self(1000314000);
        pub const RENDERING_LOCAL_READ: Self = Self(1000232000);
        pub const SHARED_PRESENT_KHR: Self = Self(1000111000);
        pub const STENCIL_ATTACHMENT_OPTIMAL: Self = Self(1000241002);
        pub const STENCIL_READ_ONLY_OPTIMAL: Self = Self(1000241003);
        pub const TENSOR_ALIASING_ARM: Self = Self(1000460000);
        pub const VIDEO_DECODE_DPB_KHR: Self = Self(1000024002);
        pub const VIDEO_DECODE_DST_KHR: Self = Self(1000024000);
        pub const VIDEO_DECODE_SRC_KHR: Self = Self(1000024001);
        pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1000299002);
        pub const VIDEO_ENCODE_DST_KHR: Self = Self(1000299000);
        pub const VIDEO_ENCODE_QUANTIZATION_MAP_KHR: Self = Self(1000553000);
        pub const VIDEO_ENCODE_SRC_KHR: Self = Self(1000299001);
        pub const ZERO_INITIALIZED_EXT: Self = Self(1000620000);
        pub const ATTACHMENT_OPTIMAL_KHR: Self = Self::ATTACHMENT_OPTIMAL;
        pub const DEPTH_ATTACHMENT_OPTIMAL_KHR: Self = Self::DEPTH_ATTACHMENT_OPTIMAL;
        pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR: Self =
            Self::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL;
        pub const DEPTH_READ_ONLY_OPTIMAL_KHR: Self = Self::DEPTH_READ_ONLY_OPTIMAL;
        pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR: Self =
            Self::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL;
        pub const READ_ONLY_OPTIMAL_KHR: Self = Self::READ_ONLY_OPTIMAL;
        pub const RENDERING_LOCAL_READ_KHR: Self = Self::RENDERING_LOCAL_READ;
        pub const SHADING_RATE_OPTIMAL_NV: Self =
            Self::FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR;
        pub const STENCIL_ATTACHMENT_OPTIMAL_KHR: Self = Self::STENCIL_ATTACHMENT_OPTIMAL;
        pub const STENCIL_READ_ONLY_OPTIMAL_KHR: Self = Self::STENCIL_READ_ONLY_OPTIMAL;
    }
    impl fmt::Debug for ImageLayout {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::UNDEFINED => Some("UNDEFINED"),
                Self::GENERAL => Some("GENERAL"),
                Self::COLOR_ATTACHMENT_OPTIMAL => Some("COLOR_ATTACHMENT_OPTIMAL"),
                Self::DEPTH_STENCIL_ATTACHMENT_OPTIMAL => Some("DEPTH_STENCIL_ATTACHMENT_OPTIMAL"),
                Self::DEPTH_STENCIL_READ_ONLY_OPTIMAL => Some("DEPTH_STENCIL_READ_ONLY_OPTIMAL"),
                Self::SHADER_READ_ONLY_OPTIMAL => Some("SHADER_READ_ONLY_OPTIMAL"),
                Self::TRANSFER_SRC_OPTIMAL => Some("TRANSFER_SRC_OPTIMAL"),
                Self::TRANSFER_DST_OPTIMAL => Some("TRANSFER_DST_OPTIMAL"),
                Self::PREINITIALIZED => Some("PREINITIALIZED"),
                Self::ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT => {
                    Some("ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT")
                }
                Self::ATTACHMENT_OPTIMAL => Some("ATTACHMENT_OPTIMAL"),
                Self::DEPTH_ATTACHMENT_OPTIMAL => Some("DEPTH_ATTACHMENT_OPTIMAL"),
                Self::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL => {
                    Some("DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL")
                }
                Self::DEPTH_READ_ONLY_OPTIMAL => Some("DEPTH_READ_ONLY_OPTIMAL"),
                Self::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL => {
                    Some("DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL")
                }
                Self::FRAGMENT_DENSITY_MAP_OPTIMAL_EXT => Some("FRAGMENT_DENSITY_MAP_OPTIMAL_EXT"),
                Self::FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR => {
                    Some("FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR")
                }
                Self::PRESENT_SRC_KHR => Some("PRESENT_SRC_KHR"),
                Self::READ_ONLY_OPTIMAL => Some("READ_ONLY_OPTIMAL"),
                Self::RENDERING_LOCAL_READ => Some("RENDERING_LOCAL_READ"),
                Self::SHARED_PRESENT_KHR => Some("SHARED_PRESENT_KHR"),
                Self::STENCIL_ATTACHMENT_OPTIMAL => Some("STENCIL_ATTACHMENT_OPTIMAL"),
                Self::STENCIL_READ_ONLY_OPTIMAL => Some("STENCIL_READ_ONLY_OPTIMAL"),
                Self::TENSOR_ALIASING_ARM => Some("TENSOR_ALIASING_ARM"),
                Self::VIDEO_DECODE_DPB_KHR => Some("VIDEO_DECODE_DPB_KHR"),
                Self::VIDEO_DECODE_DST_KHR => Some("VIDEO_DECODE_DST_KHR"),
                Self::VIDEO_DECODE_SRC_KHR => Some("VIDEO_DECODE_SRC_KHR"),
                Self::VIDEO_ENCODE_DPB_KHR => Some("VIDEO_ENCODE_DPB_KHR"),
                Self::VIDEO_ENCODE_DST_KHR => Some("VIDEO_ENCODE_DST_KHR"),
                Self::VIDEO_ENCODE_QUANTIZATION_MAP_KHR => {
                    Some("VIDEO_ENCODE_QUANTIZATION_MAP_KHR")
                }
                Self::VIDEO_ENCODE_SRC_KHR => Some("VIDEO_ENCODE_SRC_KHR"),
                Self::ZERO_INITIALIZED_EXT => Some("ZERO_INITIALIZED_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AttachmentLoadOp(i32);
    impl AttachmentLoadOp {
        pub const LOAD: Self = Self(0);
        pub const CLEAR: Self = Self(1);
        pub const DONT_CARE: Self = Self(2);
        pub const NONE: Self = Self(1000400000);
        pub const NONE_EXT: Self = Self::NONE;
        pub const NONE_KHR: Self = Self::NONE;
    }
    impl fmt::Debug for AttachmentLoadOp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::LOAD => Some("LOAD"),
                Self::CLEAR => Some("CLEAR"),
                Self::DONT_CARE => Some("DONT_CARE"),
                Self::NONE => Some("NONE"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AttachmentStoreOp(i32);
    impl AttachmentStoreOp {
        pub const STORE: Self = Self(0);
        pub const DONT_CARE: Self = Self(1);
        pub const NONE: Self = Self(1000301000);
        pub const NONE_EXT: Self = Self::NONE;
        pub const NONE_KHR: Self = Self::NONE;
        pub const NONE_QCOM: Self = Self::NONE;
    }
    impl fmt::Debug for AttachmentStoreOp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::STORE => Some("STORE"),
                Self::DONT_CARE => Some("DONT_CARE"),
                Self::NONE => Some("NONE"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ImageType(i32);
    impl ImageType {
        pub const _1D: Self = Self(0);
        pub const _2D: Self = Self(1);
        pub const _3D: Self = Self(2);
    }
    impl fmt::Debug for ImageType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_1D => Some("_1D"),
                Self::_2D => Some("_2D"),
                Self::_3D => Some("_3D"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ImageTiling(i32);
    impl ImageTiling {
        pub const OPTIMAL: Self = Self(0);
        pub const LINEAR: Self = Self(1);
        pub const DRM_FORMAT_MODIFIER_EXT: Self = Self(1000158000);
    }
    impl fmt::Debug for ImageTiling {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::OPTIMAL => Some("OPTIMAL"),
                Self::LINEAR => Some("LINEAR"),
                Self::DRM_FORMAT_MODIFIER_EXT => Some("DRM_FORMAT_MODIFIER_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ImageViewType(i32);
    impl ImageViewType {
        pub const _1D: Self = Self(0);
        pub const _2D: Self = Self(1);
        pub const _3D: Self = Self(2);
        pub const CUBE: Self = Self(3);
        pub const _1D_ARRAY: Self = Self(4);
        pub const _2D_ARRAY: Self = Self(5);
        pub const CUBE_ARRAY: Self = Self(6);
    }
    impl fmt::Debug for ImageViewType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_1D => Some("_1D"),
                Self::_2D => Some("_2D"),
                Self::_3D => Some("_3D"),
                Self::CUBE => Some("CUBE"),
                Self::_1D_ARRAY => Some("_1D_ARRAY"),
                Self::_2D_ARRAY => Some("_2D_ARRAY"),
                Self::CUBE_ARRAY => Some("CUBE_ARRAY"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CommandBufferLevel(i32);
    impl CommandBufferLevel {
        pub const PRIMARY: Self = Self(0);
        pub const SECONDARY: Self = Self(1);
    }
    impl fmt::Debug for CommandBufferLevel {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::PRIMARY => Some("PRIMARY"),
                Self::SECONDARY => Some("SECONDARY"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ComponentSwizzle(i32);
    impl ComponentSwizzle {
        pub const IDENTITY: Self = Self(0);
        pub const ZERO: Self = Self(1);
        pub const ONE: Self = Self(2);
        pub const R: Self = Self(3);
        pub const G: Self = Self(4);
        pub const B: Self = Self(5);
        pub const A: Self = Self(6);
    }
    impl fmt::Debug for ComponentSwizzle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::IDENTITY => Some("IDENTITY"),
                Self::ZERO => Some("ZERO"),
                Self::ONE => Some("ONE"),
                Self::R => Some("R"),
                Self::G => Some("G"),
                Self::B => Some("B"),
                Self::A => Some("A"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DescriptorType(i32);
    impl DescriptorType {
        pub const SAMPLER: Self = Self(0);
        pub const COMBINED_IMAGE_SAMPLER: Self = Self(1);
        pub const SAMPLED_IMAGE: Self = Self(2);
        pub const STORAGE_IMAGE: Self = Self(3);
        pub const UNIFORM_TEXEL_BUFFER: Self = Self(4);
        pub const STORAGE_TEXEL_BUFFER: Self = Self(5);
        pub const UNIFORM_BUFFER: Self = Self(6);
        pub const STORAGE_BUFFER: Self = Self(7);
        pub const UNIFORM_BUFFER_DYNAMIC: Self = Self(8);
        pub const STORAGE_BUFFER_DYNAMIC: Self = Self(9);
        pub const INPUT_ATTACHMENT: Self = Self(10);
        pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
        pub const ACCELERATION_STRUCTURE_NV: Self = Self(1000165000);
        pub const BLOCK_MATCH_IMAGE_QCOM: Self = Self(1000440001);
        pub const INLINE_UNIFORM_BLOCK: Self = Self(1000138000);
        pub const MUTABLE_EXT: Self = Self(1000351000);
        pub const PARTITIONED_ACCELERATION_STRUCTURE_NV: Self = Self(1000570000);
        pub const SAMPLE_WEIGHT_IMAGE_QCOM: Self = Self(1000440000);
        pub const TENSOR_ARM: Self = Self(1000460000);
        pub const INLINE_UNIFORM_BLOCK_EXT: Self = Self::INLINE_UNIFORM_BLOCK;
        pub const MUTABLE_VALVE: Self = Self::MUTABLE_EXT;
    }
    impl fmt::Debug for DescriptorType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SAMPLER => Some("SAMPLER"),
                Self::COMBINED_IMAGE_SAMPLER => Some("COMBINED_IMAGE_SAMPLER"),
                Self::SAMPLED_IMAGE => Some("SAMPLED_IMAGE"),
                Self::STORAGE_IMAGE => Some("STORAGE_IMAGE"),
                Self::UNIFORM_TEXEL_BUFFER => Some("UNIFORM_TEXEL_BUFFER"),
                Self::STORAGE_TEXEL_BUFFER => Some("STORAGE_TEXEL_BUFFER"),
                Self::UNIFORM_BUFFER => Some("UNIFORM_BUFFER"),
                Self::STORAGE_BUFFER => Some("STORAGE_BUFFER"),
                Self::UNIFORM_BUFFER_DYNAMIC => Some("UNIFORM_BUFFER_DYNAMIC"),
                Self::STORAGE_BUFFER_DYNAMIC => Some("STORAGE_BUFFER_DYNAMIC"),
                Self::INPUT_ATTACHMENT => Some("INPUT_ATTACHMENT"),
                Self::ACCELERATION_STRUCTURE_KHR => Some("ACCELERATION_STRUCTURE_KHR"),
                Self::ACCELERATION_STRUCTURE_NV => Some("ACCELERATION_STRUCTURE_NV"),
                Self::BLOCK_MATCH_IMAGE_QCOM => Some("BLOCK_MATCH_IMAGE_QCOM"),
                Self::INLINE_UNIFORM_BLOCK => Some("INLINE_UNIFORM_BLOCK"),
                Self::MUTABLE_EXT => Some("MUTABLE_EXT"),
                Self::PARTITIONED_ACCELERATION_STRUCTURE_NV => {
                    Some("PARTITIONED_ACCELERATION_STRUCTURE_NV")
                }
                Self::SAMPLE_WEIGHT_IMAGE_QCOM => Some("SAMPLE_WEIGHT_IMAGE_QCOM"),
                Self::TENSOR_ARM => Some("TENSOR_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct QueryType(i32);
    impl QueryType {
        pub const OCCLUSION: Self = Self(0);
        pub const PIPELINE_STATISTICS: Self = Self(1);
        pub const TIMESTAMP: Self = Self(2);
        pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR: Self = Self(1000150000);
        pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV: Self = Self(1000165000);
        pub const ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR: Self =
            Self(1000386000);
        pub const ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR: Self = Self(1000150001);
        pub const ACCELERATION_STRUCTURE_SIZE_KHR: Self = Self(1000386001);
        pub const MESH_PRIMITIVES_GENERATED_EXT: Self = Self(1000328000);
        pub const MICROMAP_COMPACTED_SIZE_EXT: Self = Self(1000396001);
        pub const MICROMAP_SERIALIZATION_SIZE_EXT: Self = Self(1000396000);
        pub const PERFORMANCE_QUERY_INTEL: Self = Self(1000210000);
        pub const PERFORMANCE_QUERY_KHR: Self = Self(1000116000);
        pub const PRIMITIVES_GENERATED_EXT: Self = Self(1000382000);
        pub const RESULT_STATUS_ONLY_KHR: Self = Self(1000023000);
        pub const TRANSFORM_FEEDBACK_STREAM_EXT: Self = Self(1000028004);
        pub const VIDEO_ENCODE_FEEDBACK_KHR: Self = Self(1000299000);
    }
    impl fmt::Debug for QueryType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::OCCLUSION => Some("OCCLUSION"),
                Self::PIPELINE_STATISTICS => Some("PIPELINE_STATISTICS"),
                Self::TIMESTAMP => Some("TIMESTAMP"),
                Self::ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR => {
                    Some("ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR")
                }
                Self::ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV => {
                    Some("ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV")
                }
                Self::ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR => {
                    Some("ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR")
                }
                Self::ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR => {
                    Some("ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR")
                }
                Self::ACCELERATION_STRUCTURE_SIZE_KHR => Some("ACCELERATION_STRUCTURE_SIZE_KHR"),
                Self::MESH_PRIMITIVES_GENERATED_EXT => Some("MESH_PRIMITIVES_GENERATED_EXT"),
                Self::MICROMAP_COMPACTED_SIZE_EXT => Some("MICROMAP_COMPACTED_SIZE_EXT"),
                Self::MICROMAP_SERIALIZATION_SIZE_EXT => Some("MICROMAP_SERIALIZATION_SIZE_EXT"),
                Self::PERFORMANCE_QUERY_INTEL => Some("PERFORMANCE_QUERY_INTEL"),
                Self::PERFORMANCE_QUERY_KHR => Some("PERFORMANCE_QUERY_KHR"),
                Self::PRIMITIVES_GENERATED_EXT => Some("PRIMITIVES_GENERATED_EXT"),
                Self::RESULT_STATUS_ONLY_KHR => Some("RESULT_STATUS_ONLY_KHR"),
                Self::TRANSFORM_FEEDBACK_STREAM_EXT => Some("TRANSFORM_FEEDBACK_STREAM_EXT"),
                Self::VIDEO_ENCODE_FEEDBACK_KHR => Some("VIDEO_ENCODE_FEEDBACK_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BorderColor(i32);
    impl BorderColor {
        pub const FLOAT_TRANSPARENT_BLACK: Self = Self(0);
        pub const INT_TRANSPARENT_BLACK: Self = Self(1);
        pub const FLOAT_OPAQUE_BLACK: Self = Self(2);
        pub const INT_OPAQUE_BLACK: Self = Self(3);
        pub const FLOAT_OPAQUE_WHITE: Self = Self(4);
        pub const INT_OPAQUE_WHITE: Self = Self(5);
        pub const FLOAT_CUSTOM_EXT: Self = Self(1000287003);
        pub const INT_CUSTOM_EXT: Self = Self(1000287004);
    }
    impl fmt::Debug for BorderColor {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::FLOAT_TRANSPARENT_BLACK => Some("FLOAT_TRANSPARENT_BLACK"),
                Self::INT_TRANSPARENT_BLACK => Some("INT_TRANSPARENT_BLACK"),
                Self::FLOAT_OPAQUE_BLACK => Some("FLOAT_OPAQUE_BLACK"),
                Self::INT_OPAQUE_BLACK => Some("INT_OPAQUE_BLACK"),
                Self::FLOAT_OPAQUE_WHITE => Some("FLOAT_OPAQUE_WHITE"),
                Self::INT_OPAQUE_WHITE => Some("INT_OPAQUE_WHITE"),
                Self::FLOAT_CUSTOM_EXT => Some("FLOAT_CUSTOM_EXT"),
                Self::INT_CUSTOM_EXT => Some("INT_CUSTOM_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineBindPoint(i32);
    impl PipelineBindPoint {
        pub const GRAPHICS: Self = Self(0);
        pub const COMPUTE: Self = Self(1);
        pub const DATA_GRAPH_ARM: Self = Self(1000507000);
        pub const EXECUTION_GRAPH_AMDX: Self = Self(1000134000);
        pub const RAY_TRACING_KHR: Self = Self(1000165000);
        pub const SUBPASS_SHADING_HUAWEI: Self = Self(1000369003);
        pub const RAY_TRACING_NV: Self = Self::RAY_TRACING_KHR;
    }
    impl fmt::Debug for PipelineBindPoint {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::GRAPHICS => Some("GRAPHICS"),
                Self::COMPUTE => Some("COMPUTE"),
                Self::DATA_GRAPH_ARM => Some("DATA_GRAPH_ARM"),
                Self::EXECUTION_GRAPH_AMDX => Some("EXECUTION_GRAPH_AMDX"),
                Self::RAY_TRACING_KHR => Some("RAY_TRACING_KHR"),
                Self::SUBPASS_SHADING_HUAWEI => Some("SUBPASS_SHADING_HUAWEI"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineCacheHeaderVersion(i32);
    impl PipelineCacheHeaderVersion {
        pub const ONE: Self = Self(1);
        pub const DATA_GRAPH_QCOM: Self = Self(1000629000);
    }
    impl fmt::Debug for PipelineCacheHeaderVersion {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ONE => Some("ONE"),
                Self::DATA_GRAPH_QCOM => Some("DATA_GRAPH_QCOM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PrimitiveTopology(i32);
    impl PrimitiveTopology {
        pub const POINT_LIST: Self = Self(0);
        pub const LINE_LIST: Self = Self(1);
        pub const LINE_STRIP: Self = Self(2);
        pub const TRIANGLE_LIST: Self = Self(3);
        pub const TRIANGLE_STRIP: Self = Self(4);
        pub const TRIANGLE_FAN: Self = Self(5);
        pub const LINE_LIST_WITH_ADJACENCY: Self = Self(6);
        pub const LINE_STRIP_WITH_ADJACENCY: Self = Self(7);
        pub const TRIANGLE_LIST_WITH_ADJACENCY: Self = Self(8);
        pub const TRIANGLE_STRIP_WITH_ADJACENCY: Self = Self(9);
        pub const PATCH_LIST: Self = Self(10);
    }
    impl fmt::Debug for PrimitiveTopology {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::POINT_LIST => Some("POINT_LIST"),
                Self::LINE_LIST => Some("LINE_LIST"),
                Self::LINE_STRIP => Some("LINE_STRIP"),
                Self::TRIANGLE_LIST => Some("TRIANGLE_LIST"),
                Self::TRIANGLE_STRIP => Some("TRIANGLE_STRIP"),
                Self::TRIANGLE_FAN => Some("TRIANGLE_FAN"),
                Self::LINE_LIST_WITH_ADJACENCY => Some("LINE_LIST_WITH_ADJACENCY"),
                Self::LINE_STRIP_WITH_ADJACENCY => Some("LINE_STRIP_WITH_ADJACENCY"),
                Self::TRIANGLE_LIST_WITH_ADJACENCY => Some("TRIANGLE_LIST_WITH_ADJACENCY"),
                Self::TRIANGLE_STRIP_WITH_ADJACENCY => Some("TRIANGLE_STRIP_WITH_ADJACENCY"),
                Self::PATCH_LIST => Some("PATCH_LIST"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SharingMode(i32);
    impl SharingMode {
        pub const EXCLUSIVE: Self = Self(0);
        pub const CONCURRENT: Self = Self(1);
    }
    impl fmt::Debug for SharingMode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::EXCLUSIVE => Some("EXCLUSIVE"),
                Self::CONCURRENT => Some("CONCURRENT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct IndexType(i32);
    impl IndexType {
        pub const UINT16: Self = Self(0);
        pub const UINT32: Self = Self(1);
        pub const NONE_KHR: Self = Self(1000165000);
        pub const UINT8: Self = Self(1000265000);
        pub const NONE_NV: Self = Self::NONE_KHR;
        pub const UINT8_EXT: Self = Self::UINT8;
        pub const UINT8_KHR: Self = Self::UINT8;
    }
    impl fmt::Debug for IndexType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::UINT16 => Some("UINT16"),
                Self::UINT32 => Some("UINT32"),
                Self::NONE_KHR => Some("NONE_KHR"),
                Self::UINT8 => Some("UINT8"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Filter(i32);
    impl Filter {
        pub const NEAREST: Self = Self(0);
        pub const LINEAR: Self = Self(1);
        pub const CUBIC_EXT: Self = Self(1000015000);
        pub const CUBIC_IMG: Self = Self::CUBIC_EXT;
    }
    impl fmt::Debug for Filter {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::NEAREST => Some("NEAREST"),
                Self::LINEAR => Some("LINEAR"),
                Self::CUBIC_EXT => Some("CUBIC_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SamplerMipmapMode(i32);
    impl SamplerMipmapMode {
        pub const NEAREST: Self = Self(0);
        pub const LINEAR: Self = Self(1);
    }
    impl fmt::Debug for SamplerMipmapMode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::NEAREST => Some("NEAREST"),
                Self::LINEAR => Some("LINEAR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SamplerAddressMode(i32);
    impl SamplerAddressMode {
        pub const REPEAT: Self = Self(0);
        pub const MIRRORED_REPEAT: Self = Self(1);
        pub const CLAMP_TO_EDGE: Self = Self(2);
        pub const CLAMP_TO_BORDER: Self = Self(3);
        pub const MIRROR_CLAMP_TO_EDGE: Self = Self(4);
        pub const MIRROR_CLAMP_TO_EDGE_KHR: Self = Self::MIRROR_CLAMP_TO_EDGE;
    }
    impl fmt::Debug for SamplerAddressMode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::REPEAT => Some("REPEAT"),
                Self::MIRRORED_REPEAT => Some("MIRRORED_REPEAT"),
                Self::CLAMP_TO_EDGE => Some("CLAMP_TO_EDGE"),
                Self::CLAMP_TO_BORDER => Some("CLAMP_TO_BORDER"),
                Self::MIRROR_CLAMP_TO_EDGE => Some("MIRROR_CLAMP_TO_EDGE"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CompareOp(i32);
    impl CompareOp {
        pub const NEVER: Self = Self(0);
        pub const LESS: Self = Self(1);
        pub const EQUAL: Self = Self(2);
        pub const LESS_OR_EQUAL: Self = Self(3);
        pub const GREATER: Self = Self(4);
        pub const NOT_EQUAL: Self = Self(5);
        pub const GREATER_OR_EQUAL: Self = Self(6);
        pub const ALWAYS: Self = Self(7);
    }
    impl fmt::Debug for CompareOp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::NEVER => Some("NEVER"),
                Self::LESS => Some("LESS"),
                Self::EQUAL => Some("EQUAL"),
                Self::LESS_OR_EQUAL => Some("LESS_OR_EQUAL"),
                Self::GREATER => Some("GREATER"),
                Self::NOT_EQUAL => Some("NOT_EQUAL"),
                Self::GREATER_OR_EQUAL => Some("GREATER_OR_EQUAL"),
                Self::ALWAYS => Some("ALWAYS"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PolygonMode(i32);
    impl PolygonMode {
        pub const FILL: Self = Self(0);
        pub const LINE: Self = Self(1);
        pub const POINT: Self = Self(2);
        pub const FILL_RECTANGLE_NV: Self = Self(1000153000);
    }
    impl fmt::Debug for PolygonMode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::FILL => Some("FILL"),
                Self::LINE => Some("LINE"),
                Self::POINT => Some("POINT"),
                Self::FILL_RECTANGLE_NV => Some("FILL_RECTANGLE_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FrontFace(i32);
    impl FrontFace {
        pub const COUNTER_CLOCKWISE: Self = Self(0);
        pub const CLOCKWISE: Self = Self(1);
    }
    impl fmt::Debug for FrontFace {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::COUNTER_CLOCKWISE => Some("COUNTER_CLOCKWISE"),
                Self::CLOCKWISE => Some("CLOCKWISE"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BlendFactor(i32);
    impl BlendFactor {
        pub const ZERO: Self = Self(0);
        pub const ONE: Self = Self(1);
        pub const SRC_COLOR: Self = Self(2);
        pub const ONE_MINUS_SRC_COLOR: Self = Self(3);
        pub const DST_COLOR: Self = Self(4);
        pub const ONE_MINUS_DST_COLOR: Self = Self(5);
        pub const SRC_ALPHA: Self = Self(6);
        pub const ONE_MINUS_SRC_ALPHA: Self = Self(7);
        pub const DST_ALPHA: Self = Self(8);
        pub const ONE_MINUS_DST_ALPHA: Self = Self(9);
        pub const CONSTANT_COLOR: Self = Self(10);
        pub const ONE_MINUS_CONSTANT_COLOR: Self = Self(11);
        pub const CONSTANT_ALPHA: Self = Self(12);
        pub const ONE_MINUS_CONSTANT_ALPHA: Self = Self(13);
        pub const SRC_ALPHA_SATURATE: Self = Self(14);
        pub const SRC1_COLOR: Self = Self(15);
        pub const ONE_MINUS_SRC1_COLOR: Self = Self(16);
        pub const SRC1_ALPHA: Self = Self(17);
        pub const ONE_MINUS_SRC1_ALPHA: Self = Self(18);
    }
    impl fmt::Debug for BlendFactor {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ZERO => Some("ZERO"),
                Self::ONE => Some("ONE"),
                Self::SRC_COLOR => Some("SRC_COLOR"),
                Self::ONE_MINUS_SRC_COLOR => Some("ONE_MINUS_SRC_COLOR"),
                Self::DST_COLOR => Some("DST_COLOR"),
                Self::ONE_MINUS_DST_COLOR => Some("ONE_MINUS_DST_COLOR"),
                Self::SRC_ALPHA => Some("SRC_ALPHA"),
                Self::ONE_MINUS_SRC_ALPHA => Some("ONE_MINUS_SRC_ALPHA"),
                Self::DST_ALPHA => Some("DST_ALPHA"),
                Self::ONE_MINUS_DST_ALPHA => Some("ONE_MINUS_DST_ALPHA"),
                Self::CONSTANT_COLOR => Some("CONSTANT_COLOR"),
                Self::ONE_MINUS_CONSTANT_COLOR => Some("ONE_MINUS_CONSTANT_COLOR"),
                Self::CONSTANT_ALPHA => Some("CONSTANT_ALPHA"),
                Self::ONE_MINUS_CONSTANT_ALPHA => Some("ONE_MINUS_CONSTANT_ALPHA"),
                Self::SRC_ALPHA_SATURATE => Some("SRC_ALPHA_SATURATE"),
                Self::SRC1_COLOR => Some("SRC1_COLOR"),
                Self::ONE_MINUS_SRC1_COLOR => Some("ONE_MINUS_SRC1_COLOR"),
                Self::SRC1_ALPHA => Some("SRC1_ALPHA"),
                Self::ONE_MINUS_SRC1_ALPHA => Some("ONE_MINUS_SRC1_ALPHA"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BlendOp(i32);
    impl BlendOp {
        pub const ADD: Self = Self(0);
        pub const SUBTRACT: Self = Self(1);
        pub const REVERSE_SUBTRACT: Self = Self(2);
        pub const MIN: Self = Self(3);
        pub const MAX: Self = Self(4);
        pub const BLUE_EXT: Self = Self(1000148045);
        pub const COLORBURN_EXT: Self = Self(1000148018);
        pub const COLORDODGE_EXT: Self = Self(1000148017);
        pub const CONTRAST_EXT: Self = Self(1000148041);
        pub const DARKEN_EXT: Self = Self(1000148015);
        pub const DIFFERENCE_EXT: Self = Self(1000148021);
        pub const DST_ATOP_EXT: Self = Self(1000148010);
        pub const DST_EXT: Self = Self(1000148002);
        pub const DST_IN_EXT: Self = Self(1000148006);
        pub const DST_OUT_EXT: Self = Self(1000148008);
        pub const DST_OVER_EXT: Self = Self(1000148004);
        pub const EXCLUSION_EXT: Self = Self(1000148022);
        pub const GREEN_EXT: Self = Self(1000148044);
        pub const HARDLIGHT_EXT: Self = Self(1000148019);
        pub const HARDMIX_EXT: Self = Self(1000148030);
        pub const HSL_COLOR_EXT: Self = Self(1000148033);
        pub const HSL_HUE_EXT: Self = Self(1000148031);
        pub const HSL_LUMINOSITY_EXT: Self = Self(1000148034);
        pub const HSL_SATURATION_EXT: Self = Self(1000148032);
        pub const INVERT_EXT: Self = Self(1000148023);
        pub const INVERT_OVG_EXT: Self = Self(1000148042);
        pub const INVERT_RGB_EXT: Self = Self(1000148024);
        pub const LIGHTEN_EXT: Self = Self(1000148016);
        pub const LINEARBURN_EXT: Self = Self(1000148026);
        pub const LINEARDODGE_EXT: Self = Self(1000148025);
        pub const LINEARLIGHT_EXT: Self = Self(1000148028);
        pub const MINUS_CLAMPED_EXT: Self = Self(1000148040);
        pub const MINUS_EXT: Self = Self(1000148039);
        pub const MULTIPLY_EXT: Self = Self(1000148012);
        pub const OVERLAY_EXT: Self = Self(1000148014);
        pub const PINLIGHT_EXT: Self = Self(1000148029);
        pub const PLUS_CLAMPED_ALPHA_EXT: Self = Self(1000148037);
        pub const PLUS_CLAMPED_EXT: Self = Self(1000148036);
        pub const PLUS_DARKER_EXT: Self = Self(1000148038);
        pub const PLUS_EXT: Self = Self(1000148035);
        pub const RED_EXT: Self = Self(1000148043);
        pub const SCREEN_EXT: Self = Self(1000148013);
        pub const SOFTLIGHT_EXT: Self = Self(1000148020);
        pub const SRC_ATOP_EXT: Self = Self(1000148009);
        pub const SRC_EXT: Self = Self(1000148001);
        pub const SRC_IN_EXT: Self = Self(1000148005);
        pub const SRC_OUT_EXT: Self = Self(1000148007);
        pub const SRC_OVER_EXT: Self = Self(1000148003);
        pub const VIVIDLIGHT_EXT: Self = Self(1000148027);
        pub const XOR_EXT: Self = Self(1000148011);
        pub const ZERO_EXT: Self = Self(1000148000);
    }
    impl fmt::Debug for BlendOp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ADD => Some("ADD"),
                Self::SUBTRACT => Some("SUBTRACT"),
                Self::REVERSE_SUBTRACT => Some("REVERSE_SUBTRACT"),
                Self::MIN => Some("MIN"),
                Self::MAX => Some("MAX"),
                Self::BLUE_EXT => Some("BLUE_EXT"),
                Self::COLORBURN_EXT => Some("COLORBURN_EXT"),
                Self::COLORDODGE_EXT => Some("COLORDODGE_EXT"),
                Self::CONTRAST_EXT => Some("CONTRAST_EXT"),
                Self::DARKEN_EXT => Some("DARKEN_EXT"),
                Self::DIFFERENCE_EXT => Some("DIFFERENCE_EXT"),
                Self::DST_ATOP_EXT => Some("DST_ATOP_EXT"),
                Self::DST_EXT => Some("DST_EXT"),
                Self::DST_IN_EXT => Some("DST_IN_EXT"),
                Self::DST_OUT_EXT => Some("DST_OUT_EXT"),
                Self::DST_OVER_EXT => Some("DST_OVER_EXT"),
                Self::EXCLUSION_EXT => Some("EXCLUSION_EXT"),
                Self::GREEN_EXT => Some("GREEN_EXT"),
                Self::HARDLIGHT_EXT => Some("HARDLIGHT_EXT"),
                Self::HARDMIX_EXT => Some("HARDMIX_EXT"),
                Self::HSL_COLOR_EXT => Some("HSL_COLOR_EXT"),
                Self::HSL_HUE_EXT => Some("HSL_HUE_EXT"),
                Self::HSL_LUMINOSITY_EXT => Some("HSL_LUMINOSITY_EXT"),
                Self::HSL_SATURATION_EXT => Some("HSL_SATURATION_EXT"),
                Self::INVERT_EXT => Some("INVERT_EXT"),
                Self::INVERT_OVG_EXT => Some("INVERT_OVG_EXT"),
                Self::INVERT_RGB_EXT => Some("INVERT_RGB_EXT"),
                Self::LIGHTEN_EXT => Some("LIGHTEN_EXT"),
                Self::LINEARBURN_EXT => Some("LINEARBURN_EXT"),
                Self::LINEARDODGE_EXT => Some("LINEARDODGE_EXT"),
                Self::LINEARLIGHT_EXT => Some("LINEARLIGHT_EXT"),
                Self::MINUS_CLAMPED_EXT => Some("MINUS_CLAMPED_EXT"),
                Self::MINUS_EXT => Some("MINUS_EXT"),
                Self::MULTIPLY_EXT => Some("MULTIPLY_EXT"),
                Self::OVERLAY_EXT => Some("OVERLAY_EXT"),
                Self::PINLIGHT_EXT => Some("PINLIGHT_EXT"),
                Self::PLUS_CLAMPED_ALPHA_EXT => Some("PLUS_CLAMPED_ALPHA_EXT"),
                Self::PLUS_CLAMPED_EXT => Some("PLUS_CLAMPED_EXT"),
                Self::PLUS_DARKER_EXT => Some("PLUS_DARKER_EXT"),
                Self::PLUS_EXT => Some("PLUS_EXT"),
                Self::RED_EXT => Some("RED_EXT"),
                Self::SCREEN_EXT => Some("SCREEN_EXT"),
                Self::SOFTLIGHT_EXT => Some("SOFTLIGHT_EXT"),
                Self::SRC_ATOP_EXT => Some("SRC_ATOP_EXT"),
                Self::SRC_EXT => Some("SRC_EXT"),
                Self::SRC_IN_EXT => Some("SRC_IN_EXT"),
                Self::SRC_OUT_EXT => Some("SRC_OUT_EXT"),
                Self::SRC_OVER_EXT => Some("SRC_OVER_EXT"),
                Self::VIVIDLIGHT_EXT => Some("VIVIDLIGHT_EXT"),
                Self::XOR_EXT => Some("XOR_EXT"),
                Self::ZERO_EXT => Some("ZERO_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StencilOp(i32);
    impl StencilOp {
        pub const KEEP: Self = Self(0);
        pub const ZERO: Self = Self(1);
        pub const REPLACE: Self = Self(2);
        pub const INCREMENT_AND_CLAMP: Self = Self(3);
        pub const DECREMENT_AND_CLAMP: Self = Self(4);
        pub const INVERT: Self = Self(5);
        pub const INCREMENT_AND_WRAP: Self = Self(6);
        pub const DECREMENT_AND_WRAP: Self = Self(7);
    }
    impl fmt::Debug for StencilOp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::KEEP => Some("KEEP"),
                Self::ZERO => Some("ZERO"),
                Self::REPLACE => Some("REPLACE"),
                Self::INCREMENT_AND_CLAMP => Some("INCREMENT_AND_CLAMP"),
                Self::DECREMENT_AND_CLAMP => Some("DECREMENT_AND_CLAMP"),
                Self::INVERT => Some("INVERT"),
                Self::INCREMENT_AND_WRAP => Some("INCREMENT_AND_WRAP"),
                Self::DECREMENT_AND_WRAP => Some("DECREMENT_AND_WRAP"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LogicOp(i32);
    impl LogicOp {
        pub const CLEAR: Self = Self(0);
        pub const AND: Self = Self(1);
        pub const AND_REVERSE: Self = Self(2);
        pub const COPY: Self = Self(3);
        pub const AND_INVERTED: Self = Self(4);
        pub const NO_OP: Self = Self(5);
        pub const XOR: Self = Self(6);
        pub const OR: Self = Self(7);
        pub const NOR: Self = Self(8);
        pub const EQUIVALENT: Self = Self(9);
        pub const INVERT: Self = Self(10);
        pub const OR_REVERSE: Self = Self(11);
        pub const COPY_INVERTED: Self = Self(12);
        pub const OR_INVERTED: Self = Self(13);
        pub const NAND: Self = Self(14);
        pub const SET: Self = Self(15);
    }
    impl fmt::Debug for LogicOp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::CLEAR => Some("CLEAR"),
                Self::AND => Some("AND"),
                Self::AND_REVERSE => Some("AND_REVERSE"),
                Self::COPY => Some("COPY"),
                Self::AND_INVERTED => Some("AND_INVERTED"),
                Self::NO_OP => Some("NO_OP"),
                Self::XOR => Some("XOR"),
                Self::OR => Some("OR"),
                Self::NOR => Some("NOR"),
                Self::EQUIVALENT => Some("EQUIVALENT"),
                Self::INVERT => Some("INVERT"),
                Self::OR_REVERSE => Some("OR_REVERSE"),
                Self::COPY_INVERTED => Some("COPY_INVERTED"),
                Self::OR_INVERTED => Some("OR_INVERTED"),
                Self::NAND => Some("NAND"),
                Self::SET => Some("SET"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct InternalAllocationType(i32);
    impl InternalAllocationType {
        pub const EXECUTABLE: Self = Self(0);
    }
    impl fmt::Debug for InternalAllocationType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::EXECUTABLE => Some("EXECUTABLE"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SystemAllocationScope(i32);
    impl SystemAllocationScope {
        pub const COMMAND: Self = Self(0);
        pub const OBJECT: Self = Self(1);
        pub const CACHE: Self = Self(2);
        pub const DEVICE: Self = Self(3);
        pub const INSTANCE: Self = Self(4);
    }
    impl fmt::Debug for SystemAllocationScope {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::COMMAND => Some("COMMAND"),
                Self::OBJECT => Some("OBJECT"),
                Self::CACHE => Some("CACHE"),
                Self::DEVICE => Some("DEVICE"),
                Self::INSTANCE => Some("INSTANCE"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PhysicalDeviceType(i32);
    impl PhysicalDeviceType {
        pub const OTHER: Self = Self(0);
        pub const INTEGRATED_GPU: Self = Self(1);
        pub const DISCRETE_GPU: Self = Self(2);
        pub const VIRTUAL_GPU: Self = Self(3);
        pub const CPU: Self = Self(4);
    }
    impl fmt::Debug for PhysicalDeviceType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::OTHER => Some("OTHER"),
                Self::INTEGRATED_GPU => Some("INTEGRATED_GPU"),
                Self::DISCRETE_GPU => Some("DISCRETE_GPU"),
                Self::VIRTUAL_GPU => Some("VIRTUAL_GPU"),
                Self::CPU => Some("CPU"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VertexInputRate(i32);
    impl VertexInputRate {
        pub const VERTEX: Self = Self(0);
        pub const INSTANCE: Self = Self(1);
    }
    impl fmt::Debug for VertexInputRate {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::VERTEX => Some("VERTEX"),
                Self::INSTANCE => Some("INSTANCE"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Format(i32);
    impl Format {
        pub const UNDEFINED: Self = Self(0);
        pub const R4G4_UNORM_PACK8: Self = Self(1);
        pub const R4G4B4A4_UNORM_PACK16: Self = Self(2);
        pub const B4G4R4A4_UNORM_PACK16: Self = Self(3);
        pub const R5G6B5_UNORM_PACK16: Self = Self(4);
        pub const B5G6R5_UNORM_PACK16: Self = Self(5);
        pub const R5G5B5A1_UNORM_PACK16: Self = Self(6);
        pub const B5G5R5A1_UNORM_PACK16: Self = Self(7);
        pub const A1R5G5B5_UNORM_PACK16: Self = Self(8);
        pub const R8_UNORM: Self = Self(9);
        pub const R8_SNORM: Self = Self(10);
        pub const R8_USCALED: Self = Self(11);
        pub const R8_SSCALED: Self = Self(12);
        pub const R8_UINT: Self = Self(13);
        pub const R8_SINT: Self = Self(14);
        pub const R8_SRGB: Self = Self(15);
        pub const R8G8_UNORM: Self = Self(16);
        pub const R8G8_SNORM: Self = Self(17);
        pub const R8G8_USCALED: Self = Self(18);
        pub const R8G8_SSCALED: Self = Self(19);
        pub const R8G8_UINT: Self = Self(20);
        pub const R8G8_SINT: Self = Self(21);
        pub const R8G8_SRGB: Self = Self(22);
        pub const R8G8B8_UNORM: Self = Self(23);
        pub const R8G8B8_SNORM: Self = Self(24);
        pub const R8G8B8_USCALED: Self = Self(25);
        pub const R8G8B8_SSCALED: Self = Self(26);
        pub const R8G8B8_UINT: Self = Self(27);
        pub const R8G8B8_SINT: Self = Self(28);
        pub const R8G8B8_SRGB: Self = Self(29);
        pub const B8G8R8_UNORM: Self = Self(30);
        pub const B8G8R8_SNORM: Self = Self(31);
        pub const B8G8R8_USCALED: Self = Self(32);
        pub const B8G8R8_SSCALED: Self = Self(33);
        pub const B8G8R8_UINT: Self = Self(34);
        pub const B8G8R8_SINT: Self = Self(35);
        pub const B8G8R8_SRGB: Self = Self(36);
        pub const R8G8B8A8_UNORM: Self = Self(37);
        pub const R8G8B8A8_SNORM: Self = Self(38);
        pub const R8G8B8A8_USCALED: Self = Self(39);
        pub const R8G8B8A8_SSCALED: Self = Self(40);
        pub const R8G8B8A8_UINT: Self = Self(41);
        pub const R8G8B8A8_SINT: Self = Self(42);
        pub const R8G8B8A8_SRGB: Self = Self(43);
        pub const B8G8R8A8_UNORM: Self = Self(44);
        pub const B8G8R8A8_SNORM: Self = Self(45);
        pub const B8G8R8A8_USCALED: Self = Self(46);
        pub const B8G8R8A8_SSCALED: Self = Self(47);
        pub const B8G8R8A8_UINT: Self = Self(48);
        pub const B8G8R8A8_SINT: Self = Self(49);
        pub const B8G8R8A8_SRGB: Self = Self(50);
        pub const A8B8G8R8_UNORM_PACK32: Self = Self(51);
        pub const A8B8G8R8_SNORM_PACK32: Self = Self(52);
        pub const A8B8G8R8_USCALED_PACK32: Self = Self(53);
        pub const A8B8G8R8_SSCALED_PACK32: Self = Self(54);
        pub const A8B8G8R8_UINT_PACK32: Self = Self(55);
        pub const A8B8G8R8_SINT_PACK32: Self = Self(56);
        pub const A8B8G8R8_SRGB_PACK32: Self = Self(57);
        pub const A2R10G10B10_UNORM_PACK32: Self = Self(58);
        pub const A2R10G10B10_SNORM_PACK32: Self = Self(59);
        pub const A2R10G10B10_USCALED_PACK32: Self = Self(60);
        pub const A2R10G10B10_SSCALED_PACK32: Self = Self(61);
        pub const A2R10G10B10_UINT_PACK32: Self = Self(62);
        pub const A2R10G10B10_SINT_PACK32: Self = Self(63);
        pub const A2B10G10R10_UNORM_PACK32: Self = Self(64);
        pub const A2B10G10R10_SNORM_PACK32: Self = Self(65);
        pub const A2B10G10R10_USCALED_PACK32: Self = Self(66);
        pub const A2B10G10R10_SSCALED_PACK32: Self = Self(67);
        pub const A2B10G10R10_UINT_PACK32: Self = Self(68);
        pub const A2B10G10R10_SINT_PACK32: Self = Self(69);
        pub const R16_UNORM: Self = Self(70);
        pub const R16_SNORM: Self = Self(71);
        pub const R16_USCALED: Self = Self(72);
        pub const R16_SSCALED: Self = Self(73);
        pub const R16_UINT: Self = Self(74);
        pub const R16_SINT: Self = Self(75);
        pub const R16_SFLOAT: Self = Self(76);
        pub const R16G16_UNORM: Self = Self(77);
        pub const R16G16_SNORM: Self = Self(78);
        pub const R16G16_USCALED: Self = Self(79);
        pub const R16G16_SSCALED: Self = Self(80);
        pub const R16G16_UINT: Self = Self(81);
        pub const R16G16_SINT: Self = Self(82);
        pub const R16G16_SFLOAT: Self = Self(83);
        pub const R16G16B16_UNORM: Self = Self(84);
        pub const R16G16B16_SNORM: Self = Self(85);
        pub const R16G16B16_USCALED: Self = Self(86);
        pub const R16G16B16_SSCALED: Self = Self(87);
        pub const R16G16B16_UINT: Self = Self(88);
        pub const R16G16B16_SINT: Self = Self(89);
        pub const R16G16B16_SFLOAT: Self = Self(90);
        pub const R16G16B16A16_UNORM: Self = Self(91);
        pub const R16G16B16A16_SNORM: Self = Self(92);
        pub const R16G16B16A16_USCALED: Self = Self(93);
        pub const R16G16B16A16_SSCALED: Self = Self(94);
        pub const R16G16B16A16_UINT: Self = Self(95);
        pub const R16G16B16A16_SINT: Self = Self(96);
        pub const R16G16B16A16_SFLOAT: Self = Self(97);
        pub const R32_UINT: Self = Self(98);
        pub const R32_SINT: Self = Self(99);
        pub const R32_SFLOAT: Self = Self(100);
        pub const R32G32_UINT: Self = Self(101);
        pub const R32G32_SINT: Self = Self(102);
        pub const R32G32_SFLOAT: Self = Self(103);
        pub const R32G32B32_UINT: Self = Self(104);
        pub const R32G32B32_SINT: Self = Self(105);
        pub const R32G32B32_SFLOAT: Self = Self(106);
        pub const R32G32B32A32_UINT: Self = Self(107);
        pub const R32G32B32A32_SINT: Self = Self(108);
        pub const R32G32B32A32_SFLOAT: Self = Self(109);
        pub const R64_UINT: Self = Self(110);
        pub const R64_SINT: Self = Self(111);
        pub const R64_SFLOAT: Self = Self(112);
        pub const R64G64_UINT: Self = Self(113);
        pub const R64G64_SINT: Self = Self(114);
        pub const R64G64_SFLOAT: Self = Self(115);
        pub const R64G64B64_UINT: Self = Self(116);
        pub const R64G64B64_SINT: Self = Self(117);
        pub const R64G64B64_SFLOAT: Self = Self(118);
        pub const R64G64B64A64_UINT: Self = Self(119);
        pub const R64G64B64A64_SINT: Self = Self(120);
        pub const R64G64B64A64_SFLOAT: Self = Self(121);
        pub const B10G11R11_UFLOAT_PACK32: Self = Self(122);
        pub const E5B9G9R9_UFLOAT_PACK32: Self = Self(123);
        pub const D16_UNORM: Self = Self(124);
        pub const X8_D24_UNORM_PACK32: Self = Self(125);
        pub const D32_SFLOAT: Self = Self(126);
        pub const S8_UINT: Self = Self(127);
        pub const D16_UNORM_S8_UINT: Self = Self(128);
        pub const D24_UNORM_S8_UINT: Self = Self(129);
        pub const D32_SFLOAT_S8_UINT: Self = Self(130);
        pub const BC1_RGB_UNORM_BLOCK: Self = Self(131);
        pub const BC1_RGB_SRGB_BLOCK: Self = Self(132);
        pub const BC1_RGBA_UNORM_BLOCK: Self = Self(133);
        pub const BC1_RGBA_SRGB_BLOCK: Self = Self(134);
        pub const BC2_UNORM_BLOCK: Self = Self(135);
        pub const BC2_SRGB_BLOCK: Self = Self(136);
        pub const BC3_UNORM_BLOCK: Self = Self(137);
        pub const BC3_SRGB_BLOCK: Self = Self(138);
        pub const BC4_UNORM_BLOCK: Self = Self(139);
        pub const BC4_SNORM_BLOCK: Self = Self(140);
        pub const BC5_UNORM_BLOCK: Self = Self(141);
        pub const BC5_SNORM_BLOCK: Self = Self(142);
        pub const BC6H_UFLOAT_BLOCK: Self = Self(143);
        pub const BC6H_SFLOAT_BLOCK: Self = Self(144);
        pub const BC7_UNORM_BLOCK: Self = Self(145);
        pub const BC7_SRGB_BLOCK: Self = Self(146);
        pub const ETC2_R8G8B8_UNORM_BLOCK: Self = Self(147);
        pub const ETC2_R8G8B8_SRGB_BLOCK: Self = Self(148);
        pub const ETC2_R8G8B8A1_UNORM_BLOCK: Self = Self(149);
        pub const ETC2_R8G8B8A1_SRGB_BLOCK: Self = Self(150);
        pub const ETC2_R8G8B8A8_UNORM_BLOCK: Self = Self(151);
        pub const ETC2_R8G8B8A8_SRGB_BLOCK: Self = Self(152);
        pub const EAC_R11_UNORM_BLOCK: Self = Self(153);
        pub const EAC_R11_SNORM_BLOCK: Self = Self(154);
        pub const EAC_R11G11_UNORM_BLOCK: Self = Self(155);
        pub const EAC_R11G11_SNORM_BLOCK: Self = Self(156);
        pub const ASTC_4X4_UNORM_BLOCK: Self = Self(157);
        pub const ASTC_4X4_SRGB_BLOCK: Self = Self(158);
        pub const ASTC_5X4_UNORM_BLOCK: Self = Self(159);
        pub const ASTC_5X4_SRGB_BLOCK: Self = Self(160);
        pub const ASTC_5X5_UNORM_BLOCK: Self = Self(161);
        pub const ASTC_5X5_SRGB_BLOCK: Self = Self(162);
        pub const ASTC_6X5_UNORM_BLOCK: Self = Self(163);
        pub const ASTC_6X5_SRGB_BLOCK: Self = Self(164);
        pub const ASTC_6X6_UNORM_BLOCK: Self = Self(165);
        pub const ASTC_6X6_SRGB_BLOCK: Self = Self(166);
        pub const ASTC_8X5_UNORM_BLOCK: Self = Self(167);
        pub const ASTC_8X5_SRGB_BLOCK: Self = Self(168);
        pub const ASTC_8X6_UNORM_BLOCK: Self = Self(169);
        pub const ASTC_8X6_SRGB_BLOCK: Self = Self(170);
        pub const ASTC_8X8_UNORM_BLOCK: Self = Self(171);
        pub const ASTC_8X8_SRGB_BLOCK: Self = Self(172);
        pub const ASTC_10X5_UNORM_BLOCK: Self = Self(173);
        pub const ASTC_10X5_SRGB_BLOCK: Self = Self(174);
        pub const ASTC_10X6_UNORM_BLOCK: Self = Self(175);
        pub const ASTC_10X6_SRGB_BLOCK: Self = Self(176);
        pub const ASTC_10X8_UNORM_BLOCK: Self = Self(177);
        pub const ASTC_10X8_SRGB_BLOCK: Self = Self(178);
        pub const ASTC_10X10_UNORM_BLOCK: Self = Self(179);
        pub const ASTC_10X10_SRGB_BLOCK: Self = Self(180);
        pub const ASTC_12X10_UNORM_BLOCK: Self = Self(181);
        pub const ASTC_12X10_SRGB_BLOCK: Self = Self(182);
        pub const ASTC_12X12_UNORM_BLOCK: Self = Self(183);
        pub const ASTC_12X12_SRGB_BLOCK: Self = Self(184);
        pub const A1B5G5R5_UNORM_PACK16: Self = Self(1000470000);
        pub const A4B4G4R4_UNORM_PACK16: Self = Self(1000340001);
        pub const A4R4G4B4_UNORM_PACK16: Self = Self(1000340000);
        pub const A8_UNORM: Self = Self(1000470001);
        pub const ASTC_10X10_SFLOAT_BLOCK: Self = Self(1000066011);
        pub const ASTC_10X5_SFLOAT_BLOCK: Self = Self(1000066008);
        pub const ASTC_10X6_SFLOAT_BLOCK: Self = Self(1000066009);
        pub const ASTC_10X8_SFLOAT_BLOCK: Self = Self(1000066010);
        pub const ASTC_12X10_SFLOAT_BLOCK: Self = Self(1000066012);
        pub const ASTC_12X12_SFLOAT_BLOCK: Self = Self(1000066013);
        pub const ASTC_3X3X3_SFLOAT_BLOCK_EXT: Self = Self(1000288002);
        pub const ASTC_3X3X3_SRGB_BLOCK_EXT: Self = Self(1000288001);
        pub const ASTC_3X3X3_UNORM_BLOCK_EXT: Self = Self(1000288000);
        pub const ASTC_4X3X3_SFLOAT_BLOCK_EXT: Self = Self(1000288005);
        pub const ASTC_4X3X3_SRGB_BLOCK_EXT: Self = Self(1000288004);
        pub const ASTC_4X3X3_UNORM_BLOCK_EXT: Self = Self(1000288003);
        pub const ASTC_4X4_SFLOAT_BLOCK: Self = Self(1000066000);
        pub const ASTC_4X4X3_SFLOAT_BLOCK_EXT: Self = Self(1000288008);
        pub const ASTC_4X4X3_SRGB_BLOCK_EXT: Self = Self(1000288007);
        pub const ASTC_4X4X3_UNORM_BLOCK_EXT: Self = Self(1000288006);
        pub const ASTC_4X4X4_SFLOAT_BLOCK_EXT: Self = Self(1000288011);
        pub const ASTC_4X4X4_SRGB_BLOCK_EXT: Self = Self(1000288010);
        pub const ASTC_4X4X4_UNORM_BLOCK_EXT: Self = Self(1000288009);
        pub const ASTC_5X4_SFLOAT_BLOCK: Self = Self(1000066001);
        pub const ASTC_5X4X4_SFLOAT_BLOCK_EXT: Self = Self(1000288014);
        pub const ASTC_5X4X4_SRGB_BLOCK_EXT: Self = Self(1000288013);
        pub const ASTC_5X4X4_UNORM_BLOCK_EXT: Self = Self(1000288012);
        pub const ASTC_5X5_SFLOAT_BLOCK: Self = Self(1000066002);
        pub const ASTC_5X5X4_SFLOAT_BLOCK_EXT: Self = Self(1000288017);
        pub const ASTC_5X5X4_SRGB_BLOCK_EXT: Self = Self(1000288016);
        pub const ASTC_5X5X4_UNORM_BLOCK_EXT: Self = Self(1000288015);
        pub const ASTC_5X5X5_SFLOAT_BLOCK_EXT: Self = Self(1000288020);
        pub const ASTC_5X5X5_SRGB_BLOCK_EXT: Self = Self(1000288019);
        pub const ASTC_5X5X5_UNORM_BLOCK_EXT: Self = Self(1000288018);
        pub const ASTC_6X5_SFLOAT_BLOCK: Self = Self(1000066003);
        pub const ASTC_6X5X5_SFLOAT_BLOCK_EXT: Self = Self(1000288023);
        pub const ASTC_6X5X5_SRGB_BLOCK_EXT: Self = Self(1000288022);
        pub const ASTC_6X5X5_UNORM_BLOCK_EXT: Self = Self(1000288021);
        pub const ASTC_6X6_SFLOAT_BLOCK: Self = Self(1000066004);
        pub const ASTC_6X6X5_SFLOAT_BLOCK_EXT: Self = Self(1000288026);
        pub const ASTC_6X6X5_SRGB_BLOCK_EXT: Self = Self(1000288025);
        pub const ASTC_6X6X5_UNORM_BLOCK_EXT: Self = Self(1000288024);
        pub const ASTC_6X6X6_SFLOAT_BLOCK_EXT: Self = Self(1000288029);
        pub const ASTC_6X6X6_SRGB_BLOCK_EXT: Self = Self(1000288028);
        pub const ASTC_6X6X6_UNORM_BLOCK_EXT: Self = Self(1000288027);
        pub const ASTC_8X5_SFLOAT_BLOCK: Self = Self(1000066005);
        pub const ASTC_8X6_SFLOAT_BLOCK: Self = Self(1000066006);
        pub const ASTC_8X8_SFLOAT_BLOCK: Self = Self(1000066007);
        pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: Self = Self(1000156011);
        pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: Self = Self(1000156021);
        pub const B16G16R16G16_422_UNORM: Self = Self(1000156028);
        pub const B8G8R8G8_422_UNORM: Self = Self(1000156001);
        pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: Self = Self(1000156010);
        pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: Self = Self(1000156013);
        pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: Self = Self(1000156015);
        pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16: Self = Self(1000330001);
        pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: Self = Self(1000156012);
        pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: Self = Self(1000156014);
        pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: Self = Self(1000156016);
        pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: Self = Self(1000156020);
        pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: Self = Self(1000156023);
        pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: Self = Self(1000156025);
        pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16: Self = Self(1000330002);
        pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: Self = Self(1000156022);
        pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: Self = Self(1000156024);
        pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: Self = Self(1000156026);
        pub const G14X2_B14X2R14X2_2PLANE_420_UNORM_3PACK16_ARM: Self = Self(1000609012);
        pub const G14X2_B14X2R14X2_2PLANE_422_UNORM_3PACK16_ARM: Self = Self(1000609013);
        pub const G16B16G16R16_422_UNORM: Self = Self(1000156027);
        pub const G16_B16R16_2PLANE_420_UNORM: Self = Self(1000156030);
        pub const G16_B16R16_2PLANE_422_UNORM: Self = Self(1000156032);
        pub const G16_B16R16_2PLANE_444_UNORM: Self = Self(1000330003);
        pub const G16_B16_R16_3PLANE_420_UNORM: Self = Self(1000156029);
        pub const G16_B16_R16_3PLANE_422_UNORM: Self = Self(1000156031);
        pub const G16_B16_R16_3PLANE_444_UNORM: Self = Self(1000156033);
        pub const G8B8G8R8_422_UNORM: Self = Self(1000156000);
        pub const G8_B8R8_2PLANE_420_UNORM: Self = Self(1000156003);
        pub const G8_B8R8_2PLANE_422_UNORM: Self = Self(1000156005);
        pub const G8_B8R8_2PLANE_444_UNORM: Self = Self(1000330000);
        pub const G8_B8_R8_3PLANE_420_UNORM: Self = Self(1000156002);
        pub const G8_B8_R8_3PLANE_422_UNORM: Self = Self(1000156004);
        pub const G8_B8_R8_3PLANE_444_UNORM: Self = Self(1000156006);
        pub const PVRTC1_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054004);
        pub const PVRTC1_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054000);
        pub const PVRTC1_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054005);
        pub const PVRTC1_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054001);
        pub const PVRTC2_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054006);
        pub const PVRTC2_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054002);
        pub const PVRTC2_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054007);
        pub const PVRTC2_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054003);
        pub const R10X6G10X6B10X6A10X6_UINT_4PACK16_ARM: Self = Self(1000609002);
        pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16: Self = Self(1000156009);
        pub const R10X6G10X6_UINT_2PACK16_ARM: Self = Self(1000609001);
        pub const R10X6G10X6_UNORM_2PACK16: Self = Self(1000156008);
        pub const R10X6_UINT_PACK16_ARM: Self = Self(1000609000);
        pub const R10X6_UNORM_PACK16: Self = Self(1000156007);
        pub const R12X4G12X4B12X4A12X4_UINT_4PACK16_ARM: Self = Self(1000609005);
        pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16: Self = Self(1000156019);
        pub const R12X4G12X4_UINT_2PACK16_ARM: Self = Self(1000609004);
        pub const R12X4G12X4_UNORM_2PACK16: Self = Self(1000156018);
        pub const R12X4_UINT_PACK16_ARM: Self = Self(1000609003);
        pub const R12X4_UNORM_PACK16: Self = Self(1000156017);
        pub const R14X2G14X2B14X2A14X2_UINT_4PACK16_ARM: Self = Self(1000609008);
        pub const R14X2G14X2B14X2A14X2_UNORM_4PACK16_ARM: Self = Self(1000609011);
        pub const R14X2G14X2_UINT_2PACK16_ARM: Self = Self(1000609007);
        pub const R14X2G14X2_UNORM_2PACK16_ARM: Self = Self(1000609010);
        pub const R14X2_UINT_PACK16_ARM: Self = Self(1000609006);
        pub const R14X2_UNORM_PACK16_ARM: Self = Self(1000609009);
        pub const R16G16_SFIXED5_NV: Self = Self(1000464000);
        pub const R8_BOOL_ARM: Self = Self(1000460000);
        pub const A1B5G5R5_UNORM_PACK16_KHR: Self = Self::A1B5G5R5_UNORM_PACK16;
        pub const A4B4G4R4_UNORM_PACK16_EXT: Self = Self::A4B4G4R4_UNORM_PACK16;
        pub const A4R4G4B4_UNORM_PACK16_EXT: Self = Self::A4R4G4B4_UNORM_PACK16;
        pub const A8_UNORM_KHR: Self = Self::A8_UNORM;
        pub const ASTC_10X10_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X10_SFLOAT_BLOCK;
        pub const ASTC_10X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X5_SFLOAT_BLOCK;
        pub const ASTC_10X6_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X6_SFLOAT_BLOCK;
        pub const ASTC_10X8_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X8_SFLOAT_BLOCK;
        pub const ASTC_12X10_SFLOAT_BLOCK_EXT: Self = Self::ASTC_12X10_SFLOAT_BLOCK;
        pub const ASTC_12X12_SFLOAT_BLOCK_EXT: Self = Self::ASTC_12X12_SFLOAT_BLOCK;
        pub const ASTC_4X4_SFLOAT_BLOCK_EXT: Self = Self::ASTC_4X4_SFLOAT_BLOCK;
        pub const ASTC_5X4_SFLOAT_BLOCK_EXT: Self = Self::ASTC_5X4_SFLOAT_BLOCK;
        pub const ASTC_5X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_5X5_SFLOAT_BLOCK;
        pub const ASTC_6X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_6X5_SFLOAT_BLOCK;
        pub const ASTC_6X6_SFLOAT_BLOCK_EXT: Self = Self::ASTC_6X6_SFLOAT_BLOCK;
        pub const ASTC_8X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_8X5_SFLOAT_BLOCK;
        pub const ASTC_8X6_SFLOAT_BLOCK_EXT: Self = Self::ASTC_8X6_SFLOAT_BLOCK;
        pub const ASTC_8X8_SFLOAT_BLOCK_EXT: Self = Self::ASTC_8X8_SFLOAT_BLOCK;
        pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR: Self =
            Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16;
        pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR: Self =
            Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16;
        pub const B16G16R16G16_422_UNORM_KHR: Self = Self::B16G16R16G16_422_UNORM;
        pub const B8G8R8G8_422_UNORM_KHR: Self = Self::B8G8R8G8_422_UNORM;
        pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR: Self =
            Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16;
        pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR: Self =
            Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16;
        pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR: Self =
            Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16;
        pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT: Self =
            Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16;
        pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR: Self =
            Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16;
        pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR: Self =
            Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16;
        pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR: Self =
            Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16;
        pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR: Self =
            Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16;
        pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR: Self =
            Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16;
        pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR: Self =
            Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16;
        pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT: Self =
            Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16;
        pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR: Self =
            Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16;
        pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR: Self =
            Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16;
        pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR: Self =
            Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16;
        pub const G16B16G16R16_422_UNORM_KHR: Self = Self::G16B16G16R16_422_UNORM;
        pub const G16_B16R16_2PLANE_420_UNORM_KHR: Self = Self::G16_B16R16_2PLANE_420_UNORM;
        pub const G16_B16R16_2PLANE_422_UNORM_KHR: Self = Self::G16_B16R16_2PLANE_422_UNORM;
        pub const G16_B16R16_2PLANE_444_UNORM_EXT: Self = Self::G16_B16R16_2PLANE_444_UNORM;
        pub const G16_B16_R16_3PLANE_420_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_420_UNORM;
        pub const G16_B16_R16_3PLANE_422_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_422_UNORM;
        pub const G16_B16_R16_3PLANE_444_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_444_UNORM;
        pub const G8B8G8R8_422_UNORM_KHR: Self = Self::G8B8G8R8_422_UNORM;
        pub const G8_B8R8_2PLANE_420_UNORM_KHR: Self = Self::G8_B8R8_2PLANE_420_UNORM;
        pub const G8_B8R8_2PLANE_422_UNORM_KHR: Self = Self::G8_B8R8_2PLANE_422_UNORM;
        pub const G8_B8R8_2PLANE_444_UNORM_EXT: Self = Self::G8_B8R8_2PLANE_444_UNORM;
        pub const G8_B8_R8_3PLANE_420_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_420_UNORM;
        pub const G8_B8_R8_3PLANE_422_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_422_UNORM;
        pub const G8_B8_R8_3PLANE_444_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_444_UNORM;
        pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR: Self =
            Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16;
        pub const R10X6G10X6_UNORM_2PACK16_KHR: Self = Self::R10X6G10X6_UNORM_2PACK16;
        pub const R10X6_UNORM_PACK16_KHR: Self = Self::R10X6_UNORM_PACK16;
        pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR: Self =
            Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16;
        pub const R12X4G12X4_UNORM_2PACK16_KHR: Self = Self::R12X4G12X4_UNORM_2PACK16;
        pub const R12X4_UNORM_PACK16_KHR: Self = Self::R12X4_UNORM_PACK16;
        pub const R16G16_S10_5_NV: Self = Self::R16G16_SFIXED5_NV;
    }
    impl fmt::Debug for Format {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::UNDEFINED => Some("UNDEFINED"),
                Self::R4G4_UNORM_PACK8 => Some("R4G4_UNORM_PACK8"),
                Self::R4G4B4A4_UNORM_PACK16 => Some("R4G4B4A4_UNORM_PACK16"),
                Self::B4G4R4A4_UNORM_PACK16 => Some("B4G4R4A4_UNORM_PACK16"),
                Self::R5G6B5_UNORM_PACK16 => Some("R5G6B5_UNORM_PACK16"),
                Self::B5G6R5_UNORM_PACK16 => Some("B5G6R5_UNORM_PACK16"),
                Self::R5G5B5A1_UNORM_PACK16 => Some("R5G5B5A1_UNORM_PACK16"),
                Self::B5G5R5A1_UNORM_PACK16 => Some("B5G5R5A1_UNORM_PACK16"),
                Self::A1R5G5B5_UNORM_PACK16 => Some("A1R5G5B5_UNORM_PACK16"),
                Self::R8_UNORM => Some("R8_UNORM"),
                Self::R8_SNORM => Some("R8_SNORM"),
                Self::R8_USCALED => Some("R8_USCALED"),
                Self::R8_SSCALED => Some("R8_SSCALED"),
                Self::R8_UINT => Some("R8_UINT"),
                Self::R8_SINT => Some("R8_SINT"),
                Self::R8_SRGB => Some("R8_SRGB"),
                Self::R8G8_UNORM => Some("R8G8_UNORM"),
                Self::R8G8_SNORM => Some("R8G8_SNORM"),
                Self::R8G8_USCALED => Some("R8G8_USCALED"),
                Self::R8G8_SSCALED => Some("R8G8_SSCALED"),
                Self::R8G8_UINT => Some("R8G8_UINT"),
                Self::R8G8_SINT => Some("R8G8_SINT"),
                Self::R8G8_SRGB => Some("R8G8_SRGB"),
                Self::R8G8B8_UNORM => Some("R8G8B8_UNORM"),
                Self::R8G8B8_SNORM => Some("R8G8B8_SNORM"),
                Self::R8G8B8_USCALED => Some("R8G8B8_USCALED"),
                Self::R8G8B8_SSCALED => Some("R8G8B8_SSCALED"),
                Self::R8G8B8_UINT => Some("R8G8B8_UINT"),
                Self::R8G8B8_SINT => Some("R8G8B8_SINT"),
                Self::R8G8B8_SRGB => Some("R8G8B8_SRGB"),
                Self::B8G8R8_UNORM => Some("B8G8R8_UNORM"),
                Self::B8G8R8_SNORM => Some("B8G8R8_SNORM"),
                Self::B8G8R8_USCALED => Some("B8G8R8_USCALED"),
                Self::B8G8R8_SSCALED => Some("B8G8R8_SSCALED"),
                Self::B8G8R8_UINT => Some("B8G8R8_UINT"),
                Self::B8G8R8_SINT => Some("B8G8R8_SINT"),
                Self::B8G8R8_SRGB => Some("B8G8R8_SRGB"),
                Self::R8G8B8A8_UNORM => Some("R8G8B8A8_UNORM"),
                Self::R8G8B8A8_SNORM => Some("R8G8B8A8_SNORM"),
                Self::R8G8B8A8_USCALED => Some("R8G8B8A8_USCALED"),
                Self::R8G8B8A8_SSCALED => Some("R8G8B8A8_SSCALED"),
                Self::R8G8B8A8_UINT => Some("R8G8B8A8_UINT"),
                Self::R8G8B8A8_SINT => Some("R8G8B8A8_SINT"),
                Self::R8G8B8A8_SRGB => Some("R8G8B8A8_SRGB"),
                Self::B8G8R8A8_UNORM => Some("B8G8R8A8_UNORM"),
                Self::B8G8R8A8_SNORM => Some("B8G8R8A8_SNORM"),
                Self::B8G8R8A8_USCALED => Some("B8G8R8A8_USCALED"),
                Self::B8G8R8A8_SSCALED => Some("B8G8R8A8_SSCALED"),
                Self::B8G8R8A8_UINT => Some("B8G8R8A8_UINT"),
                Self::B8G8R8A8_SINT => Some("B8G8R8A8_SINT"),
                Self::B8G8R8A8_SRGB => Some("B8G8R8A8_SRGB"),
                Self::A8B8G8R8_UNORM_PACK32 => Some("A8B8G8R8_UNORM_PACK32"),
                Self::A8B8G8R8_SNORM_PACK32 => Some("A8B8G8R8_SNORM_PACK32"),
                Self::A8B8G8R8_USCALED_PACK32 => Some("A8B8G8R8_USCALED_PACK32"),
                Self::A8B8G8R8_SSCALED_PACK32 => Some("A8B8G8R8_SSCALED_PACK32"),
                Self::A8B8G8R8_UINT_PACK32 => Some("A8B8G8R8_UINT_PACK32"),
                Self::A8B8G8R8_SINT_PACK32 => Some("A8B8G8R8_SINT_PACK32"),
                Self::A8B8G8R8_SRGB_PACK32 => Some("A8B8G8R8_SRGB_PACK32"),
                Self::A2R10G10B10_UNORM_PACK32 => Some("A2R10G10B10_UNORM_PACK32"),
                Self::A2R10G10B10_SNORM_PACK32 => Some("A2R10G10B10_SNORM_PACK32"),
                Self::A2R10G10B10_USCALED_PACK32 => Some("A2R10G10B10_USCALED_PACK32"),
                Self::A2R10G10B10_SSCALED_PACK32 => Some("A2R10G10B10_SSCALED_PACK32"),
                Self::A2R10G10B10_UINT_PACK32 => Some("A2R10G10B10_UINT_PACK32"),
                Self::A2R10G10B10_SINT_PACK32 => Some("A2R10G10B10_SINT_PACK32"),
                Self::A2B10G10R10_UNORM_PACK32 => Some("A2B10G10R10_UNORM_PACK32"),
                Self::A2B10G10R10_SNORM_PACK32 => Some("A2B10G10R10_SNORM_PACK32"),
                Self::A2B10G10R10_USCALED_PACK32 => Some("A2B10G10R10_USCALED_PACK32"),
                Self::A2B10G10R10_SSCALED_PACK32 => Some("A2B10G10R10_SSCALED_PACK32"),
                Self::A2B10G10R10_UINT_PACK32 => Some("A2B10G10R10_UINT_PACK32"),
                Self::A2B10G10R10_SINT_PACK32 => Some("A2B10G10R10_SINT_PACK32"),
                Self::R16_UNORM => Some("R16_UNORM"),
                Self::R16_SNORM => Some("R16_SNORM"),
                Self::R16_USCALED => Some("R16_USCALED"),
                Self::R16_SSCALED => Some("R16_SSCALED"),
                Self::R16_UINT => Some("R16_UINT"),
                Self::R16_SINT => Some("R16_SINT"),
                Self::R16_SFLOAT => Some("R16_SFLOAT"),
                Self::R16G16_UNORM => Some("R16G16_UNORM"),
                Self::R16G16_SNORM => Some("R16G16_SNORM"),
                Self::R16G16_USCALED => Some("R16G16_USCALED"),
                Self::R16G16_SSCALED => Some("R16G16_SSCALED"),
                Self::R16G16_UINT => Some("R16G16_UINT"),
                Self::R16G16_SINT => Some("R16G16_SINT"),
                Self::R16G16_SFLOAT => Some("R16G16_SFLOAT"),
                Self::R16G16B16_UNORM => Some("R16G16B16_UNORM"),
                Self::R16G16B16_SNORM => Some("R16G16B16_SNORM"),
                Self::R16G16B16_USCALED => Some("R16G16B16_USCALED"),
                Self::R16G16B16_SSCALED => Some("R16G16B16_SSCALED"),
                Self::R16G16B16_UINT => Some("R16G16B16_UINT"),
                Self::R16G16B16_SINT => Some("R16G16B16_SINT"),
                Self::R16G16B16_SFLOAT => Some("R16G16B16_SFLOAT"),
                Self::R16G16B16A16_UNORM => Some("R16G16B16A16_UNORM"),
                Self::R16G16B16A16_SNORM => Some("R16G16B16A16_SNORM"),
                Self::R16G16B16A16_USCALED => Some("R16G16B16A16_USCALED"),
                Self::R16G16B16A16_SSCALED => Some("R16G16B16A16_SSCALED"),
                Self::R16G16B16A16_UINT => Some("R16G16B16A16_UINT"),
                Self::R16G16B16A16_SINT => Some("R16G16B16A16_SINT"),
                Self::R16G16B16A16_SFLOAT => Some("R16G16B16A16_SFLOAT"),
                Self::R32_UINT => Some("R32_UINT"),
                Self::R32_SINT => Some("R32_SINT"),
                Self::R32_SFLOAT => Some("R32_SFLOAT"),
                Self::R32G32_UINT => Some("R32G32_UINT"),
                Self::R32G32_SINT => Some("R32G32_SINT"),
                Self::R32G32_SFLOAT => Some("R32G32_SFLOAT"),
                Self::R32G32B32_UINT => Some("R32G32B32_UINT"),
                Self::R32G32B32_SINT => Some("R32G32B32_SINT"),
                Self::R32G32B32_SFLOAT => Some("R32G32B32_SFLOAT"),
                Self::R32G32B32A32_UINT => Some("R32G32B32A32_UINT"),
                Self::R32G32B32A32_SINT => Some("R32G32B32A32_SINT"),
                Self::R32G32B32A32_SFLOAT => Some("R32G32B32A32_SFLOAT"),
                Self::R64_UINT => Some("R64_UINT"),
                Self::R64_SINT => Some("R64_SINT"),
                Self::R64_SFLOAT => Some("R64_SFLOAT"),
                Self::R64G64_UINT => Some("R64G64_UINT"),
                Self::R64G64_SINT => Some("R64G64_SINT"),
                Self::R64G64_SFLOAT => Some("R64G64_SFLOAT"),
                Self::R64G64B64_UINT => Some("R64G64B64_UINT"),
                Self::R64G64B64_SINT => Some("R64G64B64_SINT"),
                Self::R64G64B64_SFLOAT => Some("R64G64B64_SFLOAT"),
                Self::R64G64B64A64_UINT => Some("R64G64B64A64_UINT"),
                Self::R64G64B64A64_SINT => Some("R64G64B64A64_SINT"),
                Self::R64G64B64A64_SFLOAT => Some("R64G64B64A64_SFLOAT"),
                Self::B10G11R11_UFLOAT_PACK32 => Some("B10G11R11_UFLOAT_PACK32"),
                Self::E5B9G9R9_UFLOAT_PACK32 => Some("E5B9G9R9_UFLOAT_PACK32"),
                Self::D16_UNORM => Some("D16_UNORM"),
                Self::X8_D24_UNORM_PACK32 => Some("X8_D24_UNORM_PACK32"),
                Self::D32_SFLOAT => Some("D32_SFLOAT"),
                Self::S8_UINT => Some("S8_UINT"),
                Self::D16_UNORM_S8_UINT => Some("D16_UNORM_S8_UINT"),
                Self::D24_UNORM_S8_UINT => Some("D24_UNORM_S8_UINT"),
                Self::D32_SFLOAT_S8_UINT => Some("D32_SFLOAT_S8_UINT"),
                Self::BC1_RGB_UNORM_BLOCK => Some("BC1_RGB_UNORM_BLOCK"),
                Self::BC1_RGB_SRGB_BLOCK => Some("BC1_RGB_SRGB_BLOCK"),
                Self::BC1_RGBA_UNORM_BLOCK => Some("BC1_RGBA_UNORM_BLOCK"),
                Self::BC1_RGBA_SRGB_BLOCK => Some("BC1_RGBA_SRGB_BLOCK"),
                Self::BC2_UNORM_BLOCK => Some("BC2_UNORM_BLOCK"),
                Self::BC2_SRGB_BLOCK => Some("BC2_SRGB_BLOCK"),
                Self::BC3_UNORM_BLOCK => Some("BC3_UNORM_BLOCK"),
                Self::BC3_SRGB_BLOCK => Some("BC3_SRGB_BLOCK"),
                Self::BC4_UNORM_BLOCK => Some("BC4_UNORM_BLOCK"),
                Self::BC4_SNORM_BLOCK => Some("BC4_SNORM_BLOCK"),
                Self::BC5_UNORM_BLOCK => Some("BC5_UNORM_BLOCK"),
                Self::BC5_SNORM_BLOCK => Some("BC5_SNORM_BLOCK"),
                Self::BC6H_UFLOAT_BLOCK => Some("BC6H_UFLOAT_BLOCK"),
                Self::BC6H_SFLOAT_BLOCK => Some("BC6H_SFLOAT_BLOCK"),
                Self::BC7_UNORM_BLOCK => Some("BC7_UNORM_BLOCK"),
                Self::BC7_SRGB_BLOCK => Some("BC7_SRGB_BLOCK"),
                Self::ETC2_R8G8B8_UNORM_BLOCK => Some("ETC2_R8G8B8_UNORM_BLOCK"),
                Self::ETC2_R8G8B8_SRGB_BLOCK => Some("ETC2_R8G8B8_SRGB_BLOCK"),
                Self::ETC2_R8G8B8A1_UNORM_BLOCK => Some("ETC2_R8G8B8A1_UNORM_BLOCK"),
                Self::ETC2_R8G8B8A1_SRGB_BLOCK => Some("ETC2_R8G8B8A1_SRGB_BLOCK"),
                Self::ETC2_R8G8B8A8_UNORM_BLOCK => Some("ETC2_R8G8B8A8_UNORM_BLOCK"),
                Self::ETC2_R8G8B8A8_SRGB_BLOCK => Some("ETC2_R8G8B8A8_SRGB_BLOCK"),
                Self::EAC_R11_UNORM_BLOCK => Some("EAC_R11_UNORM_BLOCK"),
                Self::EAC_R11_SNORM_BLOCK => Some("EAC_R11_SNORM_BLOCK"),
                Self::EAC_R11G11_UNORM_BLOCK => Some("EAC_R11G11_UNORM_BLOCK"),
                Self::EAC_R11G11_SNORM_BLOCK => Some("EAC_R11G11_SNORM_BLOCK"),
                Self::ASTC_4X4_UNORM_BLOCK => Some("ASTC_4X4_UNORM_BLOCK"),
                Self::ASTC_4X4_SRGB_BLOCK => Some("ASTC_4X4_SRGB_BLOCK"),
                Self::ASTC_5X4_UNORM_BLOCK => Some("ASTC_5X4_UNORM_BLOCK"),
                Self::ASTC_5X4_SRGB_BLOCK => Some("ASTC_5X4_SRGB_BLOCK"),
                Self::ASTC_5X5_UNORM_BLOCK => Some("ASTC_5X5_UNORM_BLOCK"),
                Self::ASTC_5X5_SRGB_BLOCK => Some("ASTC_5X5_SRGB_BLOCK"),
                Self::ASTC_6X5_UNORM_BLOCK => Some("ASTC_6X5_UNORM_BLOCK"),
                Self::ASTC_6X5_SRGB_BLOCK => Some("ASTC_6X5_SRGB_BLOCK"),
                Self::ASTC_6X6_UNORM_BLOCK => Some("ASTC_6X6_UNORM_BLOCK"),
                Self::ASTC_6X6_SRGB_BLOCK => Some("ASTC_6X6_SRGB_BLOCK"),
                Self::ASTC_8X5_UNORM_BLOCK => Some("ASTC_8X5_UNORM_BLOCK"),
                Self::ASTC_8X5_SRGB_BLOCK => Some("ASTC_8X5_SRGB_BLOCK"),
                Self::ASTC_8X6_UNORM_BLOCK => Some("ASTC_8X6_UNORM_BLOCK"),
                Self::ASTC_8X6_SRGB_BLOCK => Some("ASTC_8X6_SRGB_BLOCK"),
                Self::ASTC_8X8_UNORM_BLOCK => Some("ASTC_8X8_UNORM_BLOCK"),
                Self::ASTC_8X8_SRGB_BLOCK => Some("ASTC_8X8_SRGB_BLOCK"),
                Self::ASTC_10X5_UNORM_BLOCK => Some("ASTC_10X5_UNORM_BLOCK"),
                Self::ASTC_10X5_SRGB_BLOCK => Some("ASTC_10X5_SRGB_BLOCK"),
                Self::ASTC_10X6_UNORM_BLOCK => Some("ASTC_10X6_UNORM_BLOCK"),
                Self::ASTC_10X6_SRGB_BLOCK => Some("ASTC_10X6_SRGB_BLOCK"),
                Self::ASTC_10X8_UNORM_BLOCK => Some("ASTC_10X8_UNORM_BLOCK"),
                Self::ASTC_10X8_SRGB_BLOCK => Some("ASTC_10X8_SRGB_BLOCK"),
                Self::ASTC_10X10_UNORM_BLOCK => Some("ASTC_10X10_UNORM_BLOCK"),
                Self::ASTC_10X10_SRGB_BLOCK => Some("ASTC_10X10_SRGB_BLOCK"),
                Self::ASTC_12X10_UNORM_BLOCK => Some("ASTC_12X10_UNORM_BLOCK"),
                Self::ASTC_12X10_SRGB_BLOCK => Some("ASTC_12X10_SRGB_BLOCK"),
                Self::ASTC_12X12_UNORM_BLOCK => Some("ASTC_12X12_UNORM_BLOCK"),
                Self::ASTC_12X12_SRGB_BLOCK => Some("ASTC_12X12_SRGB_BLOCK"),
                Self::A1B5G5R5_UNORM_PACK16 => Some("A1B5G5R5_UNORM_PACK16"),
                Self::A4B4G4R4_UNORM_PACK16 => Some("A4B4G4R4_UNORM_PACK16"),
                Self::A4R4G4B4_UNORM_PACK16 => Some("A4R4G4B4_UNORM_PACK16"),
                Self::A8_UNORM => Some("A8_UNORM"),
                Self::ASTC_10X10_SFLOAT_BLOCK => Some("ASTC_10X10_SFLOAT_BLOCK"),
                Self::ASTC_10X5_SFLOAT_BLOCK => Some("ASTC_10X5_SFLOAT_BLOCK"),
                Self::ASTC_10X6_SFLOAT_BLOCK => Some("ASTC_10X6_SFLOAT_BLOCK"),
                Self::ASTC_10X8_SFLOAT_BLOCK => Some("ASTC_10X8_SFLOAT_BLOCK"),
                Self::ASTC_12X10_SFLOAT_BLOCK => Some("ASTC_12X10_SFLOAT_BLOCK"),
                Self::ASTC_12X12_SFLOAT_BLOCK => Some("ASTC_12X12_SFLOAT_BLOCK"),
                Self::ASTC_3X3X3_SFLOAT_BLOCK_EXT => Some("ASTC_3X3X3_SFLOAT_BLOCK_EXT"),
                Self::ASTC_3X3X3_SRGB_BLOCK_EXT => Some("ASTC_3X3X3_SRGB_BLOCK_EXT"),
                Self::ASTC_3X3X3_UNORM_BLOCK_EXT => Some("ASTC_3X3X3_UNORM_BLOCK_EXT"),
                Self::ASTC_4X3X3_SFLOAT_BLOCK_EXT => Some("ASTC_4X3X3_SFLOAT_BLOCK_EXT"),
                Self::ASTC_4X3X3_SRGB_BLOCK_EXT => Some("ASTC_4X3X3_SRGB_BLOCK_EXT"),
                Self::ASTC_4X3X3_UNORM_BLOCK_EXT => Some("ASTC_4X3X3_UNORM_BLOCK_EXT"),
                Self::ASTC_4X4_SFLOAT_BLOCK => Some("ASTC_4X4_SFLOAT_BLOCK"),
                Self::ASTC_4X4X3_SFLOAT_BLOCK_EXT => Some("ASTC_4X4X3_SFLOAT_BLOCK_EXT"),
                Self::ASTC_4X4X3_SRGB_BLOCK_EXT => Some("ASTC_4X4X3_SRGB_BLOCK_EXT"),
                Self::ASTC_4X4X3_UNORM_BLOCK_EXT => Some("ASTC_4X4X3_UNORM_BLOCK_EXT"),
                Self::ASTC_4X4X4_SFLOAT_BLOCK_EXT => Some("ASTC_4X4X4_SFLOAT_BLOCK_EXT"),
                Self::ASTC_4X4X4_SRGB_BLOCK_EXT => Some("ASTC_4X4X4_SRGB_BLOCK_EXT"),
                Self::ASTC_4X4X4_UNORM_BLOCK_EXT => Some("ASTC_4X4X4_UNORM_BLOCK_EXT"),
                Self::ASTC_5X4_SFLOAT_BLOCK => Some("ASTC_5X4_SFLOAT_BLOCK"),
                Self::ASTC_5X4X4_SFLOAT_BLOCK_EXT => Some("ASTC_5X4X4_SFLOAT_BLOCK_EXT"),
                Self::ASTC_5X4X4_SRGB_BLOCK_EXT => Some("ASTC_5X4X4_SRGB_BLOCK_EXT"),
                Self::ASTC_5X4X4_UNORM_BLOCK_EXT => Some("ASTC_5X4X4_UNORM_BLOCK_EXT"),
                Self::ASTC_5X5_SFLOAT_BLOCK => Some("ASTC_5X5_SFLOAT_BLOCK"),
                Self::ASTC_5X5X4_SFLOAT_BLOCK_EXT => Some("ASTC_5X5X4_SFLOAT_BLOCK_EXT"),
                Self::ASTC_5X5X4_SRGB_BLOCK_EXT => Some("ASTC_5X5X4_SRGB_BLOCK_EXT"),
                Self::ASTC_5X5X4_UNORM_BLOCK_EXT => Some("ASTC_5X5X4_UNORM_BLOCK_EXT"),
                Self::ASTC_5X5X5_SFLOAT_BLOCK_EXT => Some("ASTC_5X5X5_SFLOAT_BLOCK_EXT"),
                Self::ASTC_5X5X5_SRGB_BLOCK_EXT => Some("ASTC_5X5X5_SRGB_BLOCK_EXT"),
                Self::ASTC_5X5X5_UNORM_BLOCK_EXT => Some("ASTC_5X5X5_UNORM_BLOCK_EXT"),
                Self::ASTC_6X5_SFLOAT_BLOCK => Some("ASTC_6X5_SFLOAT_BLOCK"),
                Self::ASTC_6X5X5_SFLOAT_BLOCK_EXT => Some("ASTC_6X5X5_SFLOAT_BLOCK_EXT"),
                Self::ASTC_6X5X5_SRGB_BLOCK_EXT => Some("ASTC_6X5X5_SRGB_BLOCK_EXT"),
                Self::ASTC_6X5X5_UNORM_BLOCK_EXT => Some("ASTC_6X5X5_UNORM_BLOCK_EXT"),
                Self::ASTC_6X6_SFLOAT_BLOCK => Some("ASTC_6X6_SFLOAT_BLOCK"),
                Self::ASTC_6X6X5_SFLOAT_BLOCK_EXT => Some("ASTC_6X6X5_SFLOAT_BLOCK_EXT"),
                Self::ASTC_6X6X5_SRGB_BLOCK_EXT => Some("ASTC_6X6X5_SRGB_BLOCK_EXT"),
                Self::ASTC_6X6X5_UNORM_BLOCK_EXT => Some("ASTC_6X6X5_UNORM_BLOCK_EXT"),
                Self::ASTC_6X6X6_SFLOAT_BLOCK_EXT => Some("ASTC_6X6X6_SFLOAT_BLOCK_EXT"),
                Self::ASTC_6X6X6_SRGB_BLOCK_EXT => Some("ASTC_6X6X6_SRGB_BLOCK_EXT"),
                Self::ASTC_6X6X6_UNORM_BLOCK_EXT => Some("ASTC_6X6X6_UNORM_BLOCK_EXT"),
                Self::ASTC_8X5_SFLOAT_BLOCK => Some("ASTC_8X5_SFLOAT_BLOCK"),
                Self::ASTC_8X6_SFLOAT_BLOCK => Some("ASTC_8X6_SFLOAT_BLOCK"),
                Self::ASTC_8X8_SFLOAT_BLOCK => Some("ASTC_8X8_SFLOAT_BLOCK"),
                Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => {
                    Some("B10X6G10X6R10X6G10X6_422_UNORM_4PACK16")
                }
                Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => {
                    Some("B12X4G12X4R12X4G12X4_422_UNORM_4PACK16")
                }
                Self::B16G16R16G16_422_UNORM => Some("B16G16R16G16_422_UNORM"),
                Self::B8G8R8G8_422_UNORM => Some("B8G8R8G8_422_UNORM"),
                Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => {
                    Some("G10X6B10X6G10X6R10X6_422_UNORM_4PACK16")
                }
                Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => {
                    Some("G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16")
                }
                Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => {
                    Some("G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16")
                }
                Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16 => {
                    Some("G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16")
                }
                Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => {
                    Some("G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16")
                }
                Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => {
                    Some("G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16")
                }
                Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => {
                    Some("G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16")
                }
                Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => {
                    Some("G12X4B12X4G12X4R12X4_422_UNORM_4PACK16")
                }
                Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => {
                    Some("G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16")
                }
                Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => {
                    Some("G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16")
                }
                Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16 => {
                    Some("G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16")
                }
                Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => {
                    Some("G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16")
                }
                Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => {
                    Some("G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16")
                }
                Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => {
                    Some("G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16")
                }
                Self::G14X2_B14X2R14X2_2PLANE_420_UNORM_3PACK16_ARM => {
                    Some("G14X2_B14X2R14X2_2PLANE_420_UNORM_3PACK16_ARM")
                }
                Self::G14X2_B14X2R14X2_2PLANE_422_UNORM_3PACK16_ARM => {
                    Some("G14X2_B14X2R14X2_2PLANE_422_UNORM_3PACK16_ARM")
                }
                Self::G16B16G16R16_422_UNORM => Some("G16B16G16R16_422_UNORM"),
                Self::G16_B16R16_2PLANE_420_UNORM => Some("G16_B16R16_2PLANE_420_UNORM"),
                Self::G16_B16R16_2PLANE_422_UNORM => Some("G16_B16R16_2PLANE_422_UNORM"),
                Self::G16_B16R16_2PLANE_444_UNORM => Some("G16_B16R16_2PLANE_444_UNORM"),
                Self::G16_B16_R16_3PLANE_420_UNORM => Some("G16_B16_R16_3PLANE_420_UNORM"),
                Self::G16_B16_R16_3PLANE_422_UNORM => Some("G16_B16_R16_3PLANE_422_UNORM"),
                Self::G16_B16_R16_3PLANE_444_UNORM => Some("G16_B16_R16_3PLANE_444_UNORM"),
                Self::G8B8G8R8_422_UNORM => Some("G8B8G8R8_422_UNORM"),
                Self::G8_B8R8_2PLANE_420_UNORM => Some("G8_B8R8_2PLANE_420_UNORM"),
                Self::G8_B8R8_2PLANE_422_UNORM => Some("G8_B8R8_2PLANE_422_UNORM"),
                Self::G8_B8R8_2PLANE_444_UNORM => Some("G8_B8R8_2PLANE_444_UNORM"),
                Self::G8_B8_R8_3PLANE_420_UNORM => Some("G8_B8_R8_3PLANE_420_UNORM"),
                Self::G8_B8_R8_3PLANE_422_UNORM => Some("G8_B8_R8_3PLANE_422_UNORM"),
                Self::G8_B8_R8_3PLANE_444_UNORM => Some("G8_B8_R8_3PLANE_444_UNORM"),
                Self::PVRTC1_2BPP_SRGB_BLOCK_IMG => Some("PVRTC1_2BPP_SRGB_BLOCK_IMG"),
                Self::PVRTC1_2BPP_UNORM_BLOCK_IMG => Some("PVRTC1_2BPP_UNORM_BLOCK_IMG"),
                Self::PVRTC1_4BPP_SRGB_BLOCK_IMG => Some("PVRTC1_4BPP_SRGB_BLOCK_IMG"),
                Self::PVRTC1_4BPP_UNORM_BLOCK_IMG => Some("PVRTC1_4BPP_UNORM_BLOCK_IMG"),
                Self::PVRTC2_2BPP_SRGB_BLOCK_IMG => Some("PVRTC2_2BPP_SRGB_BLOCK_IMG"),
                Self::PVRTC2_2BPP_UNORM_BLOCK_IMG => Some("PVRTC2_2BPP_UNORM_BLOCK_IMG"),
                Self::PVRTC2_4BPP_SRGB_BLOCK_IMG => Some("PVRTC2_4BPP_SRGB_BLOCK_IMG"),
                Self::PVRTC2_4BPP_UNORM_BLOCK_IMG => Some("PVRTC2_4BPP_UNORM_BLOCK_IMG"),
                Self::R10X6G10X6B10X6A10X6_UINT_4PACK16_ARM => {
                    Some("R10X6G10X6B10X6A10X6_UINT_4PACK16_ARM")
                }
                Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => {
                    Some("R10X6G10X6B10X6A10X6_UNORM_4PACK16")
                }
                Self::R10X6G10X6_UINT_2PACK16_ARM => Some("R10X6G10X6_UINT_2PACK16_ARM"),
                Self::R10X6G10X6_UNORM_2PACK16 => Some("R10X6G10X6_UNORM_2PACK16"),
                Self::R10X6_UINT_PACK16_ARM => Some("R10X6_UINT_PACK16_ARM"),
                Self::R10X6_UNORM_PACK16 => Some("R10X6_UNORM_PACK16"),
                Self::R12X4G12X4B12X4A12X4_UINT_4PACK16_ARM => {
                    Some("R12X4G12X4B12X4A12X4_UINT_4PACK16_ARM")
                }
                Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => {
                    Some("R12X4G12X4B12X4A12X4_UNORM_4PACK16")
                }
                Self::R12X4G12X4_UINT_2PACK16_ARM => Some("R12X4G12X4_UINT_2PACK16_ARM"),
                Self::R12X4G12X4_UNORM_2PACK16 => Some("R12X4G12X4_UNORM_2PACK16"),
                Self::R12X4_UINT_PACK16_ARM => Some("R12X4_UINT_PACK16_ARM"),
                Self::R12X4_UNORM_PACK16 => Some("R12X4_UNORM_PACK16"),
                Self::R14X2G14X2B14X2A14X2_UINT_4PACK16_ARM => {
                    Some("R14X2G14X2B14X2A14X2_UINT_4PACK16_ARM")
                }
                Self::R14X2G14X2B14X2A14X2_UNORM_4PACK16_ARM => {
                    Some("R14X2G14X2B14X2A14X2_UNORM_4PACK16_ARM")
                }
                Self::R14X2G14X2_UINT_2PACK16_ARM => Some("R14X2G14X2_UINT_2PACK16_ARM"),
                Self::R14X2G14X2_UNORM_2PACK16_ARM => Some("R14X2G14X2_UNORM_2PACK16_ARM"),
                Self::R14X2_UINT_PACK16_ARM => Some("R14X2_UINT_PACK16_ARM"),
                Self::R14X2_UNORM_PACK16_ARM => Some("R14X2_UNORM_PACK16_ARM"),
                Self::R16G16_SFIXED5_NV => Some("R16G16_SFIXED5_NV"),
                Self::R8_BOOL_ARM => Some("R8_BOOL_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StructureType(i32);
    impl StructureType {
        pub const APPLICATION_INFO: Self = Self(0);
        pub const INSTANCE_CREATE_INFO: Self = Self(1);
        pub const DEVICE_QUEUE_CREATE_INFO: Self = Self(2);
        pub const DEVICE_CREATE_INFO: Self = Self(3);
        pub const SUBMIT_INFO: Self = Self(4);
        pub const MEMORY_ALLOCATE_INFO: Self = Self(5);
        pub const MAPPED_MEMORY_RANGE: Self = Self(6);
        pub const BIND_SPARSE_INFO: Self = Self(7);
        pub const FENCE_CREATE_INFO: Self = Self(8);
        pub const SEMAPHORE_CREATE_INFO: Self = Self(9);
        pub const EVENT_CREATE_INFO: Self = Self(10);
        pub const QUERY_POOL_CREATE_INFO: Self = Self(11);
        pub const BUFFER_CREATE_INFO: Self = Self(12);
        pub const BUFFER_VIEW_CREATE_INFO: Self = Self(13);
        pub const IMAGE_CREATE_INFO: Self = Self(14);
        pub const IMAGE_VIEW_CREATE_INFO: Self = Self(15);
        pub const SHADER_MODULE_CREATE_INFO: Self = Self(16);
        pub const PIPELINE_CACHE_CREATE_INFO: Self = Self(17);
        pub const PIPELINE_SHADER_STAGE_CREATE_INFO: Self = Self(18);
        pub const PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: Self = Self(19);
        pub const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: Self = Self(20);
        pub const PIPELINE_TESSELLATION_STATE_CREATE_INFO: Self = Self(21);
        pub const PIPELINE_VIEWPORT_STATE_CREATE_INFO: Self = Self(22);
        pub const PIPELINE_RASTERIZATION_STATE_CREATE_INFO: Self = Self(23);
        pub const PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: Self = Self(24);
        pub const PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: Self = Self(25);
        pub const PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: Self = Self(26);
        pub const PIPELINE_DYNAMIC_STATE_CREATE_INFO: Self = Self(27);
        pub const GRAPHICS_PIPELINE_CREATE_INFO: Self = Self(28);
        pub const COMPUTE_PIPELINE_CREATE_INFO: Self = Self(29);
        pub const PIPELINE_LAYOUT_CREATE_INFO: Self = Self(30);
        pub const SAMPLER_CREATE_INFO: Self = Self(31);
        pub const DESCRIPTOR_SET_LAYOUT_CREATE_INFO: Self = Self(32);
        pub const DESCRIPTOR_POOL_CREATE_INFO: Self = Self(33);
        pub const DESCRIPTOR_SET_ALLOCATE_INFO: Self = Self(34);
        pub const WRITE_DESCRIPTOR_SET: Self = Self(35);
        pub const COPY_DESCRIPTOR_SET: Self = Self(36);
        pub const FRAMEBUFFER_CREATE_INFO: Self = Self(37);
        pub const RENDER_PASS_CREATE_INFO: Self = Self(38);
        pub const COMMAND_POOL_CREATE_INFO: Self = Self(39);
        pub const COMMAND_BUFFER_ALLOCATE_INFO: Self = Self(40);
        pub const COMMAND_BUFFER_INHERITANCE_INFO: Self = Self(41);
        pub const COMMAND_BUFFER_BEGIN_INFO: Self = Self(42);
        pub const RENDER_PASS_BEGIN_INFO: Self = Self(43);
        pub const BUFFER_MEMORY_BARRIER: Self = Self(44);
        pub const IMAGE_MEMORY_BARRIER: Self = Self(45);
        pub const MEMORY_BARRIER: Self = Self(46);
        pub const LOADER_INSTANCE_CREATE_INFO: Self = Self(47);
        pub const LOADER_DEVICE_CREATE_INFO: Self = Self(48);
        pub const ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR: Self = Self(1000150000);
        pub const ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR: Self = Self(1000150020);
        pub const ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316009);
        pub const ACCELERATION_STRUCTURE_CREATE_INFO_KHR: Self = Self(1000150017);
        pub const ACCELERATION_STRUCTURE_CREATE_INFO_NV: Self = Self(1000165001);
        pub const ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX: Self =
            Self(1000478001);
        pub const ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR: Self = Self(1000150002);
        pub const ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR: Self = Self(1000150003);
        pub const ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR: Self = Self(1000150004);
        pub const ACCELERATION_STRUCTURE_GEOMETRY_KHR: Self = Self(1000150006);
        pub const ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV: Self =
            Self(1000429009);
        pub const ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV: Self = Self(1000327000);
        pub const ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV: Self = Self(1000429010);
        pub const ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR: Self = Self(1000150005);
        pub const ACCELERATION_STRUCTURE_INFO_NV: Self = Self(1000165012);
        pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1000165008);
        pub const ACCELERATION_STRUCTURE_MOTION_INFO_NV: Self = Self(1000327002);
        pub const ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV: Self =
            Self(1000397002);
        pub const ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT: Self = Self(1000396009);
        pub const ACCELERATION_STRUCTURE_VERSION_INFO_KHR: Self = Self(1000150009);
        pub const ACQUIRE_NEXT_IMAGE_INFO_KHR: Self = Self(1000060010);
        pub const ACQUIRE_PROFILING_LOCK_INFO_KHR: Self = Self(1000116004);
        pub const AMIGO_PROFILING_SUBMIT_INFO_SEC: Self = Self(1000485001);
        pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID: Self = Self(1000129006);
        pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID: Self = Self(1000129002);
        pub const ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID: Self =
            Self(1000468002);
        pub const ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID: Self = Self(1000129001);
        pub const ANDROID_HARDWARE_BUFFER_USAGE_ANDROID: Self = Self(1000129000);
        pub const ANDROID_SURFACE_CREATE_INFO_KHR: Self = Self(1000008000);
        pub const ANTI_LAG_DATA_AMD: Self = Self(1000476001);
        pub const ANTI_LAG_PRESENTATION_INFO_AMD: Self = Self(1000476002);
        pub const ATTACHMENT_DESCRIPTION_2: Self = Self(1000109000);
        pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT: Self = Self(1000241002);
        pub const ATTACHMENT_FEEDBACK_LOOP_INFO_EXT: Self = Self(1000527001);
        pub const ATTACHMENT_REFERENCE_2: Self = Self(1000109001);
        pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT: Self = Self(1000241001);
        pub const ATTACHMENT_SAMPLE_COUNT_INFO_AMD: Self = Self(1000044008);
        pub const BEGIN_CUSTOM_RESOLVE_INFO_EXT: Self = Self(1000628001);
        pub const BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV: Self = Self(1000165006);
        pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: Self = Self(1000060013);
        pub const BIND_BUFFER_MEMORY_INFO: Self = Self(1000157000);
        pub const BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_INFO_ARM: Self = Self(1000507005);
        pub const BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT: Self = Self(1000545008);
        pub const BIND_DESCRIPTOR_SETS_INFO: Self = Self(1000545003);
        pub const BIND_HEAP_INFO_EXT: Self = Self(1000135003);
        pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: Self = Self(1000060014);
        pub const BIND_IMAGE_MEMORY_INFO: Self = Self(1000157001);
        pub const BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR: Self = Self(1000060009);
        pub const BIND_IMAGE_PLANE_MEMORY_INFO: Self = Self(1000156002);
        pub const BIND_MEMORY_STATUS: Self = Self(1000545002);
        pub const BIND_TENSOR_MEMORY_INFO_ARM: Self = Self(1000460002);
        pub const BIND_VIDEO_SESSION_MEMORY_INFO_KHR: Self = Self(1000023004);
        pub const BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM: Self = Self(1000519002);
        pub const BLIT_IMAGE_INFO_2: Self = Self(1000337004);
        pub const BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316005);
        pub const BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA: Self = Self(1000366005);
        pub const BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366009);
        pub const BUFFER_COLLECTION_CREATE_INFO_FUCHSIA: Self = Self(1000366000);
        pub const BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA: Self = Self(1000366002);
        pub const BUFFER_COLLECTION_PROPERTIES_FUCHSIA: Self = Self(1000366003);
        pub const BUFFER_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366004);
        pub const BUFFER_COPY_2: Self = Self(1000337006);
        pub const BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT: Self = Self(1000244002);
        pub const BUFFER_DEVICE_ADDRESS_INFO: Self = Self(1000244001);
        pub const BUFFER_IMAGE_COPY_2: Self = Self(1000337009);
        pub const BUFFER_MEMORY_BARRIER_2: Self = Self(1000314001);
        pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146000);
        pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO: Self = Self(1000257002);
        pub const BUFFER_USAGE_FLAGS_2_CREATE_INFO: Self = Self(1000470006);
        pub const BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV: Self = Self(1000570004);
        pub const CALIBRATED_TIMESTAMP_INFO_KHR: Self = Self(1000184000);
        pub const CHECKPOINT_DATA_2_NV: Self = Self(1000314009);
        pub const CHECKPOINT_DATA_NV: Self = Self(1000206000);
        pub const CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV: Self =
            Self(1000569002);
        pub const CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV: Self = Self(1000569006);
        pub const CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV: Self = Self(1000569005);
        pub const CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV: Self = Self(1000569004);
        pub const CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV: Self = Self(1000569003);
        pub const COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT: Self =
            Self(1000081000);
        pub const COMMAND_BUFFER_INHERITANCE_DESCRIPTOR_HEAP_INFO_EXT: Self = Self(1000135010);
        pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO: Self = Self(1000044004);
        pub const COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM: Self =
            Self(1000282000);
        pub const COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV: Self = Self(1000278001);
        pub const COMMAND_BUFFER_SUBMIT_INFO: Self = Self(1000314006);
        pub const COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV: Self = Self(1000645000);
        pub const COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV: Self = Self(1000428001);
        pub const CONDITIONAL_RENDERING_BEGIN_INFO_EXT: Self = Self(1000081002);
        pub const CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV: Self = Self(1000491004);
        pub const COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV: Self = Self(1000593001);
        pub const COOPERATIVE_MATRIX_PROPERTIES_KHR: Self = Self(1000506001);
        pub const COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1000249001);
        pub const COOPERATIVE_VECTOR_PROPERTIES_NV: Self = Self(1000491002);
        pub const COPY_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1000150010);
        pub const COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR: Self = Self(1000150011);
        pub const COPY_BUFFER_INFO_2: Self = Self(1000337000);
        pub const COPY_BUFFER_TO_IMAGE_INFO_2: Self = Self(1000337002);
        pub const COPY_COMMAND_TRANSFORM_INFO_QCOM: Self = Self(1000333000);
        pub const COPY_IMAGE_INFO_2: Self = Self(1000337001);
        pub const COPY_IMAGE_TO_BUFFER_INFO_2: Self = Self(1000337003);
        pub const COPY_IMAGE_TO_IMAGE_INFO: Self = Self(1000270007);
        pub const COPY_IMAGE_TO_MEMORY_INFO: Self = Self(1000270004);
        pub const COPY_MEMORY_INDIRECT_INFO_KHR: Self = Self(1000549002);
        pub const COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1000150012);
        pub const COPY_MEMORY_TO_IMAGE_INDIRECT_INFO_KHR: Self = Self(1000549003);
        pub const COPY_MEMORY_TO_IMAGE_INFO: Self = Self(1000270005);
        pub const COPY_MEMORY_TO_MICROMAP_INFO_EXT: Self = Self(1000396004);
        pub const COPY_MICROMAP_INFO_EXT: Self = Self(1000396002);
        pub const COPY_MICROMAP_TO_MEMORY_INFO_EXT: Self = Self(1000396003);
        pub const COPY_TENSOR_INFO_ARM: Self = Self(1000460011);
        pub const CUDA_FUNCTION_CREATE_INFO_NV: Self = Self(1000307001);
        pub const CUDA_LAUNCH_INFO_NV: Self = Self(1000307002);
        pub const CUDA_MODULE_CREATE_INFO_NV: Self = Self(1000307000);
        pub const CUSTOM_RESOLVE_CREATE_INFO_EXT: Self = Self(1000628002);
        pub const CU_FUNCTION_CREATE_INFO_NVX: Self = Self(1000029001);
        pub const CU_LAUNCH_INFO_NVX: Self = Self(1000029002);
        pub const CU_MODULE_CREATE_INFO_NVX: Self = Self(1000029000);
        pub const CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX: Self = Self(1000029004);
        pub const D3D12_FENCE_SUBMIT_INFO_KHR: Self = Self(1000078002);
        pub const DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM: Self = Self(1000629001);
        pub const DATA_GRAPH_PIPELINE_COMPILER_CONTROL_CREATE_INFO_ARM: Self = Self(1000507010);
        pub const DATA_GRAPH_PIPELINE_CONSTANT_ARM: Self = Self(1000507003);
        pub const DATA_GRAPH_PIPELINE_CONSTANT_TENSOR_SEMI_STRUCTURED_SPARSITY_INFO_ARM: Self =
            Self(1000507015);
        pub const DATA_GRAPH_PIPELINE_CREATE_INFO_ARM: Self = Self(1000507000);
        pub const DATA_GRAPH_PIPELINE_DISPATCH_INFO_ARM: Self = Self(1000507014);
        pub const DATA_GRAPH_PIPELINE_IDENTIFIER_CREATE_INFO_ARM: Self = Self(1000507013);
        pub const DATA_GRAPH_PIPELINE_INFO_ARM: Self = Self(1000507009);
        pub const DATA_GRAPH_PIPELINE_PROPERTY_QUERY_RESULT_ARM: Self = Self(1000507008);
        pub const DATA_GRAPH_PIPELINE_RESOURCE_INFO_ARM: Self = Self(1000507002);
        pub const DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_INFO_ARM: Self =
            Self(1000507011);
        pub const DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENT_ARM: Self = Self(1000507012);
        pub const DATA_GRAPH_PIPELINE_SESSION_CREATE_INFO_ARM: Self = Self(1000507001);
        pub const DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_INFO_ARM: Self = Self(1000507004);
        pub const DATA_GRAPH_PIPELINE_SHADER_MODULE_CREATE_INFO_ARM: Self = Self(1000507007);
        pub const DATA_GRAPH_PROCESSING_ENGINE_CREATE_INFO_ARM: Self = Self(1000507016);
        pub const DEBUG_MARKER_MARKER_INFO_EXT: Self = Self(1000022002);
        pub const DEBUG_MARKER_OBJECT_NAME_INFO_EXT: Self = Self(1000022000);
        pub const DEBUG_MARKER_OBJECT_TAG_INFO_EXT: Self = Self(1000022001);
        pub const DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT: Self = Self(1000011000);
        pub const DEBUG_UTILS_LABEL_EXT: Self = Self(1000128002);
        pub const DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: Self = Self(1000128003);
        pub const DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: Self = Self(1000128004);
        pub const DEBUG_UTILS_OBJECT_NAME_INFO_EXT: Self = Self(1000128000);
        pub const DEBUG_UTILS_OBJECT_TAG_INFO_EXT: Self = Self(1000128001);
        pub const DECOMPRESS_MEMORY_INFO_EXT: Self = Self(1000550002);
        pub const DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV: Self = Self(1000026001);
        pub const DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV: Self = Self(1000026000);
        pub const DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV: Self = Self(1000026002);
        pub const DEPENDENCY_INFO: Self = Self(1000314003);
        pub const DEPTH_BIAS_INFO_EXT: Self = Self(1000283001);
        pub const DEPTH_BIAS_REPRESENTATION_INFO_EXT: Self = Self(1000283002);
        pub const DESCRIPTOR_ADDRESS_INFO_EXT: Self = Self(1000316003);
        pub const DESCRIPTOR_BUFFER_BINDING_INFO_EXT: Self = Self(1000316011);
        pub const DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT: Self =
            Self(1000316012);
        pub const DESCRIPTOR_GET_INFO_EXT: Self = Self(1000316004);
        pub const DESCRIPTOR_GET_TENSOR_INFO_ARM: Self = Self(1000460020);
        pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO: Self = Self(1000138003);
        pub const DESCRIPTOR_SET_AND_BINDING_MAPPING_EXT: Self = Self(1000135005);
        pub const DESCRIPTOR_SET_BINDING_REFERENCE_VALVE: Self = Self(1000420001);
        pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO: Self = Self(1000161000);
        pub const DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE: Self = Self(1000420002);
        pub const DESCRIPTOR_SET_LAYOUT_SUPPORT: Self = Self(1000168001);
        pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO: Self = Self(1000161003);
        pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT: Self = Self(1000161004);
        pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: Self = Self(1000085000);
        pub const DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT: Self = Self(1000354001);
        pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS: Self = Self(1000413002);
        pub const DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT: Self = Self(1000284001);
        pub const DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV: Self = Self(1000300001);
        pub const DEVICE_EVENT_INFO_EXT: Self = Self(1000091001);
        pub const DEVICE_FAULT_COUNTS_EXT: Self = Self(1000341001);
        pub const DEVICE_FAULT_INFO_EXT: Self = Self(1000341002);
        pub const DEVICE_GROUP_BIND_SPARSE_INFO: Self = Self(1000060006);
        pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: Self = Self(1000060004);
        pub const DEVICE_GROUP_DEVICE_CREATE_INFO: Self = Self(1000070001);
        pub const DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: Self = Self(1000060007);
        pub const DEVICE_GROUP_PRESENT_INFO_KHR: Self = Self(1000060011);
        pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: Self = Self(1000060003);
        pub const DEVICE_GROUP_SUBMIT_INFO: Self = Self(1000060005);
        pub const DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000060012);
        pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS: Self = Self(1000413003);
        pub const DEVICE_IMAGE_SUBRESOURCE_INFO: Self = Self(1000470004);
        pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO: Self = Self(1000257004);
        pub const DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD: Self = Self(1000189000);
        pub const DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT: Self = Self(1000284002);
        pub const DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR: Self = Self(1000483008);
        pub const DEVICE_PRIVATE_DATA_CREATE_INFO: Self = Self(1000295001);
        pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO: Self = Self(1000174000);
        pub const DEVICE_QUEUE_INFO_2: Self = Self(1000145003);
        pub const DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM: Self = Self(1000417000);
        pub const DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM: Self = Self(1000460010);
        pub const DIRECTFB_SURFACE_CREATE_INFO_EXT: Self = Self(1000346000);
        pub const DIRECT_DRIVER_LOADING_INFO_LUNARG: Self = Self(1000459000);
        pub const DIRECT_DRIVER_LOADING_LIST_LUNARG: Self = Self(1000459001);
        pub const DISPATCH_TILE_INFO_QCOM: Self = Self(1000309005);
        pub const DISPLAY_EVENT_INFO_EXT: Self = Self(1000091002);
        pub const DISPLAY_MODE_CREATE_INFO_KHR: Self = Self(1000002000);
        pub const DISPLAY_MODE_PROPERTIES_2_KHR: Self = Self(1000121002);
        pub const DISPLAY_MODE_STEREO_PROPERTIES_NV: Self = Self(1000551001);
        pub const DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD: Self = Self(1000213000);
        pub const DISPLAY_PLANE_CAPABILITIES_2_KHR: Self = Self(1000121004);
        pub const DISPLAY_PLANE_INFO_2_KHR: Self = Self(1000121003);
        pub const DISPLAY_PLANE_PROPERTIES_2_KHR: Self = Self(1000121001);
        pub const DISPLAY_POWER_INFO_EXT: Self = Self(1000091000);
        pub const DISPLAY_PRESENT_INFO_KHR: Self = Self(1000003000);
        pub const DISPLAY_PROPERTIES_2_KHR: Self = Self(1000121000);
        pub const DISPLAY_SURFACE_CREATE_INFO_KHR: Self = Self(1000002001);
        pub const DISPLAY_SURFACE_STEREO_CREATE_INFO_NV: Self = Self(1000551000);
        pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT: Self = Self(1000158006);
        pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT: Self = Self(1000158000);
        pub const EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX: Self = Self(1000134003);
        pub const EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX: Self = Self(1000134002);
        pub const EXPORT_FENCE_CREATE_INFO: Self = Self(1000113000);
        pub const EXPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1000114001);
        pub const EXPORT_MEMORY_ALLOCATE_INFO: Self = Self(1000072002);
        pub const EXPORT_MEMORY_ALLOCATE_INFO_NV: Self = Self(1000056001);
        pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1000073001);
        pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1000057001);
        pub const EXPORT_METAL_BUFFER_INFO_EXT: Self = Self(1000311004);
        pub const EXPORT_METAL_COMMAND_QUEUE_INFO_EXT: Self = Self(1000311003);
        pub const EXPORT_METAL_DEVICE_INFO_EXT: Self = Self(1000311002);
        pub const EXPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1000311008);
        pub const EXPORT_METAL_OBJECTS_INFO_EXT: Self = Self(1000311001);
        pub const EXPORT_METAL_OBJECT_CREATE_INFO_EXT: Self = Self(1000311000);
        pub const EXPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1000311010);
        pub const EXPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1000311006);
        pub const EXPORT_SEMAPHORE_CREATE_INFO: Self = Self(1000077000);
        pub const EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1000078001);
        pub const EXTERNAL_BUFFER_PROPERTIES: Self = Self(1000071003);
        pub const EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV: Self = Self(1000556001);
        pub const EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV: Self = Self(1000556002);
        pub const EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV: Self = Self(1000556000);
        pub const EXTERNAL_FENCE_PROPERTIES: Self = Self(1000112001);
        pub const EXTERNAL_FORMAT_ANDROID: Self = Self(1000129005);
        pub const EXTERNAL_FORMAT_OHOS: Self = Self(1000452005);
        pub const EXTERNAL_FORMAT_QNX: Self = Self(1000529003);
        pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES: Self = Self(1000071001);
        pub const EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXT: Self = Self(1000453000);
        pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO: Self = Self(1000072000);
        pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO: Self = Self(1000072001);
        pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV: Self = Self(1000056000);
        pub const EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM: Self = Self(1000460017);
        pub const EXTERNAL_SEMAPHORE_PROPERTIES: Self = Self(1000076001);
        pub const EXTERNAL_TENSOR_PROPERTIES_ARM: Self = Self(1000460016);
        pub const FENCE_GET_FD_INFO_KHR: Self = Self(1000115001);
        pub const FENCE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000114002);
        pub const FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT: Self = Self(1000170001);
        pub const FORMAT_PROPERTIES_2: Self = Self(1000059002);
        pub const FORMAT_PROPERTIES_3: Self = Self(1000360000);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1000226000);
        pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO: Self = Self(1000108001);
        pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO: Self = Self(1000108002);
        pub const FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV: Self = Self(1000250002);
        pub const FRAME_BOUNDARY_EXT: Self = Self(1000375001);
        pub const FRAME_BOUNDARY_TENSORS_ARM: Self = Self(1000460023);
        pub const GENERATED_COMMANDS_INFO_EXT: Self = Self(1000572004);
        pub const GENERATED_COMMANDS_INFO_NV: Self = Self(1000277005);
        pub const GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_EXT: Self = Self(1000572002);
        pub const GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1000277006);
        pub const GENERATED_COMMANDS_PIPELINE_INFO_EXT: Self = Self(1000572013);
        pub const GENERATED_COMMANDS_SHADER_INFO_EXT: Self = Self(1000572014);
        pub const GEOMETRY_AABB_NV: Self = Self(1000165005);
        pub const GEOMETRY_NV: Self = Self(1000165003);
        pub const GEOMETRY_TRIANGLES_NV: Self = Self(1000165004);
        pub const GET_LATENCY_MARKER_INFO_NV: Self = Self(1000505003);
        pub const GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT: Self = Self(1000320002);
        pub const GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV: Self = Self(1000277002);
        pub const GRAPHICS_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1000277001);
        pub const HDR_METADATA_EXT: Self = Self(1000105000);
        pub const HDR_VIVID_DYNAMIC_METADATA_HUAWEI: Self = Self(1000590001);
        pub const HEADLESS_SURFACE_CREATE_INFO_EXT: Self = Self(1000256000);
        pub const HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY: Self = Self(1000270009);
        pub const HOST_IMAGE_LAYOUT_TRANSITION_INFO: Self = Self(1000270006);
        pub const IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA: Self = Self(1000214000);
        pub const IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA: Self = Self(1000575002);
        pub const IMAGE_BLIT_2: Self = Self(1000337008);
        pub const IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316006);
        pub const IMAGE_COMPRESSION_CONTROL_EXT: Self = Self(1000338001);
        pub const IMAGE_COMPRESSION_PROPERTIES_EXT: Self = Self(1000338004);
        pub const IMAGE_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366006);
        pub const IMAGE_COPY_2: Self = Self(1000337007);
        pub const IMAGE_DESCRIPTOR_INFO_EXT: Self = Self(1000135001);
        pub const IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT: Self = Self(1000158004);
        pub const IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT: Self = Self(1000158003);
        pub const IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: Self = Self(1000158005);
        pub const IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366007);
        pub const IMAGE_FORMAT_LIST_CREATE_INFO: Self = Self(1000147000);
        pub const IMAGE_FORMAT_PROPERTIES_2: Self = Self(1000059003);
        pub const IMAGE_MEMORY_BARRIER_2: Self = Self(1000314002);
        pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146001);
        pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: Self = Self(1000156003);
        pub const IMAGE_RESOLVE_2: Self = Self(1000337010);
        pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146002);
        pub const IMAGE_STENCIL_USAGE_CREATE_INFO: Self = Self(1000246000);
        pub const IMAGE_SUBRESOURCE_2: Self = Self(1000338003);
        pub const IMAGE_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000060008);
        pub const IMAGE_TO_MEMORY_COPY: Self = Self(1000270003);
        pub const IMAGE_VIEW_ADDRESS_PROPERTIES_NVX: Self = Self(1000030001);
        pub const IMAGE_VIEW_ASTC_DECODE_MODE_EXT: Self = Self(1000067000);
        pub const IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316007);
        pub const IMAGE_VIEW_HANDLE_INFO_NVX: Self = Self(1000030000);
        pub const IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT: Self = Self(1000391001);
        pub const IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM: Self = Self(1000440002);
        pub const IMAGE_VIEW_SLICED_CREATE_INFO_EXT: Self = Self(1000418001);
        pub const IMAGE_VIEW_USAGE_CREATE_INFO: Self = Self(1000117002);
        pub const IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1000129003);
        pub const IMPORT_FENCE_FD_INFO_KHR: Self = Self(1000115000);
        pub const IMPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1000114000);
        pub const IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366001);
        pub const IMPORT_MEMORY_FD_INFO_KHR: Self = Self(1000074000);
        pub const IMPORT_MEMORY_HOST_POINTER_INFO_EXT: Self = Self(1000178000);
        pub const IMPORT_MEMORY_METAL_HANDLE_INFO_EXT: Self = Self(1000602000);
        pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1000073000);
        pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1000057000);
        pub const IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000364000);
        pub const IMPORT_METAL_BUFFER_INFO_EXT: Self = Self(1000311005);
        pub const IMPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1000311009);
        pub const IMPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1000311011);
        pub const IMPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1000311007);
        pub const IMPORT_NATIVE_BUFFER_INFO_OHOS: Self = Self(1000452003);
        pub const IMPORT_SCREEN_BUFFER_INFO_QNX: Self = Self(1000529002);
        pub const IMPORT_SEMAPHORE_FD_INFO_KHR: Self = Self(1000079000);
        pub const IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1000078000);
        pub const IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000365000);
        pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_EXT: Self = Self(1000572006);
        pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV: Self = Self(1000277004);
        pub const INDIRECT_COMMANDS_LAYOUT_PUSH_DATA_TOKEN_NV: Self = Self(1000135012);
        pub const INDIRECT_COMMANDS_LAYOUT_TOKEN_EXT: Self = Self(1000572007);
        pub const INDIRECT_COMMANDS_LAYOUT_TOKEN_NV: Self = Self(1000277003);
        pub const INDIRECT_EXECUTION_SET_CREATE_INFO_EXT: Self = Self(1000572003);
        pub const INDIRECT_EXECUTION_SET_PIPELINE_INFO_EXT: Self = Self(1000572010);
        pub const INDIRECT_EXECUTION_SET_SHADER_INFO_EXT: Self = Self(1000572011);
        pub const INDIRECT_EXECUTION_SET_SHADER_LAYOUT_INFO_EXT: Self = Self(1000572012);
        pub const INITIALIZE_PERFORMANCE_API_INFO_INTEL: Self = Self(1000210001);
        pub const IOS_SURFACE_CREATE_INFO_MVK: Self = Self(1000122000);
        pub const LATENCY_SLEEP_INFO_NV: Self = Self(1000505001);
        pub const LATENCY_SLEEP_MODE_INFO_NV: Self = Self(1000505000);
        pub const LATENCY_SUBMISSION_PRESENT_ID_NV: Self = Self(1000505005);
        pub const LATENCY_SURFACE_CAPABILITIES_NV: Self = Self(1000505008);
        pub const LATENCY_TIMINGS_FRAME_REPORT_NV: Self = Self(1000505004);
        pub const LAYER_SETTINGS_CREATE_INFO_EXT: Self = Self(1000496000);
        pub const MACOS_SURFACE_CREATE_INFO_MVK: Self = Self(1000123000);
        pub const MEMORY_ALLOCATE_FLAGS_INFO: Self = Self(1000060000);
        pub const MEMORY_BARRIER_2: Self = Self(1000314000);
        pub const MEMORY_BARRIER_ACCESS_FLAGS_3_KHR: Self = Self(1000574002);
        pub const MEMORY_DEDICATED_ALLOCATE_INFO: Self = Self(1000127001);
        pub const MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM: Self = Self(1000460014);
        pub const MEMORY_DEDICATED_REQUIREMENTS: Self = Self(1000127000);
        pub const MEMORY_FD_PROPERTIES_KHR: Self = Self(1000074001);
        pub const MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1000129004);
        pub const MEMORY_GET_FD_INFO_KHR: Self = Self(1000074002);
        pub const MEMORY_GET_METAL_HANDLE_INFO_EXT: Self = Self(1000602002);
        pub const MEMORY_GET_NATIVE_BUFFER_INFO_OHOS: Self = Self(1000452004);
        pub const MEMORY_GET_REMOTE_ADDRESS_INFO_NV: Self = Self(1000371000);
        pub const MEMORY_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000073003);
        pub const MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000364002);
        pub const MEMORY_HOST_POINTER_PROPERTIES_EXT: Self = Self(1000178001);
        pub const MEMORY_MAP_INFO: Self = Self(1000271000);
        pub const MEMORY_MAP_PLACED_INFO_EXT: Self = Self(1000272002);
        pub const MEMORY_METAL_HANDLE_PROPERTIES_EXT: Self = Self(1000602001);
        pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO: Self = Self(1000257003);
        pub const MEMORY_PRIORITY_ALLOCATE_INFO_EXT: Self = Self(1000238001);
        pub const MEMORY_REQUIREMENTS_2: Self = Self(1000146003);
        pub const MEMORY_TO_IMAGE_COPY: Self = Self(1000270002);
        pub const MEMORY_UNMAP_INFO: Self = Self(1000271001);
        pub const MEMORY_WIN32_HANDLE_PROPERTIES_KHR: Self = Self(1000073002);
        pub const MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA: Self = Self(1000364001);
        pub const METAL_SURFACE_CREATE_INFO_EXT: Self = Self(1000217000);
        pub const MICROMAP_BUILD_INFO_EXT: Self = Self(1000396000);
        pub const MICROMAP_BUILD_SIZES_INFO_EXT: Self = Self(1000396008);
        pub const MICROMAP_CREATE_INFO_EXT: Self = Self(1000396007);
        pub const MICROMAP_VERSION_INFO_EXT: Self = Self(1000396001);
        pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT: Self = Self(1000376002);
        pub const MULTISAMPLE_PROPERTIES_EXT: Self = Self(1000143004);
        pub const MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX: Self = Self(1000044009);
        pub const MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM: Self =
            Self(1000510001);
        pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT: Self = Self(1000351002);
        pub const NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS: Self = Self(1000452002);
        pub const NATIVE_BUFFER_PROPERTIES_OHOS: Self = Self(1000452001);
        pub const NATIVE_BUFFER_USAGE_OHOS: Self = Self(1000452000);
        pub const OPAQUE_CAPTURE_DATA_CREATE_INFO_EXT: Self = Self(1000135007);
        pub const OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT: Self = Self(1000316010);
        pub const OPTICAL_FLOW_EXECUTE_INFO_NV: Self = Self(1000464005);
        pub const OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV: Self = Self(1000464002);
        pub const OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV: Self = Self(1000464003);
        pub const OPTICAL_FLOW_SESSION_CREATE_INFO_NV: Self = Self(1000464004);
        pub const OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV: Self = Self(1000464010);
        pub const OUT_OF_BAND_QUEUE_TYPE_INFO_NV: Self = Self(1000505006);
        pub const PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV: Self = Self(1000570005);
        pub const PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV: Self = Self(1000570003);
        pub const PAST_PRESENTATION_TIMING_EXT: Self = Self(1000208007);
        pub const PAST_PRESENTATION_TIMING_INFO_EXT: Self = Self(1000208005);
        pub const PAST_PRESENTATION_TIMING_PROPERTIES_EXT: Self = Self(1000208006);
        pub const PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL: Self = Self(1000210005);
        pub const PERFORMANCE_COUNTER_ARM: Self = Self(1000605002);
        pub const PERFORMANCE_COUNTER_DESCRIPTION_ARM: Self = Self(1000605003);
        pub const PERFORMANCE_COUNTER_DESCRIPTION_KHR: Self = Self(1000116006);
        pub const PERFORMANCE_COUNTER_KHR: Self = Self(1000116005);
        pub const PERFORMANCE_MARKER_INFO_INTEL: Self = Self(1000210002);
        pub const PERFORMANCE_OVERRIDE_INFO_INTEL: Self = Self(1000210004);
        pub const PERFORMANCE_QUERY_SUBMIT_INFO_KHR: Self = Self(1000116003);
        pub const PERFORMANCE_STREAM_MARKER_INFO_INTEL: Self = Self(1000210003);
        pub const PER_TILE_BEGIN_INFO_QCOM: Self = Self(1000309003);
        pub const PER_TILE_END_INFO_QCOM: Self = Self(1000309004);
        pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES: Self = Self(1000083000);
        pub const PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT: Self = Self(1000340000);
        pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES: Self = Self(1000177000);
        pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR: Self = Self(1000150013);
        pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR: Self = Self(1000150014);
        pub const PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT: Self = Self(1000354000);
        pub const PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC: Self = Self(1000485000);
        pub const PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD: Self = Self(1000476000);
        pub const PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT: Self = Self(1000067001);
        pub const PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT: Self =
            Self(1000524000);
        pub const PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT: Self =
            Self(1000339000);
        pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT: Self = Self(1000148000);
        pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT: Self = Self(1000148001);
        pub const PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT: Self = Self(1000411000);
        pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES: Self = Self(1000257000);
        pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT: Self = Self(1000244000);
        pub const PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV: Self =
            Self(1000569000);
        pub const PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV: Self =
            Self(1000569001);
        pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI: Self = Self(1000404000);
        pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI: Self = Self(1000404001);
        pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI: Self =
            Self(1000404002);
        pub const PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD: Self = Self(1000229000);
        pub const PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT: Self = Self(1000381000);
        pub const PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV: Self = Self(1000559000);
        pub const PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV: Self = Self(1000645001);
        pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR: Self = Self(1000201000);
        pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_PROPERTIES_KHR: Self =
            Self(1000511000);
        pub const PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT: Self = Self(1000081001);
        pub const PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT: Self =
            Self(1000101000);
        pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV: Self = Self(1000593000);
        pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV: Self = Self(1000593002);
        pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR: Self = Self(1000506000);
        pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV: Self = Self(1000249000);
        pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR: Self = Self(1000506002);
        pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1000249002);
        pub const PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV: Self = Self(1000491000);
        pub const PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV: Self = Self(1000491001);
        pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_KHR: Self = Self(1000549000);
        pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV: Self = Self(1000426000);
        pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR: Self = Self(1000426001);
        pub const PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV: Self = Self(1000050000);
        pub const PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV: Self = Self(1000250000);
        pub const PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM: Self = Self(1000521000);
        pub const PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM: Self = Self(1000519001);
        pub const PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV: Self = Self(1000307003);
        pub const PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV: Self = Self(1000307004);
        pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT: Self = Self(1000287002);
        pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT: Self = Self(1000287001);
        pub const PHYSICAL_DEVICE_CUSTOM_RESOLVE_FEATURES_EXT: Self = Self(1000628000);
        pub const PHYSICAL_DEVICE_DATA_GRAPH_FEATURES_ARM: Self = Self(1000507006);
        pub const PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM: Self = Self(1000629000);
        pub const PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV: Self =
            Self(1000240000);
        pub const PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX: Self = Self(1000478000);
        pub const PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT: Self = Self(1000283000);
        pub const PHYSICAL_DEVICE_DEPTH_CLAMP_CONTROL_FEATURES_EXT: Self = Self(1000582000);
        pub const PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR: Self = Self(1000421000);
        pub const PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT: Self = Self(1000355000);
        pub const PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT: Self = Self(1000102000);
        pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES: Self = Self(1000199000);
        pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT: Self =
            Self(1000316001);
        pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT: Self = Self(1000316002);
        pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT: Self = Self(1000316000);
        pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM: Self = Self(1000460018);
        pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM: Self = Self(1000460019);
        pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_FEATURES_EXT: Self = Self(1000135009);
        pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_PROPERTIES_EXT: Self = Self(1000135008);
        pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_TENSOR_PROPERTIES_ARM: Self = Self(1000135014);
        pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES: Self = Self(1000161001);
        pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES: Self = Self(1000161002);
        pub const PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV: Self =
            Self(1000546000);
        pub const PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE: Self =
            Self(1000420000);
        pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV: Self =
            Self(1000428000);
        pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_EXT: Self = Self(1000572000);
        pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV: Self = Self(1000277007);
        pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_EXT: Self = Self(1000572001);
        pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV: Self = Self(1000277000);
        pub const PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT: Self = Self(1000284000);
        pub const PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV: Self = Self(1000300000);
        pub const PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT: Self = Self(1000099000);
        pub const PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV: Self = Self(1000397000);
        pub const PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV: Self = Self(1000397001);
        pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES: Self = Self(1000196000);
        pub const PHYSICAL_DEVICE_DRM_PROPERTIES_EXT: Self = Self(1000353000);
        pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES: Self = Self(1000044003);
        pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES: Self = Self(1000232000);
        pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_FEATURES_EXT: Self =
            Self(1000499000);
        pub const PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV: Self = Self(1000205002);
        pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT: Self = Self(1000377000);
        pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT: Self = Self(1000455000);
        pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT: Self = Self(1000455001);
        pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1000267000);
        pub const PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV: Self =
            Self(1000492000);
        pub const PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV: Self =
            Self(1000492001);
        pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: Self = Self(1000071002);
        pub const PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV: Self = Self(1000556003);
        pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: Self = Self(1000112000);
        pub const PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID: Self = Self(1000468000);
        pub const PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID: Self =
            Self(1000468001);
        pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: Self = Self(1000071000);
        pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT: Self = Self(1000178002);
        pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV: Self = Self(1000371001);
        pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX: Self =
            Self(1000529004);
        pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: Self = Self(1000076000);
        pub const PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM: Self = Self(1000460015);
        pub const PHYSICAL_DEVICE_FAULT_FEATURES_EXT: Self = Self(1000341000);
        pub const PHYSICAL_DEVICE_FEATURES_2: Self = Self(1000059000);
        pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES: Self = Self(1000197000);
        pub const PHYSICAL_DEVICE_FORMAT_PACK_FEATURES_ARM: Self = Self(1000609000);
        pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT: Self = Self(1000332000);
        pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT: Self = Self(1000332001);
        pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT: Self = Self(1000218000);
        pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE: Self =
            Self(1000611000);
        pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE: Self =
            Self(1000611001);
        pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT: Self = Self(1000425000);
        pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT: Self =
            Self(1000425001);
        pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT: Self = Self(1000218001);
        pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR: Self = Self(1000203000);
        pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR: Self =
            Self(1000322000);
        pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT: Self = Self(1000251000);
        pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV: Self = Self(1000326001);
        pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV: Self =
            Self(1000326000);
        pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR: Self = Self(1000226003);
        pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR: Self = Self(1000226004);
        pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR: Self = Self(1000226002);
        pub const PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT: Self = Self(1000375000);
        pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES: Self = Self(1000388000);
        pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT: Self = Self(1000320000);
        pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT: Self = Self(1000320001);
        pub const PHYSICAL_DEVICE_GROUP_PROPERTIES: Self = Self(1000070000);
        pub const PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI: Self = Self(1000590000);
        pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES: Self = Self(1000270000);
        pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES: Self = Self(1000270001);
        pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES: Self = Self(1000261000);
        pub const PHYSICAL_DEVICE_ID_PROPERTIES: Self = Self(1000071004);
        pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES: Self = Self(1000108000);
        pub const PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT: Self = Self(1000393000);
        pub const PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA: Self = Self(1000575000);
        pub const PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA: Self = Self(1000575001);
        pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT: Self = Self(1000338000);
        pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT: Self =
            Self(1000437000);
        pub const PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT: Self = Self(1000158002);
        pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: Self = Self(1000059004);
        pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM: Self = Self(1000518000);
        pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM: Self = Self(1000518001);
        pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM: Self = Self(1000440000);
        pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM: Self = Self(1000440001);
        pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES: Self = Self(1000335000);
        pub const PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT: Self = Self(1000418000);
        pub const PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT: Self = Self(1000170000);
        pub const PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT: Self = Self(1000391000);
        pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES: Self = Self(1000265000);
        pub const PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV: Self = Self(1000278000);
        pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES: Self = Self(1000138000);
        pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES: Self = Self(1000138001);
        pub const PHYSICAL_DEVICE_INTERNALLY_SYNCHRONIZED_QUEUES_FEATURES_KHR: Self =
            Self(1000504000);
        pub const PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI: Self = Self(1000370000);
        pub const PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR: Self = Self(1000562003);
        pub const PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR: Self = Self(1000562002);
        pub const PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR: Self = Self(1000562004);
        pub const PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT: Self = Self(1000530000);
        pub const PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT: Self = Self(1000465000);
        pub const PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_FEATURES_EXT: Self = Self(1000495000);
        pub const PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_PROPERTIES_EXT: Self = Self(1000495001);
        pub const PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV: Self = Self(1000430000);
        pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES: Self = Self(1000259000);
        pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES: Self = Self(1000259002);
        pub const PHYSICAL_DEVICE_MAINTENANCE_10_FEATURES_KHR: Self = Self(1000630000);
        pub const PHYSICAL_DEVICE_MAINTENANCE_10_PROPERTIES_KHR: Self = Self(1000630001);
        pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: Self = Self(1000168000);
        pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES: Self = Self(1000413000);
        pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES: Self = Self(1000413001);
        pub const PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES: Self = Self(1000470000);
        pub const PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES: Self = Self(1000470001);
        pub const PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES: Self = Self(1000545000);
        pub const PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES: Self = Self(1000545001);
        pub const PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR: Self = Self(1000562000);
        pub const PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR: Self = Self(1000562001);
        pub const PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR: Self = Self(1000574000);
        pub const PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR: Self = Self(1000584000);
        pub const PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR: Self = Self(1000584001);
        pub const PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT: Self = Self(1000272000);
        pub const PHYSICAL_DEVICE_MAP_MEMORY_PLACED_PROPERTIES_EXT: Self = Self(1000272001);
        pub const PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT: Self = Self(1000237000);
        pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT: Self = Self(1000427000);
        pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT: Self = Self(1000427001);
        pub const PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT: Self = Self(1000238000);
        pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: Self = Self(1000059006);
        pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT: Self = Self(1000328000);
        pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV: Self = Self(1000202000);
        pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT: Self = Self(1000328001);
        pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV: Self = Self(1000202001);
        pub const PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT: Self =
            Self(1000376000);
        pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES: Self = Self(1000053001);
        pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX: Self =
            Self(1000097000);
        pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM: Self =
            Self(1000510000);
        pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM: Self =
            Self(1000488000);
        pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: Self = Self(1000053002);
        pub const PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT: Self = Self(1000392000);
        pub const PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT: Self = Self(1000392001);
        pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT: Self = Self(1000351000);
        pub const PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT: Self = Self(1000451000);
        pub const PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_PROPERTIES_EXT: Self = Self(1000451001);
        pub const PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT: Self = Self(1000422000);
        pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT: Self = Self(1000396005);
        pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT: Self = Self(1000396006);
        pub const PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV: Self = Self(1000464000);
        pub const PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV: Self = Self(1000464001);
        pub const PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT: Self =
            Self(1000412000);
        pub const PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV: Self =
            Self(1000570000);
        pub const PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV: Self =
            Self(1000570001);
        pub const PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT: Self = Self(1000212000);
        pub const PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM: Self =
            Self(1000605000);
        pub const PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM: Self =
            Self(1000605001);
        pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR: Self = Self(1000116000);
        pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR: Self = Self(1000116001);
        pub const PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV: Self = Self(1000516000);
        pub const PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR: Self = Self(1000483000);
        pub const PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR: Self = Self(1000483004);
        pub const PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC: Self =
            Self(1000637000);
        pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES: Self = Self(1000297000);
        pub const PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR: Self =
            Self(1000269000);
        pub const PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT: Self =
            Self(1000498000);
        pub const PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM: Self = Self(1000596000);
        pub const PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT: Self = Self(1000372001);
        pub const PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES: Self = Self(1000466000);
        pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES: Self = Self(1000068001);
        pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES: Self = Self(1000068002);
        pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: Self = Self(1000117000);
        pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR: Self = Self(1000163000);
        pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR: Self = Self(1000163001);
        pub const PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV: Self = Self(1000292000);
        pub const PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR: Self = Self(1000479002);
        pub const PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR: Self = Self(1000294001);
        pub const PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV: Self = Self(1000613001);
        pub const PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR: Self =
            Self(1000361000);
        pub const PHYSICAL_DEVICE_PRESENT_TIMING_FEATURES_EXT: Self = Self(1000208000);
        pub const PHYSICAL_DEVICE_PRESENT_WAIT_2_FEATURES_KHR: Self = Self(1000480001);
        pub const PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR: Self = Self(1000248000);
        pub const PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT: Self = Self(1000382000);
        pub const PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT: Self =
            Self(1000356000);
        pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES: Self = Self(1000295000);
        pub const PHYSICAL_DEVICE_PROPERTIES_2: Self = Self(1000059001);
        pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES: Self = Self(1000145001);
        pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES: Self = Self(1000145002);
        pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT: Self = Self(1000254000);
        pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT: Self = Self(1000254002);
        pub const PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV: Self = Self(1000580001);
        pub const PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV: Self = Self(1000580002);
        pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES: Self = Self(1000080000);
        pub const PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_INFO_ARM: Self =
            Self(1000507019);
        pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT: Self =
            Self(1000342000);
        pub const PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV: Self = Self(1000555000);
        pub const PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR: Self = Self(1000348013);
        pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_EXT: Self =
            Self(1000581000);
        pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV: Self =
            Self(1000490000);
        pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_EXT: Self =
            Self(1000581001);
        pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV: Self =
            Self(1000490001);
        pub const PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV: Self =
            Self(1000429008);
        pub const PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000386000);
        pub const PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV: Self = Self(1000327001);
        pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR: Self = Self(1000347000);
        pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR: Self = Self(1000347001);
        pub const PHYSICAL_DEVICE_RAY_TRACING_POSITION_FETCH_FEATURES_KHR: Self = Self(1000481000);
        pub const PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV: Self = Self(1000165009);
        pub const PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV: Self = Self(1000568000);
        pub const PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG: Self = Self(1000110000);
        pub const PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM: Self = Self(1000424000);
        pub const PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM: Self = Self(1000424001);
        pub const PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV: Self = Self(1000166000);
        pub const PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT: Self = Self(1000344000);
        pub const PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR: Self = Self(1000286000);
        pub const PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR: Self = Self(1000286001);
        pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES: Self = Self(1000130000);
        pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: Self = Self(1000156004);
        pub const PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT: Self = Self(1000143003);
        pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES: Self = Self(1000221000);
        pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM: Self = Self(1000417001);
        pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM: Self = Self(1000417002);
        pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES: Self = Self(1000241000);
        pub const PHYSICAL_DEVICE_SHADER_64_BIT_INDEXING_FEATURES_EXT: Self = Self(1000627000);
        pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV: Self = Self(1000563000);
        pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT: Self = Self(1000273000);
        pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT: Self = Self(1000260000);
        pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES: Self = Self(1000180000);
        pub const PHYSICAL_DEVICE_SHADER_BFLOAT16_FEATURES_KHR: Self = Self(1000141000);
        pub const PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR: Self = Self(1000181000);
        pub const PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM: Self = Self(1000497000);
        pub const PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM: Self = Self(1000497001);
        pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD: Self = Self(1000227000);
        pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD: Self = Self(1000185000);
        pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM: Self = Self(1000415000);
        pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES: Self =
            Self(1000276000);
        pub const PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES: Self = Self(1000063000);
        pub const PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD: Self =
            Self(1000321000);
        pub const PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX: Self = Self(1000134000);
        pub const PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX: Self = Self(1000134001);
        pub const PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES: Self = Self(1000544000);
        pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES: Self = Self(1000082000);
        pub const PHYSICAL_DEVICE_SHADER_FLOAT8_FEATURES_EXT: Self = Self(1000567000);
        pub const PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES: Self = Self(1000528000);
        pub const PHYSICAL_DEVICE_SHADER_FMA_FEATURES_KHR: Self = Self(1000579000);
        pub const PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT: Self = Self(1000234000);
        pub const PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV: Self = Self(1000204000);
        pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES: Self = Self(1000280000);
        pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES: Self = Self(1000280001);
        pub const PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL: Self =
            Self(1000209000);
        pub const PHYSICAL_DEVICE_SHADER_LONG_VECTOR_FEATURES_EXT: Self = Self(1000635000);
        pub const PHYSICAL_DEVICE_SHADER_LONG_VECTOR_PROPERTIES_EXT: Self = Self(1000635001);
        pub const PHYSICAL_DEVICE_SHADER_MAXIMAL_RECONVERGENCE_FEATURES_KHR: Self =
            Self(1000434000);
        pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT: Self = Self(1000462000);
        pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT: Self = Self(1000462001);
        pub const PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT: Self = Self(1000482000);
        pub const PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT: Self = Self(1000482001);
        pub const PHYSICAL_DEVICE_SHADER_QUAD_CONTROL_FEATURES_KHR: Self = Self(1000235000);
        pub const PHYSICAL_DEVICE_SHADER_RELAXED_EXTENDED_INSTRUCTION_FEATURES_KHR: Self =
            Self(1000558000);
        pub const PHYSICAL_DEVICE_SHADER_REPLICATED_COMPOSITES_FEATURES_EXT: Self =
            Self(1000564000);
        pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV: Self = Self(1000154000);
        pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV: Self = Self(1000154001);
        pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES: Self = Self(1000175000);
        pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_PARTITIONED_FEATURES_EXT: Self = Self(1000662000);
        pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES: Self = Self(1000416000);
        pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR: Self =
            Self(1000323000);
        pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES: Self = Self(1000215000);
        pub const PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT: Self = Self(1000395000);
        pub const PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT: Self = Self(1000395001);
        pub const PHYSICAL_DEVICE_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_FEATURES_EXT: Self =
            Self(1000642000);
        pub const PHYSICAL_DEVICE_SHADER_UNTYPED_POINTERS_FEATURES_KHR: Self = Self(1000387000);
        pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV: Self = Self(1000164001);
        pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV: Self = Self(1000164002);
        pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: Self = Self(1000059008);
        pub const PHYSICAL_DEVICE_SUBGROUP_PROPERTIES: Self = Self(1000094000);
        pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES: Self = Self(1000225002);
        pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES: Self = Self(1000225000);
        pub const PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT: Self = Self(1000458000);
        pub const PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI: Self = Self(1000369001);
        pub const PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI: Self = Self(1000369002);
        pub const PHYSICAL_DEVICE_SURFACE_INFO_2_KHR: Self = Self(1000119000);
        pub const PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000275000);
        pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES: Self = Self(1000314007);
        pub const PHYSICAL_DEVICE_TENSOR_FEATURES_ARM: Self = Self(1000460009);
        pub const PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM: Self = Self(1000460004);
        pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT: Self = Self(1000281000);
        pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES: Self = Self(1000281001);
        pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_3D_FEATURES_EXT: Self = Self(1000288000);
        pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES: Self = Self(1000066000);
        pub const PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM: Self = Self(1000547000);
        pub const PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM: Self = Self(1000547001);
        pub const PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM: Self = Self(1000484000);
        pub const PHYSICAL_DEVICE_TILE_SHADING_FEATURES_QCOM: Self = Self(1000309000);
        pub const PHYSICAL_DEVICE_TILE_SHADING_PROPERTIES_QCOM: Self = Self(1000309001);
        pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES: Self = Self(1000207000);
        pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES: Self = Self(1000207001);
        pub const PHYSICAL_DEVICE_TOOL_PROPERTIES: Self = Self(1000245000);
        pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT: Self = Self(1000028000);
        pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT: Self = Self(1000028001);
        pub const PHYSICAL_DEVICE_UNIFIED_IMAGE_LAYOUTS_FEATURES_KHR: Self = Self(1000527000);
        pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES: Self = Self(1000253000);
        pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES: Self = Self(1000120000);
        pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES: Self = Self(1000190002);
        pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES: Self = Self(1000525000);
        pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT: Self = Self(1000190000);
        pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_ROBUSTNESS_FEATURES_EXT: Self = Self(1000608000);
        pub const PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1000352000);
        pub const PHYSICAL_DEVICE_VIDEO_DECODE_VP9_FEATURES_KHR: Self = Self(1000514000);
        pub const PHYSICAL_DEVICE_VIDEO_ENCODE_AV1_FEATURES_KHR: Self = Self(1000513004);
        pub const PHYSICAL_DEVICE_VIDEO_ENCODE_INTRA_REFRESH_FEATURES_KHR: Self = Self(1000552004);
        pub const PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR: Self = Self(1000299006);
        pub const PHYSICAL_DEVICE_VIDEO_ENCODE_QUANTIZATION_MAP_FEATURES_KHR: Self =
            Self(1000553009);
        pub const PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE: Self =
            Self(1000390000);
        pub const PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR: Self = Self(1000023014);
        pub const PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000515000);
        pub const PHYSICAL_DEVICE_VIDEO_MAINTENANCE_2_FEATURES_KHR: Self = Self(1000586000);
        pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES: Self = Self(1000211000);
        pub const PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR: Self =
            Self(1000336000);
        pub const PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT: Self = Self(1000330000);
        pub const PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM: Self = Self(1000520000);
        pub const PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT: Self = Self(1000252000);
        pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_DEVICE_MEMORY_FEATURES_EXT: Self =
            Self(1000620000);
        pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES: Self =
            Self(1000325000);
        pub const PIPELINE_BINARY_CREATE_INFO_KHR: Self = Self(1000483001);
        pub const PIPELINE_BINARY_DATA_INFO_KHR: Self = Self(1000483006);
        pub const PIPELINE_BINARY_HANDLES_INFO_KHR: Self = Self(1000483009);
        pub const PIPELINE_BINARY_INFO_KHR: Self = Self(1000483002);
        pub const PIPELINE_BINARY_KEY_KHR: Self = Self(1000483003);
        pub const PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT: Self = Self(1000148002);
        pub const PIPELINE_COLOR_WRITE_CREATE_INFO_EXT: Self = Self(1000381001);
        pub const PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD: Self = Self(1000183000);
        pub const PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV: Self = Self(1000152000);
        pub const PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV: Self = Self(1000250001);
        pub const PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV: Self = Self(1000149000);
        pub const PIPELINE_CREATE_FLAGS_2_CREATE_INFO: Self = Self(1000470005);
        pub const PIPELINE_CREATE_INFO_KHR: Self = Self(1000483007);
        pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO: Self = Self(1000192000);
        pub const PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT: Self = Self(1000099001);
        pub const PIPELINE_EXECUTABLE_INFO_KHR: Self = Self(1000269003);
        pub const PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR: Self = Self(1000269005);
        pub const PIPELINE_EXECUTABLE_PROPERTIES_KHR: Self = Self(1000269002);
        pub const PIPELINE_EXECUTABLE_STATISTIC_KHR: Self = Self(1000269004);
        pub const PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE: Self = Self(1000611002);
        pub const PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV: Self = Self(1000326002);
        pub const PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR: Self = Self(1000226001);
        pub const PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV: Self = Self(1000428002);
        pub const PIPELINE_INFO_KHR: Self = Self(1000269001);
        pub const PIPELINE_LIBRARY_CREATE_INFO_KHR: Self = Self(1000290000);
        pub const PIPELINE_PROPERTIES_IDENTIFIER_EXT: Self = Self(1000372000);
        pub const PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT: Self =
            Self(1000101001);
        pub const PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT: Self = Self(1000102001);
        pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO: Self = Self(1000259001);
        pub const PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT: Self =
            Self(1000254001);
        pub const PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD: Self = Self(1000018000);
        pub const PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT: Self = Self(1000028002);
        pub const PIPELINE_RENDERING_CREATE_INFO: Self = Self(1000044002);
        pub const PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV: Self =
            Self(1000166001);
        pub const PIPELINE_ROBUSTNESS_CREATE_INFO: Self = Self(1000068000);
        pub const PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT: Self = Self(1000143002);
        pub const PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT: Self = Self(1000462002);
        pub const PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX: Self = Self(1000134004);
        pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO: Self = Self(1000225001);
        pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: Self = Self(1000117003);
        pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO: Self = Self(1000190001);
        pub const PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV: Self =
            Self(1000164005);
        pub const PIPELINE_VIEWPORT_DEPTH_CLAMP_CONTROL_CREATE_INFO_EXT: Self = Self(1000582001);
        pub const PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT: Self = Self(1000355001);
        pub const PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV: Self = Self(1000205000);
        pub const PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV: Self =
            Self(1000164000);
        pub const PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV: Self = Self(1000098000);
        pub const PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV: Self = Self(1000087000);
        pub const PRESENT_FRAME_TOKEN_GGP: Self = Self(1000191000);
        pub const PRESENT_ID_2_KHR: Self = Self(1000479001);
        pub const PRESENT_ID_KHR: Self = Self(1000294000);
        pub const PRESENT_INFO_KHR: Self = Self(1000001001);
        pub const PRESENT_REGIONS_KHR: Self = Self(1000084000);
        pub const PRESENT_TIMES_INFO_GOOGLE: Self = Self(1000092000);
        pub const PRESENT_TIMINGS_INFO_EXT: Self = Self(1000208003);
        pub const PRESENT_TIMING_INFO_EXT: Self = Self(1000208004);
        pub const PRESENT_TIMING_SURFACE_CAPABILITIES_EXT: Self = Self(1000208008);
        pub const PRESENT_WAIT_2_INFO_KHR: Self = Self(1000480002);
        pub const PRIVATE_DATA_SLOT_CREATE_INFO: Self = Self(1000295002);
        pub const PROTECTED_SUBMIT_INFO: Self = Self(1000145000);
        pub const PUSH_CONSTANTS_INFO: Self = Self(1000545004);
        pub const PUSH_CONSTANT_BANK_INFO_NV: Self = Self(1000580000);
        pub const PUSH_DATA_INFO_EXT: Self = Self(1000135004);
        pub const PUSH_DESCRIPTOR_SET_INFO: Self = Self(1000545005);
        pub const PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO: Self = Self(1000545006);
        pub const QUERY_LOW_LATENCY_SUPPORT_NV: Self = Self(1000310000);
        pub const QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR: Self = Self(1000116002);
        pub const QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL: Self = Self(1000210000);
        pub const QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR: Self = Self(1000299005);
        pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV: Self = Self(1000314008);
        pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV: Self = Self(1000206001);
        pub const QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM: Self = Self(1000507017);
        pub const QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM: Self = Self(1000507018);
        pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES: Self = Self(1000388001);
        pub const QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR: Self = Self(1000584002);
        pub const QUEUE_FAMILY_PROPERTIES_2: Self = Self(1000059005);
        pub const QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR: Self = Self(1000023016);
        pub const QUEUE_FAMILY_VIDEO_PROPERTIES_KHR: Self = Self(1000023012);
        pub const RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV: Self =
            Self(1000569007);
        pub const RAY_TRACING_PIPELINE_CREATE_INFO_KHR: Self = Self(1000150015);
        pub const RAY_TRACING_PIPELINE_CREATE_INFO_NV: Self = Self(1000165000);
        pub const RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR: Self = Self(1000150018);
        pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR: Self = Self(1000150016);
        pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1000165011);
        pub const RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR: Self = Self(1000483005);
        pub const RELEASE_SWAPCHAIN_IMAGES_INFO_KHR: Self = Self(1000275005);
        pub const RENDERING_AREA_INFO: Self = Self(1000470003);
        pub const RENDERING_ATTACHMENT_FLAGS_INFO_KHR: Self = Self(1000630002);
        pub const RENDERING_ATTACHMENT_INFO: Self = Self(1000044001);
        pub const RENDERING_ATTACHMENT_LOCATION_INFO: Self = Self(1000232001);
        pub const RENDERING_END_INFO_KHR: Self = Self(1000619003);
        pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT: Self = Self(1000044007);
        pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1000044006);
        pub const RENDERING_INFO: Self = Self(1000044000);
        pub const RENDERING_INPUT_ATTACHMENT_INDEX_INFO: Self = Self(1000232002);
        pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO: Self = Self(1000108003);
        pub const RENDER_PASS_CREATE_INFO_2: Self = Self(1000109004);
        pub const RENDER_PASS_CREATION_CONTROL_EXT: Self = Self(1000458001);
        pub const RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000458002);
        pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT: Self = Self(1000218002);
        pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT: Self = Self(1000425002);
        pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: Self = Self(1000117001);
        pub const RENDER_PASS_MULTIVIEW_CREATE_INFO: Self = Self(1000053000);
        pub const RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM: Self =
            Self(1000605004);
        pub const RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT: Self = Self(1000143001);
        pub const RENDER_PASS_STRIPE_BEGIN_INFO_ARM: Self = Self(1000424002);
        pub const RENDER_PASS_STRIPE_INFO_ARM: Self = Self(1000424003);
        pub const RENDER_PASS_STRIPE_SUBMIT_INFO_ARM: Self = Self(1000424004);
        pub const RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000458003);
        pub const RENDER_PASS_TILE_SHADING_CREATE_INFO_QCOM: Self = Self(1000309002);
        pub const RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM: Self = Self(1000282001);
        pub const RESOLVE_IMAGE_INFO_2: Self = Self(1000337005);
        pub const RESOLVE_IMAGE_MODE_INFO_KHR: Self = Self(1000630004);
        pub const RESOURCE_DESCRIPTOR_INFO_EXT: Self = Self(1000135002);
        pub const SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM: Self = Self(1000518002);
        pub const SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT: Self = Self(1000411001);
        pub const SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316008);
        pub const SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM: Self = Self(1000519000);
        pub const SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT: Self = Self(1000287000);
        pub const SAMPLER_CUSTOM_BORDER_COLOR_INDEX_CREATE_INFO_EXT: Self = Self(1000135011);
        pub const SAMPLER_REDUCTION_MODE_CREATE_INFO: Self = Self(1000130001);
        pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO: Self = Self(1000156000);
        pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: Self = Self(1000156005);
        pub const SAMPLER_YCBCR_CONVERSION_INFO: Self = Self(1000156001);
        pub const SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM: Self = Self(1000520001);
        pub const SAMPLE_LOCATIONS_INFO_EXT: Self = Self(1000143000);
        pub const SCREEN_BUFFER_FORMAT_PROPERTIES_QNX: Self = Self(1000529001);
        pub const SCREEN_BUFFER_PROPERTIES_QNX: Self = Self(1000529000);
        pub const SCREEN_SURFACE_CREATE_INFO_QNX: Self = Self(1000378000);
        pub const SEMAPHORE_GET_FD_INFO_KHR: Self = Self(1000079001);
        pub const SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000078003);
        pub const SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000365001);
        pub const SEMAPHORE_SIGNAL_INFO: Self = Self(1000207005);
        pub const SEMAPHORE_SUBMIT_INFO: Self = Self(1000314005);
        pub const SEMAPHORE_TYPE_CREATE_INFO: Self = Self(1000207002);
        pub const SEMAPHORE_WAIT_INFO: Self = Self(1000207004);
        pub const SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT: Self = Self(1000545007);
        pub const SET_LATENCY_MARKER_INFO_NV: Self = Self(1000505002);
        pub const SET_PRESENT_CONFIG_NV: Self = Self(1000613000);
        pub const SHADER_CREATE_INFO_EXT: Self = Self(1000482002);
        pub const SHADER_DESCRIPTOR_SET_AND_BINDING_MAPPING_INFO_EXT: Self = Self(1000135006);
        pub const SHADER_MODULE_IDENTIFIER_EXT: Self = Self(1000462003);
        pub const SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1000160001);
        pub const SHARED_PRESENT_SURFACE_CAPABILITIES_KHR: Self = Self(1000111000);
        pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2: Self = Self(1000059007);
        pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: Self = Self(1000146004);
        pub const STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP: Self = Self(1000049000);
        pub const SUBMIT_INFO_2: Self = Self(1000314004);
        pub const SUBPASS_BEGIN_INFO: Self = Self(1000109005);
        pub const SUBPASS_DEPENDENCY_2: Self = Self(1000109003);
        pub const SUBPASS_DESCRIPTION_2: Self = Self(1000109002);
        pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE: Self = Self(1000199001);
        pub const SUBPASS_END_INFO: Self = Self(1000109006);
        pub const SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT: Self = Self(1000376001);
        pub const SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI: Self = Self(1000369000);
        pub const SUBRESOURCE_HOST_MEMCPY_SIZE: Self = Self(1000270008);
        pub const SUBRESOURCE_LAYOUT_2: Self = Self(1000338002);
        pub const SUBSAMPLED_IMAGE_FORMAT_PROPERTIES_EXT: Self = Self(1000135013);
        pub const SURFACE_CAPABILITIES_2_EXT: Self = Self(1000090000);
        pub const SURFACE_CAPABILITIES_2_KHR: Self = Self(1000119001);
        pub const SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT: Self = Self(1000255002);
        pub const SURFACE_CAPABILITIES_PRESENT_BARRIER_NV: Self = Self(1000292001);
        pub const SURFACE_CAPABILITIES_PRESENT_ID_2_KHR: Self = Self(1000479000);
        pub const SURFACE_CAPABILITIES_PRESENT_WAIT_2_KHR: Self = Self(1000480000);
        pub const SURFACE_FORMAT_2_KHR: Self = Self(1000119002);
        pub const SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT: Self = Self(1000255000);
        pub const SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT: Self = Self(1000255001);
        pub const SURFACE_PRESENT_MODE_COMPATIBILITY_KHR: Self = Self(1000274002);
        pub const SURFACE_PRESENT_MODE_KHR: Self = Self(1000274000);
        pub const SURFACE_PRESENT_SCALING_CAPABILITIES_KHR: Self = Self(1000274001);
        pub const SURFACE_PROTECTED_CAPABILITIES_KHR: Self = Self(1000239000);
        pub const SWAPCHAIN_CALIBRATED_TIMESTAMP_INFO_EXT: Self = Self(1000208009);
        pub const SWAPCHAIN_COUNTER_CREATE_INFO_EXT: Self = Self(1000091003);
        pub const SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000001000);
        pub const SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD: Self = Self(1000213001);
        pub const SWAPCHAIN_LATENCY_CREATE_INFO_NV: Self = Self(1000505007);
        pub const SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV: Self = Self(1000292002);
        pub const SWAPCHAIN_PRESENT_FENCE_INFO_KHR: Self = Self(1000275001);
        pub const SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR: Self = Self(1000275002);
        pub const SWAPCHAIN_PRESENT_MODE_INFO_KHR: Self = Self(1000275003);
        pub const SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR: Self = Self(1000275004);
        pub const SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT: Self = Self(1000208002);
        pub const SWAPCHAIN_TIMING_PROPERTIES_EXT: Self = Self(1000208001);
        pub const SYSMEM_COLOR_SPACE_FUCHSIA: Self = Self(1000366008);
        pub const TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM: Self = Self(1000460021);
        pub const TENSOR_COPY_ARM: Self = Self(1000460012);
        pub const TENSOR_CREATE_INFO_ARM: Self = Self(1000460000);
        pub const TENSOR_DEPENDENCY_INFO_ARM: Self = Self(1000460013);
        pub const TENSOR_DESCRIPTION_ARM: Self = Self(1000460006);
        pub const TENSOR_FORMAT_PROPERTIES_ARM: Self = Self(1000460005);
        pub const TENSOR_MEMORY_BARRIER_ARM: Self = Self(1000460008);
        pub const TENSOR_MEMORY_REQUIREMENTS_INFO_ARM: Self = Self(1000460007);
        pub const TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM: Self = Self(1000460022);
        pub const TENSOR_VIEW_CREATE_INFO_ARM: Self = Self(1000460001);
        pub const TEXEL_BUFFER_DESCRIPTOR_INFO_EXT: Self = Self(1000135000);
        pub const TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD: Self = Self(1000041000);
        pub const TILE_MEMORY_BIND_INFO_QCOM: Self = Self(1000547003);
        pub const TILE_MEMORY_REQUIREMENTS_QCOM: Self = Self(1000547002);
        pub const TILE_MEMORY_SIZE_INFO_QCOM: Self = Self(1000547004);
        pub const TILE_PROPERTIES_QCOM: Self = Self(1000484001);
        pub const TIMELINE_SEMAPHORE_SUBMIT_INFO: Self = Self(1000207003);
        pub const VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1000160000);
        pub const VALIDATION_FEATURES_EXT: Self = Self(1000247000);
        pub const VALIDATION_FLAGS_EXT: Self = Self(1000061000);
        pub const VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT: Self = Self(1000352002);
        pub const VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT: Self = Self(1000352001);
        pub const VIDEO_BEGIN_CODING_INFO_KHR: Self = Self(1000023008);
        pub const VIDEO_CAPABILITIES_KHR: Self = Self(1000023001);
        pub const VIDEO_CODING_CONTROL_INFO_KHR: Self = Self(1000023010);
        pub const VIDEO_DECODE_AV1_CAPABILITIES_KHR: Self = Self(1000512000);
        pub const VIDEO_DECODE_AV1_DPB_SLOT_INFO_KHR: Self = Self(1000512005);
        pub const VIDEO_DECODE_AV1_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586003);
        pub const VIDEO_DECODE_AV1_PICTURE_INFO_KHR: Self = Self(1000512001);
        pub const VIDEO_DECODE_AV1_PROFILE_INFO_KHR: Self = Self(1000512003);
        pub const VIDEO_DECODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000512004);
        pub const VIDEO_DECODE_CAPABILITIES_KHR: Self = Self(1000024001);
        pub const VIDEO_DECODE_H264_CAPABILITIES_KHR: Self = Self(1000040000);
        pub const VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR: Self = Self(1000040006);
        pub const VIDEO_DECODE_H264_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586001);
        pub const VIDEO_DECODE_H264_PICTURE_INFO_KHR: Self = Self(1000040001);
        pub const VIDEO_DECODE_H264_PROFILE_INFO_KHR: Self = Self(1000040003);
        pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000040005);
        pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000040004);
        pub const VIDEO_DECODE_H265_CAPABILITIES_KHR: Self = Self(1000187000);
        pub const VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR: Self = Self(1000187005);
        pub const VIDEO_DECODE_H265_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586002);
        pub const VIDEO_DECODE_H265_PICTURE_INFO_KHR: Self = Self(1000187004);
        pub const VIDEO_DECODE_H265_PROFILE_INFO_KHR: Self = Self(1000187003);
        pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000187002);
        pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000187001);
        pub const VIDEO_DECODE_INFO_KHR: Self = Self(1000024000);
        pub const VIDEO_DECODE_USAGE_INFO_KHR: Self = Self(1000024002);
        pub const VIDEO_DECODE_VP9_CAPABILITIES_KHR: Self = Self(1000514001);
        pub const VIDEO_DECODE_VP9_PICTURE_INFO_KHR: Self = Self(1000514002);
        pub const VIDEO_DECODE_VP9_PROFILE_INFO_KHR: Self = Self(1000514003);
        pub const VIDEO_ENCODE_AV1_CAPABILITIES_KHR: Self = Self(1000513000);
        pub const VIDEO_ENCODE_AV1_DPB_SLOT_INFO_KHR: Self = Self(1000513003);
        pub const VIDEO_ENCODE_AV1_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000513010);
        pub const VIDEO_ENCODE_AV1_PICTURE_INFO_KHR: Self = Self(1000513002);
        pub const VIDEO_ENCODE_AV1_PROFILE_INFO_KHR: Self = Self(1000513005);
        pub const VIDEO_ENCODE_AV1_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000513008);
        pub const VIDEO_ENCODE_AV1_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553007);
        pub const VIDEO_ENCODE_AV1_RATE_CONTROL_INFO_KHR: Self = Self(1000513006);
        pub const VIDEO_ENCODE_AV1_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000513007);
        pub const VIDEO_ENCODE_AV1_SESSION_CREATE_INFO_KHR: Self = Self(1000513009);
        pub const VIDEO_ENCODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000513001);
        pub const VIDEO_ENCODE_CAPABILITIES_KHR: Self = Self(1000299003);
        pub const VIDEO_ENCODE_H264_CAPABILITIES_KHR: Self = Self(1000038000);
        pub const VIDEO_ENCODE_H264_DPB_SLOT_INFO_KHR: Self = Self(1000038004);
        pub const VIDEO_ENCODE_H264_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000038006);
        pub const VIDEO_ENCODE_H264_NALU_SLICE_INFO_KHR: Self = Self(1000038005);
        pub const VIDEO_ENCODE_H264_PICTURE_INFO_KHR: Self = Self(1000038003);
        pub const VIDEO_ENCODE_H264_PROFILE_INFO_KHR: Self = Self(1000038007);
        pub const VIDEO_ENCODE_H264_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000038011);
        pub const VIDEO_ENCODE_H264_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553003);
        pub const VIDEO_ENCODE_H264_RATE_CONTROL_INFO_KHR: Self = Self(1000038008);
        pub const VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000038009);
        pub const VIDEO_ENCODE_H264_SESSION_CREATE_INFO_KHR: Self = Self(1000038010);
        pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000038002);
        pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000038001);
        pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000038013);
        pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000038012);
        pub const VIDEO_ENCODE_H265_CAPABILITIES_KHR: Self = Self(1000039000);
        pub const VIDEO_ENCODE_H265_DPB_SLOT_INFO_KHR: Self = Self(1000039004);
        pub const VIDEO_ENCODE_H265_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000039006);
        pub const VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_KHR: Self = Self(1000039005);
        pub const VIDEO_ENCODE_H265_PICTURE_INFO_KHR: Self = Self(1000039003);
        pub const VIDEO_ENCODE_H265_PROFILE_INFO_KHR: Self = Self(1000039007);
        pub const VIDEO_ENCODE_H265_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000039012);
        pub const VIDEO_ENCODE_H265_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553004);
        pub const VIDEO_ENCODE_H265_RATE_CONTROL_INFO_KHR: Self = Self(1000039009);
        pub const VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000039010);
        pub const VIDEO_ENCODE_H265_SESSION_CREATE_INFO_KHR: Self = Self(1000039011);
        pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000039002);
        pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000039001);
        pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000039014);
        pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000039013);
        pub const VIDEO_ENCODE_INFO_KHR: Self = Self(1000299000);
        pub const VIDEO_ENCODE_INTRA_REFRESH_CAPABILITIES_KHR: Self = Self(1000552000);
        pub const VIDEO_ENCODE_INTRA_REFRESH_INFO_KHR: Self = Self(1000552002);
        pub const VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE: Self = Self(1000390002);
        pub const VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR: Self = Self(1000299008);
        pub const VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000299007);
        pub const VIDEO_ENCODE_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553000);
        pub const VIDEO_ENCODE_QUANTIZATION_MAP_INFO_KHR: Self = Self(1000553002);
        pub const VIDEO_ENCODE_QUANTIZATION_MAP_SESSION_PARAMETERS_CREATE_INFO_KHR: Self =
            Self(1000553005);
        pub const VIDEO_ENCODE_RATE_CONTROL_INFO_KHR: Self = Self(1000299001);
        pub const VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000299002);
        pub const VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE: Self = Self(1000390001);
        pub const VIDEO_ENCODE_SESSION_INTRA_REFRESH_CREATE_INFO_KHR: Self = Self(1000552001);
        pub const VIDEO_ENCODE_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000299010);
        pub const VIDEO_ENCODE_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000299009);
        pub const VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE: Self = Self(1000390003);
        pub const VIDEO_ENCODE_USAGE_INFO_KHR: Self = Self(1000299004);
        pub const VIDEO_END_CODING_INFO_KHR: Self = Self(1000023009);
        pub const VIDEO_FORMAT_AV1_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553008);
        pub const VIDEO_FORMAT_H265_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553006);
        pub const VIDEO_FORMAT_PROPERTIES_KHR: Self = Self(1000023015);
        pub const VIDEO_FORMAT_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553001);
        pub const VIDEO_INLINE_QUERY_INFO_KHR: Self = Self(1000515001);
        pub const VIDEO_PICTURE_RESOURCE_INFO_KHR: Self = Self(1000023002);
        pub const VIDEO_PROFILE_INFO_KHR: Self = Self(1000023000);
        pub const VIDEO_PROFILE_LIST_INFO_KHR: Self = Self(1000023013);
        pub const VIDEO_REFERENCE_INTRA_REFRESH_INFO_KHR: Self = Self(1000552003);
        pub const VIDEO_REFERENCE_SLOT_INFO_KHR: Self = Self(1000023011);
        pub const VIDEO_SESSION_CREATE_INFO_KHR: Self = Self(1000023005);
        pub const VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR: Self = Self(1000023003);
        pub const VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000023006);
        pub const VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR: Self = Self(1000023007);
        pub const VI_SURFACE_CREATE_INFO_NN: Self = Self(1000062000);
        pub const WAYLAND_SURFACE_CREATE_INFO_KHR: Self = Self(1000006000);
        pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR: Self = Self(1000075000);
        pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV: Self = Self(1000058000);
        pub const WIN32_SURFACE_CREATE_INFO_KHR: Self = Self(1000009000);
        pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR: Self = Self(1000150007);
        pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV: Self = Self(1000165007);
        pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK: Self = Self(1000138002);
        pub const WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV: Self =
            Self(1000570002);
        pub const WRITE_DESCRIPTOR_SET_TENSOR_ARM: Self = Self(1000460003);
        pub const WRITE_INDIRECT_EXECUTION_SET_PIPELINE_EXT: Self = Self(1000572008);
        pub const WRITE_INDIRECT_EXECUTION_SET_SHADER_EXT: Self = Self(1000572009);
        pub const XCB_SURFACE_CREATE_INFO_KHR: Self = Self(1000005000);
        pub const XLIB_SURFACE_CREATE_INFO_KHR: Self = Self(1000004000);
        pub const PHYSICAL_DEVICE_VULKAN_1_1_FEATURES: Self = Self(49);
        pub const PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES: Self = Self(50);
        pub const PHYSICAL_DEVICE_VULKAN_1_2_FEATURES: Self = Self(51);
        pub const PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES: Self = Self(52);
        pub const PHYSICAL_DEVICE_VULKAN_1_3_FEATURES: Self = Self(53);
        pub const PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES: Self = Self(54);
        pub const PHYSICAL_DEVICE_VULKAN_1_4_FEATURES: Self = Self(55);
        pub const PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES: Self = Self(56);
        pub const SURFACE_CREATE_INFO_OHOS: Self = Self(1000685000);
        pub const ATTACHMENT_DESCRIPTION_2_KHR: Self = Self::ATTACHMENT_DESCRIPTION_2;
        pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT_KHR: Self =
            Self::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT;
        pub const ATTACHMENT_REFERENCE_2_KHR: Self = Self::ATTACHMENT_REFERENCE_2;
        pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT_KHR: Self =
            Self::ATTACHMENT_REFERENCE_STENCIL_LAYOUT;
        pub const ATTACHMENT_SAMPLE_COUNT_INFO_NV: Self = Self::ATTACHMENT_SAMPLE_COUNT_INFO_AMD;
        pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR: Self =
            Self::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO;
        pub const BIND_BUFFER_MEMORY_INFO_KHR: Self = Self::BIND_BUFFER_MEMORY_INFO;
        pub const BIND_DESCRIPTOR_SETS_INFO_KHR: Self = Self::BIND_DESCRIPTOR_SETS_INFO;
        pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR: Self =
            Self::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO;
        pub const BIND_IMAGE_MEMORY_INFO_KHR: Self = Self::BIND_IMAGE_MEMORY_INFO;
        pub const BIND_IMAGE_PLANE_MEMORY_INFO_KHR: Self = Self::BIND_IMAGE_PLANE_MEMORY_INFO;
        pub const BIND_MEMORY_STATUS_KHR: Self = Self::BIND_MEMORY_STATUS;
        pub const BLIT_IMAGE_INFO_2_KHR: Self = Self::BLIT_IMAGE_INFO_2;
        pub const BUFFER_COPY_2_KHR: Self = Self::BUFFER_COPY_2;
        pub const BUFFER_DEVICE_ADDRESS_INFO_EXT: Self = Self::BUFFER_DEVICE_ADDRESS_INFO;
        pub const BUFFER_DEVICE_ADDRESS_INFO_KHR: Self = Self::BUFFER_DEVICE_ADDRESS_INFO;
        pub const BUFFER_IMAGE_COPY_2_KHR: Self = Self::BUFFER_IMAGE_COPY_2;
        pub const BUFFER_MEMORY_BARRIER_2_KHR: Self = Self::BUFFER_MEMORY_BARRIER_2;
        pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR: Self =
            Self::BUFFER_MEMORY_REQUIREMENTS_INFO_2;
        pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR: Self =
            Self::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO;
        pub const BUFFER_USAGE_FLAGS_2_CREATE_INFO_KHR: Self =
            Self::BUFFER_USAGE_FLAGS_2_CREATE_INFO;
        pub const CALIBRATED_TIMESTAMP_INFO_EXT: Self = Self::CALIBRATED_TIMESTAMP_INFO_KHR;
        pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR: Self =
            Self::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO;
        pub const COMMAND_BUFFER_SUBMIT_INFO_KHR: Self = Self::COMMAND_BUFFER_SUBMIT_INFO;
        pub const COPY_BUFFER_INFO_2_KHR: Self = Self::COPY_BUFFER_INFO_2;
        pub const COPY_BUFFER_TO_IMAGE_INFO_2_KHR: Self = Self::COPY_BUFFER_TO_IMAGE_INFO_2;
        pub const COPY_IMAGE_INFO_2_KHR: Self = Self::COPY_IMAGE_INFO_2;
        pub const COPY_IMAGE_TO_BUFFER_INFO_2_KHR: Self = Self::COPY_IMAGE_TO_BUFFER_INFO_2;
        pub const COPY_IMAGE_TO_IMAGE_INFO_EXT: Self = Self::COPY_IMAGE_TO_IMAGE_INFO;
        pub const COPY_IMAGE_TO_MEMORY_INFO_EXT: Self = Self::COPY_IMAGE_TO_MEMORY_INFO;
        pub const COPY_MEMORY_TO_IMAGE_INFO_EXT: Self = Self::COPY_MEMORY_TO_IMAGE_INFO;
        pub const DEBUG_REPORT_CREATE_INFO_EXT: Self = Self::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT;
        pub const DEPENDENCY_INFO_KHR: Self = Self::DEPENDENCY_INFO;
        pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT: Self =
            Self::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO;
        pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT: Self =
            Self::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO;
        pub const DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: Self = Self::DESCRIPTOR_SET_LAYOUT_SUPPORT;
        pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT: Self =
            Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO;
        pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT: Self =
            Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT;
        pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR: Self =
            Self::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO;
        pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR: Self =
            Self::DEVICE_BUFFER_MEMORY_REQUIREMENTS;
        pub const DEVICE_GROUP_BIND_SPARSE_INFO_KHR: Self = Self::DEVICE_GROUP_BIND_SPARSE_INFO;
        pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR: Self =
            Self::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO;
        pub const DEVICE_GROUP_DEVICE_CREATE_INFO_KHR: Self = Self::DEVICE_GROUP_DEVICE_CREATE_INFO;
        pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR: Self =
            Self::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO;
        pub const DEVICE_GROUP_SUBMIT_INFO_KHR: Self = Self::DEVICE_GROUP_SUBMIT_INFO;
        pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR: Self =
            Self::DEVICE_IMAGE_MEMORY_REQUIREMENTS;
        pub const DEVICE_IMAGE_SUBRESOURCE_INFO_KHR: Self = Self::DEVICE_IMAGE_SUBRESOURCE_INFO;
        pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR: Self =
            Self::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO;
        pub const DEVICE_PRIVATE_DATA_CREATE_INFO_EXT: Self = Self::DEVICE_PRIVATE_DATA_CREATE_INFO;
        pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT: Self =
            Self::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO;
        pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR: Self =
            Self::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO;
        pub const EXPORT_FENCE_CREATE_INFO_KHR: Self = Self::EXPORT_FENCE_CREATE_INFO;
        pub const EXPORT_MEMORY_ALLOCATE_INFO_KHR: Self = Self::EXPORT_MEMORY_ALLOCATE_INFO;
        pub const EXPORT_SEMAPHORE_CREATE_INFO_KHR: Self = Self::EXPORT_SEMAPHORE_CREATE_INFO;
        pub const EXTERNAL_BUFFER_PROPERTIES_KHR: Self = Self::EXTERNAL_BUFFER_PROPERTIES;
        pub const EXTERNAL_FENCE_PROPERTIES_KHR: Self = Self::EXTERNAL_FENCE_PROPERTIES;
        pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR: Self =
            Self::EXTERNAL_IMAGE_FORMAT_PROPERTIES;
        pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR: Self =
            Self::EXTERNAL_MEMORY_BUFFER_CREATE_INFO;
        pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR: Self =
            Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO;
        pub const EXTERNAL_SEMAPHORE_PROPERTIES_KHR: Self = Self::EXTERNAL_SEMAPHORE_PROPERTIES;
        pub const FORMAT_PROPERTIES_2_KHR: Self = Self::FORMAT_PROPERTIES_2;
        pub const FORMAT_PROPERTIES_3_KHR: Self = Self::FORMAT_PROPERTIES_3;
        pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO_KHR: Self =
            Self::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO;
        pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO_KHR: Self =
            Self::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO;
        pub const HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY_EXT: Self =
            Self::HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY;
        pub const HOST_IMAGE_LAYOUT_TRANSITION_INFO_EXT: Self =
            Self::HOST_IMAGE_LAYOUT_TRANSITION_INFO;
        pub const IMAGE_BLIT_2_KHR: Self = Self::IMAGE_BLIT_2;
        pub const IMAGE_COPY_2_KHR: Self = Self::IMAGE_COPY_2;
        pub const IMAGE_FORMAT_LIST_CREATE_INFO_KHR: Self = Self::IMAGE_FORMAT_LIST_CREATE_INFO;
        pub const IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self::IMAGE_FORMAT_PROPERTIES_2;
        pub const IMAGE_MEMORY_BARRIER_2_KHR: Self = Self::IMAGE_MEMORY_BARRIER_2;
        pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self =
            Self::IMAGE_MEMORY_REQUIREMENTS_INFO_2;
        pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR: Self =
            Self::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO;
        pub const IMAGE_RESOLVE_2_KHR: Self = Self::IMAGE_RESOLVE_2;
        pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self =
            Self::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2;
        pub const IMAGE_STENCIL_USAGE_CREATE_INFO_EXT: Self = Self::IMAGE_STENCIL_USAGE_CREATE_INFO;
        pub const IMAGE_SUBRESOURCE_2_EXT: Self = Self::IMAGE_SUBRESOURCE_2;
        pub const IMAGE_SUBRESOURCE_2_KHR: Self = Self::IMAGE_SUBRESOURCE_2;
        pub const IMAGE_TO_MEMORY_COPY_EXT: Self = Self::IMAGE_TO_MEMORY_COPY;
        pub const IMAGE_VIEW_USAGE_CREATE_INFO_KHR: Self = Self::IMAGE_VIEW_USAGE_CREATE_INFO;
        pub const MEMORY_ALLOCATE_FLAGS_INFO_KHR: Self = Self::MEMORY_ALLOCATE_FLAGS_INFO;
        pub const MEMORY_BARRIER_2_KHR: Self = Self::MEMORY_BARRIER_2;
        pub const MEMORY_DEDICATED_ALLOCATE_INFO_KHR: Self = Self::MEMORY_DEDICATED_ALLOCATE_INFO;
        pub const MEMORY_DEDICATED_REQUIREMENTS_KHR: Self = Self::MEMORY_DEDICATED_REQUIREMENTS;
        pub const MEMORY_MAP_INFO_KHR: Self = Self::MEMORY_MAP_INFO;
        pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR: Self =
            Self::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO;
        pub const MEMORY_REQUIREMENTS_2_KHR: Self = Self::MEMORY_REQUIREMENTS_2;
        pub const MEMORY_TO_IMAGE_COPY_EXT: Self = Self::MEMORY_TO_IMAGE_COPY;
        pub const MEMORY_UNMAP_INFO_KHR: Self = Self::MEMORY_UNMAP_INFO;
        pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE: Self =
            Self::MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT;
        pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES;
        pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES;
        pub const PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT;
        pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES;
        pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV: Self =
            Self::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR;
        pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_NV: Self =
            Self::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR;
        pub const PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR;
        pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR: Self =
            Self::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES;
        pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES;
        pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT: Self =
            Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES;
        pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES_KHR: Self =
            Self::PHYSICAL_DEVICE_DRIVER_PROPERTIES;
        pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES;
        pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES;
        pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR: Self =
            Self::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO;
        pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR: Self =
            Self::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO;
        pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR: Self =
            Self::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO;
        pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR: Self =
            Self::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO;
        pub const PHYSICAL_DEVICE_FEATURES_2_KHR: Self = Self::PHYSICAL_DEVICE_FEATURES_2;
        pub const PHYSICAL_DEVICE_FLOAT16_INT8_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
        pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR: Self =
            Self::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES;
        pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM: Self =
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT;
        pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM: Self =
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT;
        pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV: Self =
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR;
        pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES;
        pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES;
        pub const PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR: Self =
            Self::PHYSICAL_DEVICE_GROUP_PROPERTIES;
        pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES;
        pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES_EXT: Self =
            Self::PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES;
        pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES;
        pub const PHYSICAL_DEVICE_ID_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_ID_PROPERTIES;
        pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES;
        pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR: Self =
            Self::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2;
        pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES;
        pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES;
        pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES;
        pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES;
        pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT: Self =
            Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES;
        pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES;
        pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES;
        pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT: Self =
            Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES;
        pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_KHR: Self =
            Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES;
        pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR: Self =
            Self::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES;
        pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES;
        pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR: Self =
            Self::PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES;
        pub const PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES;
        pub const PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES_KHR: Self =
            Self::PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES;
        pub const PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES;
        pub const PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES_KHR: Self =
            Self::PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES;
        pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV: Self =
            Self::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT;
        pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_NV: Self =
            Self::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT;
        pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR: Self =
            Self::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2;
        pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_MULTIVIEW_FEATURES;
        pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR: Self =
            Self::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES;
        pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE: Self =
            Self::PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT;
        pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES;
        pub const PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES;
        pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES;
        pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT: Self =
            Self::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES;
        pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR: Self =
            Self::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES;
        pub const PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR;
        pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES;
        pub const PHYSICAL_DEVICE_PROPERTIES_2_KHR: Self = Self::PHYSICAL_DEVICE_PROPERTIES_2;
        pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR: Self =
            Self::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES;
        pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM: Self =
            Self::PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT;
        pub const PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR;
        pub const PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT: Self =
            Self::PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR;
        pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT: Self =
            Self::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES;
        pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES;
        pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES;
        pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES;
        pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES;
        pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES;
        pub const PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES: Self =
            Self::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES;
        pub const PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES;
        pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
        pub const PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES;
        pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES;
        pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES_KHR: Self =
            Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES;
        pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES;
        pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES;
        pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES;
        pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR: Self =
            Self::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2;
        pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES;
        pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT: Self =
            Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES;
        pub const PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR;
        pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES;
        pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT: Self =
            Self::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES;
        pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES;
        pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES;
        pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR: Self =
            Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES;
        pub const PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT: Self = Self::PHYSICAL_DEVICE_TOOL_PROPERTIES;
        pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES;
        pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
        pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES: Self =
            Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
        pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR;
        pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT: Self =
            Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES;
        pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES;
        pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_KHR: Self =
            Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES;
        pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES;
        pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR: Self =
            Self::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES;
        pub const PIPELINE_CREATE_FLAGS_2_CREATE_INFO_KHR: Self =
            Self::PIPELINE_CREATE_FLAGS_2_CREATE_INFO;
        pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT: Self =
            Self::PIPELINE_CREATION_FEEDBACK_CREATE_INFO;
        pub const PIPELINE_INFO_EXT: Self = Self::PIPELINE_INFO_KHR;
        pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT: Self =
            Self::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO;
        pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_KHR: Self =
            Self::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO;
        pub const PIPELINE_RENDERING_CREATE_INFO_KHR: Self = Self::PIPELINE_RENDERING_CREATE_INFO;
        pub const PIPELINE_ROBUSTNESS_CREATE_INFO_EXT: Self = Self::PIPELINE_ROBUSTNESS_CREATE_INFO;
        pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self =
            Self::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO;
        pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR: Self =
            Self::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO;
        pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT: Self =
            Self::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO;
        pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_KHR: Self =
            Self::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO;
        pub const PRIVATE_DATA_SLOT_CREATE_INFO_EXT: Self = Self::PRIVATE_DATA_SLOT_CREATE_INFO;
        pub const PUSH_CONSTANTS_INFO_KHR: Self = Self::PUSH_CONSTANTS_INFO;
        pub const PUSH_DESCRIPTOR_SET_INFO_KHR: Self = Self::PUSH_DESCRIPTOR_SET_INFO;
        pub const PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO_KHR: Self =
            Self::PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO;
        pub const QUERY_POOL_CREATE_INFO_INTEL: Self =
            Self::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL;
        pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT: Self =
            Self::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES;
        pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR: Self =
            Self::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES;
        pub const QUEUE_FAMILY_PROPERTIES_2_KHR: Self = Self::QUEUE_FAMILY_PROPERTIES_2;
        pub const RELEASE_SWAPCHAIN_IMAGES_INFO_EXT: Self = Self::RELEASE_SWAPCHAIN_IMAGES_INFO_KHR;
        pub const RENDERING_AREA_INFO_KHR: Self = Self::RENDERING_AREA_INFO;
        pub const RENDERING_ATTACHMENT_INFO_KHR: Self = Self::RENDERING_ATTACHMENT_INFO;
        pub const RENDERING_ATTACHMENT_LOCATION_INFO_KHR: Self =
            Self::RENDERING_ATTACHMENT_LOCATION_INFO;
        pub const RENDERING_END_INFO_EXT: Self = Self::RENDERING_END_INFO_KHR;
        pub const RENDERING_INFO_KHR: Self = Self::RENDERING_INFO;
        pub const RENDERING_INPUT_ATTACHMENT_INDEX_INFO_KHR: Self =
            Self::RENDERING_INPUT_ATTACHMENT_INDEX_INFO;
        pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO_KHR: Self =
            Self::RENDER_PASS_ATTACHMENT_BEGIN_INFO;
        pub const RENDER_PASS_CREATE_INFO_2_KHR: Self = Self::RENDER_PASS_CREATE_INFO_2;
        pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR: Self =
            Self::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO;
        pub const RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR: Self =
            Self::RENDER_PASS_MULTIVIEW_CREATE_INFO;
        pub const RESOLVE_IMAGE_INFO_2_KHR: Self = Self::RESOLVE_IMAGE_INFO_2;
        pub const SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT: Self =
            Self::SAMPLER_REDUCTION_MODE_CREATE_INFO;
        pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR: Self =
            Self::SAMPLER_YCBCR_CONVERSION_CREATE_INFO;
        pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR: Self =
            Self::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES;
        pub const SAMPLER_YCBCR_CONVERSION_INFO_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION_INFO;
        pub const SEMAPHORE_SIGNAL_INFO_KHR: Self = Self::SEMAPHORE_SIGNAL_INFO;
        pub const SEMAPHORE_SUBMIT_INFO_KHR: Self = Self::SEMAPHORE_SUBMIT_INFO;
        pub const SEMAPHORE_TYPE_CREATE_INFO_KHR: Self = Self::SEMAPHORE_TYPE_CREATE_INFO;
        pub const SEMAPHORE_WAIT_INFO_KHR: Self = Self::SEMAPHORE_WAIT_INFO;
        pub const SHADER_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self =
            Self::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO;
        pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR: Self =
            Self::SPARSE_IMAGE_FORMAT_PROPERTIES_2;
        pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR: Self =
            Self::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2;
        pub const SUBMIT_INFO_2_KHR: Self = Self::SUBMIT_INFO_2;
        pub const SUBPASS_BEGIN_INFO_KHR: Self = Self::SUBPASS_BEGIN_INFO;
        pub const SUBPASS_DEPENDENCY_2_KHR: Self = Self::SUBPASS_DEPENDENCY_2;
        pub const SUBPASS_DESCRIPTION_2_KHR: Self = Self::SUBPASS_DESCRIPTION_2;
        pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR: Self =
            Self::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE;
        pub const SUBPASS_END_INFO_KHR: Self = Self::SUBPASS_END_INFO;
        pub const SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM: Self =
            Self::RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT;
        pub const SUBRESOURCE_HOST_MEMCPY_SIZE_EXT: Self = Self::SUBRESOURCE_HOST_MEMCPY_SIZE;
        pub const SUBRESOURCE_LAYOUT_2_EXT: Self = Self::SUBRESOURCE_LAYOUT_2;
        pub const SUBRESOURCE_LAYOUT_2_KHR: Self = Self::SUBRESOURCE_LAYOUT_2;
        pub const SURFACE_CAPABILITIES2_EXT: Self = Self::SURFACE_CAPABILITIES_2_EXT;
        pub const SURFACE_PRESENT_MODE_COMPATIBILITY_EXT: Self =
            Self::SURFACE_PRESENT_MODE_COMPATIBILITY_KHR;
        pub const SURFACE_PRESENT_MODE_EXT: Self = Self::SURFACE_PRESENT_MODE_KHR;
        pub const SURFACE_PRESENT_SCALING_CAPABILITIES_EXT: Self =
            Self::SURFACE_PRESENT_SCALING_CAPABILITIES_KHR;
        pub const SWAPCHAIN_PRESENT_FENCE_INFO_EXT: Self = Self::SWAPCHAIN_PRESENT_FENCE_INFO_KHR;
        pub const SWAPCHAIN_PRESENT_MODES_CREATE_INFO_EXT: Self =
            Self::SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR;
        pub const SWAPCHAIN_PRESENT_MODE_INFO_EXT: Self = Self::SWAPCHAIN_PRESENT_MODE_INFO_KHR;
        pub const SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_EXT: Self =
            Self::SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR;
        pub const TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR: Self = Self::TIMELINE_SEMAPHORE_SUBMIT_INFO;
        pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT: Self =
            Self::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK;
    }
    impl fmt::Debug for StructureType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::APPLICATION_INFO => Some("APPLICATION_INFO"),
                Self::INSTANCE_CREATE_INFO => Some("INSTANCE_CREATE_INFO"),
                Self::DEVICE_QUEUE_CREATE_INFO => Some("DEVICE_QUEUE_CREATE_INFO"),
                Self::DEVICE_CREATE_INFO => Some("DEVICE_CREATE_INFO"),
                Self::SUBMIT_INFO => Some("SUBMIT_INFO"),
                Self::MEMORY_ALLOCATE_INFO => Some("MEMORY_ALLOCATE_INFO"),
                Self::MAPPED_MEMORY_RANGE => Some("MAPPED_MEMORY_RANGE"),
                Self::BIND_SPARSE_INFO => Some("BIND_SPARSE_INFO"),
                Self::FENCE_CREATE_INFO => Some("FENCE_CREATE_INFO"),
                Self::SEMAPHORE_CREATE_INFO => Some("SEMAPHORE_CREATE_INFO"),
                Self::EVENT_CREATE_INFO => Some("EVENT_CREATE_INFO"),
                Self::QUERY_POOL_CREATE_INFO => Some("QUERY_POOL_CREATE_INFO"),
                Self::BUFFER_CREATE_INFO => Some("BUFFER_CREATE_INFO"),
                Self::BUFFER_VIEW_CREATE_INFO => Some("BUFFER_VIEW_CREATE_INFO"),
                Self::IMAGE_CREATE_INFO => Some("IMAGE_CREATE_INFO"),
                Self::IMAGE_VIEW_CREATE_INFO => Some("IMAGE_VIEW_CREATE_INFO"),
                Self::SHADER_MODULE_CREATE_INFO => Some("SHADER_MODULE_CREATE_INFO"),
                Self::PIPELINE_CACHE_CREATE_INFO => Some("PIPELINE_CACHE_CREATE_INFO"),
                Self::PIPELINE_SHADER_STAGE_CREATE_INFO => {
                    Some("PIPELINE_SHADER_STAGE_CREATE_INFO")
                }
                Self::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO => {
                    Some("PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO")
                }
                Self::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO => {
                    Some("PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO")
                }
                Self::PIPELINE_TESSELLATION_STATE_CREATE_INFO => {
                    Some("PIPELINE_TESSELLATION_STATE_CREATE_INFO")
                }
                Self::PIPELINE_VIEWPORT_STATE_CREATE_INFO => {
                    Some("PIPELINE_VIEWPORT_STATE_CREATE_INFO")
                }
                Self::PIPELINE_RASTERIZATION_STATE_CREATE_INFO => {
                    Some("PIPELINE_RASTERIZATION_STATE_CREATE_INFO")
                }
                Self::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO => {
                    Some("PIPELINE_MULTISAMPLE_STATE_CREATE_INFO")
                }
                Self::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO => {
                    Some("PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO")
                }
                Self::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO => {
                    Some("PIPELINE_COLOR_BLEND_STATE_CREATE_INFO")
                }
                Self::PIPELINE_DYNAMIC_STATE_CREATE_INFO => {
                    Some("PIPELINE_DYNAMIC_STATE_CREATE_INFO")
                }
                Self::GRAPHICS_PIPELINE_CREATE_INFO => Some("GRAPHICS_PIPELINE_CREATE_INFO"),
                Self::COMPUTE_PIPELINE_CREATE_INFO => Some("COMPUTE_PIPELINE_CREATE_INFO"),
                Self::PIPELINE_LAYOUT_CREATE_INFO => Some("PIPELINE_LAYOUT_CREATE_INFO"),
                Self::SAMPLER_CREATE_INFO => Some("SAMPLER_CREATE_INFO"),
                Self::DESCRIPTOR_SET_LAYOUT_CREATE_INFO => {
                    Some("DESCRIPTOR_SET_LAYOUT_CREATE_INFO")
                }
                Self::DESCRIPTOR_POOL_CREATE_INFO => Some("DESCRIPTOR_POOL_CREATE_INFO"),
                Self::DESCRIPTOR_SET_ALLOCATE_INFO => Some("DESCRIPTOR_SET_ALLOCATE_INFO"),
                Self::WRITE_DESCRIPTOR_SET => Some("WRITE_DESCRIPTOR_SET"),
                Self::COPY_DESCRIPTOR_SET => Some("COPY_DESCRIPTOR_SET"),
                Self::FRAMEBUFFER_CREATE_INFO => Some("FRAMEBUFFER_CREATE_INFO"),
                Self::RENDER_PASS_CREATE_INFO => Some("RENDER_PASS_CREATE_INFO"),
                Self::COMMAND_POOL_CREATE_INFO => Some("COMMAND_POOL_CREATE_INFO"),
                Self::COMMAND_BUFFER_ALLOCATE_INFO => Some("COMMAND_BUFFER_ALLOCATE_INFO"),
                Self::COMMAND_BUFFER_INHERITANCE_INFO => Some("COMMAND_BUFFER_INHERITANCE_INFO"),
                Self::COMMAND_BUFFER_BEGIN_INFO => Some("COMMAND_BUFFER_BEGIN_INFO"),
                Self::RENDER_PASS_BEGIN_INFO => Some("RENDER_PASS_BEGIN_INFO"),
                Self::BUFFER_MEMORY_BARRIER => Some("BUFFER_MEMORY_BARRIER"),
                Self::IMAGE_MEMORY_BARRIER => Some("IMAGE_MEMORY_BARRIER"),
                Self::MEMORY_BARRIER => Some("MEMORY_BARRIER"),
                Self::LOADER_INSTANCE_CREATE_INFO => Some("LOADER_INSTANCE_CREATE_INFO"),
                Self::LOADER_DEVICE_CREATE_INFO => Some("LOADER_DEVICE_CREATE_INFO"),
                Self::ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR => {
                    Some("ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR")
                }
                Self::ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR => {
                    Some("ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR")
                }
                Self::ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => {
                    Some("ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT")
                }
                Self::ACCELERATION_STRUCTURE_CREATE_INFO_KHR => {
                    Some("ACCELERATION_STRUCTURE_CREATE_INFO_KHR")
                }
                Self::ACCELERATION_STRUCTURE_CREATE_INFO_NV => {
                    Some("ACCELERATION_STRUCTURE_CREATE_INFO_NV")
                }
                Self::ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX => {
                    Some("ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX")
                }
                Self::ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR => {
                    Some("ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR")
                }
                Self::ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR => {
                    Some("ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR")
                }
                Self::ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR => {
                    Some("ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR")
                }
                Self::ACCELERATION_STRUCTURE_GEOMETRY_KHR => {
                    Some("ACCELERATION_STRUCTURE_GEOMETRY_KHR")
                }
                Self::ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV => {
                    Some("ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV")
                }
                Self::ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV => {
                    Some("ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV")
                }
                Self::ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV => {
                    Some("ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV")
                }
                Self::ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR => {
                    Some("ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR")
                }
                Self::ACCELERATION_STRUCTURE_INFO_NV => Some("ACCELERATION_STRUCTURE_INFO_NV"),
                Self::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV => {
                    Some("ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV")
                }
                Self::ACCELERATION_STRUCTURE_MOTION_INFO_NV => {
                    Some("ACCELERATION_STRUCTURE_MOTION_INFO_NV")
                }
                Self::ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV => {
                    Some("ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV")
                }
                Self::ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT => {
                    Some("ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT")
                }
                Self::ACCELERATION_STRUCTURE_VERSION_INFO_KHR => {
                    Some("ACCELERATION_STRUCTURE_VERSION_INFO_KHR")
                }
                Self::ACQUIRE_NEXT_IMAGE_INFO_KHR => Some("ACQUIRE_NEXT_IMAGE_INFO_KHR"),
                Self::ACQUIRE_PROFILING_LOCK_INFO_KHR => Some("ACQUIRE_PROFILING_LOCK_INFO_KHR"),
                Self::AMIGO_PROFILING_SUBMIT_INFO_SEC => Some("AMIGO_PROFILING_SUBMIT_INFO_SEC"),
                Self::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID => {
                    Some("ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID")
                }
                Self::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID => {
                    Some("ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID")
                }
                Self::ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID => {
                    Some("ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID")
                }
                Self::ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID => {
                    Some("ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID")
                }
                Self::ANDROID_HARDWARE_BUFFER_USAGE_ANDROID => {
                    Some("ANDROID_HARDWARE_BUFFER_USAGE_ANDROID")
                }
                Self::ANDROID_SURFACE_CREATE_INFO_KHR => Some("ANDROID_SURFACE_CREATE_INFO_KHR"),
                Self::ANTI_LAG_DATA_AMD => Some("ANTI_LAG_DATA_AMD"),
                Self::ANTI_LAG_PRESENTATION_INFO_AMD => Some("ANTI_LAG_PRESENTATION_INFO_AMD"),
                Self::ATTACHMENT_DESCRIPTION_2 => Some("ATTACHMENT_DESCRIPTION_2"),
                Self::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT => {
                    Some("ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT")
                }
                Self::ATTACHMENT_FEEDBACK_LOOP_INFO_EXT => {
                    Some("ATTACHMENT_FEEDBACK_LOOP_INFO_EXT")
                }
                Self::ATTACHMENT_REFERENCE_2 => Some("ATTACHMENT_REFERENCE_2"),
                Self::ATTACHMENT_REFERENCE_STENCIL_LAYOUT => {
                    Some("ATTACHMENT_REFERENCE_STENCIL_LAYOUT")
                }
                Self::ATTACHMENT_SAMPLE_COUNT_INFO_AMD => Some("ATTACHMENT_SAMPLE_COUNT_INFO_AMD"),
                Self::BEGIN_CUSTOM_RESOLVE_INFO_EXT => Some("BEGIN_CUSTOM_RESOLVE_INFO_EXT"),
                Self::BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV => {
                    Some("BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV")
                }
                Self::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO => {
                    Some("BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO")
                }
                Self::BIND_BUFFER_MEMORY_INFO => Some("BIND_BUFFER_MEMORY_INFO"),
                Self::BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_INFO_ARM => {
                    Some("BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_INFO_ARM")
                }
                Self::BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT => {
                    Some("BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT")
                }
                Self::BIND_DESCRIPTOR_SETS_INFO => Some("BIND_DESCRIPTOR_SETS_INFO"),
                Self::BIND_HEAP_INFO_EXT => Some("BIND_HEAP_INFO_EXT"),
                Self::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO => {
                    Some("BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO")
                }
                Self::BIND_IMAGE_MEMORY_INFO => Some("BIND_IMAGE_MEMORY_INFO"),
                Self::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR => {
                    Some("BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR")
                }
                Self::BIND_IMAGE_PLANE_MEMORY_INFO => Some("BIND_IMAGE_PLANE_MEMORY_INFO"),
                Self::BIND_MEMORY_STATUS => Some("BIND_MEMORY_STATUS"),
                Self::BIND_TENSOR_MEMORY_INFO_ARM => Some("BIND_TENSOR_MEMORY_INFO_ARM"),
                Self::BIND_VIDEO_SESSION_MEMORY_INFO_KHR => {
                    Some("BIND_VIDEO_SESSION_MEMORY_INFO_KHR")
                }
                Self::BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM => {
                    Some("BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM")
                }
                Self::BLIT_IMAGE_INFO_2 => Some("BLIT_IMAGE_INFO_2"),
                Self::BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => {
                    Some("BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT")
                }
                Self::BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA => {
                    Some("BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA")
                }
                Self::BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA => {
                    Some("BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA")
                }
                Self::BUFFER_COLLECTION_CREATE_INFO_FUCHSIA => {
                    Some("BUFFER_COLLECTION_CREATE_INFO_FUCHSIA")
                }
                Self::BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA => {
                    Some("BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA")
                }
                Self::BUFFER_COLLECTION_PROPERTIES_FUCHSIA => {
                    Some("BUFFER_COLLECTION_PROPERTIES_FUCHSIA")
                }
                Self::BUFFER_CONSTRAINTS_INFO_FUCHSIA => Some("BUFFER_CONSTRAINTS_INFO_FUCHSIA"),
                Self::BUFFER_COPY_2 => Some("BUFFER_COPY_2"),
                Self::BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT => {
                    Some("BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT")
                }
                Self::BUFFER_DEVICE_ADDRESS_INFO => Some("BUFFER_DEVICE_ADDRESS_INFO"),
                Self::BUFFER_IMAGE_COPY_2 => Some("BUFFER_IMAGE_COPY_2"),
                Self::BUFFER_MEMORY_BARRIER_2 => Some("BUFFER_MEMORY_BARRIER_2"),
                Self::BUFFER_MEMORY_REQUIREMENTS_INFO_2 => {
                    Some("BUFFER_MEMORY_REQUIREMENTS_INFO_2")
                }
                Self::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO => {
                    Some("BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO")
                }
                Self::BUFFER_USAGE_FLAGS_2_CREATE_INFO => Some("BUFFER_USAGE_FLAGS_2_CREATE_INFO"),
                Self::BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV => {
                    Some("BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV")
                }
                Self::CALIBRATED_TIMESTAMP_INFO_KHR => Some("CALIBRATED_TIMESTAMP_INFO_KHR"),
                Self::CHECKPOINT_DATA_2_NV => Some("CHECKPOINT_DATA_2_NV"),
                Self::CHECKPOINT_DATA_NV => Some("CHECKPOINT_DATA_NV"),
                Self::CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV => {
                    Some("CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV")
                }
                Self::CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV => {
                    Some("CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV")
                }
                Self::CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV => {
                    Some("CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV")
                }
                Self::CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV => {
                    Some("CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV")
                }
                Self::CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV => {
                    Some("CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV")
                }
                Self::COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT => {
                    Some("COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT")
                }
                Self::COMMAND_BUFFER_INHERITANCE_DESCRIPTOR_HEAP_INFO_EXT => {
                    Some("COMMAND_BUFFER_INHERITANCE_DESCRIPTOR_HEAP_INFO_EXT")
                }
                Self::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO => {
                    Some("COMMAND_BUFFER_INHERITANCE_RENDERING_INFO")
                }
                Self::COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM => {
                    Some("COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM")
                }
                Self::COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV => {
                    Some("COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV")
                }
                Self::COMMAND_BUFFER_SUBMIT_INFO => Some("COMMAND_BUFFER_SUBMIT_INFO"),
                Self::COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV => {
                    Some("COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV")
                }
                Self::COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV => {
                    Some("COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV")
                }
                Self::CONDITIONAL_RENDERING_BEGIN_INFO_EXT => {
                    Some("CONDITIONAL_RENDERING_BEGIN_INFO_EXT")
                }
                Self::CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV => {
                    Some("CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV")
                }
                Self::COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV => {
                    Some("COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV")
                }
                Self::COOPERATIVE_MATRIX_PROPERTIES_KHR => {
                    Some("COOPERATIVE_MATRIX_PROPERTIES_KHR")
                }
                Self::COOPERATIVE_MATRIX_PROPERTIES_NV => Some("COOPERATIVE_MATRIX_PROPERTIES_NV"),
                Self::COOPERATIVE_VECTOR_PROPERTIES_NV => Some("COOPERATIVE_VECTOR_PROPERTIES_NV"),
                Self::COPY_ACCELERATION_STRUCTURE_INFO_KHR => {
                    Some("COPY_ACCELERATION_STRUCTURE_INFO_KHR")
                }
                Self::COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR => {
                    Some("COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR")
                }
                Self::COPY_BUFFER_INFO_2 => Some("COPY_BUFFER_INFO_2"),
                Self::COPY_BUFFER_TO_IMAGE_INFO_2 => Some("COPY_BUFFER_TO_IMAGE_INFO_2"),
                Self::COPY_COMMAND_TRANSFORM_INFO_QCOM => Some("COPY_COMMAND_TRANSFORM_INFO_QCOM"),
                Self::COPY_IMAGE_INFO_2 => Some("COPY_IMAGE_INFO_2"),
                Self::COPY_IMAGE_TO_BUFFER_INFO_2 => Some("COPY_IMAGE_TO_BUFFER_INFO_2"),
                Self::COPY_IMAGE_TO_IMAGE_INFO => Some("COPY_IMAGE_TO_IMAGE_INFO"),
                Self::COPY_IMAGE_TO_MEMORY_INFO => Some("COPY_IMAGE_TO_MEMORY_INFO"),
                Self::COPY_MEMORY_INDIRECT_INFO_KHR => Some("COPY_MEMORY_INDIRECT_INFO_KHR"),
                Self::COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR => {
                    Some("COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR")
                }
                Self::COPY_MEMORY_TO_IMAGE_INDIRECT_INFO_KHR => {
                    Some("COPY_MEMORY_TO_IMAGE_INDIRECT_INFO_KHR")
                }
                Self::COPY_MEMORY_TO_IMAGE_INFO => Some("COPY_MEMORY_TO_IMAGE_INFO"),
                Self::COPY_MEMORY_TO_MICROMAP_INFO_EXT => Some("COPY_MEMORY_TO_MICROMAP_INFO_EXT"),
                Self::COPY_MICROMAP_INFO_EXT => Some("COPY_MICROMAP_INFO_EXT"),
                Self::COPY_MICROMAP_TO_MEMORY_INFO_EXT => Some("COPY_MICROMAP_TO_MEMORY_INFO_EXT"),
                Self::COPY_TENSOR_INFO_ARM => Some("COPY_TENSOR_INFO_ARM"),
                Self::CUDA_FUNCTION_CREATE_INFO_NV => Some("CUDA_FUNCTION_CREATE_INFO_NV"),
                Self::CUDA_LAUNCH_INFO_NV => Some("CUDA_LAUNCH_INFO_NV"),
                Self::CUDA_MODULE_CREATE_INFO_NV => Some("CUDA_MODULE_CREATE_INFO_NV"),
                Self::CUSTOM_RESOLVE_CREATE_INFO_EXT => Some("CUSTOM_RESOLVE_CREATE_INFO_EXT"),
                Self::CU_FUNCTION_CREATE_INFO_NVX => Some("CU_FUNCTION_CREATE_INFO_NVX"),
                Self::CU_LAUNCH_INFO_NVX => Some("CU_LAUNCH_INFO_NVX"),
                Self::CU_MODULE_CREATE_INFO_NVX => Some("CU_MODULE_CREATE_INFO_NVX"),
                Self::CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX => {
                    Some("CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX")
                }
                Self::D3D12_FENCE_SUBMIT_INFO_KHR => Some("D3D12_FENCE_SUBMIT_INFO_KHR"),
                Self::DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM => {
                    Some("DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM")
                }
                Self::DATA_GRAPH_PIPELINE_COMPILER_CONTROL_CREATE_INFO_ARM => {
                    Some("DATA_GRAPH_PIPELINE_COMPILER_CONTROL_CREATE_INFO_ARM")
                }
                Self::DATA_GRAPH_PIPELINE_CONSTANT_ARM => Some("DATA_GRAPH_PIPELINE_CONSTANT_ARM"),
                Self::DATA_GRAPH_PIPELINE_CONSTANT_TENSOR_SEMI_STRUCTURED_SPARSITY_INFO_ARM => {
                    Some("DATA_GRAPH_PIPELINE_CONSTANT_TENSOR_SEMI_STRUCTURED_SPARSITY_INFO_ARM")
                }
                Self::DATA_GRAPH_PIPELINE_CREATE_INFO_ARM => {
                    Some("DATA_GRAPH_PIPELINE_CREATE_INFO_ARM")
                }
                Self::DATA_GRAPH_PIPELINE_DISPATCH_INFO_ARM => {
                    Some("DATA_GRAPH_PIPELINE_DISPATCH_INFO_ARM")
                }
                Self::DATA_GRAPH_PIPELINE_IDENTIFIER_CREATE_INFO_ARM => {
                    Some("DATA_GRAPH_PIPELINE_IDENTIFIER_CREATE_INFO_ARM")
                }
                Self::DATA_GRAPH_PIPELINE_INFO_ARM => Some("DATA_GRAPH_PIPELINE_INFO_ARM"),
                Self::DATA_GRAPH_PIPELINE_PROPERTY_QUERY_RESULT_ARM => {
                    Some("DATA_GRAPH_PIPELINE_PROPERTY_QUERY_RESULT_ARM")
                }
                Self::DATA_GRAPH_PIPELINE_RESOURCE_INFO_ARM => {
                    Some("DATA_GRAPH_PIPELINE_RESOURCE_INFO_ARM")
                }
                Self::DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_INFO_ARM => {
                    Some("DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_INFO_ARM")
                }
                Self::DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENT_ARM => {
                    Some("DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENT_ARM")
                }
                Self::DATA_GRAPH_PIPELINE_SESSION_CREATE_INFO_ARM => {
                    Some("DATA_GRAPH_PIPELINE_SESSION_CREATE_INFO_ARM")
                }
                Self::DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_INFO_ARM => {
                    Some("DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_INFO_ARM")
                }
                Self::DATA_GRAPH_PIPELINE_SHADER_MODULE_CREATE_INFO_ARM => {
                    Some("DATA_GRAPH_PIPELINE_SHADER_MODULE_CREATE_INFO_ARM")
                }
                Self::DATA_GRAPH_PROCESSING_ENGINE_CREATE_INFO_ARM => {
                    Some("DATA_GRAPH_PROCESSING_ENGINE_CREATE_INFO_ARM")
                }
                Self::DEBUG_MARKER_MARKER_INFO_EXT => Some("DEBUG_MARKER_MARKER_INFO_EXT"),
                Self::DEBUG_MARKER_OBJECT_NAME_INFO_EXT => {
                    Some("DEBUG_MARKER_OBJECT_NAME_INFO_EXT")
                }
                Self::DEBUG_MARKER_OBJECT_TAG_INFO_EXT => Some("DEBUG_MARKER_OBJECT_TAG_INFO_EXT"),
                Self::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT => {
                    Some("DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT")
                }
                Self::DEBUG_UTILS_LABEL_EXT => Some("DEBUG_UTILS_LABEL_EXT"),
                Self::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT => {
                    Some("DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT")
                }
                Self::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT => {
                    Some("DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT")
                }
                Self::DEBUG_UTILS_OBJECT_NAME_INFO_EXT => Some("DEBUG_UTILS_OBJECT_NAME_INFO_EXT"),
                Self::DEBUG_UTILS_OBJECT_TAG_INFO_EXT => Some("DEBUG_UTILS_OBJECT_TAG_INFO_EXT"),
                Self::DECOMPRESS_MEMORY_INFO_EXT => Some("DECOMPRESS_MEMORY_INFO_EXT"),
                Self::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV => {
                    Some("DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV")
                }
                Self::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV => {
                    Some("DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV")
                }
                Self::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV => {
                    Some("DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV")
                }
                Self::DEPENDENCY_INFO => Some("DEPENDENCY_INFO"),
                Self::DEPTH_BIAS_INFO_EXT => Some("DEPTH_BIAS_INFO_EXT"),
                Self::DEPTH_BIAS_REPRESENTATION_INFO_EXT => {
                    Some("DEPTH_BIAS_REPRESENTATION_INFO_EXT")
                }
                Self::DESCRIPTOR_ADDRESS_INFO_EXT => Some("DESCRIPTOR_ADDRESS_INFO_EXT"),
                Self::DESCRIPTOR_BUFFER_BINDING_INFO_EXT => {
                    Some("DESCRIPTOR_BUFFER_BINDING_INFO_EXT")
                }
                Self::DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT => {
                    Some("DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT")
                }
                Self::DESCRIPTOR_GET_INFO_EXT => Some("DESCRIPTOR_GET_INFO_EXT"),
                Self::DESCRIPTOR_GET_TENSOR_INFO_ARM => Some("DESCRIPTOR_GET_TENSOR_INFO_ARM"),
                Self::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO => {
                    Some("DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO")
                }
                Self::DESCRIPTOR_SET_AND_BINDING_MAPPING_EXT => {
                    Some("DESCRIPTOR_SET_AND_BINDING_MAPPING_EXT")
                }
                Self::DESCRIPTOR_SET_BINDING_REFERENCE_VALVE => {
                    Some("DESCRIPTOR_SET_BINDING_REFERENCE_VALVE")
                }
                Self::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO => {
                    Some("DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO")
                }
                Self::DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE => {
                    Some("DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE")
                }
                Self::DESCRIPTOR_SET_LAYOUT_SUPPORT => Some("DESCRIPTOR_SET_LAYOUT_SUPPORT"),
                Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO => {
                    Some("DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO")
                }
                Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT => {
                    Some("DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT")
                }
                Self::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO => {
                    Some("DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO")
                }
                Self::DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT => {
                    Some("DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT")
                }
                Self::DEVICE_BUFFER_MEMORY_REQUIREMENTS => {
                    Some("DEVICE_BUFFER_MEMORY_REQUIREMENTS")
                }
                Self::DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT => {
                    Some("DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT")
                }
                Self::DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV => {
                    Some("DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV")
                }
                Self::DEVICE_EVENT_INFO_EXT => Some("DEVICE_EVENT_INFO_EXT"),
                Self::DEVICE_FAULT_COUNTS_EXT => Some("DEVICE_FAULT_COUNTS_EXT"),
                Self::DEVICE_FAULT_INFO_EXT => Some("DEVICE_FAULT_INFO_EXT"),
                Self::DEVICE_GROUP_BIND_SPARSE_INFO => Some("DEVICE_GROUP_BIND_SPARSE_INFO"),
                Self::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO => {
                    Some("DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO")
                }
                Self::DEVICE_GROUP_DEVICE_CREATE_INFO => Some("DEVICE_GROUP_DEVICE_CREATE_INFO"),
                Self::DEVICE_GROUP_PRESENT_CAPABILITIES_KHR => {
                    Some("DEVICE_GROUP_PRESENT_CAPABILITIES_KHR")
                }
                Self::DEVICE_GROUP_PRESENT_INFO_KHR => Some("DEVICE_GROUP_PRESENT_INFO_KHR"),
                Self::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO => {
                    Some("DEVICE_GROUP_RENDER_PASS_BEGIN_INFO")
                }
                Self::DEVICE_GROUP_SUBMIT_INFO => Some("DEVICE_GROUP_SUBMIT_INFO"),
                Self::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR => {
                    Some("DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR")
                }
                Self::DEVICE_IMAGE_MEMORY_REQUIREMENTS => Some("DEVICE_IMAGE_MEMORY_REQUIREMENTS"),
                Self::DEVICE_IMAGE_SUBRESOURCE_INFO => Some("DEVICE_IMAGE_SUBRESOURCE_INFO"),
                Self::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO => {
                    Some("DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO")
                }
                Self::DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD => {
                    Some("DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD")
                }
                Self::DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT => {
                    Some("DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT")
                }
                Self::DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR => {
                    Some("DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR")
                }
                Self::DEVICE_PRIVATE_DATA_CREATE_INFO => Some("DEVICE_PRIVATE_DATA_CREATE_INFO"),
                Self::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO => {
                    Some("DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO")
                }
                Self::DEVICE_QUEUE_INFO_2 => Some("DEVICE_QUEUE_INFO_2"),
                Self::DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM => {
                    Some("DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM")
                }
                Self::DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM => {
                    Some("DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM")
                }
                Self::DIRECTFB_SURFACE_CREATE_INFO_EXT => Some("DIRECTFB_SURFACE_CREATE_INFO_EXT"),
                Self::DIRECT_DRIVER_LOADING_INFO_LUNARG => {
                    Some("DIRECT_DRIVER_LOADING_INFO_LUNARG")
                }
                Self::DIRECT_DRIVER_LOADING_LIST_LUNARG => {
                    Some("DIRECT_DRIVER_LOADING_LIST_LUNARG")
                }
                Self::DISPATCH_TILE_INFO_QCOM => Some("DISPATCH_TILE_INFO_QCOM"),
                Self::DISPLAY_EVENT_INFO_EXT => Some("DISPLAY_EVENT_INFO_EXT"),
                Self::DISPLAY_MODE_CREATE_INFO_KHR => Some("DISPLAY_MODE_CREATE_INFO_KHR"),
                Self::DISPLAY_MODE_PROPERTIES_2_KHR => Some("DISPLAY_MODE_PROPERTIES_2_KHR"),
                Self::DISPLAY_MODE_STEREO_PROPERTIES_NV => {
                    Some("DISPLAY_MODE_STEREO_PROPERTIES_NV")
                }
                Self::DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD => {
                    Some("DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD")
                }
                Self::DISPLAY_PLANE_CAPABILITIES_2_KHR => Some("DISPLAY_PLANE_CAPABILITIES_2_KHR"),
                Self::DISPLAY_PLANE_INFO_2_KHR => Some("DISPLAY_PLANE_INFO_2_KHR"),
                Self::DISPLAY_PLANE_PROPERTIES_2_KHR => Some("DISPLAY_PLANE_PROPERTIES_2_KHR"),
                Self::DISPLAY_POWER_INFO_EXT => Some("DISPLAY_POWER_INFO_EXT"),
                Self::DISPLAY_PRESENT_INFO_KHR => Some("DISPLAY_PRESENT_INFO_KHR"),
                Self::DISPLAY_PROPERTIES_2_KHR => Some("DISPLAY_PROPERTIES_2_KHR"),
                Self::DISPLAY_SURFACE_CREATE_INFO_KHR => Some("DISPLAY_SURFACE_CREATE_INFO_KHR"),
                Self::DISPLAY_SURFACE_STEREO_CREATE_INFO_NV => {
                    Some("DISPLAY_SURFACE_STEREO_CREATE_INFO_NV")
                }
                Self::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT => {
                    Some("DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT")
                }
                Self::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT => {
                    Some("DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT")
                }
                Self::EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX => {
                    Some("EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX")
                }
                Self::EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX => {
                    Some("EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX")
                }
                Self::EXPORT_FENCE_CREATE_INFO => Some("EXPORT_FENCE_CREATE_INFO"),
                Self::EXPORT_FENCE_WIN32_HANDLE_INFO_KHR => {
                    Some("EXPORT_FENCE_WIN32_HANDLE_INFO_KHR")
                }
                Self::EXPORT_MEMORY_ALLOCATE_INFO => Some("EXPORT_MEMORY_ALLOCATE_INFO"),
                Self::EXPORT_MEMORY_ALLOCATE_INFO_NV => Some("EXPORT_MEMORY_ALLOCATE_INFO_NV"),
                Self::EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR => {
                    Some("EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR")
                }
                Self::EXPORT_MEMORY_WIN32_HANDLE_INFO_NV => {
                    Some("EXPORT_MEMORY_WIN32_HANDLE_INFO_NV")
                }
                Self::EXPORT_METAL_BUFFER_INFO_EXT => Some("EXPORT_METAL_BUFFER_INFO_EXT"),
                Self::EXPORT_METAL_COMMAND_QUEUE_INFO_EXT => {
                    Some("EXPORT_METAL_COMMAND_QUEUE_INFO_EXT")
                }
                Self::EXPORT_METAL_DEVICE_INFO_EXT => Some("EXPORT_METAL_DEVICE_INFO_EXT"),
                Self::EXPORT_METAL_IO_SURFACE_INFO_EXT => Some("EXPORT_METAL_IO_SURFACE_INFO_EXT"),
                Self::EXPORT_METAL_OBJECTS_INFO_EXT => Some("EXPORT_METAL_OBJECTS_INFO_EXT"),
                Self::EXPORT_METAL_OBJECT_CREATE_INFO_EXT => {
                    Some("EXPORT_METAL_OBJECT_CREATE_INFO_EXT")
                }
                Self::EXPORT_METAL_SHARED_EVENT_INFO_EXT => {
                    Some("EXPORT_METAL_SHARED_EVENT_INFO_EXT")
                }
                Self::EXPORT_METAL_TEXTURE_INFO_EXT => Some("EXPORT_METAL_TEXTURE_INFO_EXT"),
                Self::EXPORT_SEMAPHORE_CREATE_INFO => Some("EXPORT_SEMAPHORE_CREATE_INFO"),
                Self::EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR => {
                    Some("EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR")
                }
                Self::EXTERNAL_BUFFER_PROPERTIES => Some("EXTERNAL_BUFFER_PROPERTIES"),
                Self::EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV => {
                    Some("EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV")
                }
                Self::EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV => {
                    Some("EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV")
                }
                Self::EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV => {
                    Some("EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV")
                }
                Self::EXTERNAL_FENCE_PROPERTIES => Some("EXTERNAL_FENCE_PROPERTIES"),
                Self::EXTERNAL_FORMAT_ANDROID => Some("EXTERNAL_FORMAT_ANDROID"),
                Self::EXTERNAL_FORMAT_OHOS => Some("EXTERNAL_FORMAT_OHOS"),
                Self::EXTERNAL_FORMAT_QNX => Some("EXTERNAL_FORMAT_QNX"),
                Self::EXTERNAL_IMAGE_FORMAT_PROPERTIES => Some("EXTERNAL_IMAGE_FORMAT_PROPERTIES"),
                Self::EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXT => {
                    Some("EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXT")
                }
                Self::EXTERNAL_MEMORY_BUFFER_CREATE_INFO => {
                    Some("EXTERNAL_MEMORY_BUFFER_CREATE_INFO")
                }
                Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO => {
                    Some("EXTERNAL_MEMORY_IMAGE_CREATE_INFO")
                }
                Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV => {
                    Some("EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV")
                }
                Self::EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM => {
                    Some("EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM")
                }
                Self::EXTERNAL_SEMAPHORE_PROPERTIES => Some("EXTERNAL_SEMAPHORE_PROPERTIES"),
                Self::EXTERNAL_TENSOR_PROPERTIES_ARM => Some("EXTERNAL_TENSOR_PROPERTIES_ARM"),
                Self::FENCE_GET_FD_INFO_KHR => Some("FENCE_GET_FD_INFO_KHR"),
                Self::FENCE_GET_WIN32_HANDLE_INFO_KHR => Some("FENCE_GET_WIN32_HANDLE_INFO_KHR"),
                Self::FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT => {
                    Some("FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT")
                }
                Self::FORMAT_PROPERTIES_2 => Some("FORMAT_PROPERTIES_2"),
                Self::FORMAT_PROPERTIES_3 => Some("FORMAT_PROPERTIES_3"),
                Self::FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR => {
                    Some("FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR")
                }
                Self::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO => {
                    Some("FRAMEBUFFER_ATTACHMENTS_CREATE_INFO")
                }
                Self::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO => {
                    Some("FRAMEBUFFER_ATTACHMENT_IMAGE_INFO")
                }
                Self::FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV => {
                    Some("FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV")
                }
                Self::FRAME_BOUNDARY_EXT => Some("FRAME_BOUNDARY_EXT"),
                Self::FRAME_BOUNDARY_TENSORS_ARM => Some("FRAME_BOUNDARY_TENSORS_ARM"),
                Self::GENERATED_COMMANDS_INFO_EXT => Some("GENERATED_COMMANDS_INFO_EXT"),
                Self::GENERATED_COMMANDS_INFO_NV => Some("GENERATED_COMMANDS_INFO_NV"),
                Self::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_EXT => {
                    Some("GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_EXT")
                }
                Self::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV => {
                    Some("GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV")
                }
                Self::GENERATED_COMMANDS_PIPELINE_INFO_EXT => {
                    Some("GENERATED_COMMANDS_PIPELINE_INFO_EXT")
                }
                Self::GENERATED_COMMANDS_SHADER_INFO_EXT => {
                    Some("GENERATED_COMMANDS_SHADER_INFO_EXT")
                }
                Self::GEOMETRY_AABB_NV => Some("GEOMETRY_AABB_NV"),
                Self::GEOMETRY_NV => Some("GEOMETRY_NV"),
                Self::GEOMETRY_TRIANGLES_NV => Some("GEOMETRY_TRIANGLES_NV"),
                Self::GET_LATENCY_MARKER_INFO_NV => Some("GET_LATENCY_MARKER_INFO_NV"),
                Self::GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT => {
                    Some("GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT")
                }
                Self::GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV => {
                    Some("GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV")
                }
                Self::GRAPHICS_SHADER_GROUP_CREATE_INFO_NV => {
                    Some("GRAPHICS_SHADER_GROUP_CREATE_INFO_NV")
                }
                Self::HDR_METADATA_EXT => Some("HDR_METADATA_EXT"),
                Self::HDR_VIVID_DYNAMIC_METADATA_HUAWEI => {
                    Some("HDR_VIVID_DYNAMIC_METADATA_HUAWEI")
                }
                Self::HEADLESS_SURFACE_CREATE_INFO_EXT => Some("HEADLESS_SURFACE_CREATE_INFO_EXT"),
                Self::HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY => {
                    Some("HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY")
                }
                Self::HOST_IMAGE_LAYOUT_TRANSITION_INFO => {
                    Some("HOST_IMAGE_LAYOUT_TRANSITION_INFO")
                }
                Self::IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA => {
                    Some("IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA")
                }
                Self::IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA => {
                    Some("IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA")
                }
                Self::IMAGE_BLIT_2 => Some("IMAGE_BLIT_2"),
                Self::IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => {
                    Some("IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT")
                }
                Self::IMAGE_COMPRESSION_CONTROL_EXT => Some("IMAGE_COMPRESSION_CONTROL_EXT"),
                Self::IMAGE_COMPRESSION_PROPERTIES_EXT => Some("IMAGE_COMPRESSION_PROPERTIES_EXT"),
                Self::IMAGE_CONSTRAINTS_INFO_FUCHSIA => Some("IMAGE_CONSTRAINTS_INFO_FUCHSIA"),
                Self::IMAGE_COPY_2 => Some("IMAGE_COPY_2"),
                Self::IMAGE_DESCRIPTOR_INFO_EXT => Some("IMAGE_DESCRIPTOR_INFO_EXT"),
                Self::IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT => {
                    Some("IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT")
                }
                Self::IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT => {
                    Some("IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT")
                }
                Self::IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT => {
                    Some("IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT")
                }
                Self::IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA => {
                    Some("IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA")
                }
                Self::IMAGE_FORMAT_LIST_CREATE_INFO => Some("IMAGE_FORMAT_LIST_CREATE_INFO"),
                Self::IMAGE_FORMAT_PROPERTIES_2 => Some("IMAGE_FORMAT_PROPERTIES_2"),
                Self::IMAGE_MEMORY_BARRIER_2 => Some("IMAGE_MEMORY_BARRIER_2"),
                Self::IMAGE_MEMORY_REQUIREMENTS_INFO_2 => Some("IMAGE_MEMORY_REQUIREMENTS_INFO_2"),
                Self::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO => {
                    Some("IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO")
                }
                Self::IMAGE_RESOLVE_2 => Some("IMAGE_RESOLVE_2"),
                Self::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2 => {
                    Some("IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2")
                }
                Self::IMAGE_STENCIL_USAGE_CREATE_INFO => Some("IMAGE_STENCIL_USAGE_CREATE_INFO"),
                Self::IMAGE_SUBRESOURCE_2 => Some("IMAGE_SUBRESOURCE_2"),
                Self::IMAGE_SWAPCHAIN_CREATE_INFO_KHR => Some("IMAGE_SWAPCHAIN_CREATE_INFO_KHR"),
                Self::IMAGE_TO_MEMORY_COPY => Some("IMAGE_TO_MEMORY_COPY"),
                Self::IMAGE_VIEW_ADDRESS_PROPERTIES_NVX => {
                    Some("IMAGE_VIEW_ADDRESS_PROPERTIES_NVX")
                }
                Self::IMAGE_VIEW_ASTC_DECODE_MODE_EXT => Some("IMAGE_VIEW_ASTC_DECODE_MODE_EXT"),
                Self::IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => {
                    Some("IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT")
                }
                Self::IMAGE_VIEW_HANDLE_INFO_NVX => Some("IMAGE_VIEW_HANDLE_INFO_NVX"),
                Self::IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT => {
                    Some("IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT")
                }
                Self::IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM => {
                    Some("IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM")
                }
                Self::IMAGE_VIEW_SLICED_CREATE_INFO_EXT => {
                    Some("IMAGE_VIEW_SLICED_CREATE_INFO_EXT")
                }
                Self::IMAGE_VIEW_USAGE_CREATE_INFO => Some("IMAGE_VIEW_USAGE_CREATE_INFO"),
                Self::IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID => {
                    Some("IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID")
                }
                Self::IMPORT_FENCE_FD_INFO_KHR => Some("IMPORT_FENCE_FD_INFO_KHR"),
                Self::IMPORT_FENCE_WIN32_HANDLE_INFO_KHR => {
                    Some("IMPORT_FENCE_WIN32_HANDLE_INFO_KHR")
                }
                Self::IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA => {
                    Some("IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA")
                }
                Self::IMPORT_MEMORY_FD_INFO_KHR => Some("IMPORT_MEMORY_FD_INFO_KHR"),
                Self::IMPORT_MEMORY_HOST_POINTER_INFO_EXT => {
                    Some("IMPORT_MEMORY_HOST_POINTER_INFO_EXT")
                }
                Self::IMPORT_MEMORY_METAL_HANDLE_INFO_EXT => {
                    Some("IMPORT_MEMORY_METAL_HANDLE_INFO_EXT")
                }
                Self::IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR => {
                    Some("IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR")
                }
                Self::IMPORT_MEMORY_WIN32_HANDLE_INFO_NV => {
                    Some("IMPORT_MEMORY_WIN32_HANDLE_INFO_NV")
                }
                Self::IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA => {
                    Some("IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA")
                }
                Self::IMPORT_METAL_BUFFER_INFO_EXT => Some("IMPORT_METAL_BUFFER_INFO_EXT"),
                Self::IMPORT_METAL_IO_SURFACE_INFO_EXT => Some("IMPORT_METAL_IO_SURFACE_INFO_EXT"),
                Self::IMPORT_METAL_SHARED_EVENT_INFO_EXT => {
                    Some("IMPORT_METAL_SHARED_EVENT_INFO_EXT")
                }
                Self::IMPORT_METAL_TEXTURE_INFO_EXT => Some("IMPORT_METAL_TEXTURE_INFO_EXT"),
                Self::IMPORT_NATIVE_BUFFER_INFO_OHOS => Some("IMPORT_NATIVE_BUFFER_INFO_OHOS"),
                Self::IMPORT_SCREEN_BUFFER_INFO_QNX => Some("IMPORT_SCREEN_BUFFER_INFO_QNX"),
                Self::IMPORT_SEMAPHORE_FD_INFO_KHR => Some("IMPORT_SEMAPHORE_FD_INFO_KHR"),
                Self::IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR => {
                    Some("IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR")
                }
                Self::IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA => {
                    Some("IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA")
                }
                Self::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_EXT => {
                    Some("INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_EXT")
                }
                Self::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV => {
                    Some("INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV")
                }
                Self::INDIRECT_COMMANDS_LAYOUT_PUSH_DATA_TOKEN_NV => {
                    Some("INDIRECT_COMMANDS_LAYOUT_PUSH_DATA_TOKEN_NV")
                }
                Self::INDIRECT_COMMANDS_LAYOUT_TOKEN_EXT => {
                    Some("INDIRECT_COMMANDS_LAYOUT_TOKEN_EXT")
                }
                Self::INDIRECT_COMMANDS_LAYOUT_TOKEN_NV => {
                    Some("INDIRECT_COMMANDS_LAYOUT_TOKEN_NV")
                }
                Self::INDIRECT_EXECUTION_SET_CREATE_INFO_EXT => {
                    Some("INDIRECT_EXECUTION_SET_CREATE_INFO_EXT")
                }
                Self::INDIRECT_EXECUTION_SET_PIPELINE_INFO_EXT => {
                    Some("INDIRECT_EXECUTION_SET_PIPELINE_INFO_EXT")
                }
                Self::INDIRECT_EXECUTION_SET_SHADER_INFO_EXT => {
                    Some("INDIRECT_EXECUTION_SET_SHADER_INFO_EXT")
                }
                Self::INDIRECT_EXECUTION_SET_SHADER_LAYOUT_INFO_EXT => {
                    Some("INDIRECT_EXECUTION_SET_SHADER_LAYOUT_INFO_EXT")
                }
                Self::INITIALIZE_PERFORMANCE_API_INFO_INTEL => {
                    Some("INITIALIZE_PERFORMANCE_API_INFO_INTEL")
                }
                Self::IOS_SURFACE_CREATE_INFO_MVK => Some("IOS_SURFACE_CREATE_INFO_MVK"),
                Self::LATENCY_SLEEP_INFO_NV => Some("LATENCY_SLEEP_INFO_NV"),
                Self::LATENCY_SLEEP_MODE_INFO_NV => Some("LATENCY_SLEEP_MODE_INFO_NV"),
                Self::LATENCY_SUBMISSION_PRESENT_ID_NV => Some("LATENCY_SUBMISSION_PRESENT_ID_NV"),
                Self::LATENCY_SURFACE_CAPABILITIES_NV => Some("LATENCY_SURFACE_CAPABILITIES_NV"),
                Self::LATENCY_TIMINGS_FRAME_REPORT_NV => Some("LATENCY_TIMINGS_FRAME_REPORT_NV"),
                Self::LAYER_SETTINGS_CREATE_INFO_EXT => Some("LAYER_SETTINGS_CREATE_INFO_EXT"),
                Self::MACOS_SURFACE_CREATE_INFO_MVK => Some("MACOS_SURFACE_CREATE_INFO_MVK"),
                Self::MEMORY_ALLOCATE_FLAGS_INFO => Some("MEMORY_ALLOCATE_FLAGS_INFO"),
                Self::MEMORY_BARRIER_2 => Some("MEMORY_BARRIER_2"),
                Self::MEMORY_BARRIER_ACCESS_FLAGS_3_KHR => {
                    Some("MEMORY_BARRIER_ACCESS_FLAGS_3_KHR")
                }
                Self::MEMORY_DEDICATED_ALLOCATE_INFO => Some("MEMORY_DEDICATED_ALLOCATE_INFO"),
                Self::MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM => {
                    Some("MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM")
                }
                Self::MEMORY_DEDICATED_REQUIREMENTS => Some("MEMORY_DEDICATED_REQUIREMENTS"),
                Self::MEMORY_FD_PROPERTIES_KHR => Some("MEMORY_FD_PROPERTIES_KHR"),
                Self::MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID => {
                    Some("MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID")
                }
                Self::MEMORY_GET_FD_INFO_KHR => Some("MEMORY_GET_FD_INFO_KHR"),
                Self::MEMORY_GET_METAL_HANDLE_INFO_EXT => Some("MEMORY_GET_METAL_HANDLE_INFO_EXT"),
                Self::MEMORY_GET_NATIVE_BUFFER_INFO_OHOS => {
                    Some("MEMORY_GET_NATIVE_BUFFER_INFO_OHOS")
                }
                Self::MEMORY_GET_REMOTE_ADDRESS_INFO_NV => {
                    Some("MEMORY_GET_REMOTE_ADDRESS_INFO_NV")
                }
                Self::MEMORY_GET_WIN32_HANDLE_INFO_KHR => Some("MEMORY_GET_WIN32_HANDLE_INFO_KHR"),
                Self::MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA => {
                    Some("MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA")
                }
                Self::MEMORY_HOST_POINTER_PROPERTIES_EXT => {
                    Some("MEMORY_HOST_POINTER_PROPERTIES_EXT")
                }
                Self::MEMORY_MAP_INFO => Some("MEMORY_MAP_INFO"),
                Self::MEMORY_MAP_PLACED_INFO_EXT => Some("MEMORY_MAP_PLACED_INFO_EXT"),
                Self::MEMORY_METAL_HANDLE_PROPERTIES_EXT => {
                    Some("MEMORY_METAL_HANDLE_PROPERTIES_EXT")
                }
                Self::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO => {
                    Some("MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO")
                }
                Self::MEMORY_PRIORITY_ALLOCATE_INFO_EXT => {
                    Some("MEMORY_PRIORITY_ALLOCATE_INFO_EXT")
                }
                Self::MEMORY_REQUIREMENTS_2 => Some("MEMORY_REQUIREMENTS_2"),
                Self::MEMORY_TO_IMAGE_COPY => Some("MEMORY_TO_IMAGE_COPY"),
                Self::MEMORY_UNMAP_INFO => Some("MEMORY_UNMAP_INFO"),
                Self::MEMORY_WIN32_HANDLE_PROPERTIES_KHR => {
                    Some("MEMORY_WIN32_HANDLE_PROPERTIES_KHR")
                }
                Self::MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA => {
                    Some("MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA")
                }
                Self::METAL_SURFACE_CREATE_INFO_EXT => Some("METAL_SURFACE_CREATE_INFO_EXT"),
                Self::MICROMAP_BUILD_INFO_EXT => Some("MICROMAP_BUILD_INFO_EXT"),
                Self::MICROMAP_BUILD_SIZES_INFO_EXT => Some("MICROMAP_BUILD_SIZES_INFO_EXT"),
                Self::MICROMAP_CREATE_INFO_EXT => Some("MICROMAP_CREATE_INFO_EXT"),
                Self::MICROMAP_VERSION_INFO_EXT => Some("MICROMAP_VERSION_INFO_EXT"),
                Self::MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT => {
                    Some("MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT")
                }
                Self::MULTISAMPLE_PROPERTIES_EXT => Some("MULTISAMPLE_PROPERTIES_EXT"),
                Self::MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX => {
                    Some("MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX")
                }
                Self::MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM => {
                    Some("MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM")
                }
                Self::MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT => {
                    Some("MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT")
                }
                Self::NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS => {
                    Some("NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS")
                }
                Self::NATIVE_BUFFER_PROPERTIES_OHOS => Some("NATIVE_BUFFER_PROPERTIES_OHOS"),
                Self::NATIVE_BUFFER_USAGE_OHOS => Some("NATIVE_BUFFER_USAGE_OHOS"),
                Self::OPAQUE_CAPTURE_DATA_CREATE_INFO_EXT => {
                    Some("OPAQUE_CAPTURE_DATA_CREATE_INFO_EXT")
                }
                Self::OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT => {
                    Some("OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT")
                }
                Self::OPTICAL_FLOW_EXECUTE_INFO_NV => Some("OPTICAL_FLOW_EXECUTE_INFO_NV"),
                Self::OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV => {
                    Some("OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV")
                }
                Self::OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV => {
                    Some("OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV")
                }
                Self::OPTICAL_FLOW_SESSION_CREATE_INFO_NV => {
                    Some("OPTICAL_FLOW_SESSION_CREATE_INFO_NV")
                }
                Self::OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV => {
                    Some("OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV")
                }
                Self::OUT_OF_BAND_QUEUE_TYPE_INFO_NV => Some("OUT_OF_BAND_QUEUE_TYPE_INFO_NV"),
                Self::PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV => {
                    Some("PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV")
                }
                Self::PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV => {
                    Some("PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV")
                }
                Self::PAST_PRESENTATION_TIMING_EXT => Some("PAST_PRESENTATION_TIMING_EXT"),
                Self::PAST_PRESENTATION_TIMING_INFO_EXT => {
                    Some("PAST_PRESENTATION_TIMING_INFO_EXT")
                }
                Self::PAST_PRESENTATION_TIMING_PROPERTIES_EXT => {
                    Some("PAST_PRESENTATION_TIMING_PROPERTIES_EXT")
                }
                Self::PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL => {
                    Some("PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL")
                }
                Self::PERFORMANCE_COUNTER_ARM => Some("PERFORMANCE_COUNTER_ARM"),
                Self::PERFORMANCE_COUNTER_DESCRIPTION_ARM => {
                    Some("PERFORMANCE_COUNTER_DESCRIPTION_ARM")
                }
                Self::PERFORMANCE_COUNTER_DESCRIPTION_KHR => {
                    Some("PERFORMANCE_COUNTER_DESCRIPTION_KHR")
                }
                Self::PERFORMANCE_COUNTER_KHR => Some("PERFORMANCE_COUNTER_KHR"),
                Self::PERFORMANCE_MARKER_INFO_INTEL => Some("PERFORMANCE_MARKER_INFO_INTEL"),
                Self::PERFORMANCE_OVERRIDE_INFO_INTEL => Some("PERFORMANCE_OVERRIDE_INFO_INTEL"),
                Self::PERFORMANCE_QUERY_SUBMIT_INFO_KHR => {
                    Some("PERFORMANCE_QUERY_SUBMIT_INFO_KHR")
                }
                Self::PERFORMANCE_STREAM_MARKER_INFO_INTEL => {
                    Some("PERFORMANCE_STREAM_MARKER_INFO_INTEL")
                }
                Self::PER_TILE_BEGIN_INFO_QCOM => Some("PER_TILE_BEGIN_INFO_QCOM"),
                Self::PER_TILE_END_INFO_QCOM => Some("PER_TILE_END_INFO_QCOM"),
                Self::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES => {
                    Some("PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES")
                }
                Self::PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES => {
                    Some("PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES")
                }
                Self::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR => {
                    Some("PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR")
                }
                Self::PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC => {
                    Some("PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC")
                }
                Self::PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD => {
                    Some("PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD")
                }
                Self::PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES => {
                    Some("PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES")
                }
                Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI => {
                    Some("PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI")
                }
                Self::PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI => {
                    Some("PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI")
                }
                Self::PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI => {
                    Some("PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI")
                }
                Self::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD => {
                    Some("PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD")
                }
                Self::PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_PROPERTIES_KHR => {
                    Some("PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_PROPERTIES_KHR")
                }
                Self::PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR => {
                    Some("PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR")
                }
                Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR => {
                    Some("PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR")
                }
                Self::PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM => {
                    Some("PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM")
                }
                Self::PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM => {
                    Some("PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM")
                }
                Self::PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_CUSTOM_RESOLVE_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_CUSTOM_RESOLVE_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_DATA_GRAPH_FEATURES_ARM => {
                    Some("PHYSICAL_DEVICE_DATA_GRAPH_FEATURES_ARM")
                }
                Self::PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM => {
                    Some("PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM")
                }
                Self::PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX => {
                    Some("PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX")
                }
                Self::PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_DEPTH_CLAMP_CONTROL_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_DEPTH_CLAMP_CONTROL_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM => {
                    Some("PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM")
                }
                Self::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM => {
                    Some("PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM")
                }
                Self::PHYSICAL_DEVICE_DESCRIPTOR_HEAP_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_DESCRIPTOR_HEAP_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_DESCRIPTOR_HEAP_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_DESCRIPTOR_HEAP_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_DESCRIPTOR_HEAP_TENSOR_PROPERTIES_ARM => {
                    Some("PHYSICAL_DEVICE_DESCRIPTOR_HEAP_TENSOR_PROPERTIES_ARM")
                }
                Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES => {
                    Some("PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES")
                }
                Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE => {
                    Some("PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE")
                }
                Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_DRIVER_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_DRIVER_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_DRM_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_DRM_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES => {
                    Some("PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES")
                }
                Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES => {
                    Some("PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES")
                }
                Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO => {
                    Some("PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO")
                }
                Self::PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO => {
                    Some("PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO")
                }
                Self::PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID => {
                    Some("PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID")
                }
                Self::PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID => {
                    Some("PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID")
                }
                Self::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO => {
                    Some("PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO")
                }
                Self::PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX => {
                    Some("PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX")
                }
                Self::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO => {
                    Some("PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO")
                }
                Self::PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM => {
                    Some("PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM")
                }
                Self::PHYSICAL_DEVICE_FAULT_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_FAULT_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_FEATURES_2 => Some("PHYSICAL_DEVICE_FEATURES_2"),
                Self::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_FORMAT_PACK_FEATURES_ARM => {
                    Some("PHYSICAL_DEVICE_FORMAT_PACK_FEATURES_ARM")
                }
                Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE => {
                    Some("PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE")
                }
                Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE => {
                    Some("PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE")
                }
                Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR => {
                    Some("PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR")
                }
                Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR => {
                    Some("PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR")
                }
                Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR => {
                    Some("PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR")
                }
                Self::PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES => {
                    Some("PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES")
                }
                Self::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_GROUP_PROPERTIES => Some("PHYSICAL_DEVICE_GROUP_PROPERTIES"),
                Self::PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI => {
                    Some("PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI")
                }
                Self::PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES => {
                    Some("PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES")
                }
                Self::PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES => {
                    Some("PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES")
                }
                Self::PHYSICAL_DEVICE_ID_PROPERTIES => Some("PHYSICAL_DEVICE_ID_PROPERTIES"),
                Self::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES => {
                    Some("PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES")
                }
                Self::PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA => {
                    Some("PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA")
                }
                Self::PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA => {
                    Some("PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA")
                }
                Self::PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT => {
                    Some("PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT")
                }
                Self::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2 => {
                    Some("PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2")
                }
                Self::PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM => {
                    Some("PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM")
                }
                Self::PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM => {
                    Some("PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM")
                }
                Self::PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM => {
                    Some("PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM")
                }
                Self::PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM => {
                    Some("PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM")
                }
                Self::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES => {
                    Some("PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES")
                }
                Self::PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT => {
                    Some("PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT")
                }
                Self::PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES => {
                    Some("PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES")
                }
                Self::PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES => {
                    Some("PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES")
                }
                Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_INTERNALLY_SYNCHRONIZED_QUEUES_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_INTERNALLY_SYNCHRONIZED_QUEUES_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI => {
                    Some("PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI")
                }
                Self::PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR => {
                    Some("PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR")
                }
                Self::PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR => {
                    Some("PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR")
                }
                Self::PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR => {
                    Some("PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR")
                }
                Self::PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT => {
                    Some("PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT")
                }
                Self::PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES => {
                    Some("PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES")
                }
                Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_MAINTENANCE_10_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_MAINTENANCE_10_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_MAINTENANCE_10_PROPERTIES_KHR => {
                    Some("PHYSICAL_DEVICE_MAINTENANCE_10_PROPERTIES_KHR")
                }
                Self::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES => {
                    Some("PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES")
                }
                Self::PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES => {
                    Some("PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES")
                }
                Self::PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES => {
                    Some("PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES")
                }
                Self::PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR => {
                    Some("PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR")
                }
                Self::PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR => {
                    Some("PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR")
                }
                Self::PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_MAP_MEMORY_PLACED_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_MAP_MEMORY_PLACED_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2 => {
                    Some("PHYSICAL_DEVICE_MEMORY_PROPERTIES_2")
                }
                Self::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_MULTIVIEW_FEATURES => {
                    Some("PHYSICAL_DEVICE_MULTIVIEW_FEATURES")
                }
                Self::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX => {
                    Some("PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX")
                }
                Self::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM => {
                    Some("PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM")
                }
                Self::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM => {
                    Some("PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM")
                }
                Self::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM => {
                    Some("PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM")
                }
                Self::PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM => {
                    Some("PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM")
                }
                Self::PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR => {
                    Some("PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR")
                }
                Self::PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR => {
                    Some("PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR")
                }
                Self::PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC => {
                    Some("PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC")
                }
                Self::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES => {
                    Some("PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES")
                }
                Self::PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM => {
                    Some("PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM")
                }
                Self::PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES => {
                    Some("PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES")
                }
                Self::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES => {
                    Some("PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES")
                }
                Self::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR => {
                    Some("PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR")
                }
                Self::PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_PRESENT_TIMING_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_PRESENT_TIMING_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_PRESENT_WAIT_2_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_PRESENT_WAIT_2_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES => {
                    Some("PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES")
                }
                Self::PHYSICAL_DEVICE_PROPERTIES_2 => Some("PHYSICAL_DEVICE_PROPERTIES_2"),
                Self::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES => {
                    Some("PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES")
                }
                Self::PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_INFO_ARM => {
                    Some("PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_INFO_ARM")
                }
                Self::PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR => {
                    Some("PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR")
                }
                Self::PHYSICAL_DEVICE_RAY_TRACING_POSITION_FETCH_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_RAY_TRACING_POSITION_FETCH_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG => {
                    Some("PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG")
                }
                Self::PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM => {
                    Some("PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM")
                }
                Self::PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM => {
                    Some("PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM")
                }
                Self::PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR => {
                    Some("PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR")
                }
                Self::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES => {
                    Some("PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES")
                }
                Self::PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES => {
                    Some("PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES")
                }
                Self::PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM => {
                    Some("PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM")
                }
                Self::PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM => {
                    Some("PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM")
                }
                Self::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES => {
                    Some("PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES")
                }
                Self::PHYSICAL_DEVICE_SHADER_64_BIT_INDEXING_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_SHADER_64_BIT_INDEXING_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES => {
                    Some("PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES")
                }
                Self::PHYSICAL_DEVICE_SHADER_BFLOAT16_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_SHADER_BFLOAT16_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM => {
                    Some("PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM")
                }
                Self::PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM => {
                    Some("PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM")
                }
                Self::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD => {
                    Some("PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD")
                }
                Self::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD => {
                    Some("PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD")
                }
                Self::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM => {
                    Some("PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM")
                }
                Self::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES => {
                    Some("PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES")
                }
                Self::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES => {
                    Some("PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES")
                }
                Self::PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD => {
                    Some("PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD")
                }
                Self::PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX => {
                    Some("PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX")
                }
                Self::PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX => {
                    Some("PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX")
                }
                Self::PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES => {
                    Some("PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES")
                }
                Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES => {
                    Some("PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES")
                }
                Self::PHYSICAL_DEVICE_SHADER_FLOAT8_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_SHADER_FLOAT8_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES => {
                    Some("PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES")
                }
                Self::PHYSICAL_DEVICE_SHADER_FMA_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_SHADER_FMA_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES => {
                    Some("PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES")
                }
                Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL => {
                    Some("PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL")
                }
                Self::PHYSICAL_DEVICE_SHADER_LONG_VECTOR_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_SHADER_LONG_VECTOR_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_SHADER_LONG_VECTOR_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_SHADER_LONG_VECTOR_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_SHADER_MAXIMAL_RECONVERGENCE_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_SHADER_MAXIMAL_RECONVERGENCE_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_SHADER_QUAD_CONTROL_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_SHADER_QUAD_CONTROL_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_SHADER_RELAXED_EXTENDED_INSTRUCTION_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_SHADER_RELAXED_EXTENDED_INSTRUCTION_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_SHADER_REPLICATED_COMPOSITES_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_SHADER_REPLICATED_COMPOSITES_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES => {
                    Some("PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES")
                }
                Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_PARTITIONED_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_SHADER_SUBGROUP_PARTITIONED_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES => {
                    Some("PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES")
                }
                Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES => {
                    Some("PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES")
                }
                Self::PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_SHADER_UNTYPED_POINTERS_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_SHADER_UNTYPED_POINTERS_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV => {
                    Some("PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV")
                }
                Self::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV => {
                    Some("PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV")
                }
                Self::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2 => {
                    Some("PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2")
                }
                Self::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_SUBGROUP_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES => {
                    Some("PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES")
                }
                Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI => {
                    Some("PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI")
                }
                Self::PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI => {
                    Some("PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI")
                }
                Self::PHYSICAL_DEVICE_SURFACE_INFO_2_KHR => {
                    Some("PHYSICAL_DEVICE_SURFACE_INFO_2_KHR")
                }
                Self::PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES => {
                    Some("PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES")
                }
                Self::PHYSICAL_DEVICE_TENSOR_FEATURES_ARM => {
                    Some("PHYSICAL_DEVICE_TENSOR_FEATURES_ARM")
                }
                Self::PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM => {
                    Some("PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM")
                }
                Self::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_3D_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_3D_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES => {
                    Some("PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES")
                }
                Self::PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM => {
                    Some("PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM")
                }
                Self::PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM => {
                    Some("PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM")
                }
                Self::PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM => {
                    Some("PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM")
                }
                Self::PHYSICAL_DEVICE_TILE_SHADING_FEATURES_QCOM => {
                    Some("PHYSICAL_DEVICE_TILE_SHADING_FEATURES_QCOM")
                }
                Self::PHYSICAL_DEVICE_TILE_SHADING_PROPERTIES_QCOM => {
                    Some("PHYSICAL_DEVICE_TILE_SHADING_PROPERTIES_QCOM")
                }
                Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES => {
                    Some("PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES")
                }
                Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_TOOL_PROPERTIES => Some("PHYSICAL_DEVICE_TOOL_PROPERTIES"),
                Self::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_UNIFIED_IMAGE_LAYOUTS_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_UNIFIED_IMAGE_LAYOUTS_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES => {
                    Some("PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES")
                }
                Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES => {
                    Some("PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES")
                }
                Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES => {
                    Some("PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES")
                }
                Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT => {
                    Some("PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT")
                }
                Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_ROBUSTNESS_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_ROBUSTNESS_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_VIDEO_DECODE_VP9_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_VIDEO_DECODE_VP9_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_VIDEO_ENCODE_AV1_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_VIDEO_ENCODE_AV1_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_VIDEO_ENCODE_INTRA_REFRESH_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_VIDEO_ENCODE_INTRA_REFRESH_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR => {
                    Some("PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR")
                }
                Self::PHYSICAL_DEVICE_VIDEO_ENCODE_QUANTIZATION_MAP_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_VIDEO_ENCODE_QUANTIZATION_MAP_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE => {
                    Some("PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE")
                }
                Self::PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR => {
                    Some("PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR")
                }
                Self::PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_VIDEO_MAINTENANCE_2_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_VIDEO_MAINTENANCE_2_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES => {
                    Some("PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES")
                }
                Self::PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR => {
                    Some("PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR")
                }
                Self::PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM => {
                    Some("PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM")
                }
                Self::PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_ZERO_INITIALIZE_DEVICE_MEMORY_FEATURES_EXT => {
                    Some("PHYSICAL_DEVICE_ZERO_INITIALIZE_DEVICE_MEMORY_FEATURES_EXT")
                }
                Self::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES => {
                    Some("PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES")
                }
                Self::PIPELINE_BINARY_CREATE_INFO_KHR => Some("PIPELINE_BINARY_CREATE_INFO_KHR"),
                Self::PIPELINE_BINARY_DATA_INFO_KHR => Some("PIPELINE_BINARY_DATA_INFO_KHR"),
                Self::PIPELINE_BINARY_HANDLES_INFO_KHR => Some("PIPELINE_BINARY_HANDLES_INFO_KHR"),
                Self::PIPELINE_BINARY_INFO_KHR => Some("PIPELINE_BINARY_INFO_KHR"),
                Self::PIPELINE_BINARY_KEY_KHR => Some("PIPELINE_BINARY_KEY_KHR"),
                Self::PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT => {
                    Some("PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT")
                }
                Self::PIPELINE_COLOR_WRITE_CREATE_INFO_EXT => {
                    Some("PIPELINE_COLOR_WRITE_CREATE_INFO_EXT")
                }
                Self::PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD => {
                    Some("PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD")
                }
                Self::PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV => {
                    Some("PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV")
                }
                Self::PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV => {
                    Some("PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV")
                }
                Self::PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV => {
                    Some("PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV")
                }
                Self::PIPELINE_CREATE_FLAGS_2_CREATE_INFO => {
                    Some("PIPELINE_CREATE_FLAGS_2_CREATE_INFO")
                }
                Self::PIPELINE_CREATE_INFO_KHR => Some("PIPELINE_CREATE_INFO_KHR"),
                Self::PIPELINE_CREATION_FEEDBACK_CREATE_INFO => {
                    Some("PIPELINE_CREATION_FEEDBACK_CREATE_INFO")
                }
                Self::PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT => {
                    Some("PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT")
                }
                Self::PIPELINE_EXECUTABLE_INFO_KHR => Some("PIPELINE_EXECUTABLE_INFO_KHR"),
                Self::PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR => {
                    Some("PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR")
                }
                Self::PIPELINE_EXECUTABLE_PROPERTIES_KHR => {
                    Some("PIPELINE_EXECUTABLE_PROPERTIES_KHR")
                }
                Self::PIPELINE_EXECUTABLE_STATISTIC_KHR => {
                    Some("PIPELINE_EXECUTABLE_STATISTIC_KHR")
                }
                Self::PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE => {
                    Some("PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE")
                }
                Self::PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV => {
                    Some("PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV")
                }
                Self::PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR => {
                    Some("PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR")
                }
                Self::PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV => {
                    Some("PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV")
                }
                Self::PIPELINE_INFO_KHR => Some("PIPELINE_INFO_KHR"),
                Self::PIPELINE_LIBRARY_CREATE_INFO_KHR => Some("PIPELINE_LIBRARY_CREATE_INFO_KHR"),
                Self::PIPELINE_PROPERTIES_IDENTIFIER_EXT => {
                    Some("PIPELINE_PROPERTIES_IDENTIFIER_EXT")
                }
                Self::PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT => {
                    Some("PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT")
                }
                Self::PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT => {
                    Some("PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT")
                }
                Self::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO => {
                    Some("PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO")
                }
                Self::PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT => {
                    Some("PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT")
                }
                Self::PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD => {
                    Some("PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD")
                }
                Self::PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT => {
                    Some("PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT")
                }
                Self::PIPELINE_RENDERING_CREATE_INFO => Some("PIPELINE_RENDERING_CREATE_INFO"),
                Self::PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV => {
                    Some("PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV")
                }
                Self::PIPELINE_ROBUSTNESS_CREATE_INFO => Some("PIPELINE_ROBUSTNESS_CREATE_INFO"),
                Self::PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT => {
                    Some("PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT")
                }
                Self::PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT => {
                    Some("PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT")
                }
                Self::PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX => {
                    Some("PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX")
                }
                Self::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO => {
                    Some("PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO")
                }
                Self::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO => {
                    Some("PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO")
                }
                Self::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO => {
                    Some("PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO")
                }
                Self::PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV => {
                    Some("PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV")
                }
                Self::PIPELINE_VIEWPORT_DEPTH_CLAMP_CONTROL_CREATE_INFO_EXT => {
                    Some("PIPELINE_VIEWPORT_DEPTH_CLAMP_CONTROL_CREATE_INFO_EXT")
                }
                Self::PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT => {
                    Some("PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT")
                }
                Self::PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV => {
                    Some("PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV")
                }
                Self::PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV => {
                    Some("PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV")
                }
                Self::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV => {
                    Some("PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV")
                }
                Self::PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV => {
                    Some("PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV")
                }
                Self::PRESENT_FRAME_TOKEN_GGP => Some("PRESENT_FRAME_TOKEN_GGP"),
                Self::PRESENT_ID_2_KHR => Some("PRESENT_ID_2_KHR"),
                Self::PRESENT_ID_KHR => Some("PRESENT_ID_KHR"),
                Self::PRESENT_INFO_KHR => Some("PRESENT_INFO_KHR"),
                Self::PRESENT_REGIONS_KHR => Some("PRESENT_REGIONS_KHR"),
                Self::PRESENT_TIMES_INFO_GOOGLE => Some("PRESENT_TIMES_INFO_GOOGLE"),
                Self::PRESENT_TIMINGS_INFO_EXT => Some("PRESENT_TIMINGS_INFO_EXT"),
                Self::PRESENT_TIMING_INFO_EXT => Some("PRESENT_TIMING_INFO_EXT"),
                Self::PRESENT_TIMING_SURFACE_CAPABILITIES_EXT => {
                    Some("PRESENT_TIMING_SURFACE_CAPABILITIES_EXT")
                }
                Self::PRESENT_WAIT_2_INFO_KHR => Some("PRESENT_WAIT_2_INFO_KHR"),
                Self::PRIVATE_DATA_SLOT_CREATE_INFO => Some("PRIVATE_DATA_SLOT_CREATE_INFO"),
                Self::PROTECTED_SUBMIT_INFO => Some("PROTECTED_SUBMIT_INFO"),
                Self::PUSH_CONSTANTS_INFO => Some("PUSH_CONSTANTS_INFO"),
                Self::PUSH_CONSTANT_BANK_INFO_NV => Some("PUSH_CONSTANT_BANK_INFO_NV"),
                Self::PUSH_DATA_INFO_EXT => Some("PUSH_DATA_INFO_EXT"),
                Self::PUSH_DESCRIPTOR_SET_INFO => Some("PUSH_DESCRIPTOR_SET_INFO"),
                Self::PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO => {
                    Some("PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO")
                }
                Self::QUERY_LOW_LATENCY_SUPPORT_NV => Some("QUERY_LOW_LATENCY_SUPPORT_NV"),
                Self::QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR => {
                    Some("QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR")
                }
                Self::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL => {
                    Some("QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL")
                }
                Self::QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR => {
                    Some("QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR")
                }
                Self::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV => {
                    Some("QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV")
                }
                Self::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV => {
                    Some("QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV")
                }
                Self::QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM => {
                    Some("QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM")
                }
                Self::QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM => {
                    Some("QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM")
                }
                Self::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES => {
                    Some("QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES")
                }
                Self::QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR => {
                    Some("QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR")
                }
                Self::QUEUE_FAMILY_PROPERTIES_2 => Some("QUEUE_FAMILY_PROPERTIES_2"),
                Self::QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR => {
                    Some("QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR")
                }
                Self::QUEUE_FAMILY_VIDEO_PROPERTIES_KHR => {
                    Some("QUEUE_FAMILY_VIDEO_PROPERTIES_KHR")
                }
                Self::RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV => {
                    Some("RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV")
                }
                Self::RAY_TRACING_PIPELINE_CREATE_INFO_KHR => {
                    Some("RAY_TRACING_PIPELINE_CREATE_INFO_KHR")
                }
                Self::RAY_TRACING_PIPELINE_CREATE_INFO_NV => {
                    Some("RAY_TRACING_PIPELINE_CREATE_INFO_NV")
                }
                Self::RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR => {
                    Some("RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR")
                }
                Self::RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR => {
                    Some("RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR")
                }
                Self::RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV => {
                    Some("RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV")
                }
                Self::RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR => {
                    Some("RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR")
                }
                Self::RELEASE_SWAPCHAIN_IMAGES_INFO_KHR => {
                    Some("RELEASE_SWAPCHAIN_IMAGES_INFO_KHR")
                }
                Self::RENDERING_AREA_INFO => Some("RENDERING_AREA_INFO"),
                Self::RENDERING_ATTACHMENT_FLAGS_INFO_KHR => {
                    Some("RENDERING_ATTACHMENT_FLAGS_INFO_KHR")
                }
                Self::RENDERING_ATTACHMENT_INFO => Some("RENDERING_ATTACHMENT_INFO"),
                Self::RENDERING_ATTACHMENT_LOCATION_INFO => {
                    Some("RENDERING_ATTACHMENT_LOCATION_INFO")
                }
                Self::RENDERING_END_INFO_KHR => Some("RENDERING_END_INFO_KHR"),
                Self::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT => {
                    Some("RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT")
                }
                Self::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR => {
                    Some("RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR")
                }
                Self::RENDERING_INFO => Some("RENDERING_INFO"),
                Self::RENDERING_INPUT_ATTACHMENT_INDEX_INFO => {
                    Some("RENDERING_INPUT_ATTACHMENT_INDEX_INFO")
                }
                Self::RENDER_PASS_ATTACHMENT_BEGIN_INFO => {
                    Some("RENDER_PASS_ATTACHMENT_BEGIN_INFO")
                }
                Self::RENDER_PASS_CREATE_INFO_2 => Some("RENDER_PASS_CREATE_INFO_2"),
                Self::RENDER_PASS_CREATION_CONTROL_EXT => Some("RENDER_PASS_CREATION_CONTROL_EXT"),
                Self::RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT => {
                    Some("RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT")
                }
                Self::RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT => {
                    Some("RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT")
                }
                Self::RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT => {
                    Some("RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT")
                }
                Self::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO => {
                    Some("RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO")
                }
                Self::RENDER_PASS_MULTIVIEW_CREATE_INFO => {
                    Some("RENDER_PASS_MULTIVIEW_CREATE_INFO")
                }
                Self::RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM => {
                    Some("RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM")
                }
                Self::RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT => {
                    Some("RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT")
                }
                Self::RENDER_PASS_STRIPE_BEGIN_INFO_ARM => {
                    Some("RENDER_PASS_STRIPE_BEGIN_INFO_ARM")
                }
                Self::RENDER_PASS_STRIPE_INFO_ARM => Some("RENDER_PASS_STRIPE_INFO_ARM"),
                Self::RENDER_PASS_STRIPE_SUBMIT_INFO_ARM => {
                    Some("RENDER_PASS_STRIPE_SUBMIT_INFO_ARM")
                }
                Self::RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT => {
                    Some("RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT")
                }
                Self::RENDER_PASS_TILE_SHADING_CREATE_INFO_QCOM => {
                    Some("RENDER_PASS_TILE_SHADING_CREATE_INFO_QCOM")
                }
                Self::RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM => {
                    Some("RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM")
                }
                Self::RESOLVE_IMAGE_INFO_2 => Some("RESOLVE_IMAGE_INFO_2"),
                Self::RESOLVE_IMAGE_MODE_INFO_KHR => Some("RESOLVE_IMAGE_MODE_INFO_KHR"),
                Self::RESOURCE_DESCRIPTOR_INFO_EXT => Some("RESOURCE_DESCRIPTOR_INFO_EXT"),
                Self::SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM => {
                    Some("SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM")
                }
                Self::SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT => {
                    Some("SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT")
                }
                Self::SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => {
                    Some("SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT")
                }
                Self::SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM => {
                    Some("SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM")
                }
                Self::SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT => {
                    Some("SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT")
                }
                Self::SAMPLER_CUSTOM_BORDER_COLOR_INDEX_CREATE_INFO_EXT => {
                    Some("SAMPLER_CUSTOM_BORDER_COLOR_INDEX_CREATE_INFO_EXT")
                }
                Self::SAMPLER_REDUCTION_MODE_CREATE_INFO => {
                    Some("SAMPLER_REDUCTION_MODE_CREATE_INFO")
                }
                Self::SAMPLER_YCBCR_CONVERSION_CREATE_INFO => {
                    Some("SAMPLER_YCBCR_CONVERSION_CREATE_INFO")
                }
                Self::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES => {
                    Some("SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES")
                }
                Self::SAMPLER_YCBCR_CONVERSION_INFO => Some("SAMPLER_YCBCR_CONVERSION_INFO"),
                Self::SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM => {
                    Some("SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM")
                }
                Self::SAMPLE_LOCATIONS_INFO_EXT => Some("SAMPLE_LOCATIONS_INFO_EXT"),
                Self::SCREEN_BUFFER_FORMAT_PROPERTIES_QNX => {
                    Some("SCREEN_BUFFER_FORMAT_PROPERTIES_QNX")
                }
                Self::SCREEN_BUFFER_PROPERTIES_QNX => Some("SCREEN_BUFFER_PROPERTIES_QNX"),
                Self::SCREEN_SURFACE_CREATE_INFO_QNX => Some("SCREEN_SURFACE_CREATE_INFO_QNX"),
                Self::SEMAPHORE_GET_FD_INFO_KHR => Some("SEMAPHORE_GET_FD_INFO_KHR"),
                Self::SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR => {
                    Some("SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR")
                }
                Self::SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA => {
                    Some("SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA")
                }
                Self::SEMAPHORE_SIGNAL_INFO => Some("SEMAPHORE_SIGNAL_INFO"),
                Self::SEMAPHORE_SUBMIT_INFO => Some("SEMAPHORE_SUBMIT_INFO"),
                Self::SEMAPHORE_TYPE_CREATE_INFO => Some("SEMAPHORE_TYPE_CREATE_INFO"),
                Self::SEMAPHORE_WAIT_INFO => Some("SEMAPHORE_WAIT_INFO"),
                Self::SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT => {
                    Some("SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT")
                }
                Self::SET_LATENCY_MARKER_INFO_NV => Some("SET_LATENCY_MARKER_INFO_NV"),
                Self::SET_PRESENT_CONFIG_NV => Some("SET_PRESENT_CONFIG_NV"),
                Self::SHADER_CREATE_INFO_EXT => Some("SHADER_CREATE_INFO_EXT"),
                Self::SHADER_DESCRIPTOR_SET_AND_BINDING_MAPPING_INFO_EXT => {
                    Some("SHADER_DESCRIPTOR_SET_AND_BINDING_MAPPING_INFO_EXT")
                }
                Self::SHADER_MODULE_IDENTIFIER_EXT => Some("SHADER_MODULE_IDENTIFIER_EXT"),
                Self::SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT => {
                    Some("SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT")
                }
                Self::SHARED_PRESENT_SURFACE_CAPABILITIES_KHR => {
                    Some("SHARED_PRESENT_SURFACE_CAPABILITIES_KHR")
                }
                Self::SPARSE_IMAGE_FORMAT_PROPERTIES_2 => Some("SPARSE_IMAGE_FORMAT_PROPERTIES_2"),
                Self::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2 => {
                    Some("SPARSE_IMAGE_MEMORY_REQUIREMENTS_2")
                }
                Self::STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP => {
                    Some("STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP")
                }
                Self::SUBMIT_INFO_2 => Some("SUBMIT_INFO_2"),
                Self::SUBPASS_BEGIN_INFO => Some("SUBPASS_BEGIN_INFO"),
                Self::SUBPASS_DEPENDENCY_2 => Some("SUBPASS_DEPENDENCY_2"),
                Self::SUBPASS_DESCRIPTION_2 => Some("SUBPASS_DESCRIPTION_2"),
                Self::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE => {
                    Some("SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE")
                }
                Self::SUBPASS_END_INFO => Some("SUBPASS_END_INFO"),
                Self::SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT => {
                    Some("SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT")
                }
                Self::SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI => {
                    Some("SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI")
                }
                Self::SUBRESOURCE_HOST_MEMCPY_SIZE => Some("SUBRESOURCE_HOST_MEMCPY_SIZE"),
                Self::SUBRESOURCE_LAYOUT_2 => Some("SUBRESOURCE_LAYOUT_2"),
                Self::SUBSAMPLED_IMAGE_FORMAT_PROPERTIES_EXT => {
                    Some("SUBSAMPLED_IMAGE_FORMAT_PROPERTIES_EXT")
                }
                Self::SURFACE_CAPABILITIES_2_EXT => Some("SURFACE_CAPABILITIES_2_EXT"),
                Self::SURFACE_CAPABILITIES_2_KHR => Some("SURFACE_CAPABILITIES_2_KHR"),
                Self::SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT => {
                    Some("SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT")
                }
                Self::SURFACE_CAPABILITIES_PRESENT_BARRIER_NV => {
                    Some("SURFACE_CAPABILITIES_PRESENT_BARRIER_NV")
                }
                Self::SURFACE_CAPABILITIES_PRESENT_ID_2_KHR => {
                    Some("SURFACE_CAPABILITIES_PRESENT_ID_2_KHR")
                }
                Self::SURFACE_CAPABILITIES_PRESENT_WAIT_2_KHR => {
                    Some("SURFACE_CAPABILITIES_PRESENT_WAIT_2_KHR")
                }
                Self::SURFACE_FORMAT_2_KHR => Some("SURFACE_FORMAT_2_KHR"),
                Self::SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT => {
                    Some("SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT")
                }
                Self::SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT => {
                    Some("SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT")
                }
                Self::SURFACE_PRESENT_MODE_COMPATIBILITY_KHR => {
                    Some("SURFACE_PRESENT_MODE_COMPATIBILITY_KHR")
                }
                Self::SURFACE_PRESENT_MODE_KHR => Some("SURFACE_PRESENT_MODE_KHR"),
                Self::SURFACE_PRESENT_SCALING_CAPABILITIES_KHR => {
                    Some("SURFACE_PRESENT_SCALING_CAPABILITIES_KHR")
                }
                Self::SURFACE_PROTECTED_CAPABILITIES_KHR => {
                    Some("SURFACE_PROTECTED_CAPABILITIES_KHR")
                }
                Self::SWAPCHAIN_CALIBRATED_TIMESTAMP_INFO_EXT => {
                    Some("SWAPCHAIN_CALIBRATED_TIMESTAMP_INFO_EXT")
                }
                Self::SWAPCHAIN_COUNTER_CREATE_INFO_EXT => {
                    Some("SWAPCHAIN_COUNTER_CREATE_INFO_EXT")
                }
                Self::SWAPCHAIN_CREATE_INFO_KHR => Some("SWAPCHAIN_CREATE_INFO_KHR"),
                Self::SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD => {
                    Some("SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD")
                }
                Self::SWAPCHAIN_LATENCY_CREATE_INFO_NV => Some("SWAPCHAIN_LATENCY_CREATE_INFO_NV"),
                Self::SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV => {
                    Some("SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV")
                }
                Self::SWAPCHAIN_PRESENT_FENCE_INFO_KHR => Some("SWAPCHAIN_PRESENT_FENCE_INFO_KHR"),
                Self::SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR => {
                    Some("SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR")
                }
                Self::SWAPCHAIN_PRESENT_MODE_INFO_KHR => Some("SWAPCHAIN_PRESENT_MODE_INFO_KHR"),
                Self::SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR => {
                    Some("SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR")
                }
                Self::SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT => {
                    Some("SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT")
                }
                Self::SWAPCHAIN_TIMING_PROPERTIES_EXT => Some("SWAPCHAIN_TIMING_PROPERTIES_EXT"),
                Self::SYSMEM_COLOR_SPACE_FUCHSIA => Some("SYSMEM_COLOR_SPACE_FUCHSIA"),
                Self::TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM => {
                    Some("TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM")
                }
                Self::TENSOR_COPY_ARM => Some("TENSOR_COPY_ARM"),
                Self::TENSOR_CREATE_INFO_ARM => Some("TENSOR_CREATE_INFO_ARM"),
                Self::TENSOR_DEPENDENCY_INFO_ARM => Some("TENSOR_DEPENDENCY_INFO_ARM"),
                Self::TENSOR_DESCRIPTION_ARM => Some("TENSOR_DESCRIPTION_ARM"),
                Self::TENSOR_FORMAT_PROPERTIES_ARM => Some("TENSOR_FORMAT_PROPERTIES_ARM"),
                Self::TENSOR_MEMORY_BARRIER_ARM => Some("TENSOR_MEMORY_BARRIER_ARM"),
                Self::TENSOR_MEMORY_REQUIREMENTS_INFO_ARM => {
                    Some("TENSOR_MEMORY_REQUIREMENTS_INFO_ARM")
                }
                Self::TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM => {
                    Some("TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM")
                }
                Self::TENSOR_VIEW_CREATE_INFO_ARM => Some("TENSOR_VIEW_CREATE_INFO_ARM"),
                Self::TEXEL_BUFFER_DESCRIPTOR_INFO_EXT => Some("TEXEL_BUFFER_DESCRIPTOR_INFO_EXT"),
                Self::TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD => {
                    Some("TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD")
                }
                Self::TILE_MEMORY_BIND_INFO_QCOM => Some("TILE_MEMORY_BIND_INFO_QCOM"),
                Self::TILE_MEMORY_REQUIREMENTS_QCOM => Some("TILE_MEMORY_REQUIREMENTS_QCOM"),
                Self::TILE_MEMORY_SIZE_INFO_QCOM => Some("TILE_MEMORY_SIZE_INFO_QCOM"),
                Self::TILE_PROPERTIES_QCOM => Some("TILE_PROPERTIES_QCOM"),
                Self::TIMELINE_SEMAPHORE_SUBMIT_INFO => Some("TIMELINE_SEMAPHORE_SUBMIT_INFO"),
                Self::VALIDATION_CACHE_CREATE_INFO_EXT => Some("VALIDATION_CACHE_CREATE_INFO_EXT"),
                Self::VALIDATION_FEATURES_EXT => Some("VALIDATION_FEATURES_EXT"),
                Self::VALIDATION_FLAGS_EXT => Some("VALIDATION_FLAGS_EXT"),
                Self::VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT => {
                    Some("VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT")
                }
                Self::VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT => {
                    Some("VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT")
                }
                Self::VIDEO_BEGIN_CODING_INFO_KHR => Some("VIDEO_BEGIN_CODING_INFO_KHR"),
                Self::VIDEO_CAPABILITIES_KHR => Some("VIDEO_CAPABILITIES_KHR"),
                Self::VIDEO_CODING_CONTROL_INFO_KHR => Some("VIDEO_CODING_CONTROL_INFO_KHR"),
                Self::VIDEO_DECODE_AV1_CAPABILITIES_KHR => {
                    Some("VIDEO_DECODE_AV1_CAPABILITIES_KHR")
                }
                Self::VIDEO_DECODE_AV1_DPB_SLOT_INFO_KHR => {
                    Some("VIDEO_DECODE_AV1_DPB_SLOT_INFO_KHR")
                }
                Self::VIDEO_DECODE_AV1_INLINE_SESSION_PARAMETERS_INFO_KHR => {
                    Some("VIDEO_DECODE_AV1_INLINE_SESSION_PARAMETERS_INFO_KHR")
                }
                Self::VIDEO_DECODE_AV1_PICTURE_INFO_KHR => {
                    Some("VIDEO_DECODE_AV1_PICTURE_INFO_KHR")
                }
                Self::VIDEO_DECODE_AV1_PROFILE_INFO_KHR => {
                    Some("VIDEO_DECODE_AV1_PROFILE_INFO_KHR")
                }
                Self::VIDEO_DECODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                    Some("VIDEO_DECODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR")
                }
                Self::VIDEO_DECODE_CAPABILITIES_KHR => Some("VIDEO_DECODE_CAPABILITIES_KHR"),
                Self::VIDEO_DECODE_H264_CAPABILITIES_KHR => {
                    Some("VIDEO_DECODE_H264_CAPABILITIES_KHR")
                }
                Self::VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR => {
                    Some("VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR")
                }
                Self::VIDEO_DECODE_H264_INLINE_SESSION_PARAMETERS_INFO_KHR => {
                    Some("VIDEO_DECODE_H264_INLINE_SESSION_PARAMETERS_INFO_KHR")
                }
                Self::VIDEO_DECODE_H264_PICTURE_INFO_KHR => {
                    Some("VIDEO_DECODE_H264_PICTURE_INFO_KHR")
                }
                Self::VIDEO_DECODE_H264_PROFILE_INFO_KHR => {
                    Some("VIDEO_DECODE_H264_PROFILE_INFO_KHR")
                }
                Self::VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR => {
                    Some("VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR")
                }
                Self::VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                    Some("VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR")
                }
                Self::VIDEO_DECODE_H265_CAPABILITIES_KHR => {
                    Some("VIDEO_DECODE_H265_CAPABILITIES_KHR")
                }
                Self::VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR => {
                    Some("VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR")
                }
                Self::VIDEO_DECODE_H265_INLINE_SESSION_PARAMETERS_INFO_KHR => {
                    Some("VIDEO_DECODE_H265_INLINE_SESSION_PARAMETERS_INFO_KHR")
                }
                Self::VIDEO_DECODE_H265_PICTURE_INFO_KHR => {
                    Some("VIDEO_DECODE_H265_PICTURE_INFO_KHR")
                }
                Self::VIDEO_DECODE_H265_PROFILE_INFO_KHR => {
                    Some("VIDEO_DECODE_H265_PROFILE_INFO_KHR")
                }
                Self::VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR => {
                    Some("VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR")
                }
                Self::VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                    Some("VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR")
                }
                Self::VIDEO_DECODE_INFO_KHR => Some("VIDEO_DECODE_INFO_KHR"),
                Self::VIDEO_DECODE_USAGE_INFO_KHR => Some("VIDEO_DECODE_USAGE_INFO_KHR"),
                Self::VIDEO_DECODE_VP9_CAPABILITIES_KHR => {
                    Some("VIDEO_DECODE_VP9_CAPABILITIES_KHR")
                }
                Self::VIDEO_DECODE_VP9_PICTURE_INFO_KHR => {
                    Some("VIDEO_DECODE_VP9_PICTURE_INFO_KHR")
                }
                Self::VIDEO_DECODE_VP9_PROFILE_INFO_KHR => {
                    Some("VIDEO_DECODE_VP9_PROFILE_INFO_KHR")
                }
                Self::VIDEO_ENCODE_AV1_CAPABILITIES_KHR => {
                    Some("VIDEO_ENCODE_AV1_CAPABILITIES_KHR")
                }
                Self::VIDEO_ENCODE_AV1_DPB_SLOT_INFO_KHR => {
                    Some("VIDEO_ENCODE_AV1_DPB_SLOT_INFO_KHR")
                }
                Self::VIDEO_ENCODE_AV1_GOP_REMAINING_FRAME_INFO_KHR => {
                    Some("VIDEO_ENCODE_AV1_GOP_REMAINING_FRAME_INFO_KHR")
                }
                Self::VIDEO_ENCODE_AV1_PICTURE_INFO_KHR => {
                    Some("VIDEO_ENCODE_AV1_PICTURE_INFO_KHR")
                }
                Self::VIDEO_ENCODE_AV1_PROFILE_INFO_KHR => {
                    Some("VIDEO_ENCODE_AV1_PROFILE_INFO_KHR")
                }
                Self::VIDEO_ENCODE_AV1_QUALITY_LEVEL_PROPERTIES_KHR => {
                    Some("VIDEO_ENCODE_AV1_QUALITY_LEVEL_PROPERTIES_KHR")
                }
                Self::VIDEO_ENCODE_AV1_QUANTIZATION_MAP_CAPABILITIES_KHR => {
                    Some("VIDEO_ENCODE_AV1_QUANTIZATION_MAP_CAPABILITIES_KHR")
                }
                Self::VIDEO_ENCODE_AV1_RATE_CONTROL_INFO_KHR => {
                    Some("VIDEO_ENCODE_AV1_RATE_CONTROL_INFO_KHR")
                }
                Self::VIDEO_ENCODE_AV1_RATE_CONTROL_LAYER_INFO_KHR => {
                    Some("VIDEO_ENCODE_AV1_RATE_CONTROL_LAYER_INFO_KHR")
                }
                Self::VIDEO_ENCODE_AV1_SESSION_CREATE_INFO_KHR => {
                    Some("VIDEO_ENCODE_AV1_SESSION_CREATE_INFO_KHR")
                }
                Self::VIDEO_ENCODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                    Some("VIDEO_ENCODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR")
                }
                Self::VIDEO_ENCODE_CAPABILITIES_KHR => Some("VIDEO_ENCODE_CAPABILITIES_KHR"),
                Self::VIDEO_ENCODE_H264_CAPABILITIES_KHR => {
                    Some("VIDEO_ENCODE_H264_CAPABILITIES_KHR")
                }
                Self::VIDEO_ENCODE_H264_DPB_SLOT_INFO_KHR => {
                    Some("VIDEO_ENCODE_H264_DPB_SLOT_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H264_GOP_REMAINING_FRAME_INFO_KHR => {
                    Some("VIDEO_ENCODE_H264_GOP_REMAINING_FRAME_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H264_NALU_SLICE_INFO_KHR => {
                    Some("VIDEO_ENCODE_H264_NALU_SLICE_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H264_PICTURE_INFO_KHR => {
                    Some("VIDEO_ENCODE_H264_PICTURE_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H264_PROFILE_INFO_KHR => {
                    Some("VIDEO_ENCODE_H264_PROFILE_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H264_QUALITY_LEVEL_PROPERTIES_KHR => {
                    Some("VIDEO_ENCODE_H264_QUALITY_LEVEL_PROPERTIES_KHR")
                }
                Self::VIDEO_ENCODE_H264_QUANTIZATION_MAP_CAPABILITIES_KHR => {
                    Some("VIDEO_ENCODE_H264_QUANTIZATION_MAP_CAPABILITIES_KHR")
                }
                Self::VIDEO_ENCODE_H264_RATE_CONTROL_INFO_KHR => {
                    Some("VIDEO_ENCODE_H264_RATE_CONTROL_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_KHR => {
                    Some("VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H264_SESSION_CREATE_INFO_KHR => {
                    Some("VIDEO_ENCODE_H264_SESSION_CREATE_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR => {
                    Some("VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                    Some("VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H264_SESSION_PARAMETERS_FEEDBACK_INFO_KHR => {
                    Some("VIDEO_ENCODE_H264_SESSION_PARAMETERS_FEEDBACK_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H264_SESSION_PARAMETERS_GET_INFO_KHR => {
                    Some("VIDEO_ENCODE_H264_SESSION_PARAMETERS_GET_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H265_CAPABILITIES_KHR => {
                    Some("VIDEO_ENCODE_H265_CAPABILITIES_KHR")
                }
                Self::VIDEO_ENCODE_H265_DPB_SLOT_INFO_KHR => {
                    Some("VIDEO_ENCODE_H265_DPB_SLOT_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H265_GOP_REMAINING_FRAME_INFO_KHR => {
                    Some("VIDEO_ENCODE_H265_GOP_REMAINING_FRAME_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_KHR => {
                    Some("VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H265_PICTURE_INFO_KHR => {
                    Some("VIDEO_ENCODE_H265_PICTURE_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H265_PROFILE_INFO_KHR => {
                    Some("VIDEO_ENCODE_H265_PROFILE_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H265_QUALITY_LEVEL_PROPERTIES_KHR => {
                    Some("VIDEO_ENCODE_H265_QUALITY_LEVEL_PROPERTIES_KHR")
                }
                Self::VIDEO_ENCODE_H265_QUANTIZATION_MAP_CAPABILITIES_KHR => {
                    Some("VIDEO_ENCODE_H265_QUANTIZATION_MAP_CAPABILITIES_KHR")
                }
                Self::VIDEO_ENCODE_H265_RATE_CONTROL_INFO_KHR => {
                    Some("VIDEO_ENCODE_H265_RATE_CONTROL_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_KHR => {
                    Some("VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H265_SESSION_CREATE_INFO_KHR => {
                    Some("VIDEO_ENCODE_H265_SESSION_CREATE_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR => {
                    Some("VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                    Some("VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H265_SESSION_PARAMETERS_FEEDBACK_INFO_KHR => {
                    Some("VIDEO_ENCODE_H265_SESSION_PARAMETERS_FEEDBACK_INFO_KHR")
                }
                Self::VIDEO_ENCODE_H265_SESSION_PARAMETERS_GET_INFO_KHR => {
                    Some("VIDEO_ENCODE_H265_SESSION_PARAMETERS_GET_INFO_KHR")
                }
                Self::VIDEO_ENCODE_INFO_KHR => Some("VIDEO_ENCODE_INFO_KHR"),
                Self::VIDEO_ENCODE_INTRA_REFRESH_CAPABILITIES_KHR => {
                    Some("VIDEO_ENCODE_INTRA_REFRESH_CAPABILITIES_KHR")
                }
                Self::VIDEO_ENCODE_INTRA_REFRESH_INFO_KHR => {
                    Some("VIDEO_ENCODE_INTRA_REFRESH_INFO_KHR")
                }
                Self::VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE => {
                    Some("VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE")
                }
                Self::VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR => {
                    Some("VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR")
                }
                Self::VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR => {
                    Some("VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR")
                }
                Self::VIDEO_ENCODE_QUANTIZATION_MAP_CAPABILITIES_KHR => {
                    Some("VIDEO_ENCODE_QUANTIZATION_MAP_CAPABILITIES_KHR")
                }
                Self::VIDEO_ENCODE_QUANTIZATION_MAP_INFO_KHR => {
                    Some("VIDEO_ENCODE_QUANTIZATION_MAP_INFO_KHR")
                }
                Self::VIDEO_ENCODE_QUANTIZATION_MAP_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                    Some("VIDEO_ENCODE_QUANTIZATION_MAP_SESSION_PARAMETERS_CREATE_INFO_KHR")
                }
                Self::VIDEO_ENCODE_RATE_CONTROL_INFO_KHR => {
                    Some("VIDEO_ENCODE_RATE_CONTROL_INFO_KHR")
                }
                Self::VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR => {
                    Some("VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR")
                }
                Self::VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE => {
                    Some("VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE")
                }
                Self::VIDEO_ENCODE_SESSION_INTRA_REFRESH_CREATE_INFO_KHR => {
                    Some("VIDEO_ENCODE_SESSION_INTRA_REFRESH_CREATE_INFO_KHR")
                }
                Self::VIDEO_ENCODE_SESSION_PARAMETERS_FEEDBACK_INFO_KHR => {
                    Some("VIDEO_ENCODE_SESSION_PARAMETERS_FEEDBACK_INFO_KHR")
                }
                Self::VIDEO_ENCODE_SESSION_PARAMETERS_GET_INFO_KHR => {
                    Some("VIDEO_ENCODE_SESSION_PARAMETERS_GET_INFO_KHR")
                }
                Self::VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE => {
                    Some("VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE")
                }
                Self::VIDEO_ENCODE_USAGE_INFO_KHR => Some("VIDEO_ENCODE_USAGE_INFO_KHR"),
                Self::VIDEO_END_CODING_INFO_KHR => Some("VIDEO_END_CODING_INFO_KHR"),
                Self::VIDEO_FORMAT_AV1_QUANTIZATION_MAP_PROPERTIES_KHR => {
                    Some("VIDEO_FORMAT_AV1_QUANTIZATION_MAP_PROPERTIES_KHR")
                }
                Self::VIDEO_FORMAT_H265_QUANTIZATION_MAP_PROPERTIES_KHR => {
                    Some("VIDEO_FORMAT_H265_QUANTIZATION_MAP_PROPERTIES_KHR")
                }
                Self::VIDEO_FORMAT_PROPERTIES_KHR => Some("VIDEO_FORMAT_PROPERTIES_KHR"),
                Self::VIDEO_FORMAT_QUANTIZATION_MAP_PROPERTIES_KHR => {
                    Some("VIDEO_FORMAT_QUANTIZATION_MAP_PROPERTIES_KHR")
                }
                Self::VIDEO_INLINE_QUERY_INFO_KHR => Some("VIDEO_INLINE_QUERY_INFO_KHR"),
                Self::VIDEO_PICTURE_RESOURCE_INFO_KHR => Some("VIDEO_PICTURE_RESOURCE_INFO_KHR"),
                Self::VIDEO_PROFILE_INFO_KHR => Some("VIDEO_PROFILE_INFO_KHR"),
                Self::VIDEO_PROFILE_LIST_INFO_KHR => Some("VIDEO_PROFILE_LIST_INFO_KHR"),
                Self::VIDEO_REFERENCE_INTRA_REFRESH_INFO_KHR => {
                    Some("VIDEO_REFERENCE_INTRA_REFRESH_INFO_KHR")
                }
                Self::VIDEO_REFERENCE_SLOT_INFO_KHR => Some("VIDEO_REFERENCE_SLOT_INFO_KHR"),
                Self::VIDEO_SESSION_CREATE_INFO_KHR => Some("VIDEO_SESSION_CREATE_INFO_KHR"),
                Self::VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR => {
                    Some("VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR")
                }
                Self::VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                    Some("VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR")
                }
                Self::VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR => {
                    Some("VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR")
                }
                Self::VI_SURFACE_CREATE_INFO_NN => Some("VI_SURFACE_CREATE_INFO_NN"),
                Self::WAYLAND_SURFACE_CREATE_INFO_KHR => Some("WAYLAND_SURFACE_CREATE_INFO_KHR"),
                Self::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR => {
                    Some("WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR")
                }
                Self::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV => {
                    Some("WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV")
                }
                Self::WIN32_SURFACE_CREATE_INFO_KHR => Some("WIN32_SURFACE_CREATE_INFO_KHR"),
                Self::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR => {
                    Some("WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR")
                }
                Self::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV => {
                    Some("WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV")
                }
                Self::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK => {
                    Some("WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK")
                }
                Self::WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV => {
                    Some("WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV")
                }
                Self::WRITE_DESCRIPTOR_SET_TENSOR_ARM => Some("WRITE_DESCRIPTOR_SET_TENSOR_ARM"),
                Self::WRITE_INDIRECT_EXECUTION_SET_PIPELINE_EXT => {
                    Some("WRITE_INDIRECT_EXECUTION_SET_PIPELINE_EXT")
                }
                Self::WRITE_INDIRECT_EXECUTION_SET_SHADER_EXT => {
                    Some("WRITE_INDIRECT_EXECUTION_SET_SHADER_EXT")
                }
                Self::XCB_SURFACE_CREATE_INFO_KHR => Some("XCB_SURFACE_CREATE_INFO_KHR"),
                Self::XLIB_SURFACE_CREATE_INFO_KHR => Some("XLIB_SURFACE_CREATE_INFO_KHR"),
                Self::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES => {
                    Some("PHYSICAL_DEVICE_VULKAN_1_1_FEATURES")
                }
                Self::PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES => {
                    Some("PHYSICAL_DEVICE_VULKAN_1_2_FEATURES")
                }
                Self::PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_VULKAN_1_3_FEATURES => {
                    Some("PHYSICAL_DEVICE_VULKAN_1_3_FEATURES")
                }
                Self::PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES")
                }
                Self::PHYSICAL_DEVICE_VULKAN_1_4_FEATURES => {
                    Some("PHYSICAL_DEVICE_VULKAN_1_4_FEATURES")
                }
                Self::PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES => {
                    Some("PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES")
                }
                Self::SURFACE_CREATE_INFO_OHOS => Some("SURFACE_CREATE_INFO_OHOS"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SubpassContents(i32);
    impl SubpassContents {
        pub const INLINE: Self = Self(0);
        pub const SECONDARY_COMMAND_BUFFERS: Self = Self(1);
        pub const INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR: Self = Self(1000451000);
        pub const INLINE_AND_SECONDARY_COMMAND_BUFFERS_EXT: Self =
            Self::INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR;
    }
    impl fmt::Debug for SubpassContents {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::INLINE => Some("INLINE"),
                Self::SECONDARY_COMMAND_BUFFERS => Some("SECONDARY_COMMAND_BUFFERS"),
                Self::INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR => {
                    Some("INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR")
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Result(i32);
    impl Result {
        pub const SUCCESS: Self = Self(0);
        pub const NOT_READY: Self = Self(1);
        pub const TIMEOUT: Self = Self(2);
        pub const EVENT_SET: Self = Self(3);
        pub const EVENT_RESET: Self = Self(4);
        pub const INCOMPLETE: Self = Self(5);
        pub const ERROR_OUT_OF_HOST_MEMORY: Self = Self(-1);
        pub const ERROR_OUT_OF_DEVICE_MEMORY: Self = Self(-2);
        pub const ERROR_INITIALIZATION_FAILED: Self = Self(-3);
        pub const ERROR_DEVICE_LOST: Self = Self(-4);
        pub const ERROR_MEMORY_MAP_FAILED: Self = Self(-5);
        pub const ERROR_LAYER_NOT_PRESENT: Self = Self(-6);
        pub const ERROR_EXTENSION_NOT_PRESENT: Self = Self(-7);
        pub const ERROR_FEATURE_NOT_PRESENT: Self = Self(-8);
        pub const ERROR_INCOMPATIBLE_DRIVER: Self = Self(-9);
        pub const ERROR_TOO_MANY_OBJECTS: Self = Self(-10);
        pub const ERROR_FORMAT_NOT_SUPPORTED: Self = Self(-11);
        pub const ERROR_FRAGMENTED_POOL: Self = Self(-12);
        pub const ERROR_UNKNOWN: Self = Self(-13);
        pub const ERROR_COMPRESSION_EXHAUSTED_EXT: Self = Self(-1000338000);
        pub const ERROR_FRAGMENTATION: Self = Self(-1000161000);
        pub const ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: Self = Self(-1000255000);
        pub const ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR: Self = Self(-1000023000);
        pub const ERROR_INCOMPATIBLE_DISPLAY_KHR: Self = Self(-1000003001);
        pub const ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: Self = Self(-1000158000);
        pub const ERROR_INVALID_EXTERNAL_HANDLE: Self = Self(-1000072003);
        pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: Self = Self(-1000257000);
        pub const ERROR_INVALID_SHADER_NV: Self = Self(-1000012000);
        pub const ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR: Self = Self(-1000299000);
        pub const ERROR_NATIVE_WINDOW_IN_USE_KHR: Self = Self(-1000000001);
        pub const ERROR_NOT_ENOUGH_SPACE_KHR: Self = Self(-1000483000);
        pub const ERROR_NOT_PERMITTED: Self = Self(-1000174001);
        pub const ERROR_OUT_OF_DATE_KHR: Self = Self(-1000001004);
        pub const ERROR_OUT_OF_POOL_MEMORY: Self = Self(-1000069000);
        pub const ERROR_PRESENT_TIMING_QUEUE_FULL_EXT: Self = Self(-1000208000);
        pub const ERROR_SURFACE_LOST_KHR: Self = Self(-1000000000);
        pub const ERROR_VALIDATION_FAILED: Self = Self(-1000011001);
        pub const ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR: Self = Self(-1000023001);
        pub const ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR: Self = Self(-1000023004);
        pub const ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR: Self = Self(-1000023003);
        pub const ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR: Self = Self(-1000023002);
        pub const ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR: Self = Self(-1000023005);
        pub const INCOMPATIBLE_SHADER_BINARY_EXT: Self = Self(1000482000);
        pub const OPERATION_DEFERRED_KHR: Self = Self(1000268002);
        pub const OPERATION_NOT_DEFERRED_KHR: Self = Self(1000268003);
        pub const PIPELINE_BINARY_MISSING_KHR: Self = Self(1000483000);
        pub const PIPELINE_COMPILE_REQUIRED: Self = Self(1000297000);
        pub const SUBOPTIMAL_KHR: Self = Self(1000001003);
        pub const THREAD_DONE_KHR: Self = Self(1000268001);
        pub const THREAD_IDLE_KHR: Self = Self(1000268000);
        pub const ERROR_FRAGMENTATION_EXT: Self = Self::ERROR_FRAGMENTATION;
        pub const ERROR_INCOMPATIBLE_SHADER_BINARY_EXT: Self = Self::INCOMPATIBLE_SHADER_BINARY_EXT;
        pub const ERROR_INVALID_DEVICE_ADDRESS_EXT: Self =
            Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
        pub const ERROR_INVALID_EXTERNAL_HANDLE_KHR: Self = Self::ERROR_INVALID_EXTERNAL_HANDLE;
        pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: Self =
            Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
        pub const ERROR_NOT_PERMITTED_EXT: Self = Self::ERROR_NOT_PERMITTED;
        pub const ERROR_NOT_PERMITTED_KHR: Self = Self::ERROR_NOT_PERMITTED;
        pub const ERROR_OUT_OF_POOL_MEMORY_KHR: Self = Self::ERROR_OUT_OF_POOL_MEMORY;
        pub const ERROR_PIPELINE_COMPILE_REQUIRED_EXT: Self = Self::PIPELINE_COMPILE_REQUIRED;
        pub const ERROR_VALIDATION_FAILED_EXT: Self = Self::ERROR_VALIDATION_FAILED;
        pub const PIPELINE_COMPILE_REQUIRED_EXT: Self = Self::PIPELINE_COMPILE_REQUIRED;
    }
    impl fmt::Debug for Result {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SUCCESS => Some("SUCCESS"),
                Self::NOT_READY => Some("NOT_READY"),
                Self::TIMEOUT => Some("TIMEOUT"),
                Self::EVENT_SET => Some("EVENT_SET"),
                Self::EVENT_RESET => Some("EVENT_RESET"),
                Self::INCOMPLETE => Some("INCOMPLETE"),
                Self::ERROR_OUT_OF_HOST_MEMORY => Some("ERROR_OUT_OF_HOST_MEMORY"),
                Self::ERROR_OUT_OF_DEVICE_MEMORY => Some("ERROR_OUT_OF_DEVICE_MEMORY"),
                Self::ERROR_INITIALIZATION_FAILED => Some("ERROR_INITIALIZATION_FAILED"),
                Self::ERROR_DEVICE_LOST => Some("ERROR_DEVICE_LOST"),
                Self::ERROR_MEMORY_MAP_FAILED => Some("ERROR_MEMORY_MAP_FAILED"),
                Self::ERROR_LAYER_NOT_PRESENT => Some("ERROR_LAYER_NOT_PRESENT"),
                Self::ERROR_EXTENSION_NOT_PRESENT => Some("ERROR_EXTENSION_NOT_PRESENT"),
                Self::ERROR_FEATURE_NOT_PRESENT => Some("ERROR_FEATURE_NOT_PRESENT"),
                Self::ERROR_INCOMPATIBLE_DRIVER => Some("ERROR_INCOMPATIBLE_DRIVER"),
                Self::ERROR_TOO_MANY_OBJECTS => Some("ERROR_TOO_MANY_OBJECTS"),
                Self::ERROR_FORMAT_NOT_SUPPORTED => Some("ERROR_FORMAT_NOT_SUPPORTED"),
                Self::ERROR_FRAGMENTED_POOL => Some("ERROR_FRAGMENTED_POOL"),
                Self::ERROR_UNKNOWN => Some("ERROR_UNKNOWN"),
                Self::ERROR_COMPRESSION_EXHAUSTED_EXT => Some("ERROR_COMPRESSION_EXHAUSTED_EXT"),
                Self::ERROR_FRAGMENTATION => Some("ERROR_FRAGMENTATION"),
                Self::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => {
                    Some("ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT")
                }
                Self::ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR => {
                    Some("ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR")
                }
                Self::ERROR_INCOMPATIBLE_DISPLAY_KHR => Some("ERROR_INCOMPATIBLE_DISPLAY_KHR"),
                Self::ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT => {
                    Some("ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT")
                }
                Self::ERROR_INVALID_EXTERNAL_HANDLE => Some("ERROR_INVALID_EXTERNAL_HANDLE"),
                Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS => {
                    Some("ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS")
                }
                Self::ERROR_INVALID_SHADER_NV => Some("ERROR_INVALID_SHADER_NV"),
                Self::ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR => {
                    Some("ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR")
                }
                Self::ERROR_NATIVE_WINDOW_IN_USE_KHR => Some("ERROR_NATIVE_WINDOW_IN_USE_KHR"),
                Self::ERROR_NOT_ENOUGH_SPACE_KHR => Some("ERROR_NOT_ENOUGH_SPACE_KHR"),
                Self::ERROR_NOT_PERMITTED => Some("ERROR_NOT_PERMITTED"),
                Self::ERROR_OUT_OF_DATE_KHR => Some("ERROR_OUT_OF_DATE_KHR"),
                Self::ERROR_OUT_OF_POOL_MEMORY => Some("ERROR_OUT_OF_POOL_MEMORY"),
                Self::ERROR_PRESENT_TIMING_QUEUE_FULL_EXT => {
                    Some("ERROR_PRESENT_TIMING_QUEUE_FULL_EXT")
                }
                Self::ERROR_SURFACE_LOST_KHR => Some("ERROR_SURFACE_LOST_KHR"),
                Self::ERROR_VALIDATION_FAILED => Some("ERROR_VALIDATION_FAILED"),
                Self::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR => {
                    Some("ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR")
                }
                Self::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR => {
                    Some("ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR")
                }
                Self::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR => {
                    Some("ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR")
                }
                Self::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR => {
                    Some("ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR")
                }
                Self::ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR => {
                    Some("ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR")
                }
                Self::INCOMPATIBLE_SHADER_BINARY_EXT => Some("INCOMPATIBLE_SHADER_BINARY_EXT"),
                Self::OPERATION_DEFERRED_KHR => Some("OPERATION_DEFERRED_KHR"),
                Self::OPERATION_NOT_DEFERRED_KHR => Some("OPERATION_NOT_DEFERRED_KHR"),
                Self::PIPELINE_BINARY_MISSING_KHR => Some("PIPELINE_BINARY_MISSING_KHR"),
                Self::PIPELINE_COMPILE_REQUIRED => Some("PIPELINE_COMPILE_REQUIRED"),
                Self::SUBOPTIMAL_KHR => Some("SUBOPTIMAL_KHR"),
                Self::THREAD_DONE_KHR => Some("THREAD_DONE_KHR"),
                Self::THREAD_IDLE_KHR => Some("THREAD_IDLE_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DynamicState(i32);
    impl DynamicState {
        pub const VIEWPORT: Self = Self(0);
        pub const SCISSOR: Self = Self(1);
        pub const LINE_WIDTH: Self = Self(2);
        pub const DEPTH_BIAS: Self = Self(3);
        pub const BLEND_CONSTANTS: Self = Self(4);
        pub const DEPTH_BOUNDS: Self = Self(5);
        pub const STENCIL_COMPARE_MASK: Self = Self(6);
        pub const STENCIL_WRITE_MASK: Self = Self(7);
        pub const STENCIL_REFERENCE: Self = Self(8);
        pub const ALPHA_TO_COVERAGE_ENABLE_EXT: Self = Self(1000455007);
        pub const ALPHA_TO_ONE_ENABLE_EXT: Self = Self(1000455008);
        pub const ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT: Self = Self(1000524000);
        pub const COLOR_BLEND_ADVANCED_EXT: Self = Self(1000455018);
        pub const COLOR_BLEND_ENABLE_EXT: Self = Self(1000455010);
        pub const COLOR_BLEND_EQUATION_EXT: Self = Self(1000455011);
        pub const COLOR_WRITE_ENABLE_EXT: Self = Self(1000381000);
        pub const COLOR_WRITE_MASK_EXT: Self = Self(1000455012);
        pub const CONSERVATIVE_RASTERIZATION_MODE_EXT: Self = Self(1000455014);
        pub const COVERAGE_MODULATION_MODE_NV: Self = Self(1000455027);
        pub const COVERAGE_MODULATION_TABLE_ENABLE_NV: Self = Self(1000455028);
        pub const COVERAGE_MODULATION_TABLE_NV: Self = Self(1000455029);
        pub const COVERAGE_REDUCTION_MODE_NV: Self = Self(1000455032);
        pub const COVERAGE_TO_COLOR_ENABLE_NV: Self = Self(1000455025);
        pub const COVERAGE_TO_COLOR_LOCATION_NV: Self = Self(1000455026);
        pub const CULL_MODE: Self = Self(1000267000);
        pub const DEPTH_BIAS_ENABLE: Self = Self(1000377002);
        pub const DEPTH_BOUNDS_TEST_ENABLE: Self = Self(1000267009);
        pub const DEPTH_CLAMP_ENABLE_EXT: Self = Self(1000455003);
        pub const DEPTH_CLAMP_RANGE_EXT: Self = Self(1000582000);
        pub const DEPTH_CLIP_ENABLE_EXT: Self = Self(1000455016);
        pub const DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT: Self = Self(1000455022);
        pub const DEPTH_COMPARE_OP: Self = Self(1000267008);
        pub const DEPTH_TEST_ENABLE: Self = Self(1000267006);
        pub const DEPTH_WRITE_ENABLE: Self = Self(1000267007);
        pub const DISCARD_RECTANGLE_ENABLE_EXT: Self = Self(1000099001);
        pub const DISCARD_RECTANGLE_EXT: Self = Self(1000099000);
        pub const DISCARD_RECTANGLE_MODE_EXT: Self = Self(1000099002);
        pub const EXCLUSIVE_SCISSOR_ENABLE_NV: Self = Self(1000205000);
        pub const EXCLUSIVE_SCISSOR_NV: Self = Self(1000205001);
        pub const EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT: Self = Self(1000455015);
        pub const FRAGMENT_SHADING_RATE_KHR: Self = Self(1000226000);
        pub const FRONT_FACE: Self = Self(1000267001);
        pub const LINE_RASTERIZATION_MODE_EXT: Self = Self(1000455020);
        pub const LINE_STIPPLE: Self = Self(1000259000);
        pub const LINE_STIPPLE_ENABLE_EXT: Self = Self(1000455021);
        pub const LOGIC_OP_ENABLE_EXT: Self = Self(1000455009);
        pub const LOGIC_OP_EXT: Self = Self(1000377003);
        pub const PATCH_CONTROL_POINTS_EXT: Self = Self(1000377000);
        pub const POLYGON_MODE_EXT: Self = Self(1000455004);
        pub const PRIMITIVE_RESTART_ENABLE: Self = Self(1000377004);
        pub const PRIMITIVE_TOPOLOGY: Self = Self(1000267002);
        pub const PROVOKING_VERTEX_MODE_EXT: Self = Self(1000455019);
        pub const RASTERIZATION_SAMPLES_EXT: Self = Self(1000455005);
        pub const RASTERIZATION_STREAM_EXT: Self = Self(1000455013);
        pub const RASTERIZER_DISCARD_ENABLE: Self = Self(1000377001);
        pub const RAY_TRACING_PIPELINE_STACK_SIZE_KHR: Self = Self(1000347000);
        pub const REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV: Self = Self(1000455031);
        pub const SAMPLE_LOCATIONS_ENABLE_EXT: Self = Self(1000455017);
        pub const SAMPLE_LOCATIONS_EXT: Self = Self(1000143000);
        pub const SAMPLE_MASK_EXT: Self = Self(1000455006);
        pub const SCISSOR_WITH_COUNT: Self = Self(1000267004);
        pub const SHADING_RATE_IMAGE_ENABLE_NV: Self = Self(1000455030);
        pub const STENCIL_OP: Self = Self(1000267011);
        pub const STENCIL_TEST_ENABLE: Self = Self(1000267010);
        pub const TESSELLATION_DOMAIN_ORIGIN_EXT: Self = Self(1000455002);
        pub const VERTEX_INPUT_BINDING_STRIDE: Self = Self(1000267005);
        pub const VERTEX_INPUT_EXT: Self = Self(1000352000);
        pub const VIEWPORT_COARSE_SAMPLE_ORDER_NV: Self = Self(1000164006);
        pub const VIEWPORT_SHADING_RATE_PALETTE_NV: Self = Self(1000164004);
        pub const VIEWPORT_SWIZZLE_NV: Self = Self(1000455024);
        pub const VIEWPORT_WITH_COUNT: Self = Self(1000267003);
        pub const VIEWPORT_W_SCALING_ENABLE_NV: Self = Self(1000455023);
        pub const VIEWPORT_W_SCALING_NV: Self = Self(1000087000);
        pub const CULL_MODE_EXT: Self = Self::CULL_MODE;
        pub const DEPTH_BIAS_ENABLE_EXT: Self = Self::DEPTH_BIAS_ENABLE;
        pub const DEPTH_BOUNDS_TEST_ENABLE_EXT: Self = Self::DEPTH_BOUNDS_TEST_ENABLE;
        pub const DEPTH_COMPARE_OP_EXT: Self = Self::DEPTH_COMPARE_OP;
        pub const DEPTH_TEST_ENABLE_EXT: Self = Self::DEPTH_TEST_ENABLE;
        pub const DEPTH_WRITE_ENABLE_EXT: Self = Self::DEPTH_WRITE_ENABLE;
        pub const FRONT_FACE_EXT: Self = Self::FRONT_FACE;
        pub const LINE_STIPPLE_EXT: Self = Self::LINE_STIPPLE;
        pub const LINE_STIPPLE_KHR: Self = Self::LINE_STIPPLE;
        pub const PRIMITIVE_RESTART_ENABLE_EXT: Self = Self::PRIMITIVE_RESTART_ENABLE;
        pub const PRIMITIVE_TOPOLOGY_EXT: Self = Self::PRIMITIVE_TOPOLOGY;
        pub const RASTERIZER_DISCARD_ENABLE_EXT: Self = Self::RASTERIZER_DISCARD_ENABLE;
        pub const SCISSOR_WITH_COUNT_EXT: Self = Self::SCISSOR_WITH_COUNT;
        pub const STENCIL_OP_EXT: Self = Self::STENCIL_OP;
        pub const STENCIL_TEST_ENABLE_EXT: Self = Self::STENCIL_TEST_ENABLE;
        pub const VERTEX_INPUT_BINDING_STRIDE_EXT: Self = Self::VERTEX_INPUT_BINDING_STRIDE;
        pub const VIEWPORT_WITH_COUNT_EXT: Self = Self::VIEWPORT_WITH_COUNT;
    }
    impl fmt::Debug for DynamicState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::VIEWPORT => Some("VIEWPORT"),
                Self::SCISSOR => Some("SCISSOR"),
                Self::LINE_WIDTH => Some("LINE_WIDTH"),
                Self::DEPTH_BIAS => Some("DEPTH_BIAS"),
                Self::BLEND_CONSTANTS => Some("BLEND_CONSTANTS"),
                Self::DEPTH_BOUNDS => Some("DEPTH_BOUNDS"),
                Self::STENCIL_COMPARE_MASK => Some("STENCIL_COMPARE_MASK"),
                Self::STENCIL_WRITE_MASK => Some("STENCIL_WRITE_MASK"),
                Self::STENCIL_REFERENCE => Some("STENCIL_REFERENCE"),
                Self::ALPHA_TO_COVERAGE_ENABLE_EXT => Some("ALPHA_TO_COVERAGE_ENABLE_EXT"),
                Self::ALPHA_TO_ONE_ENABLE_EXT => Some("ALPHA_TO_ONE_ENABLE_EXT"),
                Self::ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT => {
                    Some("ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT")
                }
                Self::COLOR_BLEND_ADVANCED_EXT => Some("COLOR_BLEND_ADVANCED_EXT"),
                Self::COLOR_BLEND_ENABLE_EXT => Some("COLOR_BLEND_ENABLE_EXT"),
                Self::COLOR_BLEND_EQUATION_EXT => Some("COLOR_BLEND_EQUATION_EXT"),
                Self::COLOR_WRITE_ENABLE_EXT => Some("COLOR_WRITE_ENABLE_EXT"),
                Self::COLOR_WRITE_MASK_EXT => Some("COLOR_WRITE_MASK_EXT"),
                Self::CONSERVATIVE_RASTERIZATION_MODE_EXT => {
                    Some("CONSERVATIVE_RASTERIZATION_MODE_EXT")
                }
                Self::COVERAGE_MODULATION_MODE_NV => Some("COVERAGE_MODULATION_MODE_NV"),
                Self::COVERAGE_MODULATION_TABLE_ENABLE_NV => {
                    Some("COVERAGE_MODULATION_TABLE_ENABLE_NV")
                }
                Self::COVERAGE_MODULATION_TABLE_NV => Some("COVERAGE_MODULATION_TABLE_NV"),
                Self::COVERAGE_REDUCTION_MODE_NV => Some("COVERAGE_REDUCTION_MODE_NV"),
                Self::COVERAGE_TO_COLOR_ENABLE_NV => Some("COVERAGE_TO_COLOR_ENABLE_NV"),
                Self::COVERAGE_TO_COLOR_LOCATION_NV => Some("COVERAGE_TO_COLOR_LOCATION_NV"),
                Self::CULL_MODE => Some("CULL_MODE"),
                Self::DEPTH_BIAS_ENABLE => Some("DEPTH_BIAS_ENABLE"),
                Self::DEPTH_BOUNDS_TEST_ENABLE => Some("DEPTH_BOUNDS_TEST_ENABLE"),
                Self::DEPTH_CLAMP_ENABLE_EXT => Some("DEPTH_CLAMP_ENABLE_EXT"),
                Self::DEPTH_CLAMP_RANGE_EXT => Some("DEPTH_CLAMP_RANGE_EXT"),
                Self::DEPTH_CLIP_ENABLE_EXT => Some("DEPTH_CLIP_ENABLE_EXT"),
                Self::DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT => {
                    Some("DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT")
                }
                Self::DEPTH_COMPARE_OP => Some("DEPTH_COMPARE_OP"),
                Self::DEPTH_TEST_ENABLE => Some("DEPTH_TEST_ENABLE"),
                Self::DEPTH_WRITE_ENABLE => Some("DEPTH_WRITE_ENABLE"),
                Self::DISCARD_RECTANGLE_ENABLE_EXT => Some("DISCARD_RECTANGLE_ENABLE_EXT"),
                Self::DISCARD_RECTANGLE_EXT => Some("DISCARD_RECTANGLE_EXT"),
                Self::DISCARD_RECTANGLE_MODE_EXT => Some("DISCARD_RECTANGLE_MODE_EXT"),
                Self::EXCLUSIVE_SCISSOR_ENABLE_NV => Some("EXCLUSIVE_SCISSOR_ENABLE_NV"),
                Self::EXCLUSIVE_SCISSOR_NV => Some("EXCLUSIVE_SCISSOR_NV"),
                Self::EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT => {
                    Some("EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT")
                }
                Self::FRAGMENT_SHADING_RATE_KHR => Some("FRAGMENT_SHADING_RATE_KHR"),
                Self::FRONT_FACE => Some("FRONT_FACE"),
                Self::LINE_RASTERIZATION_MODE_EXT => Some("LINE_RASTERIZATION_MODE_EXT"),
                Self::LINE_STIPPLE => Some("LINE_STIPPLE"),
                Self::LINE_STIPPLE_ENABLE_EXT => Some("LINE_STIPPLE_ENABLE_EXT"),
                Self::LOGIC_OP_ENABLE_EXT => Some("LOGIC_OP_ENABLE_EXT"),
                Self::LOGIC_OP_EXT => Some("LOGIC_OP_EXT"),
                Self::PATCH_CONTROL_POINTS_EXT => Some("PATCH_CONTROL_POINTS_EXT"),
                Self::POLYGON_MODE_EXT => Some("POLYGON_MODE_EXT"),
                Self::PRIMITIVE_RESTART_ENABLE => Some("PRIMITIVE_RESTART_ENABLE"),
                Self::PRIMITIVE_TOPOLOGY => Some("PRIMITIVE_TOPOLOGY"),
                Self::PROVOKING_VERTEX_MODE_EXT => Some("PROVOKING_VERTEX_MODE_EXT"),
                Self::RASTERIZATION_SAMPLES_EXT => Some("RASTERIZATION_SAMPLES_EXT"),
                Self::RASTERIZATION_STREAM_EXT => Some("RASTERIZATION_STREAM_EXT"),
                Self::RASTERIZER_DISCARD_ENABLE => Some("RASTERIZER_DISCARD_ENABLE"),
                Self::RAY_TRACING_PIPELINE_STACK_SIZE_KHR => {
                    Some("RAY_TRACING_PIPELINE_STACK_SIZE_KHR")
                }
                Self::REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV => {
                    Some("REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV")
                }
                Self::SAMPLE_LOCATIONS_ENABLE_EXT => Some("SAMPLE_LOCATIONS_ENABLE_EXT"),
                Self::SAMPLE_LOCATIONS_EXT => Some("SAMPLE_LOCATIONS_EXT"),
                Self::SAMPLE_MASK_EXT => Some("SAMPLE_MASK_EXT"),
                Self::SCISSOR_WITH_COUNT => Some("SCISSOR_WITH_COUNT"),
                Self::SHADING_RATE_IMAGE_ENABLE_NV => Some("SHADING_RATE_IMAGE_ENABLE_NV"),
                Self::STENCIL_OP => Some("STENCIL_OP"),
                Self::STENCIL_TEST_ENABLE => Some("STENCIL_TEST_ENABLE"),
                Self::TESSELLATION_DOMAIN_ORIGIN_EXT => Some("TESSELLATION_DOMAIN_ORIGIN_EXT"),
                Self::VERTEX_INPUT_BINDING_STRIDE => Some("VERTEX_INPUT_BINDING_STRIDE"),
                Self::VERTEX_INPUT_EXT => Some("VERTEX_INPUT_EXT"),
                Self::VIEWPORT_COARSE_SAMPLE_ORDER_NV => Some("VIEWPORT_COARSE_SAMPLE_ORDER_NV"),
                Self::VIEWPORT_SHADING_RATE_PALETTE_NV => Some("VIEWPORT_SHADING_RATE_PALETTE_NV"),
                Self::VIEWPORT_SWIZZLE_NV => Some("VIEWPORT_SWIZZLE_NV"),
                Self::VIEWPORT_WITH_COUNT => Some("VIEWPORT_WITH_COUNT"),
                Self::VIEWPORT_W_SCALING_ENABLE_NV => Some("VIEWPORT_W_SCALING_ENABLE_NV"),
                Self::VIEWPORT_W_SCALING_NV => Some("VIEWPORT_W_SCALING_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ObjectType(i32);
    impl ObjectType {
        pub const UNKNOWN: Self = Self(0);
        pub const INSTANCE: Self = Self(1);
        pub const PHYSICAL_DEVICE: Self = Self(2);
        pub const DEVICE: Self = Self(3);
        pub const QUEUE: Self = Self(4);
        pub const SEMAPHORE: Self = Self(5);
        pub const COMMAND_BUFFER: Self = Self(6);
        pub const FENCE: Self = Self(7);
        pub const DEVICE_MEMORY: Self = Self(8);
        pub const BUFFER: Self = Self(9);
        pub const IMAGE: Self = Self(10);
        pub const EVENT: Self = Self(11);
        pub const QUERY_POOL: Self = Self(12);
        pub const BUFFER_VIEW: Self = Self(13);
        pub const IMAGE_VIEW: Self = Self(14);
        pub const SHADER_MODULE: Self = Self(15);
        pub const PIPELINE_CACHE: Self = Self(16);
        pub const PIPELINE_LAYOUT: Self = Self(17);
        pub const RENDER_PASS: Self = Self(18);
        pub const PIPELINE: Self = Self(19);
        pub const DESCRIPTOR_SET_LAYOUT: Self = Self(20);
        pub const SAMPLER: Self = Self(21);
        pub const DESCRIPTOR_POOL: Self = Self(22);
        pub const DESCRIPTOR_SET: Self = Self(23);
        pub const FRAMEBUFFER: Self = Self(24);
        pub const COMMAND_POOL: Self = Self(25);
        pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
        pub const ACCELERATION_STRUCTURE_NV: Self = Self(1000165000);
        pub const BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366000);
        pub const CUDA_FUNCTION_NV: Self = Self(1000307001);
        pub const CUDA_MODULE_NV: Self = Self(1000307000);
        pub const CU_FUNCTION_NVX: Self = Self(1000029001);
        pub const CU_MODULE_NVX: Self = Self(1000029000);
        pub const DATA_GRAPH_PIPELINE_SESSION_ARM: Self = Self(1000507000);
        pub const DEBUG_REPORT_CALLBACK_EXT: Self = Self(1000011000);
        pub const DEBUG_UTILS_MESSENGER_EXT: Self = Self(1000128000);
        pub const DEFERRED_OPERATION_KHR: Self = Self(1000268000);
        pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = Self(1000085000);
        pub const DISPLAY_KHR: Self = Self(1000002000);
        pub const DISPLAY_MODE_KHR: Self = Self(1000002001);
        pub const EXTERNAL_COMPUTE_QUEUE_NV: Self = Self(1000556000);
        pub const INDIRECT_COMMANDS_LAYOUT_EXT: Self = Self(1000572000);
        pub const INDIRECT_COMMANDS_LAYOUT_NV: Self = Self(1000277000);
        pub const INDIRECT_EXECUTION_SET_EXT: Self = Self(1000572001);
        pub const MICROMAP_EXT: Self = Self(1000396000);
        pub const OPTICAL_FLOW_SESSION_NV: Self = Self(1000464000);
        pub const PERFORMANCE_CONFIGURATION_INTEL: Self = Self(1000210000);
        pub const PIPELINE_BINARY_KHR: Self = Self(1000483000);
        pub const PRIVATE_DATA_SLOT: Self = Self(1000295000);
        pub const SAMPLER_YCBCR_CONVERSION: Self = Self(1000156000);
        pub const SHADER_EXT: Self = Self(1000482000);
        pub const SURFACE_KHR: Self = Self(1000000000);
        pub const SWAPCHAIN_KHR: Self = Self(1000001000);
        pub const TENSOR_ARM: Self = Self(1000460000);
        pub const TENSOR_VIEW_ARM: Self = Self(1000460001);
        pub const VALIDATION_CACHE_EXT: Self = Self(1000160000);
        pub const VIDEO_SESSION_KHR: Self = Self(1000023000);
        pub const VIDEO_SESSION_PARAMETERS_KHR: Self = Self(1000023001);
        pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR: Self = Self::DESCRIPTOR_UPDATE_TEMPLATE;
        pub const PRIVATE_DATA_SLOT_EXT: Self = Self::PRIVATE_DATA_SLOT;
        pub const SAMPLER_YCBCR_CONVERSION_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION;
    }
    impl fmt::Debug for ObjectType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::UNKNOWN => Some("UNKNOWN"),
                Self::INSTANCE => Some("INSTANCE"),
                Self::PHYSICAL_DEVICE => Some("PHYSICAL_DEVICE"),
                Self::DEVICE => Some("DEVICE"),
                Self::QUEUE => Some("QUEUE"),
                Self::SEMAPHORE => Some("SEMAPHORE"),
                Self::COMMAND_BUFFER => Some("COMMAND_BUFFER"),
                Self::FENCE => Some("FENCE"),
                Self::DEVICE_MEMORY => Some("DEVICE_MEMORY"),
                Self::BUFFER => Some("BUFFER"),
                Self::IMAGE => Some("IMAGE"),
                Self::EVENT => Some("EVENT"),
                Self::QUERY_POOL => Some("QUERY_POOL"),
                Self::BUFFER_VIEW => Some("BUFFER_VIEW"),
                Self::IMAGE_VIEW => Some("IMAGE_VIEW"),
                Self::SHADER_MODULE => Some("SHADER_MODULE"),
                Self::PIPELINE_CACHE => Some("PIPELINE_CACHE"),
                Self::PIPELINE_LAYOUT => Some("PIPELINE_LAYOUT"),
                Self::RENDER_PASS => Some("RENDER_PASS"),
                Self::PIPELINE => Some("PIPELINE"),
                Self::DESCRIPTOR_SET_LAYOUT => Some("DESCRIPTOR_SET_LAYOUT"),
                Self::SAMPLER => Some("SAMPLER"),
                Self::DESCRIPTOR_POOL => Some("DESCRIPTOR_POOL"),
                Self::DESCRIPTOR_SET => Some("DESCRIPTOR_SET"),
                Self::FRAMEBUFFER => Some("FRAMEBUFFER"),
                Self::COMMAND_POOL => Some("COMMAND_POOL"),
                Self::ACCELERATION_STRUCTURE_KHR => Some("ACCELERATION_STRUCTURE_KHR"),
                Self::ACCELERATION_STRUCTURE_NV => Some("ACCELERATION_STRUCTURE_NV"),
                Self::BUFFER_COLLECTION_FUCHSIA => Some("BUFFER_COLLECTION_FUCHSIA"),
                Self::CUDA_FUNCTION_NV => Some("CUDA_FUNCTION_NV"),
                Self::CUDA_MODULE_NV => Some("CUDA_MODULE_NV"),
                Self::CU_FUNCTION_NVX => Some("CU_FUNCTION_NVX"),
                Self::CU_MODULE_NVX => Some("CU_MODULE_NVX"),
                Self::DATA_GRAPH_PIPELINE_SESSION_ARM => Some("DATA_GRAPH_PIPELINE_SESSION_ARM"),
                Self::DEBUG_REPORT_CALLBACK_EXT => Some("DEBUG_REPORT_CALLBACK_EXT"),
                Self::DEBUG_UTILS_MESSENGER_EXT => Some("DEBUG_UTILS_MESSENGER_EXT"),
                Self::DEFERRED_OPERATION_KHR => Some("DEFERRED_OPERATION_KHR"),
                Self::DESCRIPTOR_UPDATE_TEMPLATE => Some("DESCRIPTOR_UPDATE_TEMPLATE"),
                Self::DISPLAY_KHR => Some("DISPLAY_KHR"),
                Self::DISPLAY_MODE_KHR => Some("DISPLAY_MODE_KHR"),
                Self::EXTERNAL_COMPUTE_QUEUE_NV => Some("EXTERNAL_COMPUTE_QUEUE_NV"),
                Self::INDIRECT_COMMANDS_LAYOUT_EXT => Some("INDIRECT_COMMANDS_LAYOUT_EXT"),
                Self::INDIRECT_COMMANDS_LAYOUT_NV => Some("INDIRECT_COMMANDS_LAYOUT_NV"),
                Self::INDIRECT_EXECUTION_SET_EXT => Some("INDIRECT_EXECUTION_SET_EXT"),
                Self::MICROMAP_EXT => Some("MICROMAP_EXT"),
                Self::OPTICAL_FLOW_SESSION_NV => Some("OPTICAL_FLOW_SESSION_NV"),
                Self::PERFORMANCE_CONFIGURATION_INTEL => Some("PERFORMANCE_CONFIGURATION_INTEL"),
                Self::PIPELINE_BINARY_KHR => Some("PIPELINE_BINARY_KHR"),
                Self::PRIVATE_DATA_SLOT => Some("PRIVATE_DATA_SLOT"),
                Self::SAMPLER_YCBCR_CONVERSION => Some("SAMPLER_YCBCR_CONVERSION"),
                Self::SHADER_EXT => Some("SHADER_EXT"),
                Self::SURFACE_KHR => Some("SURFACE_KHR"),
                Self::SWAPCHAIN_KHR => Some("SWAPCHAIN_KHR"),
                Self::TENSOR_ARM => Some("TENSOR_ARM"),
                Self::TENSOR_VIEW_ARM => Some("TENSOR_VIEW_ARM"),
                Self::VALIDATION_CACHE_EXT => Some("VALIDATION_CACHE_EXT"),
                Self::VIDEO_SESSION_KHR => Some("VIDEO_SESSION_KHR"),
                Self::VIDEO_SESSION_PARAMETERS_KHR => Some("VIDEO_SESSION_PARAMETERS_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VendorId(i32);
    impl VendorId {
        pub const KHRONOS: Self = Self(0x10000);
        pub const VIV: Self = Self(0x10001);
        pub const VSI: Self = Self(0x10002);
        pub const KAZAN: Self = Self(0x10003);
        pub const CODEPLAY: Self = Self(0x10004);
        pub const MESA: Self = Self(0x10005);
        pub const POCL: Self = Self(0x10006);
        pub const MOBILEYE: Self = Self(0x10007);
    }
    impl fmt::Debug for VendorId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::KHRONOS => Some("KHRONOS"),
                Self::VIV => Some("VIV"),
                Self::VSI => Some("VSI"),
                Self::KAZAN => Some("KAZAN"),
                Self::CODEPLAY => Some("CODEPLAY"),
                Self::MESA => Some("MESA"),
                Self::POCL => Some("POCL"),
                Self::MOBILEYE => Some("MOBILEYE"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct FramebufferCreateFlags(Flags);
    vk_bitflags_wrapped!(FramebufferCreateFlags, Flags);
    impl FramebufferCreateFlags {
        pub const IMAGELESS: Self = Self(FramebufferCreateFlagBits::IMAGELESS.0);
        pub const IMAGELESS_KHR: Self = Self::IMAGELESS;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct FramebufferCreateFlagBits(u32);
    impl FramebufferCreateFlagBits {
        pub const IMAGELESS: Self = Self(1 << 0);
        pub const IMAGELESS_KHR: Self = Self::IMAGELESS;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct QueryPoolCreateFlags(Flags);
    vk_bitflags_wrapped!(QueryPoolCreateFlags, Flags);
    impl QueryPoolCreateFlags {
        pub const RESET_KHR: Self = Self(QueryPoolCreateFlagBits::RESET_KHR.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct QueryPoolCreateFlagBits(u32);
    impl QueryPoolCreateFlagBits {
        pub const RESET_KHR: Self = Self(1 << 0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct RenderPassCreateFlags(Flags);
    vk_bitflags_wrapped!(RenderPassCreateFlags, Flags);
    impl RenderPassCreateFlags {
        pub const TRANSFORM_QCOM: Self = Self(RenderPassCreateFlagBits::TRANSFORM_QCOM.0);
        pub const PER_LAYER_FRAGMENT_DENSITY_VALVE: Self =
            Self(RenderPassCreateFlagBits::PER_LAYER_FRAGMENT_DENSITY_VALVE.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct RenderPassCreateFlagBits(u32);
    impl RenderPassCreateFlagBits {
        pub const TRANSFORM_QCOM: Self = Self(1 << 1);
        pub const PER_LAYER_FRAGMENT_DENSITY_VALVE: Self = Self(1 << 2);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SamplerCreateFlags(Flags);
    vk_bitflags_wrapped!(SamplerCreateFlags, Flags);
    impl SamplerCreateFlags {
        pub const SUBSAMPLED_EXT: Self = Self(SamplerCreateFlagBits::SUBSAMPLED_EXT.0);
        pub const SUBSAMPLED_COARSE_RECONSTRUCTION_EXT: Self =
            Self(SamplerCreateFlagBits::SUBSAMPLED_COARSE_RECONSTRUCTION_EXT.0);
        pub const NON_SEAMLESS_CUBE_MAP_EXT: Self =
            Self(SamplerCreateFlagBits::NON_SEAMLESS_CUBE_MAP_EXT.0);
        pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self =
            Self(SamplerCreateFlagBits::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT.0);
        pub const IMAGE_PROCESSING_QCOM: Self =
            Self(SamplerCreateFlagBits::IMAGE_PROCESSING_QCOM.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct SamplerCreateFlagBits(u32);
    impl SamplerCreateFlagBits {
        pub const SUBSAMPLED_EXT: Self = Self(1 << 0);
        pub const SUBSAMPLED_COARSE_RECONSTRUCTION_EXT: Self = Self(1 << 1);
        pub const NON_SEAMLESS_CUBE_MAP_EXT: Self = Self(1 << 2);
        pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(1 << 3);
        pub const IMAGE_PROCESSING_QCOM: Self = Self(1 << 4);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineLayoutCreateFlags(Flags);
    vk_bitflags_wrapped!(PipelineLayoutCreateFlags, Flags);
    impl PipelineLayoutCreateFlags {
        pub const INDEPENDENT_SETS_EXT: Self =
            Self(PipelineLayoutCreateFlagBits::INDEPENDENT_SETS_EXT.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PipelineLayoutCreateFlagBits(u32);
    impl PipelineLayoutCreateFlagBits {
        pub const INDEPENDENT_SETS_EXT: Self = Self(1 << 1);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineCacheCreateFlags(Flags);
    vk_bitflags_wrapped!(PipelineCacheCreateFlags, Flags);
    impl PipelineCacheCreateFlags {
        pub const EXTERNALLY_SYNCHRONIZED: Self =
            Self(PipelineCacheCreateFlagBits::EXTERNALLY_SYNCHRONIZED.0);
        pub const INTERNALLY_SYNCHRONIZED_MERGE_KHR: Self =
            Self(PipelineCacheCreateFlagBits::INTERNALLY_SYNCHRONIZED_MERGE_KHR.0);
        pub const EXTERNALLY_SYNCHRONIZED_EXT: Self = Self::EXTERNALLY_SYNCHRONIZED;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PipelineCacheCreateFlagBits(u32);
    impl PipelineCacheCreateFlagBits {
        pub const EXTERNALLY_SYNCHRONIZED: Self = Self(1 << 0);
        pub const INTERNALLY_SYNCHRONIZED_MERGE_KHR: Self = Self(1 << 3);
        pub const EXTERNALLY_SYNCHRONIZED_EXT: Self = Self::EXTERNALLY_SYNCHRONIZED;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineDepthStencilStateCreateFlags(Flags);
    vk_bitflags_wrapped!(PipelineDepthStencilStateCreateFlags, Flags);
    impl PipelineDepthStencilStateCreateFlags {
        pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT: Self = Self(PipelineDepthStencilStateCreateFlagBits::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT.0);
        pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT: Self = Self(PipelineDepthStencilStateCreateFlagBits::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT.0);
        pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM: Self =
            Self::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT;
        pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM: Self =
            Self::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PipelineDepthStencilStateCreateFlagBits(u32);
    impl PipelineDepthStencilStateCreateFlagBits {
        pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT: Self = Self(1 << 0);
        pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT: Self = Self(1 << 1);
        pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM: Self =
            Self::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT;
        pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM: Self =
            Self::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineDynamicStateCreateFlags(Flags);
    vk_bitflags_wrapped!(PipelineDynamicStateCreateFlags, Flags);
    impl PipelineDynamicStateCreateFlags {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineColorBlendStateCreateFlags(Flags);
    vk_bitflags_wrapped!(PipelineColorBlendStateCreateFlags, Flags);
    impl PipelineColorBlendStateCreateFlags {
        pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT: Self = Self(
            PipelineColorBlendStateCreateFlagBits::RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT.0,
        );
        pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_ARM: Self =
            Self::RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PipelineColorBlendStateCreateFlagBits(u32);
    impl PipelineColorBlendStateCreateFlagBits {
        pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT: Self = Self(1 << 0);
        pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_ARM: Self =
            Self::RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineMultisampleStateCreateFlags(Flags);
    vk_bitflags_wrapped!(PipelineMultisampleStateCreateFlags, Flags);
    impl PipelineMultisampleStateCreateFlags {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineRasterizationStateCreateFlags(Flags);
    vk_bitflags_wrapped!(PipelineRasterizationStateCreateFlags, Flags);
    impl PipelineRasterizationStateCreateFlags {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineViewportStateCreateFlags(Flags);
    vk_bitflags_wrapped!(PipelineViewportStateCreateFlags, Flags);
    impl PipelineViewportStateCreateFlags {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineTessellationStateCreateFlags(Flags);
    vk_bitflags_wrapped!(PipelineTessellationStateCreateFlags, Flags);
    impl PipelineTessellationStateCreateFlags {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineInputAssemblyStateCreateFlags(Flags);
    vk_bitflags_wrapped!(PipelineInputAssemblyStateCreateFlags, Flags);
    impl PipelineInputAssemblyStateCreateFlags {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineVertexInputStateCreateFlags(Flags);
    vk_bitflags_wrapped!(PipelineVertexInputStateCreateFlags, Flags);
    impl PipelineVertexInputStateCreateFlags {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineShaderStageCreateFlags(Flags);
    vk_bitflags_wrapped!(PipelineShaderStageCreateFlags, Flags);
    impl PipelineShaderStageCreateFlags {
        pub const ALLOW_VARYING_SUBGROUP_SIZE: Self =
            Self(PipelineShaderStageCreateFlagBits::ALLOW_VARYING_SUBGROUP_SIZE.0);
        pub const REQUIRE_FULL_SUBGROUPS: Self =
            Self(PipelineShaderStageCreateFlagBits::REQUIRE_FULL_SUBGROUPS.0);
        pub const ALLOW_VARYING_SUBGROUP_SIZE_EXT: Self = Self::ALLOW_VARYING_SUBGROUP_SIZE;
        pub const REQUIRE_FULL_SUBGROUPS_EXT: Self = Self::REQUIRE_FULL_SUBGROUPS;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PipelineShaderStageCreateFlagBits(u32);
    impl PipelineShaderStageCreateFlagBits {
        pub const ALLOW_VARYING_SUBGROUP_SIZE: Self = Self(1 << 0);
        pub const REQUIRE_FULL_SUBGROUPS: Self = Self(1 << 1);
        pub const ALLOW_VARYING_SUBGROUP_SIZE_EXT: Self = Self::ALLOW_VARYING_SUBGROUP_SIZE;
        pub const REQUIRE_FULL_SUBGROUPS_EXT: Self = Self::REQUIRE_FULL_SUBGROUPS;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DescriptorSetLayoutCreateFlags(Flags);
    vk_bitflags_wrapped!(DescriptorSetLayoutCreateFlags, Flags);
    impl DescriptorSetLayoutCreateFlags {
        pub const PUSH_DESCRIPTOR: Self =
            Self(DescriptorSetLayoutCreateFlagBits::PUSH_DESCRIPTOR.0);
        pub const UPDATE_AFTER_BIND_POOL: Self =
            Self(DescriptorSetLayoutCreateFlagBits::UPDATE_AFTER_BIND_POOL.0);
        pub const HOST_ONLY_POOL_EXT: Self =
            Self(DescriptorSetLayoutCreateFlagBits::HOST_ONLY_POOL_EXT.0);
        pub const DESCRIPTOR_BUFFER_EXT: Self =
            Self(DescriptorSetLayoutCreateFlagBits::DESCRIPTOR_BUFFER_EXT.0);
        pub const EMBEDDED_IMMUTABLE_SAMPLERS_EXT: Self =
            Self(DescriptorSetLayoutCreateFlagBits::EMBEDDED_IMMUTABLE_SAMPLERS_EXT.0);
        pub const PER_STAGE_NV: Self = Self(DescriptorSetLayoutCreateFlagBits::PER_STAGE_NV.0);
        pub const INDIRECT_BINDABLE_NV: Self =
            Self(DescriptorSetLayoutCreateFlagBits::INDIRECT_BINDABLE_NV.0);
        pub const HOST_ONLY_POOL_VALVE: Self = Self::HOST_ONLY_POOL_EXT;
        pub const PUSH_DESCRIPTOR_KHR: Self = Self::PUSH_DESCRIPTOR;
        pub const UPDATE_AFTER_BIND_POOL_EXT: Self = Self::UPDATE_AFTER_BIND_POOL;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DescriptorSetLayoutCreateFlagBits(u32);
    impl DescriptorSetLayoutCreateFlagBits {
        pub const PUSH_DESCRIPTOR: Self = Self(1 << 0);
        pub const UPDATE_AFTER_BIND_POOL: Self = Self(1 << 1);
        pub const HOST_ONLY_POOL_EXT: Self = Self(1 << 2);
        pub const DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 4);
        pub const EMBEDDED_IMMUTABLE_SAMPLERS_EXT: Self = Self(1 << 5);
        pub const PER_STAGE_NV: Self = Self(1 << 6);
        pub const INDIRECT_BINDABLE_NV: Self = Self(1 << 7);
        pub const HOST_ONLY_POOL_VALVE: Self = Self::HOST_ONLY_POOL_EXT;
        pub const PUSH_DESCRIPTOR_KHR: Self = Self::PUSH_DESCRIPTOR;
        pub const UPDATE_AFTER_BIND_POOL_EXT: Self = Self::UPDATE_AFTER_BIND_POOL;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct BufferViewCreateFlags(Flags);
    vk_bitflags_wrapped!(BufferViewCreateFlags, Flags);
    impl BufferViewCreateFlags {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct InstanceCreateFlags(Flags);
    vk_bitflags_wrapped!(InstanceCreateFlags, Flags);
    impl InstanceCreateFlags {
        pub const ENUMERATE_PORTABILITY_KHR: Self =
            Self(InstanceCreateFlagBits::ENUMERATE_PORTABILITY_KHR.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct InstanceCreateFlagBits(u32);
    impl InstanceCreateFlagBits {
        pub const ENUMERATE_PORTABILITY_KHR: Self = Self(1 << 0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DeviceCreateFlags(Flags);
    vk_bitflags_wrapped!(DeviceCreateFlags, Flags);
    impl DeviceCreateFlags {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DeviceQueueCreateFlags(Flags);
    vk_bitflags_wrapped!(DeviceQueueCreateFlags, Flags);
    impl DeviceQueueCreateFlags {
        pub const PROTECTED: Self = Self(DeviceQueueCreateFlagBits::PROTECTED.0);
        pub const INTERNALLY_SYNCHRONIZED_KHR: Self =
            Self(DeviceQueueCreateFlagBits::INTERNALLY_SYNCHRONIZED_KHR.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DeviceQueueCreateFlagBits(u32);
    impl DeviceQueueCreateFlagBits {
        pub const PROTECTED: Self = Self(1 << 0);
        pub const INTERNALLY_SYNCHRONIZED_KHR: Self = Self(1 << 2);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct QueueFlags(Flags);
    vk_bitflags_wrapped!(QueueFlags, Flags);
    impl QueueFlags {
        pub const GRAPHICS: Self = Self(QueueFlagBits::GRAPHICS.0);
        pub const COMPUTE: Self = Self(QueueFlagBits::COMPUTE.0);
        pub const TRANSFER: Self = Self(QueueFlagBits::TRANSFER.0);
        pub const SPARSE_BINDING: Self = Self(QueueFlagBits::SPARSE_BINDING.0);
        pub const PROTECTED: Self = Self(QueueFlagBits::PROTECTED.0);
        pub const VIDEO_DECODE_KHR: Self = Self(QueueFlagBits::VIDEO_DECODE_KHR.0);
        pub const VIDEO_ENCODE_KHR: Self = Self(QueueFlagBits::VIDEO_ENCODE_KHR.0);
        pub const OPTICAL_FLOW_NV: Self = Self(QueueFlagBits::OPTICAL_FLOW_NV.0);
        pub const DATA_GRAPH_ARM: Self = Self(QueueFlagBits::DATA_GRAPH_ARM.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct QueueFlagBits(u32);
    impl QueueFlagBits {
        pub const GRAPHICS: Self = Self(1 << 0);
        pub const COMPUTE: Self = Self(1 << 1);
        pub const TRANSFER: Self = Self(1 << 2);
        pub const SPARSE_BINDING: Self = Self(1 << 3);
        pub const PROTECTED: Self = Self(1 << 4);
        pub const VIDEO_DECODE_KHR: Self = Self(1 << 5);
        pub const VIDEO_ENCODE_KHR: Self = Self(1 << 6);
        pub const OPTICAL_FLOW_NV: Self = Self(1 << 8);
        pub const DATA_GRAPH_ARM: Self = Self(1 << 10);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MemoryPropertyFlags(Flags);
    vk_bitflags_wrapped!(MemoryPropertyFlags, Flags);
    impl MemoryPropertyFlags {
        pub const DEVICE_LOCAL: Self = Self(MemoryPropertyFlagBits::DEVICE_LOCAL.0);
        pub const HOST_VISIBLE: Self = Self(MemoryPropertyFlagBits::HOST_VISIBLE.0);
        pub const HOST_COHERENT: Self = Self(MemoryPropertyFlagBits::HOST_COHERENT.0);
        pub const HOST_CACHED: Self = Self(MemoryPropertyFlagBits::HOST_CACHED.0);
        pub const LAZILY_ALLOCATED: Self = Self(MemoryPropertyFlagBits::LAZILY_ALLOCATED.0);
        pub const PROTECTED: Self = Self(MemoryPropertyFlagBits::PROTECTED.0);
        pub const DEVICE_COHERENT_AMD: Self = Self(MemoryPropertyFlagBits::DEVICE_COHERENT_AMD.0);
        pub const DEVICE_UNCACHED_AMD: Self = Self(MemoryPropertyFlagBits::DEVICE_UNCACHED_AMD.0);
        pub const RDMA_CAPABLE_NV: Self = Self(MemoryPropertyFlagBits::RDMA_CAPABLE_NV.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct MemoryPropertyFlagBits(u32);
    impl MemoryPropertyFlagBits {
        pub const DEVICE_LOCAL: Self = Self(1 << 0);
        pub const HOST_VISIBLE: Self = Self(1 << 1);
        pub const HOST_COHERENT: Self = Self(1 << 2);
        pub const HOST_CACHED: Self = Self(1 << 3);
        pub const LAZILY_ALLOCATED: Self = Self(1 << 4);
        pub const PROTECTED: Self = Self(1 << 5);
        pub const DEVICE_COHERENT_AMD: Self = Self(1 << 6);
        pub const DEVICE_UNCACHED_AMD: Self = Self(1 << 7);
        pub const RDMA_CAPABLE_NV: Self = Self(1 << 8);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MemoryHeapFlags(Flags);
    vk_bitflags_wrapped!(MemoryHeapFlags, Flags);
    impl MemoryHeapFlags {
        pub const DEVICE_LOCAL: Self = Self(MemoryHeapFlagBits::DEVICE_LOCAL.0);
        pub const MULTI_INSTANCE: Self = Self(MemoryHeapFlagBits::MULTI_INSTANCE.0);
        pub const TILE_MEMORY_QCOM: Self = Self(MemoryHeapFlagBits::TILE_MEMORY_QCOM.0);
        pub const MULTI_INSTANCE_KHR: Self = Self::MULTI_INSTANCE;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct MemoryHeapFlagBits(u32);
    impl MemoryHeapFlagBits {
        pub const DEVICE_LOCAL: Self = Self(1 << 0);
        pub const MULTI_INSTANCE: Self = Self(1 << 1);
        pub const TILE_MEMORY_QCOM: Self = Self(1 << 3);
        pub const MULTI_INSTANCE_KHR: Self = Self::MULTI_INSTANCE;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct AccessFlags(Flags);
    vk_bitflags_wrapped!(AccessFlags, Flags);
    impl AccessFlags {
        pub const INDIRECT_COMMAND_READ: Self = Self(AccessFlagBits::INDIRECT_COMMAND_READ.0);
        pub const INDEX_READ: Self = Self(AccessFlagBits::INDEX_READ.0);
        pub const VERTEX_ATTRIBUTE_READ: Self = Self(AccessFlagBits::VERTEX_ATTRIBUTE_READ.0);
        pub const UNIFORM_READ: Self = Self(AccessFlagBits::UNIFORM_READ.0);
        pub const INPUT_ATTACHMENT_READ: Self = Self(AccessFlagBits::INPUT_ATTACHMENT_READ.0);
        pub const SHADER_READ: Self = Self(AccessFlagBits::SHADER_READ.0);
        pub const SHADER_WRITE: Self = Self(AccessFlagBits::SHADER_WRITE.0);
        pub const COLOR_ATTACHMENT_READ: Self = Self(AccessFlagBits::COLOR_ATTACHMENT_READ.0);
        pub const COLOR_ATTACHMENT_WRITE: Self = Self(AccessFlagBits::COLOR_ATTACHMENT_WRITE.0);
        pub const DEPTH_STENCIL_ATTACHMENT_READ: Self =
            Self(AccessFlagBits::DEPTH_STENCIL_ATTACHMENT_READ.0);
        pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self =
            Self(AccessFlagBits::DEPTH_STENCIL_ATTACHMENT_WRITE.0);
        pub const TRANSFER_READ: Self = Self(AccessFlagBits::TRANSFER_READ.0);
        pub const TRANSFER_WRITE: Self = Self(AccessFlagBits::TRANSFER_WRITE.0);
        pub const HOST_READ: Self = Self(AccessFlagBits::HOST_READ.0);
        pub const HOST_WRITE: Self = Self(AccessFlagBits::HOST_WRITE.0);
        pub const MEMORY_READ: Self = Self(AccessFlagBits::MEMORY_READ.0);
        pub const MEMORY_WRITE: Self = Self(AccessFlagBits::MEMORY_WRITE.0);
        pub const COMMAND_PREPROCESS_READ_EXT: Self =
            Self(AccessFlagBits::COMMAND_PREPROCESS_READ_EXT.0);
        pub const COMMAND_PREPROCESS_WRITE_EXT: Self =
            Self(AccessFlagBits::COMMAND_PREPROCESS_WRITE_EXT.0);
        pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self =
            Self(AccessFlagBits::COLOR_ATTACHMENT_READ_NONCOHERENT_EXT.0);
        pub const CONDITIONAL_RENDERING_READ_EXT: Self =
            Self(AccessFlagBits::CONDITIONAL_RENDERING_READ_EXT.0);
        pub const ACCELERATION_STRUCTURE_READ_KHR: Self =
            Self(AccessFlagBits::ACCELERATION_STRUCTURE_READ_KHR.0);
        pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self =
            Self(AccessFlagBits::ACCELERATION_STRUCTURE_WRITE_KHR.0);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self =
            Self(AccessFlagBits::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR.0);
        pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self =
            Self(AccessFlagBits::FRAGMENT_DENSITY_MAP_READ_EXT.0);
        pub const TRANSFORM_FEEDBACK_WRITE_EXT: Self =
            Self(AccessFlagBits::TRANSFORM_FEEDBACK_WRITE_EXT.0);
        pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self =
            Self(AccessFlagBits::TRANSFORM_FEEDBACK_COUNTER_READ_EXT.0);
        pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self =
            Self(AccessFlagBits::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT.0);
        pub const ACCELERATION_STRUCTURE_READ_NV: Self = Self::ACCELERATION_STRUCTURE_READ_KHR;
        pub const ACCELERATION_STRUCTURE_WRITE_NV: Self = Self::ACCELERATION_STRUCTURE_WRITE_KHR;
        pub const COMMAND_PREPROCESS_READ_NV: Self = Self::COMMAND_PREPROCESS_READ_EXT;
        pub const COMMAND_PREPROCESS_WRITE_NV: Self = Self::COMMAND_PREPROCESS_WRITE_EXT;
        pub const NONE_KHR: Self = Self::NONE;
        pub const SHADING_RATE_IMAGE_READ_NV: Self =
            Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
        pub const NONE: Self = Self(0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct AccessFlagBits(u32);
    impl AccessFlagBits {
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
        pub const COMMAND_PREPROCESS_READ_EXT: Self = Self(1 << 17);
        pub const COMMAND_PREPROCESS_WRITE_EXT: Self = Self(1 << 18);
        pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(1 << 19);
        pub const CONDITIONAL_RENDERING_READ_EXT: Self = Self(1 << 20);
        pub const ACCELERATION_STRUCTURE_READ_KHR: Self = Self(1 << 21);
        pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(1 << 22);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self = Self(1 << 23);
        pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(1 << 24);
        pub const TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(1 << 25);
        pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(1 << 26);
        pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self = Self(1 << 27);
        pub const ACCELERATION_STRUCTURE_READ_NV: Self = Self::ACCELERATION_STRUCTURE_READ_KHR;
        pub const ACCELERATION_STRUCTURE_WRITE_NV: Self = Self::ACCELERATION_STRUCTURE_WRITE_KHR;
        pub const COMMAND_PREPROCESS_READ_NV: Self = Self::COMMAND_PREPROCESS_READ_EXT;
        pub const COMMAND_PREPROCESS_WRITE_NV: Self = Self::COMMAND_PREPROCESS_WRITE_EXT;
        pub const SHADING_RATE_IMAGE_READ_NV: Self =
            Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct BufferUsageFlags(Flags);
    vk_bitflags_wrapped!(BufferUsageFlags, Flags);
    impl BufferUsageFlags {
        pub const TRANSFER_SRC: Self = Self(BufferUsageFlagBits::TRANSFER_SRC.0);
        pub const TRANSFER_DST: Self = Self(BufferUsageFlagBits::TRANSFER_DST.0);
        pub const UNIFORM_TEXEL_BUFFER: Self = Self(BufferUsageFlagBits::UNIFORM_TEXEL_BUFFER.0);
        pub const STORAGE_TEXEL_BUFFER: Self = Self(BufferUsageFlagBits::STORAGE_TEXEL_BUFFER.0);
        pub const UNIFORM_BUFFER: Self = Self(BufferUsageFlagBits::UNIFORM_BUFFER.0);
        pub const STORAGE_BUFFER: Self = Self(BufferUsageFlagBits::STORAGE_BUFFER.0);
        pub const INDEX_BUFFER: Self = Self(BufferUsageFlagBits::INDEX_BUFFER.0);
        pub const VERTEX_BUFFER: Self = Self(BufferUsageFlagBits::VERTEX_BUFFER.0);
        pub const INDIRECT_BUFFER: Self = Self(BufferUsageFlagBits::INDIRECT_BUFFER.0);
        pub const CONDITIONAL_RENDERING_EXT: Self =
            Self(BufferUsageFlagBits::CONDITIONAL_RENDERING_EXT.0);
        pub const SHADER_BINDING_TABLE_KHR: Self =
            Self(BufferUsageFlagBits::SHADER_BINDING_TABLE_KHR.0);
        pub const TRANSFORM_FEEDBACK_BUFFER_EXT: Self =
            Self(BufferUsageFlagBits::TRANSFORM_FEEDBACK_BUFFER_EXT.0);
        pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT: Self =
            Self(BufferUsageFlagBits::TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT.0);
        pub const VIDEO_DECODE_SRC_KHR: Self = Self(BufferUsageFlagBits::VIDEO_DECODE_SRC_KHR.0);
        pub const VIDEO_DECODE_DST_KHR: Self = Self(BufferUsageFlagBits::VIDEO_DECODE_DST_KHR.0);
        pub const VIDEO_ENCODE_DST_KHR: Self = Self(BufferUsageFlagBits::VIDEO_ENCODE_DST_KHR.0);
        pub const VIDEO_ENCODE_SRC_KHR: Self = Self(BufferUsageFlagBits::VIDEO_ENCODE_SRC_KHR.0);
        pub const SHADER_DEVICE_ADDRESS: Self = Self(BufferUsageFlagBits::SHADER_DEVICE_ADDRESS.0);
        pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR: Self =
            Self(BufferUsageFlagBits::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR.0);
        pub const ACCELERATION_STRUCTURE_STORAGE_KHR: Self =
            Self(BufferUsageFlagBits::ACCELERATION_STRUCTURE_STORAGE_KHR.0);
        pub const SAMPLER_DESCRIPTOR_BUFFER_EXT: Self =
            Self(BufferUsageFlagBits::SAMPLER_DESCRIPTOR_BUFFER_EXT.0);
        pub const RESOURCE_DESCRIPTOR_BUFFER_EXT: Self =
            Self(BufferUsageFlagBits::RESOURCE_DESCRIPTOR_BUFFER_EXT.0);
        pub const MICROMAP_BUILD_INPUT_READ_ONLY_EXT: Self =
            Self(BufferUsageFlagBits::MICROMAP_BUILD_INPUT_READ_ONLY_EXT.0);
        pub const MICROMAP_STORAGE_EXT: Self = Self(BufferUsageFlagBits::MICROMAP_STORAGE_EXT.0);
        pub const EXECUTION_GRAPH_SCRATCH_AMDX: Self =
            Self(BufferUsageFlagBits::EXECUTION_GRAPH_SCRATCH_AMDX.0);
        pub const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT: Self =
            Self(BufferUsageFlagBits::PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT.0);
        pub const TILE_MEMORY_QCOM: Self = Self(BufferUsageFlagBits::TILE_MEMORY_QCOM.0);
        pub const DESCRIPTOR_HEAP_EXT: Self = Self(BufferUsageFlagBits::DESCRIPTOR_HEAP_EXT.0);
        pub const RAY_TRACING_NV: Self = Self::SHADER_BINDING_TABLE_KHR;
        pub const SHADER_DEVICE_ADDRESS_EXT: Self = Self::SHADER_DEVICE_ADDRESS;
        pub const SHADER_DEVICE_ADDRESS_KHR: Self = Self::SHADER_DEVICE_ADDRESS;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct BufferUsageFlagBits(u32);
    impl BufferUsageFlagBits {
        pub const TRANSFER_SRC: Self = Self(1 << 0);
        pub const TRANSFER_DST: Self = Self(1 << 1);
        pub const UNIFORM_TEXEL_BUFFER: Self = Self(1 << 2);
        pub const STORAGE_TEXEL_BUFFER: Self = Self(1 << 3);
        pub const UNIFORM_BUFFER: Self = Self(1 << 4);
        pub const STORAGE_BUFFER: Self = Self(1 << 5);
        pub const INDEX_BUFFER: Self = Self(1 << 6);
        pub const VERTEX_BUFFER: Self = Self(1 << 7);
        pub const INDIRECT_BUFFER: Self = Self(1 << 8);
        pub const CONDITIONAL_RENDERING_EXT: Self = Self(1 << 9);
        pub const SHADER_BINDING_TABLE_KHR: Self = Self(1 << 10);
        pub const TRANSFORM_FEEDBACK_BUFFER_EXT: Self = Self(1 << 11);
        pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT: Self = Self(1 << 12);
        pub const VIDEO_DECODE_SRC_KHR: Self = Self(1 << 13);
        pub const VIDEO_DECODE_DST_KHR: Self = Self(1 << 14);
        pub const VIDEO_ENCODE_DST_KHR: Self = Self(1 << 15);
        pub const VIDEO_ENCODE_SRC_KHR: Self = Self(1 << 16);
        pub const SHADER_DEVICE_ADDRESS: Self = Self(1 << 17);
        pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR: Self = Self(1 << 19);
        pub const ACCELERATION_STRUCTURE_STORAGE_KHR: Self = Self(1 << 20);
        pub const SAMPLER_DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 21);
        pub const RESOURCE_DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 22);
        pub const MICROMAP_BUILD_INPUT_READ_ONLY_EXT: Self = Self(1 << 23);
        pub const MICROMAP_STORAGE_EXT: Self = Self(1 << 24);
        pub const EXECUTION_GRAPH_SCRATCH_AMDX: Self = Self(1 << 25);
        pub const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 26);
        pub const TILE_MEMORY_QCOM: Self = Self(1 << 27);
        pub const DESCRIPTOR_HEAP_EXT: Self = Self(1 << 28);
        pub const RAY_TRACING_NV: Self = Self::SHADER_BINDING_TABLE_KHR;
        pub const SHADER_DEVICE_ADDRESS_EXT: Self = Self::SHADER_DEVICE_ADDRESS;
        pub const SHADER_DEVICE_ADDRESS_KHR: Self = Self::SHADER_DEVICE_ADDRESS;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct BufferCreateFlags(Flags);
    vk_bitflags_wrapped!(BufferCreateFlags, Flags);
    impl BufferCreateFlags {
        pub const SPARSE_BINDING: Self = Self(BufferCreateFlagBits::SPARSE_BINDING.0);
        pub const SPARSE_RESIDENCY: Self = Self(BufferCreateFlagBits::SPARSE_RESIDENCY.0);
        pub const SPARSE_ALIASED: Self = Self(BufferCreateFlagBits::SPARSE_ALIASED.0);
        pub const PROTECTED: Self = Self(BufferCreateFlagBits::PROTECTED.0);
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self =
            Self(BufferCreateFlagBits::DEVICE_ADDRESS_CAPTURE_REPLAY.0);
        pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self =
            Self(BufferCreateFlagBits::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT.0);
        pub const VIDEO_PROFILE_INDEPENDENT_KHR: Self =
            Self(BufferCreateFlagBits::VIDEO_PROFILE_INDEPENDENT_KHR.0);
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct BufferCreateFlagBits(u32);
    impl BufferCreateFlagBits {
        pub const SPARSE_BINDING: Self = Self(1 << 0);
        pub const SPARSE_RESIDENCY: Self = Self(1 << 1);
        pub const SPARSE_ALIASED: Self = Self(1 << 2);
        pub const PROTECTED: Self = Self(1 << 3);
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(1 << 4);
        pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(1 << 5);
        pub const VIDEO_PROFILE_INDEPENDENT_KHR: Self = Self(1 << 6);
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ShaderStageFlags(Flags);
    vk_bitflags_wrapped!(ShaderStageFlags, Flags);
    impl ShaderStageFlags {
        pub const VERTEX: Self = Self(ShaderStageFlagBits::VERTEX.0);
        pub const TESSELLATION_CONTROL: Self = Self(ShaderStageFlagBits::TESSELLATION_CONTROL.0);
        pub const TESSELLATION_EVALUATION: Self =
            Self(ShaderStageFlagBits::TESSELLATION_EVALUATION.0);
        pub const GEOMETRY: Self = Self(ShaderStageFlagBits::GEOMETRY.0);
        pub const FRAGMENT: Self = Self(ShaderStageFlagBits::FRAGMENT.0);
        pub const COMPUTE: Self = Self(ShaderStageFlagBits::COMPUTE.0);
        pub const TASK_EXT: Self = Self(ShaderStageFlagBits::TASK_EXT.0);
        pub const MESH_EXT: Self = Self(ShaderStageFlagBits::MESH_EXT.0);
        pub const RAYGEN_KHR: Self = Self(ShaderStageFlagBits::RAYGEN_KHR.0);
        pub const ANY_HIT_KHR: Self = Self(ShaderStageFlagBits::ANY_HIT_KHR.0);
        pub const CLOSEST_HIT_KHR: Self = Self(ShaderStageFlagBits::CLOSEST_HIT_KHR.0);
        pub const MISS_KHR: Self = Self(ShaderStageFlagBits::MISS_KHR.0);
        pub const INTERSECTION_KHR: Self = Self(ShaderStageFlagBits::INTERSECTION_KHR.0);
        pub const CALLABLE_KHR: Self = Self(ShaderStageFlagBits::CALLABLE_KHR.0);
        pub const SUBPASS_SHADING_HUAWEI: Self =
            Self(ShaderStageFlagBits::SUBPASS_SHADING_HUAWEI.0);
        pub const CLUSTER_CULLING_HUAWEI: Self =
            Self(ShaderStageFlagBits::CLUSTER_CULLING_HUAWEI.0);
        pub const ANY_HIT_NV: Self = Self::ANY_HIT_KHR;
        pub const CALLABLE_NV: Self = Self::CALLABLE_KHR;
        pub const CLOSEST_HIT_NV: Self = Self::CLOSEST_HIT_KHR;
        pub const INTERSECTION_NV: Self = Self::INTERSECTION_KHR;
        pub const MESH_NV: Self = Self::MESH_EXT;
        pub const MISS_NV: Self = Self::MISS_KHR;
        pub const RAYGEN_NV: Self = Self::RAYGEN_KHR;
        pub const TASK_NV: Self = Self::TASK_EXT;
        pub const ALL_GRAPHICS: Self = Self(0x0000001F);
        pub const ALL: Self = Self(0x7FFFFFFF);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ShaderStageFlagBits(u32);
    impl ShaderStageFlagBits {
        pub const VERTEX: Self = Self(1 << 0);
        pub const TESSELLATION_CONTROL: Self = Self(1 << 1);
        pub const TESSELLATION_EVALUATION: Self = Self(1 << 2);
        pub const GEOMETRY: Self = Self(1 << 3);
        pub const FRAGMENT: Self = Self(1 << 4);
        pub const COMPUTE: Self = Self(1 << 5);
        pub const TASK_EXT: Self = Self(1 << 6);
        pub const MESH_EXT: Self = Self(1 << 7);
        pub const RAYGEN_KHR: Self = Self(1 << 8);
        pub const ANY_HIT_KHR: Self = Self(1 << 9);
        pub const CLOSEST_HIT_KHR: Self = Self(1 << 10);
        pub const MISS_KHR: Self = Self(1 << 11);
        pub const INTERSECTION_KHR: Self = Self(1 << 12);
        pub const CALLABLE_KHR: Self = Self(1 << 13);
        pub const SUBPASS_SHADING_HUAWEI: Self = Self(1 << 14);
        pub const CLUSTER_CULLING_HUAWEI: Self = Self(1 << 19);
        pub const ANY_HIT_NV: Self = Self::ANY_HIT_KHR;
        pub const CALLABLE_NV: Self = Self::CALLABLE_KHR;
        pub const CLOSEST_HIT_NV: Self = Self::CLOSEST_HIT_KHR;
        pub const INTERSECTION_NV: Self = Self::INTERSECTION_KHR;
        pub const MESH_NV: Self = Self::MESH_EXT;
        pub const MISS_NV: Self = Self::MISS_KHR;
        pub const RAYGEN_NV: Self = Self::RAYGEN_KHR;
        pub const TASK_NV: Self = Self::TASK_EXT;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ImageUsageFlags(Flags);
    vk_bitflags_wrapped!(ImageUsageFlags, Flags);
    impl ImageUsageFlags {
        pub const TRANSFER_SRC: Self = Self(ImageUsageFlagBits::TRANSFER_SRC.0);
        pub const TRANSFER_DST: Self = Self(ImageUsageFlagBits::TRANSFER_DST.0);
        pub const SAMPLED: Self = Self(ImageUsageFlagBits::SAMPLED.0);
        pub const STORAGE: Self = Self(ImageUsageFlagBits::STORAGE.0);
        pub const COLOR_ATTACHMENT: Self = Self(ImageUsageFlagBits::COLOR_ATTACHMENT.0);
        pub const DEPTH_STENCIL_ATTACHMENT: Self =
            Self(ImageUsageFlagBits::DEPTH_STENCIL_ATTACHMENT.0);
        pub const TRANSIENT_ATTACHMENT: Self = Self(ImageUsageFlagBits::TRANSIENT_ATTACHMENT.0);
        pub const INPUT_ATTACHMENT: Self = Self(ImageUsageFlagBits::INPUT_ATTACHMENT.0);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
            Self(ImageUsageFlagBits::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0);
        pub const FRAGMENT_DENSITY_MAP_EXT: Self =
            Self(ImageUsageFlagBits::FRAGMENT_DENSITY_MAP_EXT.0);
        pub const VIDEO_DECODE_DST_KHR: Self = Self(ImageUsageFlagBits::VIDEO_DECODE_DST_KHR.0);
        pub const VIDEO_DECODE_SRC_KHR: Self = Self(ImageUsageFlagBits::VIDEO_DECODE_SRC_KHR.0);
        pub const VIDEO_DECODE_DPB_KHR: Self = Self(ImageUsageFlagBits::VIDEO_DECODE_DPB_KHR.0);
        pub const VIDEO_ENCODE_DST_KHR: Self = Self(ImageUsageFlagBits::VIDEO_ENCODE_DST_KHR.0);
        pub const VIDEO_ENCODE_SRC_KHR: Self = Self(ImageUsageFlagBits::VIDEO_ENCODE_SRC_KHR.0);
        pub const VIDEO_ENCODE_DPB_KHR: Self = Self(ImageUsageFlagBits::VIDEO_ENCODE_DPB_KHR.0);
        pub const INVOCATION_MASK_HUAWEI: Self = Self(ImageUsageFlagBits::INVOCATION_MASK_HUAWEI.0);
        pub const ATTACHMENT_FEEDBACK_LOOP_EXT: Self =
            Self(ImageUsageFlagBits::ATTACHMENT_FEEDBACK_LOOP_EXT.0);
        pub const SAMPLE_WEIGHT_QCOM: Self = Self(ImageUsageFlagBits::SAMPLE_WEIGHT_QCOM.0);
        pub const SAMPLE_BLOCK_MATCH_QCOM: Self =
            Self(ImageUsageFlagBits::SAMPLE_BLOCK_MATCH_QCOM.0);
        pub const HOST_TRANSFER: Self = Self(ImageUsageFlagBits::HOST_TRANSFER.0);
        pub const TENSOR_ALIASING_ARM: Self = Self(ImageUsageFlagBits::TENSOR_ALIASING_ARM.0);
        pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self =
            Self(ImageUsageFlagBits::VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR.0);
        pub const VIDEO_ENCODE_EMPHASIS_MAP_KHR: Self =
            Self(ImageUsageFlagBits::VIDEO_ENCODE_EMPHASIS_MAP_KHR.0);
        pub const TILE_MEMORY_QCOM: Self = Self(ImageUsageFlagBits::TILE_MEMORY_QCOM.0);
        pub const HOST_TRANSFER_EXT: Self = Self::HOST_TRANSFER;
        pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ImageUsageFlagBits(u32);
    impl ImageUsageFlagBits {
        pub const TRANSFER_SRC: Self = Self(1 << 0);
        pub const TRANSFER_DST: Self = Self(1 << 1);
        pub const SAMPLED: Self = Self(1 << 2);
        pub const STORAGE: Self = Self(1 << 3);
        pub const COLOR_ATTACHMENT: Self = Self(1 << 4);
        pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(1 << 5);
        pub const TRANSIENT_ATTACHMENT: Self = Self(1 << 6);
        pub const INPUT_ATTACHMENT: Self = Self(1 << 7);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 8);
        pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(1 << 9);
        pub const VIDEO_DECODE_DST_KHR: Self = Self(1 << 10);
        pub const VIDEO_DECODE_SRC_KHR: Self = Self(1 << 11);
        pub const VIDEO_DECODE_DPB_KHR: Self = Self(1 << 12);
        pub const VIDEO_ENCODE_DST_KHR: Self = Self(1 << 13);
        pub const VIDEO_ENCODE_SRC_KHR: Self = Self(1 << 14);
        pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1 << 15);
        pub const INVOCATION_MASK_HUAWEI: Self = Self(1 << 18);
        pub const ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(1 << 19);
        pub const SAMPLE_WEIGHT_QCOM: Self = Self(1 << 20);
        pub const SAMPLE_BLOCK_MATCH_QCOM: Self = Self(1 << 21);
        pub const HOST_TRANSFER: Self = Self(1 << 22);
        pub const TENSOR_ALIASING_ARM: Self = Self(1 << 23);
        pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self = Self(1 << 25);
        pub const VIDEO_ENCODE_EMPHASIS_MAP_KHR: Self = Self(1 << 26);
        pub const TILE_MEMORY_QCOM: Self = Self(1 << 27);
        pub const HOST_TRANSFER_EXT: Self = Self::HOST_TRANSFER;
        pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ImageCreateFlags(Flags);
    vk_bitflags_wrapped!(ImageCreateFlags, Flags);
    impl ImageCreateFlags {
        pub const SPARSE_BINDING: Self = Self(ImageCreateFlagBits::SPARSE_BINDING.0);
        pub const SPARSE_RESIDENCY: Self = Self(ImageCreateFlagBits::SPARSE_RESIDENCY.0);
        pub const SPARSE_ALIASED: Self = Self(ImageCreateFlagBits::SPARSE_ALIASED.0);
        pub const MUTABLE_FORMAT: Self = Self(ImageCreateFlagBits::MUTABLE_FORMAT.0);
        pub const CUBE_COMPATIBLE: Self = Self(ImageCreateFlagBits::CUBE_COMPATIBLE.0);
        pub const _2D_ARRAY_COMPATIBLE: Self = Self(ImageCreateFlagBits::_2D_ARRAY_COMPATIBLE.0);
        pub const SPLIT_INSTANCE_BIND_REGIONS: Self =
            Self(ImageCreateFlagBits::SPLIT_INSTANCE_BIND_REGIONS.0);
        pub const BLOCK_TEXEL_VIEW_COMPATIBLE: Self =
            Self(ImageCreateFlagBits::BLOCK_TEXEL_VIEW_COMPATIBLE.0);
        pub const EXTENDED_USAGE: Self = Self(ImageCreateFlagBits::EXTENDED_USAGE.0);
        pub const DISJOINT: Self = Self(ImageCreateFlagBits::DISJOINT.0);
        pub const ALIAS: Self = Self(ImageCreateFlagBits::ALIAS.0);
        pub const PROTECTED: Self = Self(ImageCreateFlagBits::PROTECTED.0);
        pub const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT: Self =
            Self(ImageCreateFlagBits::SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT.0);
        pub const CORNER_SAMPLED_NV: Self = Self(ImageCreateFlagBits::CORNER_SAMPLED_NV.0);
        pub const SUBSAMPLED_EXT: Self = Self(ImageCreateFlagBits::SUBSAMPLED_EXT.0);
        pub const FRAGMENT_DENSITY_MAP_OFFSET_EXT: Self =
            Self(ImageCreateFlagBits::FRAGMENT_DENSITY_MAP_OFFSET_EXT.0);
        pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT: Self =
            Self(ImageCreateFlagBits::DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT.0);
        pub const _2D_VIEW_COMPATIBLE_EXT: Self =
            Self(ImageCreateFlagBits::_2D_VIEW_COMPATIBLE_EXT.0);
        pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT: Self =
            Self(ImageCreateFlagBits::MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT.0);
        pub const VIDEO_PROFILE_INDEPENDENT_KHR: Self =
            Self(ImageCreateFlagBits::VIDEO_PROFILE_INDEPENDENT_KHR.0);
        pub const _2D_ARRAY_COMPATIBLE_KHR: Self = Self::_2D_ARRAY_COMPATIBLE;
        pub const ALIAS_KHR: Self = Self::ALIAS;
        pub const BLOCK_TEXEL_VIEW_COMPATIBLE_KHR: Self = Self::BLOCK_TEXEL_VIEW_COMPATIBLE;
        pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self =
            Self::DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT;
        pub const DISJOINT_KHR: Self = Self::DISJOINT;
        pub const EXTENDED_USAGE_KHR: Self = Self::EXTENDED_USAGE;
        pub const FRAGMENT_DENSITY_MAP_OFFSET_QCOM: Self = Self::FRAGMENT_DENSITY_MAP_OFFSET_EXT;
        pub const SPLIT_INSTANCE_BIND_REGIONS_KHR: Self = Self::SPLIT_INSTANCE_BIND_REGIONS;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ImageCreateFlagBits(u32);
    impl ImageCreateFlagBits {
        pub const SPARSE_BINDING: Self = Self(1 << 0);
        pub const SPARSE_RESIDENCY: Self = Self(1 << 1);
        pub const SPARSE_ALIASED: Self = Self(1 << 2);
        pub const MUTABLE_FORMAT: Self = Self(1 << 3);
        pub const CUBE_COMPATIBLE: Self = Self(1 << 4);
        pub const _2D_ARRAY_COMPATIBLE: Self = Self(1 << 5);
        pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(1 << 6);
        pub const BLOCK_TEXEL_VIEW_COMPATIBLE: Self = Self(1 << 7);
        pub const EXTENDED_USAGE: Self = Self(1 << 8);
        pub const DISJOINT: Self = Self(1 << 9);
        pub const ALIAS: Self = Self(1 << 10);
        pub const PROTECTED: Self = Self(1 << 11);
        pub const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT: Self = Self(1 << 12);
        pub const CORNER_SAMPLED_NV: Self = Self(1 << 13);
        pub const SUBSAMPLED_EXT: Self = Self(1 << 14);
        pub const FRAGMENT_DENSITY_MAP_OFFSET_EXT: Self = Self(1 << 15);
        pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT: Self = Self(1 << 16);
        pub const _2D_VIEW_COMPATIBLE_EXT: Self = Self(1 << 17);
        pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT: Self = Self(1 << 18);
        pub const VIDEO_PROFILE_INDEPENDENT_KHR: Self = Self(1 << 20);
        pub const _2D_ARRAY_COMPATIBLE_KHR: Self = Self::_2D_ARRAY_COMPATIBLE;
        pub const ALIAS_KHR: Self = Self::ALIAS;
        pub const BLOCK_TEXEL_VIEW_COMPATIBLE_KHR: Self = Self::BLOCK_TEXEL_VIEW_COMPATIBLE;
        pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self =
            Self::DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT;
        pub const DISJOINT_KHR: Self = Self::DISJOINT;
        pub const EXTENDED_USAGE_KHR: Self = Self::EXTENDED_USAGE;
        pub const FRAGMENT_DENSITY_MAP_OFFSET_QCOM: Self = Self::FRAGMENT_DENSITY_MAP_OFFSET_EXT;
        pub const SPLIT_INSTANCE_BIND_REGIONS_KHR: Self = Self::SPLIT_INSTANCE_BIND_REGIONS;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ImageViewCreateFlags(Flags);
    vk_bitflags_wrapped!(ImageViewCreateFlags, Flags);
    impl ImageViewCreateFlags {
        pub const FRAGMENT_DENSITY_MAP_DYNAMIC_EXT: Self =
            Self(ImageViewCreateFlagBits::FRAGMENT_DENSITY_MAP_DYNAMIC_EXT.0);
        pub const FRAGMENT_DENSITY_MAP_DEFERRED_EXT: Self =
            Self(ImageViewCreateFlagBits::FRAGMENT_DENSITY_MAP_DEFERRED_EXT.0);
        pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self =
            Self(ImageViewCreateFlagBits::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ImageViewCreateFlagBits(u32);
    impl ImageViewCreateFlagBits {
        pub const FRAGMENT_DENSITY_MAP_DYNAMIC_EXT: Self = Self(1 << 0);
        pub const FRAGMENT_DENSITY_MAP_DEFERRED_EXT: Self = Self(1 << 1);
        pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(1 << 2);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineCreateFlags(Flags);
    vk_bitflags_wrapped!(PipelineCreateFlags, Flags);
    impl PipelineCreateFlags {
        pub const DISABLE_OPTIMIZATION: Self = Self(PipelineCreateFlagBits::DISABLE_OPTIMIZATION.0);
        pub const ALLOW_DERIVATIVES: Self = Self(PipelineCreateFlagBits::ALLOW_DERIVATIVES.0);
        pub const DERIVATIVE: Self = Self(PipelineCreateFlagBits::DERIVATIVE.0);
        pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self =
            Self(PipelineCreateFlagBits::VIEW_INDEX_FROM_DEVICE_INDEX.0);
        pub const DISPATCH_BASE: Self = Self(PipelineCreateFlagBits::DISPATCH_BASE.0);
        pub const DEFER_COMPILE_NV: Self = Self(PipelineCreateFlagBits::DEFER_COMPILE_NV.0);
        pub const CAPTURE_STATISTICS_KHR: Self =
            Self(PipelineCreateFlagBits::CAPTURE_STATISTICS_KHR.0);
        pub const CAPTURE_INTERNAL_REPRESENTATIONS_KHR: Self =
            Self(PipelineCreateFlagBits::CAPTURE_INTERNAL_REPRESENTATIONS_KHR.0);
        pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self =
            Self(PipelineCreateFlagBits::FAIL_ON_PIPELINE_COMPILE_REQUIRED.0);
        pub const EARLY_RETURN_ON_FAILURE: Self =
            Self(PipelineCreateFlagBits::EARLY_RETURN_ON_FAILURE.0);
        pub const LINK_TIME_OPTIMIZATION_EXT: Self =
            Self(PipelineCreateFlagBits::LINK_TIME_OPTIMIZATION_EXT.0);
        pub const LIBRARY_KHR: Self = Self(PipelineCreateFlagBits::LIBRARY_KHR.0);
        pub const RAY_TRACING_SKIP_TRIANGLES_KHR: Self =
            Self(PipelineCreateFlagBits::RAY_TRACING_SKIP_TRIANGLES_KHR.0);
        pub const RAY_TRACING_SKIP_AABBS_KHR: Self =
            Self(PipelineCreateFlagBits::RAY_TRACING_SKIP_AABBS_KHR.0);
        pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR: Self =
            Self(PipelineCreateFlagBits::RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR.0);
        pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR: Self =
            Self(PipelineCreateFlagBits::RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR.0);
        pub const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR: Self =
            Self(PipelineCreateFlagBits::RAY_TRACING_NO_NULL_MISS_SHADERS_KHR.0);
        pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR: Self =
            Self(PipelineCreateFlagBits::RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR.0);
        pub const INDIRECT_BINDABLE_NV: Self = Self(PipelineCreateFlagBits::INDIRECT_BINDABLE_NV.0);
        pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR: Self =
            Self(PipelineCreateFlagBits::RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR.0);
        pub const RAY_TRACING_ALLOW_MOTION_NV: Self =
            Self(PipelineCreateFlagBits::RAY_TRACING_ALLOW_MOTION_NV.0);
        pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
            Self(PipelineCreateFlagBits::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0);
        pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self =
            Self(PipelineCreateFlagBits::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT.0);
        pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT: Self =
            Self(PipelineCreateFlagBits::RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT.0);
        pub const RAY_TRACING_OPACITY_MICROMAP_EXT: Self =
            Self(PipelineCreateFlagBits::RAY_TRACING_OPACITY_MICROMAP_EXT.0);
        pub const COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT: Self =
            Self(PipelineCreateFlagBits::COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT.0);
        pub const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT: Self =
            Self(PipelineCreateFlagBits::DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT.0);
        pub const NO_PROTECTED_ACCESS: Self = Self(PipelineCreateFlagBits::NO_PROTECTED_ACCESS.0);
        pub const RAY_TRACING_DISPLACEMENT_MICROMAP_NV: Self =
            Self(PipelineCreateFlagBits::RAY_TRACING_DISPLACEMENT_MICROMAP_NV.0);
        pub const DESCRIPTOR_BUFFER_EXT: Self =
            Self(PipelineCreateFlagBits::DESCRIPTOR_BUFFER_EXT.0);
        pub const PROTECTED_ACCESS_ONLY: Self =
            Self(PipelineCreateFlagBits::PROTECTED_ACCESS_ONLY.0);
        pub const DISPATCH_BASE_KHR: Self = Self::DISPATCH_BASE;
        pub const EARLY_RETURN_ON_FAILURE_EXT: Self = Self::EARLY_RETURN_ON_FAILURE;
        pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_EXT: Self =
            Self::FAIL_ON_PIPELINE_COMPILE_REQUIRED;
        pub const NO_PROTECTED_ACCESS_EXT: Self = Self::NO_PROTECTED_ACCESS;
        pub const PROTECTED_ACCESS_ONLY_EXT: Self = Self::PROTECTED_ACCESS_ONLY;
        pub const VIEW_INDEX_FROM_DEVICE_INDEX_KHR: Self = Self::VIEW_INDEX_FROM_DEVICE_INDEX;
        pub const PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self =
            Self::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT;
        pub const PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
            Self::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PipelineCreateFlagBits(u32);
    impl PipelineCreateFlagBits {
        pub const DISABLE_OPTIMIZATION: Self = Self(1 << 0);
        pub const ALLOW_DERIVATIVES: Self = Self(1 << 1);
        pub const DERIVATIVE: Self = Self(1 << 2);
        pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self(1 << 3);
        pub const DISPATCH_BASE: Self = Self(1 << 4);
        pub const DEFER_COMPILE_NV: Self = Self(1 << 5);
        pub const CAPTURE_STATISTICS_KHR: Self = Self(1 << 6);
        pub const CAPTURE_INTERNAL_REPRESENTATIONS_KHR: Self = Self(1 << 7);
        pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self = Self(1 << 8);
        pub const EARLY_RETURN_ON_FAILURE: Self = Self(1 << 9);
        pub const LINK_TIME_OPTIMIZATION_EXT: Self = Self(1 << 10);
        pub const LIBRARY_KHR: Self = Self(1 << 11);
        pub const RAY_TRACING_SKIP_TRIANGLES_KHR: Self = Self(1 << 12);
        pub const RAY_TRACING_SKIP_AABBS_KHR: Self = Self(1 << 13);
        pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR: Self = Self(1 << 14);
        pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR: Self = Self(1 << 15);
        pub const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR: Self = Self(1 << 16);
        pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR: Self = Self(1 << 17);
        pub const INDIRECT_BINDABLE_NV: Self = Self(1 << 18);
        pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR: Self = Self(1 << 19);
        pub const RAY_TRACING_ALLOW_MOTION_NV: Self = Self(1 << 20);
        pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 21);
        pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self = Self(1 << 22);
        pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT: Self = Self(1 << 23);
        pub const RAY_TRACING_OPACITY_MICROMAP_EXT: Self = Self(1 << 24);
        pub const COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(1 << 25);
        pub const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(1 << 26);
        pub const NO_PROTECTED_ACCESS: Self = Self(1 << 27);
        pub const RAY_TRACING_DISPLACEMENT_MICROMAP_NV: Self = Self(1 << 28);
        pub const DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 29);
        pub const PROTECTED_ACCESS_ONLY: Self = Self(1 << 30);
        pub const DISPATCH_BASE_KHR: Self = Self::DISPATCH_BASE;
        pub const EARLY_RETURN_ON_FAILURE_EXT: Self = Self::EARLY_RETURN_ON_FAILURE;
        pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_EXT: Self =
            Self::FAIL_ON_PIPELINE_COMPILE_REQUIRED;
        pub const NO_PROTECTED_ACCESS_EXT: Self = Self::NO_PROTECTED_ACCESS;
        pub const PROTECTED_ACCESS_ONLY_EXT: Self = Self::PROTECTED_ACCESS_ONLY;
        pub const VIEW_INDEX_FROM_DEVICE_INDEX_KHR: Self = Self::VIEW_INDEX_FROM_DEVICE_INDEX;
        pub const PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self =
            Self::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT;
        pub const PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
            Self::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ColorComponentFlags(Flags);
    vk_bitflags_wrapped!(ColorComponentFlags, Flags);
    impl ColorComponentFlags {
        pub const R: Self = Self(ColorComponentFlagBits::R.0);
        pub const G: Self = Self(ColorComponentFlagBits::G.0);
        pub const B: Self = Self(ColorComponentFlagBits::B.0);
        pub const A: Self = Self(ColorComponentFlagBits::A.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ColorComponentFlagBits(u32);
    impl ColorComponentFlagBits {
        pub const R: Self = Self(1 << 0);
        pub const G: Self = Self(1 << 1);
        pub const B: Self = Self(1 << 2);
        pub const A: Self = Self(1 << 3);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct FenceCreateFlags(Flags);
    vk_bitflags_wrapped!(FenceCreateFlags, Flags);
    impl FenceCreateFlags {
        pub const SIGNALED: Self = Self(FenceCreateFlagBits::SIGNALED.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct FenceCreateFlagBits(u32);
    impl FenceCreateFlagBits {
        pub const SIGNALED: Self = Self(1 << 0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SemaphoreCreateFlags(Flags);
    vk_bitflags_wrapped!(SemaphoreCreateFlags, Flags);
    impl SemaphoreCreateFlags {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct FormatFeatureFlags(Flags);
    vk_bitflags_wrapped!(FormatFeatureFlags, Flags);
    impl FormatFeatureFlags {
        pub const SAMPLED_IMAGE: Self = Self(FormatFeatureFlagBits::SAMPLED_IMAGE.0);
        pub const STORAGE_IMAGE: Self = Self(FormatFeatureFlagBits::STORAGE_IMAGE.0);
        pub const STORAGE_IMAGE_ATOMIC: Self = Self(FormatFeatureFlagBits::STORAGE_IMAGE_ATOMIC.0);
        pub const UNIFORM_TEXEL_BUFFER: Self = Self(FormatFeatureFlagBits::UNIFORM_TEXEL_BUFFER.0);
        pub const STORAGE_TEXEL_BUFFER: Self = Self(FormatFeatureFlagBits::STORAGE_TEXEL_BUFFER.0);
        pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self =
            Self(FormatFeatureFlagBits::STORAGE_TEXEL_BUFFER_ATOMIC.0);
        pub const VERTEX_BUFFER: Self = Self(FormatFeatureFlagBits::VERTEX_BUFFER.0);
        pub const COLOR_ATTACHMENT: Self = Self(FormatFeatureFlagBits::COLOR_ATTACHMENT.0);
        pub const COLOR_ATTACHMENT_BLEND: Self =
            Self(FormatFeatureFlagBits::COLOR_ATTACHMENT_BLEND.0);
        pub const DEPTH_STENCIL_ATTACHMENT: Self =
            Self(FormatFeatureFlagBits::DEPTH_STENCIL_ATTACHMENT.0);
        pub const BLIT_SRC: Self = Self(FormatFeatureFlagBits::BLIT_SRC.0);
        pub const BLIT_DST: Self = Self(FormatFeatureFlagBits::BLIT_DST.0);
        pub const SAMPLED_IMAGE_FILTER_LINEAR: Self =
            Self(FormatFeatureFlagBits::SAMPLED_IMAGE_FILTER_LINEAR.0);
        pub const SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self =
            Self(FormatFeatureFlagBits::SAMPLED_IMAGE_FILTER_CUBIC_EXT.0);
        pub const TRANSFER_SRC: Self = Self(FormatFeatureFlagBits::TRANSFER_SRC.0);
        pub const TRANSFER_DST: Self = Self(FormatFeatureFlagBits::TRANSFER_DST.0);
        pub const SAMPLED_IMAGE_FILTER_MINMAX: Self =
            Self(FormatFeatureFlagBits::SAMPLED_IMAGE_FILTER_MINMAX.0);
        pub const MIDPOINT_CHROMA_SAMPLES: Self =
            Self(FormatFeatureFlagBits::MIDPOINT_CHROMA_SAMPLES.0);
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self =
            Self(FormatFeatureFlagBits::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER.0);
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self = Self(
            FormatFeatureFlagBits::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER.0,
        );
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self = Self(
            FormatFeatureFlagBits::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT.0,
        );
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self = Self(FormatFeatureFlagBits::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE.0);
        pub const DISJOINT: Self = Self(FormatFeatureFlagBits::DISJOINT.0);
        pub const COSITED_CHROMA_SAMPLES: Self =
            Self(FormatFeatureFlagBits::COSITED_CHROMA_SAMPLES.0);
        pub const FRAGMENT_DENSITY_MAP_EXT: Self =
            Self(FormatFeatureFlagBits::FRAGMENT_DENSITY_MAP_EXT.0);
        pub const VIDEO_DECODE_OUTPUT_KHR: Self =
            Self(FormatFeatureFlagBits::VIDEO_DECODE_OUTPUT_KHR.0);
        pub const VIDEO_DECODE_DPB_KHR: Self = Self(FormatFeatureFlagBits::VIDEO_DECODE_DPB_KHR.0);
        pub const VIDEO_ENCODE_INPUT_KHR: Self =
            Self(FormatFeatureFlagBits::VIDEO_ENCODE_INPUT_KHR.0);
        pub const VIDEO_ENCODE_DPB_KHR: Self = Self(FormatFeatureFlagBits::VIDEO_ENCODE_DPB_KHR.0);
        pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self =
            Self(FormatFeatureFlagBits::ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR.0);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
            Self(FormatFeatureFlagBits::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0);
        pub const COSITED_CHROMA_SAMPLES_KHR: Self = Self::COSITED_CHROMA_SAMPLES;
        pub const DISJOINT_KHR: Self = Self::DISJOINT;
        pub const MIDPOINT_CHROMA_SAMPLES_KHR: Self = Self::MIDPOINT_CHROMA_SAMPLES;
        pub const SAMPLED_IMAGE_FILTER_CUBIC_IMG: Self = Self::SAMPLED_IMAGE_FILTER_CUBIC_EXT;
        pub const SAMPLED_IMAGE_FILTER_MINMAX_EXT: Self = Self::SAMPLED_IMAGE_FILTER_MINMAX;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR: Self =
            Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR:
            Self = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR: Self =
            Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR: Self =
            Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER;
        pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
        pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct FormatFeatureFlagBits(u32);
    impl FormatFeatureFlagBits {
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
        pub const SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self = Self(1 << 13);
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
        pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(1 << 24);
        pub const VIDEO_DECODE_OUTPUT_KHR: Self = Self(1 << 25);
        pub const VIDEO_DECODE_DPB_KHR: Self = Self(1 << 26);
        pub const VIDEO_ENCODE_INPUT_KHR: Self = Self(1 << 27);
        pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1 << 28);
        pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self = Self(1 << 29);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 30);
        pub const COSITED_CHROMA_SAMPLES_KHR: Self = Self::COSITED_CHROMA_SAMPLES;
        pub const DISJOINT_KHR: Self = Self::DISJOINT;
        pub const MIDPOINT_CHROMA_SAMPLES_KHR: Self = Self::MIDPOINT_CHROMA_SAMPLES;
        pub const SAMPLED_IMAGE_FILTER_CUBIC_IMG: Self = Self::SAMPLED_IMAGE_FILTER_CUBIC_EXT;
        pub const SAMPLED_IMAGE_FILTER_MINMAX_EXT: Self = Self::SAMPLED_IMAGE_FILTER_MINMAX;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR: Self =
            Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR:
            Self = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR: Self =
            Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER;
        pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR: Self =
            Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER;
        pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
        pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct QueryControlFlags(Flags);
    vk_bitflags_wrapped!(QueryControlFlags, Flags);
    impl QueryControlFlags {
        pub const PRECISE: Self = Self(QueryControlFlagBits::PRECISE.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct QueryControlFlagBits(u32);
    impl QueryControlFlagBits {
        pub const PRECISE: Self = Self(1 << 0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct QueryResultFlags(Flags);
    vk_bitflags_wrapped!(QueryResultFlags, Flags);
    impl QueryResultFlags {
        pub const _64: Self = Self(QueryResultFlagBits::_64.0);
        pub const WAIT: Self = Self(QueryResultFlagBits::WAIT.0);
        pub const WITH_AVAILABILITY: Self = Self(QueryResultFlagBits::WITH_AVAILABILITY.0);
        pub const PARTIAL: Self = Self(QueryResultFlagBits::PARTIAL.0);
        pub const WITH_STATUS_KHR: Self = Self(QueryResultFlagBits::WITH_STATUS_KHR.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct QueryResultFlagBits(u32);
    impl QueryResultFlagBits {
        pub const _64: Self = Self(1 << 0);
        pub const WAIT: Self = Self(1 << 1);
        pub const WITH_AVAILABILITY: Self = Self(1 << 2);
        pub const PARTIAL: Self = Self(1 << 3);
        pub const WITH_STATUS_KHR: Self = Self(1 << 4);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ShaderModuleCreateFlags(Flags);
    vk_bitflags_wrapped!(ShaderModuleCreateFlags, Flags);
    impl ShaderModuleCreateFlags {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct EventCreateFlags(Flags);
    vk_bitflags_wrapped!(EventCreateFlags, Flags);
    impl EventCreateFlags {
        pub const DEVICE_ONLY: Self = Self(EventCreateFlagBits::DEVICE_ONLY.0);
        pub const DEVICE_ONLY_KHR: Self = Self::DEVICE_ONLY;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct EventCreateFlagBits(u32);
    impl EventCreateFlagBits {
        pub const DEVICE_ONLY: Self = Self(1 << 0);
        pub const DEVICE_ONLY_KHR: Self = Self::DEVICE_ONLY;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CommandPoolCreateFlags(Flags);
    vk_bitflags_wrapped!(CommandPoolCreateFlags, Flags);
    impl CommandPoolCreateFlags {
        pub const TRANSIENT: Self = Self(CommandPoolCreateFlagBits::TRANSIENT.0);
        pub const RESET_COMMAND_BUFFER: Self =
            Self(CommandPoolCreateFlagBits::RESET_COMMAND_BUFFER.0);
        pub const PROTECTED: Self = Self(CommandPoolCreateFlagBits::PROTECTED.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct CommandPoolCreateFlagBits(u32);
    impl CommandPoolCreateFlagBits {
        pub const TRANSIENT: Self = Self(1 << 0);
        pub const RESET_COMMAND_BUFFER: Self = Self(1 << 1);
        pub const PROTECTED: Self = Self(1 << 2);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CommandPoolResetFlags(Flags);
    vk_bitflags_wrapped!(CommandPoolResetFlags, Flags);
    impl CommandPoolResetFlags {
        pub const RELEASE_RESOURCES: Self = Self(CommandPoolResetFlagBits::RELEASE_RESOURCES.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct CommandPoolResetFlagBits(u32);
    impl CommandPoolResetFlagBits {
        pub const RELEASE_RESOURCES: Self = Self(1 << 0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CommandBufferResetFlags(Flags);
    vk_bitflags_wrapped!(CommandBufferResetFlags, Flags);
    impl CommandBufferResetFlags {
        pub const RELEASE_RESOURCES: Self = Self(CommandBufferResetFlagBits::RELEASE_RESOURCES.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct CommandBufferResetFlagBits(u32);
    impl CommandBufferResetFlagBits {
        pub const RELEASE_RESOURCES: Self = Self(1 << 0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CommandBufferUsageFlags(Flags);
    vk_bitflags_wrapped!(CommandBufferUsageFlags, Flags);
    impl CommandBufferUsageFlags {
        pub const ONE_TIME_SUBMIT: Self = Self(CommandBufferUsageFlagBits::ONE_TIME_SUBMIT.0);
        pub const RENDER_PASS_CONTINUE: Self =
            Self(CommandBufferUsageFlagBits::RENDER_PASS_CONTINUE.0);
        pub const SIMULTANEOUS_USE: Self = Self(CommandBufferUsageFlagBits::SIMULTANEOUS_USE.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct CommandBufferUsageFlagBits(u32);
    impl CommandBufferUsageFlagBits {
        pub const ONE_TIME_SUBMIT: Self = Self(1 << 0);
        pub const RENDER_PASS_CONTINUE: Self = Self(1 << 1);
        pub const SIMULTANEOUS_USE: Self = Self(1 << 2);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct QueryPipelineStatisticFlags(Flags);
    vk_bitflags_wrapped!(QueryPipelineStatisticFlags, Flags);
    impl QueryPipelineStatisticFlags {
        pub const INPUT_ASSEMBLY_VERTICES: Self =
            Self(QueryPipelineStatisticFlagBits::INPUT_ASSEMBLY_VERTICES.0);
        pub const INPUT_ASSEMBLY_PRIMITIVES: Self =
            Self(QueryPipelineStatisticFlagBits::INPUT_ASSEMBLY_PRIMITIVES.0);
        pub const VERTEX_SHADER_INVOCATIONS: Self =
            Self(QueryPipelineStatisticFlagBits::VERTEX_SHADER_INVOCATIONS.0);
        pub const GEOMETRY_SHADER_INVOCATIONS: Self =
            Self(QueryPipelineStatisticFlagBits::GEOMETRY_SHADER_INVOCATIONS.0);
        pub const GEOMETRY_SHADER_PRIMITIVES: Self =
            Self(QueryPipelineStatisticFlagBits::GEOMETRY_SHADER_PRIMITIVES.0);
        pub const CLIPPING_INVOCATIONS: Self =
            Self(QueryPipelineStatisticFlagBits::CLIPPING_INVOCATIONS.0);
        pub const CLIPPING_PRIMITIVES: Self =
            Self(QueryPipelineStatisticFlagBits::CLIPPING_PRIMITIVES.0);
        pub const FRAGMENT_SHADER_INVOCATIONS: Self =
            Self(QueryPipelineStatisticFlagBits::FRAGMENT_SHADER_INVOCATIONS.0);
        pub const TESSELLATION_CONTROL_SHADER_PATCHES: Self =
            Self(QueryPipelineStatisticFlagBits::TESSELLATION_CONTROL_SHADER_PATCHES.0);
        pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS: Self =
            Self(QueryPipelineStatisticFlagBits::TESSELLATION_EVALUATION_SHADER_INVOCATIONS.0);
        pub const COMPUTE_SHADER_INVOCATIONS: Self =
            Self(QueryPipelineStatisticFlagBits::COMPUTE_SHADER_INVOCATIONS.0);
        pub const TASK_SHADER_INVOCATIONS_EXT: Self =
            Self(QueryPipelineStatisticFlagBits::TASK_SHADER_INVOCATIONS_EXT.0);
        pub const MESH_SHADER_INVOCATIONS_EXT: Self =
            Self(QueryPipelineStatisticFlagBits::MESH_SHADER_INVOCATIONS_EXT.0);
        pub const CLUSTER_CULLING_SHADER_INVOCATIONS_HUAWEI: Self =
            Self(QueryPipelineStatisticFlagBits::CLUSTER_CULLING_SHADER_INVOCATIONS_HUAWEI.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct QueryPipelineStatisticFlagBits(u32);
    impl QueryPipelineStatisticFlagBits {
        pub const INPUT_ASSEMBLY_VERTICES: Self = Self(1 << 0);
        pub const INPUT_ASSEMBLY_PRIMITIVES: Self = Self(1 << 1);
        pub const VERTEX_SHADER_INVOCATIONS: Self = Self(1 << 2);
        pub const GEOMETRY_SHADER_INVOCATIONS: Self = Self(1 << 3);
        pub const GEOMETRY_SHADER_PRIMITIVES: Self = Self(1 << 4);
        pub const CLIPPING_INVOCATIONS: Self = Self(1 << 5);
        pub const CLIPPING_PRIMITIVES: Self = Self(1 << 6);
        pub const FRAGMENT_SHADER_INVOCATIONS: Self = Self(1 << 7);
        pub const TESSELLATION_CONTROL_SHADER_PATCHES: Self = Self(1 << 8);
        pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS: Self = Self(1 << 9);
        pub const COMPUTE_SHADER_INVOCATIONS: Self = Self(1 << 10);
        pub const TASK_SHADER_INVOCATIONS_EXT: Self = Self(1 << 11);
        pub const MESH_SHADER_INVOCATIONS_EXT: Self = Self(1 << 12);
        pub const CLUSTER_CULLING_SHADER_INVOCATIONS_HUAWEI: Self = Self(1 << 13);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MemoryMapFlags(Flags);
    vk_bitflags_wrapped!(MemoryMapFlags, Flags);
    impl MemoryMapFlags {
        pub const PLACED_EXT: Self = Self(MemoryMapFlagBits::PLACED_EXT.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct MemoryMapFlagBits(u32);
    impl MemoryMapFlagBits {
        pub const PLACED_EXT: Self = Self(1 << 0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ImageAspectFlags(Flags);
    vk_bitflags_wrapped!(ImageAspectFlags, Flags);
    impl ImageAspectFlags {
        pub const COLOR: Self = Self(ImageAspectFlagBits::COLOR.0);
        pub const DEPTH: Self = Self(ImageAspectFlagBits::DEPTH.0);
        pub const STENCIL: Self = Self(ImageAspectFlagBits::STENCIL.0);
        pub const METADATA: Self = Self(ImageAspectFlagBits::METADATA.0);
        pub const PLANE_0: Self = Self(ImageAspectFlagBits::PLANE_0.0);
        pub const PLANE_1: Self = Self(ImageAspectFlagBits::PLANE_1.0);
        pub const PLANE_2: Self = Self(ImageAspectFlagBits::PLANE_2.0);
        pub const MEMORY_PLANE_0_EXT: Self = Self(ImageAspectFlagBits::MEMORY_PLANE_0_EXT.0);
        pub const MEMORY_PLANE_1_EXT: Self = Self(ImageAspectFlagBits::MEMORY_PLANE_1_EXT.0);
        pub const MEMORY_PLANE_2_EXT: Self = Self(ImageAspectFlagBits::MEMORY_PLANE_2_EXT.0);
        pub const MEMORY_PLANE_3_EXT: Self = Self(ImageAspectFlagBits::MEMORY_PLANE_3_EXT.0);
        pub const NONE_KHR: Self = Self::NONE;
        pub const PLANE_0_KHR: Self = Self::PLANE_0;
        pub const PLANE_1_KHR: Self = Self::PLANE_1;
        pub const PLANE_2_KHR: Self = Self::PLANE_2;
        pub const NONE: Self = Self(0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ImageAspectFlagBits(u32);
    impl ImageAspectFlagBits {
        pub const COLOR: Self = Self(1 << 0);
        pub const DEPTH: Self = Self(1 << 1);
        pub const STENCIL: Self = Self(1 << 2);
        pub const METADATA: Self = Self(1 << 3);
        pub const PLANE_0: Self = Self(1 << 4);
        pub const PLANE_1: Self = Self(1 << 5);
        pub const PLANE_2: Self = Self(1 << 6);
        pub const MEMORY_PLANE_0_EXT: Self = Self(1 << 7);
        pub const MEMORY_PLANE_1_EXT: Self = Self(1 << 8);
        pub const MEMORY_PLANE_2_EXT: Self = Self(1 << 9);
        pub const MEMORY_PLANE_3_EXT: Self = Self(1 << 10);
        pub const PLANE_0_KHR: Self = Self::PLANE_0;
        pub const PLANE_1_KHR: Self = Self::PLANE_1;
        pub const PLANE_2_KHR: Self = Self::PLANE_2;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SparseMemoryBindFlags(Flags);
    vk_bitflags_wrapped!(SparseMemoryBindFlags, Flags);
    impl SparseMemoryBindFlags {
        pub const METADATA: Self = Self(SparseMemoryBindFlagBits::METADATA.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct SparseMemoryBindFlagBits(u32);
    impl SparseMemoryBindFlagBits {
        pub const METADATA: Self = Self(1 << 0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SparseImageFormatFlags(Flags);
    vk_bitflags_wrapped!(SparseImageFormatFlags, Flags);
    impl SparseImageFormatFlags {
        pub const SINGLE_MIPTAIL: Self = Self(SparseImageFormatFlagBits::SINGLE_MIPTAIL.0);
        pub const ALIGNED_MIP_SIZE: Self = Self(SparseImageFormatFlagBits::ALIGNED_MIP_SIZE.0);
        pub const NONSTANDARD_BLOCK_SIZE: Self =
            Self(SparseImageFormatFlagBits::NONSTANDARD_BLOCK_SIZE.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct SparseImageFormatFlagBits(u32);
    impl SparseImageFormatFlagBits {
        pub const SINGLE_MIPTAIL: Self = Self(1 << 0);
        pub const ALIGNED_MIP_SIZE: Self = Self(1 << 1);
        pub const NONSTANDARD_BLOCK_SIZE: Self = Self(1 << 2);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SubpassDescriptionFlags(Flags);
    vk_bitflags_wrapped!(SubpassDescriptionFlags, Flags);
    impl SubpassDescriptionFlags {
        pub const PER_VIEW_ATTRIBUTES_NVX: Self =
            Self(SubpassDescriptionFlagBits::PER_VIEW_ATTRIBUTES_NVX.0);
        pub const PER_VIEW_POSITION_X_ONLY_NVX: Self =
            Self(SubpassDescriptionFlagBits::PER_VIEW_POSITION_X_ONLY_NVX.0);
        pub const FRAGMENT_REGION_EXT: Self =
            Self(SubpassDescriptionFlagBits::FRAGMENT_REGION_EXT.0);
        pub const CUSTOM_RESOLVE_EXT: Self = Self(SubpassDescriptionFlagBits::CUSTOM_RESOLVE_EXT.0);
        pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT: Self =
            Self(SubpassDescriptionFlagBits::RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT.0);
        pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT: Self =
            Self(SubpassDescriptionFlagBits::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT.0);
        pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT: Self =
            Self(SubpassDescriptionFlagBits::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT.0);
        pub const ENABLE_LEGACY_DITHERING_EXT: Self =
            Self(SubpassDescriptionFlagBits::ENABLE_LEGACY_DITHERING_EXT.0);
        pub const TILE_SHADING_APRON_QCOM: Self =
            Self(SubpassDescriptionFlagBits::TILE_SHADING_APRON_QCOM.0);
        pub const FRAGMENT_REGION_QCOM: Self = Self::FRAGMENT_REGION_EXT;
        pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_ARM: Self =
            Self::RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT;
        pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM: Self =
            Self::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT;
        pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM: Self =
            Self::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT;
        pub const SHADER_RESOLVE_QCOM: Self = Self::CUSTOM_RESOLVE_EXT;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct SubpassDescriptionFlagBits(u32);
    impl SubpassDescriptionFlagBits {
        pub const PER_VIEW_ATTRIBUTES_NVX: Self = Self(1 << 0);
        pub const PER_VIEW_POSITION_X_ONLY_NVX: Self = Self(1 << 1);
        pub const FRAGMENT_REGION_EXT: Self = Self(1 << 2);
        pub const CUSTOM_RESOLVE_EXT: Self = Self(1 << 3);
        pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT: Self = Self(1 << 4);
        pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT: Self = Self(1 << 5);
        pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT: Self = Self(1 << 6);
        pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(1 << 7);
        pub const TILE_SHADING_APRON_QCOM: Self = Self(1 << 8);
        pub const FRAGMENT_REGION_QCOM: Self = Self::FRAGMENT_REGION_EXT;
        pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_ARM: Self =
            Self::RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT;
        pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM: Self =
            Self::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT;
        pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM: Self =
            Self::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT;
        pub const SHADER_RESOLVE_QCOM: Self = Self::CUSTOM_RESOLVE_EXT;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineStageFlags(Flags);
    vk_bitflags_wrapped!(PipelineStageFlags, Flags);
    impl PipelineStageFlags {
        pub const TOP_OF_PIPE: Self = Self(PipelineStageFlagBits::TOP_OF_PIPE.0);
        pub const DRAW_INDIRECT: Self = Self(PipelineStageFlagBits::DRAW_INDIRECT.0);
        pub const VERTEX_INPUT: Self = Self(PipelineStageFlagBits::VERTEX_INPUT.0);
        pub const VERTEX_SHADER: Self = Self(PipelineStageFlagBits::VERTEX_SHADER.0);
        pub const TESSELLATION_CONTROL_SHADER: Self =
            Self(PipelineStageFlagBits::TESSELLATION_CONTROL_SHADER.0);
        pub const TESSELLATION_EVALUATION_SHADER: Self =
            Self(PipelineStageFlagBits::TESSELLATION_EVALUATION_SHADER.0);
        pub const GEOMETRY_SHADER: Self = Self(PipelineStageFlagBits::GEOMETRY_SHADER.0);
        pub const FRAGMENT_SHADER: Self = Self(PipelineStageFlagBits::FRAGMENT_SHADER.0);
        pub const EARLY_FRAGMENT_TESTS: Self = Self(PipelineStageFlagBits::EARLY_FRAGMENT_TESTS.0);
        pub const LATE_FRAGMENT_TESTS: Self = Self(PipelineStageFlagBits::LATE_FRAGMENT_TESTS.0);
        pub const COLOR_ATTACHMENT_OUTPUT: Self =
            Self(PipelineStageFlagBits::COLOR_ATTACHMENT_OUTPUT.0);
        pub const COMPUTE_SHADER: Self = Self(PipelineStageFlagBits::COMPUTE_SHADER.0);
        pub const TRANSFER: Self = Self(PipelineStageFlagBits::TRANSFER.0);
        pub const BOTTOM_OF_PIPE: Self = Self(PipelineStageFlagBits::BOTTOM_OF_PIPE.0);
        pub const HOST: Self = Self(PipelineStageFlagBits::HOST.0);
        pub const ALL_GRAPHICS: Self = Self(PipelineStageFlagBits::ALL_GRAPHICS.0);
        pub const ALL_COMMANDS: Self = Self(PipelineStageFlagBits::ALL_COMMANDS.0);
        pub const COMMAND_PREPROCESS_EXT: Self =
            Self(PipelineStageFlagBits::COMMAND_PREPROCESS_EXT.0);
        pub const CONDITIONAL_RENDERING_EXT: Self =
            Self(PipelineStageFlagBits::CONDITIONAL_RENDERING_EXT.0);
        pub const TASK_SHADER_EXT: Self = Self(PipelineStageFlagBits::TASK_SHADER_EXT.0);
        pub const MESH_SHADER_EXT: Self = Self(PipelineStageFlagBits::MESH_SHADER_EXT.0);
        pub const RAY_TRACING_SHADER_KHR: Self =
            Self(PipelineStageFlagBits::RAY_TRACING_SHADER_KHR.0);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
            Self(PipelineStageFlagBits::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0);
        pub const FRAGMENT_DENSITY_PROCESS_EXT: Self =
            Self(PipelineStageFlagBits::FRAGMENT_DENSITY_PROCESS_EXT.0);
        pub const TRANSFORM_FEEDBACK_EXT: Self =
            Self(PipelineStageFlagBits::TRANSFORM_FEEDBACK_EXT.0);
        pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self =
            Self(PipelineStageFlagBits::ACCELERATION_STRUCTURE_BUILD_KHR.0);
        pub const ACCELERATION_STRUCTURE_BUILD_NV: Self = Self::ACCELERATION_STRUCTURE_BUILD_KHR;
        pub const COMMAND_PREPROCESS_NV: Self = Self::COMMAND_PREPROCESS_EXT;
        pub const MESH_SHADER_NV: Self = Self::MESH_SHADER_EXT;
        pub const NONE_KHR: Self = Self::NONE;
        pub const RAY_TRACING_SHADER_NV: Self = Self::RAY_TRACING_SHADER_KHR;
        pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
        pub const TASK_SHADER_NV: Self = Self::TASK_SHADER_EXT;
        pub const NONE: Self = Self(0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PipelineStageFlagBits(u32);
    impl PipelineStageFlagBits {
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
        pub const TRANSFER: Self = Self(1 << 12);
        pub const BOTTOM_OF_PIPE: Self = Self(1 << 13);
        pub const HOST: Self = Self(1 << 14);
        pub const ALL_GRAPHICS: Self = Self(1 << 15);
        pub const ALL_COMMANDS: Self = Self(1 << 16);
        pub const COMMAND_PREPROCESS_EXT: Self = Self(1 << 17);
        pub const CONDITIONAL_RENDERING_EXT: Self = Self(1 << 18);
        pub const TASK_SHADER_EXT: Self = Self(1 << 19);
        pub const MESH_SHADER_EXT: Self = Self(1 << 20);
        pub const RAY_TRACING_SHADER_KHR: Self = Self(1 << 21);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 22);
        pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(1 << 23);
        pub const TRANSFORM_FEEDBACK_EXT: Self = Self(1 << 24);
        pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(1 << 25);
        pub const ACCELERATION_STRUCTURE_BUILD_NV: Self = Self::ACCELERATION_STRUCTURE_BUILD_KHR;
        pub const COMMAND_PREPROCESS_NV: Self = Self::COMMAND_PREPROCESS_EXT;
        pub const MESH_SHADER_NV: Self = Self::MESH_SHADER_EXT;
        pub const RAY_TRACING_SHADER_NV: Self = Self::RAY_TRACING_SHADER_KHR;
        pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
        pub const TASK_SHADER_NV: Self = Self::TASK_SHADER_EXT;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SampleCountFlags(Flags);
    vk_bitflags_wrapped!(SampleCountFlags, Flags);
    impl SampleCountFlags {
        pub const _1: Self = Self(SampleCountFlagBits::_1.0);
        pub const _2: Self = Self(SampleCountFlagBits::_2.0);
        pub const _4: Self = Self(SampleCountFlagBits::_4.0);
        pub const _8: Self = Self(SampleCountFlagBits::_8.0);
        pub const _16: Self = Self(SampleCountFlagBits::_16.0);
        pub const _32: Self = Self(SampleCountFlagBits::_32.0);
        pub const _64: Self = Self(SampleCountFlagBits::_64.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct SampleCountFlagBits(u32);
    impl SampleCountFlagBits {
        pub const _1: Self = Self(1 << 0);
        pub const _2: Self = Self(1 << 1);
        pub const _4: Self = Self(1 << 2);
        pub const _8: Self = Self(1 << 3);
        pub const _16: Self = Self(1 << 4);
        pub const _32: Self = Self(1 << 5);
        pub const _64: Self = Self(1 << 6);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct AttachmentDescriptionFlags(Flags);
    vk_bitflags_wrapped!(AttachmentDescriptionFlags, Flags);
    impl AttachmentDescriptionFlags {
        pub const MAY_ALIAS: Self = Self(AttachmentDescriptionFlagBits::MAY_ALIAS.0);
        pub const RESOLVE_SKIP_TRANSFER_FUNCTION_KHR: Self =
            Self(AttachmentDescriptionFlagBits::RESOLVE_SKIP_TRANSFER_FUNCTION_KHR.0);
        pub const RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR: Self =
            Self(AttachmentDescriptionFlagBits::RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct AttachmentDescriptionFlagBits(u32);
    impl AttachmentDescriptionFlagBits {
        pub const MAY_ALIAS: Self = Self(1 << 0);
        pub const RESOLVE_SKIP_TRANSFER_FUNCTION_KHR: Self = Self(1 << 1);
        pub const RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR: Self = Self(1 << 2);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct StencilFaceFlags(Flags);
    vk_bitflags_wrapped!(StencilFaceFlags, Flags);
    impl StencilFaceFlags {
        pub const FRONT: Self = Self(StencilFaceFlagBits::FRONT.0);
        pub const BACK: Self = Self(StencilFaceFlagBits::BACK.0);
        pub const STENCIL_FRONT_AND_BACK: Self = Self::FRONT_AND_BACK;
        pub const FRONT_AND_BACK: Self = Self(0x00000003);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct StencilFaceFlagBits(u32);
    impl StencilFaceFlagBits {
        pub const FRONT: Self = Self(1 << 0);
        pub const BACK: Self = Self(1 << 1);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CullModeFlags(Flags);
    vk_bitflags_wrapped!(CullModeFlags, Flags);
    impl CullModeFlags {
        pub const FRONT: Self = Self(CullModeFlagBits::FRONT.0);
        pub const BACK: Self = Self(CullModeFlagBits::BACK.0);
        pub const NONE: Self = Self(0);
        pub const FRONT_AND_BACK: Self = Self(0x00000003);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct CullModeFlagBits(u32);
    impl CullModeFlagBits {
        pub const FRONT: Self = Self(1 << 0);
        pub const BACK: Self = Self(1 << 1);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DescriptorPoolCreateFlags(Flags);
    vk_bitflags_wrapped!(DescriptorPoolCreateFlags, Flags);
    impl DescriptorPoolCreateFlags {
        pub const FREE_DESCRIPTOR_SET: Self =
            Self(DescriptorPoolCreateFlagBits::FREE_DESCRIPTOR_SET.0);
        pub const UPDATE_AFTER_BIND: Self = Self(DescriptorPoolCreateFlagBits::UPDATE_AFTER_BIND.0);
        pub const HOST_ONLY_EXT: Self = Self(DescriptorPoolCreateFlagBits::HOST_ONLY_EXT.0);
        pub const ALLOW_OVERALLOCATION_SETS_NV: Self =
            Self(DescriptorPoolCreateFlagBits::ALLOW_OVERALLOCATION_SETS_NV.0);
        pub const ALLOW_OVERALLOCATION_POOLS_NV: Self =
            Self(DescriptorPoolCreateFlagBits::ALLOW_OVERALLOCATION_POOLS_NV.0);
        pub const HOST_ONLY_VALVE: Self = Self::HOST_ONLY_EXT;
        pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DescriptorPoolCreateFlagBits(u32);
    impl DescriptorPoolCreateFlagBits {
        pub const FREE_DESCRIPTOR_SET: Self = Self(1 << 0);
        pub const UPDATE_AFTER_BIND: Self = Self(1 << 1);
        pub const HOST_ONLY_EXT: Self = Self(1 << 2);
        pub const ALLOW_OVERALLOCATION_SETS_NV: Self = Self(1 << 3);
        pub const ALLOW_OVERALLOCATION_POOLS_NV: Self = Self(1 << 4);
        pub const HOST_ONLY_VALVE: Self = Self::HOST_ONLY_EXT;
        pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DescriptorPoolResetFlags(Flags);
    vk_bitflags_wrapped!(DescriptorPoolResetFlags, Flags);
    impl DescriptorPoolResetFlags {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DependencyFlags(Flags);
    vk_bitflags_wrapped!(DependencyFlags, Flags);
    impl DependencyFlags {
        pub const BY_REGION: Self = Self(DependencyFlagBits::BY_REGION.0);
        pub const VIEW_LOCAL: Self = Self(DependencyFlagBits::VIEW_LOCAL.0);
        pub const DEVICE_GROUP: Self = Self(DependencyFlagBits::DEVICE_GROUP.0);
        pub const FEEDBACK_LOOP_EXT: Self = Self(DependencyFlagBits::FEEDBACK_LOOP_EXT.0);
        pub const QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_KHR: Self =
            Self(DependencyFlagBits::QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_KHR.0);
        pub const ASYMMETRIC_EVENT_KHR: Self = Self(DependencyFlagBits::ASYMMETRIC_EVENT_KHR.0);
        pub const DEVICE_GROUP_KHR: Self = Self::DEVICE_GROUP;
        pub const VIEW_LOCAL_KHR: Self = Self::VIEW_LOCAL;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DependencyFlagBits(u32);
    impl DependencyFlagBits {
        pub const BY_REGION: Self = Self(1 << 0);
        pub const VIEW_LOCAL: Self = Self(1 << 1);
        pub const DEVICE_GROUP: Self = Self(1 << 2);
        pub const FEEDBACK_LOOP_EXT: Self = Self(1 << 3);
        pub const QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_KHR: Self = Self(1 << 5);
        pub const ASYMMETRIC_EVENT_KHR: Self = Self(1 << 6);
        pub const DEVICE_GROUP_KHR: Self = Self::DEVICE_GROUP;
        pub const VIEW_LOCAL_KHR: Self = Self::VIEW_LOCAL;
    }
    pub type PFN_vkInternalAllocationNotification = unsafe extern "system" fn(
        p_user_data: *mut c_void,
        size: usize,
        allocation_type: InternalAllocationType,
        allocation_scope: SystemAllocationScope,
    );
    pub type PFN_vkInternalFreeNotification = unsafe extern "system" fn(
        p_user_data: *mut c_void,
        size: usize,
        allocation_type: InternalAllocationType,
        allocation_scope: SystemAllocationScope,
    );
    pub type PFN_vkReallocationFunction = unsafe extern "system" fn(
        p_user_data: *mut c_void,
        p_original: *mut c_void,
        size: usize,
        alignment: usize,
        allocation_scope: SystemAllocationScope,
    ) -> *mut c_void;
    pub type PFN_vkAllocationFunction = unsafe extern "system" fn(
        p_user_data: *mut c_void,
        size: usize,
        alignment: usize,
        allocation_scope: SystemAllocationScope,
    ) -> *mut c_void;
    pub type PFN_vkFreeFunction =
        unsafe extern "system" fn(p_user_data: *mut c_void, p_memory: *mut c_void);
    pub type PFN_vkVoidFunction = unsafe extern "system" fn();
    pub type PFN_vkCreateInstance = unsafe extern "system" fn(
        p_create_info: *const InstanceCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_instance: *mut Instance,
    ) -> vk::Result;
    pub type PFN_vkDestroyInstance =
        unsafe extern "system" fn(instance: Instance, p_allocator: *const AllocationCallbacks<'_>);
    pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn(
        instance: Instance,
        p_physical_device_count: *mut u32,
        p_physical_devices: *mut PhysicalDevice,
    ) -> vk::Result;
    pub type PFN_vkGetDeviceProcAddr =
        unsafe extern "system" fn(device: Device, p_name: *const c_char) -> PFN_vkVoidFunction;
    pub type PFN_vkGetInstanceProcAddr =
        unsafe extern "system" fn(instance: Instance, p_name: *const c_char) -> PFN_vkVoidFunction;
    pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_properties: *mut PhysicalDeviceProperties,
    );
    pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties,
    );
    pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties,
    );
    pub type PFN_vkGetPhysicalDeviceFeatures = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures,
    );
    pub type PFN_vkGetPhysicalDeviceFormatProperties = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *mut FormatProperties,
    );
    pub type PFN_vkGetPhysicalDeviceImageFormatProperties = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        p_image_format_properties: *mut ImageFormatProperties,
    )
        -> vk::Result;
    pub type PFN_vkCreateDevice = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_create_info: *const DeviceCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_device: *mut Device,
    ) -> vk::Result;
    pub type PFN_vkDestroyDevice =
        unsafe extern "system" fn(device: Device, p_allocator: *const AllocationCallbacks<'_>);
    pub type PFN_vkEnumerateInstanceLayerProperties = unsafe extern "system" fn(
        p_property_count: *mut u32,
        p_properties: *mut LayerProperties,
    ) -> vk::Result;
    pub type PFN_vkEnumerateInstanceExtensionProperties = unsafe extern "system" fn(
        p_layer_name: *const c_char,
        p_property_count: *mut u32,
        p_properties: *mut ExtensionProperties,
    )
        -> vk::Result;
    pub type PFN_vkEnumerateDeviceLayerProperties = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut LayerProperties,
    ) -> vk::Result;
    pub type PFN_vkEnumerateDeviceExtensionProperties = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_layer_name: *const c_char,
        p_property_count: *mut u32,
        p_properties: *mut ExtensionProperties,
    )
        -> vk::Result;
    pub type PFN_vkGetDeviceQueue = unsafe extern "system" fn(
        device: Device,
        queue_family_index: u32,
        queue_index: u32,
        p_queue: *mut Queue,
    );
    pub type PFN_vkQueueSubmit = unsafe extern "system" fn(
        queue: Queue,
        submit_count: u32,
        p_submits: *const SubmitInfo<'_>,
        fence: Fence,
    ) -> vk::Result;
    pub type PFN_vkQueueWaitIdle = unsafe extern "system" fn(queue: Queue) -> vk::Result;
    pub type PFN_vkDeviceWaitIdle = unsafe extern "system" fn(device: Device) -> vk::Result;
    pub type PFN_vkAllocateMemory = unsafe extern "system" fn(
        device: Device,
        p_allocate_info: *const MemoryAllocateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_memory: *mut DeviceMemory,
    ) -> vk::Result;
    pub type PFN_vkFreeMemory = unsafe extern "system" fn(
        device: Device,
        memory: DeviceMemory,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkMapMemory = unsafe extern "system" fn(
        device: Device,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
        pp_data: *mut *mut c_void,
    ) -> vk::Result;
    pub type PFN_vkUnmapMemory = unsafe extern "system" fn(device: Device, memory: DeviceMemory);
    pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "system" fn(
        device: Device,
        memory_range_count: u32,
        p_memory_ranges: *const MappedMemoryRange<'_>,
    ) -> vk::Result;
    pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(
        device: Device,
        memory_range_count: u32,
        p_memory_ranges: *const MappedMemoryRange<'_>,
    ) -> vk::Result;
    pub type PFN_vkGetDeviceMemoryCommitment = unsafe extern "system" fn(
        device: Device,
        memory: DeviceMemory,
        p_committed_memory_in_bytes: *mut DeviceSize,
    );
    pub type PFN_vkGetBufferMemoryRequirements = unsafe extern "system" fn(
        device: Device,
        buffer: Buffer,
        p_memory_requirements: *mut MemoryRequirements,
    );
    pub type PFN_vkBindBufferMemory = unsafe extern "system" fn(
        device: Device,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> vk::Result;
    pub type PFN_vkGetImageMemoryRequirements = unsafe extern "system" fn(
        device: Device,
        image: Image,
        p_memory_requirements: *mut MemoryRequirements,
    );
    pub type PFN_vkBindImageMemory = unsafe extern "system" fn(
        device: Device,
        image: Image,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> vk::Result;
    pub type PFN_vkGetImageSparseMemoryRequirements = unsafe extern "system" fn(
        device: Device,
        image: Image,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
    );
    pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        samples: SampleCountFlagBits,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties,
    );
    pub type PFN_vkQueueBindSparse = unsafe extern "system" fn(
        queue: Queue,
        bind_info_count: u32,
        p_bind_info: *const BindSparseInfo<'_>,
        fence: Fence,
    ) -> vk::Result;
    pub type PFN_vkCreateFence = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const FenceCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_fence: *mut Fence,
    ) -> vk::Result;
    pub type PFN_vkDestroyFence = unsafe extern "system" fn(
        device: Device,
        fence: Fence,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkResetFences = unsafe extern "system" fn(
        device: Device,
        fence_count: u32,
        p_fences: *const Fence,
    ) -> vk::Result;
    pub type PFN_vkGetFenceStatus =
        unsafe extern "system" fn(device: Device, fence: Fence) -> vk::Result;
    pub type PFN_vkWaitForFences = unsafe extern "system" fn(
        device: Device,
        fence_count: u32,
        p_fences: *const Fence,
        wait_all: Bool32,
        timeout: u64,
    ) -> vk::Result;
    pub type PFN_vkCreateSemaphore = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const SemaphoreCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_semaphore: *mut Semaphore,
    ) -> vk::Result;
    pub type PFN_vkDestroySemaphore = unsafe extern "system" fn(
        device: Device,
        semaphore: Semaphore,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkCreateEvent = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const EventCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_event: *mut Event,
    ) -> vk::Result;
    pub type PFN_vkDestroyEvent = unsafe extern "system" fn(
        device: Device,
        event: Event,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkGetEventStatus =
        unsafe extern "system" fn(device: Device, event: Event) -> vk::Result;
    pub type PFN_vkSetEvent = unsafe extern "system" fn(device: Device, event: Event) -> vk::Result;
    pub type PFN_vkResetEvent =
        unsafe extern "system" fn(device: Device, event: Event) -> vk::Result;
    pub type PFN_vkCreateQueryPool = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const QueryPoolCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_query_pool: *mut QueryPool,
    ) -> vk::Result;
    pub type PFN_vkDestroyQueryPool = unsafe extern "system" fn(
        device: Device,
        query_pool: QueryPool,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkGetQueryPoolResults = unsafe extern "system" fn(
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        p_data: *mut c_void,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> vk::Result;
    pub type PFN_vkCreateBuffer = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const BufferCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_buffer: *mut Buffer,
    ) -> vk::Result;
    pub type PFN_vkDestroyBuffer = unsafe extern "system" fn(
        device: Device,
        buffer: Buffer,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkCreateBufferView = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const BufferViewCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_view: *mut BufferView,
    ) -> vk::Result;
    pub type PFN_vkDestroyBufferView = unsafe extern "system" fn(
        device: Device,
        buffer_view: BufferView,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkCreateImage = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ImageCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_image: *mut Image,
    ) -> vk::Result;
    pub type PFN_vkDestroyImage = unsafe extern "system" fn(
        device: Device,
        image: Image,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkGetImageSubresourceLayout = unsafe extern "system" fn(
        device: Device,
        image: Image,
        p_subresource: *const ImageSubresource,
        p_layout: *mut SubresourceLayout,
    );
    pub type PFN_vkCreateImageView = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ImageViewCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_view: *mut ImageView,
    ) -> vk::Result;
    pub type PFN_vkDestroyImageView = unsafe extern "system" fn(
        device: Device,
        image_view: ImageView,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkCreateShaderModule = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ShaderModuleCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_shader_module: *mut ShaderModule,
    ) -> vk::Result;
    pub type PFN_vkDestroyShaderModule = unsafe extern "system" fn(
        device: Device,
        shader_module: ShaderModule,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkCreatePipelineCache = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const PipelineCacheCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipeline_cache: *mut PipelineCache,
    ) -> vk::Result;
    pub type PFN_vkDestroyPipelineCache = unsafe extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkGetPipelineCacheData = unsafe extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> vk::Result;
    pub type PFN_vkMergePipelineCaches = unsafe extern "system" fn(
        device: Device,
        dst_cache: PipelineCache,
        src_cache_count: u32,
        p_src_caches: *const PipelineCache,
    ) -> vk::Result;
    pub type PFN_vkCreateGraphicsPipelines = unsafe extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const GraphicsPipelineCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> vk::Result;
    pub type PFN_vkCreateComputePipelines = unsafe extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const ComputePipelineCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> vk::Result;
    pub type PFN_vkDestroyPipeline = unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkCreatePipelineLayout = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const PipelineLayoutCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipeline_layout: *mut PipelineLayout,
    ) -> vk::Result;
    pub type PFN_vkDestroyPipelineLayout = unsafe extern "system" fn(
        device: Device,
        pipeline_layout: PipelineLayout,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkCreateSampler = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const SamplerCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_sampler: *mut Sampler,
    ) -> vk::Result;
    pub type PFN_vkDestroySampler = unsafe extern "system" fn(
        device: Device,
        sampler: Sampler,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkCreateDescriptorSetLayout = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_set_layout: *mut DescriptorSetLayout,
    ) -> vk::Result;
    pub type PFN_vkDestroyDescriptorSetLayout = unsafe extern "system" fn(
        device: Device,
        descriptor_set_layout: DescriptorSetLayout,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkCreateDescriptorPool = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const DescriptorPoolCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_descriptor_pool: *mut DescriptorPool,
    ) -> vk::Result;
    pub type PFN_vkDestroyDescriptorPool = unsafe extern "system" fn(
        device: Device,
        descriptor_pool: DescriptorPool,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkResetDescriptorPool = unsafe extern "system" fn(
        device: Device,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> vk::Result;
    pub type PFN_vkAllocateDescriptorSets = unsafe extern "system" fn(
        device: Device,
        p_allocate_info: *const DescriptorSetAllocateInfo<'_>,
        p_descriptor_sets: *mut DescriptorSet,
    ) -> vk::Result;
    pub type PFN_vkFreeDescriptorSets = unsafe extern "system" fn(
        device: Device,
        descriptor_pool: DescriptorPool,
        descriptor_set_count: u32,
        p_descriptor_sets: *const DescriptorSet,
    ) -> vk::Result;
    pub type PFN_vkUpdateDescriptorSets = unsafe extern "system" fn(
        device: Device,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet<'_>,
        descriptor_copy_count: u32,
        p_descriptor_copies: *const CopyDescriptorSet<'_>,
    );
    pub type PFN_vkCreateFramebuffer = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const FramebufferCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_framebuffer: *mut Framebuffer,
    ) -> vk::Result;
    pub type PFN_vkDestroyFramebuffer = unsafe extern "system" fn(
        device: Device,
        framebuffer: Framebuffer,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkCreateRenderPass = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const RenderPassCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_render_pass: *mut RenderPass,
    ) -> vk::Result;
    pub type PFN_vkDestroyRenderPass = unsafe extern "system" fn(
        device: Device,
        render_pass: RenderPass,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkGetRenderAreaGranularity = unsafe extern "system" fn(
        device: Device,
        render_pass: RenderPass,
        p_granularity: *mut Extent2D,
    );
    pub type PFN_vkCreateCommandPool = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const CommandPoolCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_command_pool: *mut CommandPool,
    ) -> vk::Result;
    pub type PFN_vkDestroyCommandPool = unsafe extern "system" fn(
        device: Device,
        command_pool: CommandPool,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkResetCommandPool = unsafe extern "system" fn(
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> vk::Result;
    pub type PFN_vkAllocateCommandBuffers = unsafe extern "system" fn(
        device: Device,
        p_allocate_info: *const CommandBufferAllocateInfo<'_>,
        p_command_buffers: *mut CommandBuffer,
    ) -> vk::Result;
    pub type PFN_vkFreeCommandBuffers = unsafe extern "system" fn(
        device: Device,
        command_pool: CommandPool,
        command_buffer_count: u32,
        p_command_buffers: *const CommandBuffer,
    );
    pub type PFN_vkBeginCommandBuffer = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_begin_info: *const CommandBufferBeginInfo<'_>,
    ) -> vk::Result;
    pub type PFN_vkEndCommandBuffer =
        unsafe extern "system" fn(command_buffer: CommandBuffer) -> vk::Result;
    pub type PFN_vkResetCommandBuffer = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        flags: CommandBufferResetFlags,
    ) -> vk::Result;
    pub type PFN_vkCmdBindPipeline = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    );
    pub type PFN_vkCmdSetViewport = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewports: *const Viewport,
    );
    pub type PFN_vkCmdSetScissor = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_scissor: u32,
        scissor_count: u32,
        p_scissors: *const Rect2D,
    );
    pub type PFN_vkCmdSetLineWidth =
        unsafe extern "system" fn(command_buffer: CommandBuffer, line_width: f32);
    pub type PFN_vkCmdSetDepthBias = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    );
    pub type PFN_vkCmdSetBlendConstants =
        unsafe extern "system" fn(command_buffer: CommandBuffer, blend_constants: *const [f32; 4]);
    pub type PFN_vkCmdSetDepthBounds = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    );
    pub type PFN_vkCmdSetStencilCompareMask = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: u32,
    );
    pub type PFN_vkCmdSetStencilWriteMask = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: u32,
    );
    pub type PFN_vkCmdSetStencilReference = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: u32,
    );
    pub type PFN_vkCmdBindDescriptorSets = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_set_count: u32,
        p_descriptor_sets: *const DescriptorSet,
        dynamic_offset_count: u32,
        p_dynamic_offsets: *const u32,
    );
    pub type PFN_vkCmdBindIndexBuffer = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        index_type: IndexType,
    );
    pub type PFN_vkCmdBindVertexBuffers = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
    );
    pub type PFN_vkCmdDraw = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    );
    pub type PFN_vkCmdDrawIndexed = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    );
    pub type PFN_vkCmdDrawIndirect = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    );
    pub type PFN_vkCmdDrawIndexedIndirect = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    );
    pub type PFN_vkCmdDispatch = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    );
    pub type PFN_vkCmdDispatchIndirect = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
    );
    pub type PFN_vkCmdCopyBuffer = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        region_count: u32,
        p_regions: *const BufferCopy,
    );
    pub type PFN_vkCmdCopyImage = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageCopy,
    );
    pub type PFN_vkCmdBlitImage = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageBlit,
        filter: Filter,
    );
    pub type PFN_vkCmdCopyBufferToImage = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const BufferImageCopy,
    );
    pub type PFN_vkCmdCopyImageToBuffer = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        region_count: u32,
        p_regions: *const BufferImageCopy,
    );
    pub type PFN_vkCmdUpdateBuffer = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        data_size: DeviceSize,
        p_data: *const c_void,
    );
    pub type PFN_vkCmdFillBuffer = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: u32,
    );
    pub type PFN_vkCmdClearColorImage = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_color: *const ClearColorValue,
        range_count: u32,
        p_ranges: *const ImageSubresourceRange,
    );
    pub type PFN_vkCmdClearDepthStencilImage = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_depth_stencil: *const ClearDepthStencilValue,
        range_count: u32,
        p_ranges: *const ImageSubresourceRange,
    );
    pub type PFN_vkCmdClearAttachments = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        attachment_count: u32,
        p_attachments: *const ClearAttachment,
        rect_count: u32,
        p_rects: *const ClearRect,
    );
    pub type PFN_vkCmdResolveImage = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageResolve,
    );
    pub type PFN_vkCmdSetEvent = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    );
    pub type PFN_vkCmdResetEvent = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    );
    pub type PFN_vkCmdWaitEvents = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event_count: u32,
        p_events: *const Event,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const MemoryBarrier<'_>,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const BufferMemoryBarrier<'_>,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const ImageMemoryBarrier<'_>,
    );
    pub type PFN_vkCmdPipelineBarrier = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const MemoryBarrier<'_>,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const BufferMemoryBarrier<'_>,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const ImageMemoryBarrier<'_>,
    );
    pub type PFN_vkCmdBeginQuery = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
    );
    pub type PFN_vkCmdEndQuery =
        unsafe extern "system" fn(command_buffer: CommandBuffer, query_pool: QueryPool, query: u32);
    pub type PFN_vkCmdResetQueryPool = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    );
    pub type PFN_vkCmdWriteTimestamp = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlagBits,
        query_pool: QueryPool,
        query: u32,
    );
    pub type PFN_vkCmdCopyQueryPoolResults = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    );
    pub type PFN_vkCmdPushConstants = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        size: u32,
        p_values: *const c_void,
    );
    pub type PFN_vkCmdBeginRenderPass = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_render_pass_begin: *const RenderPassBeginInfo<'_>,
        contents: SubpassContents,
    );
    pub type PFN_vkCmdNextSubpass =
        unsafe extern "system" fn(command_buffer: CommandBuffer, contents: SubpassContents);
    pub type PFN_vkCmdEndRenderPass = unsafe extern "system" fn(command_buffer: CommandBuffer);
    pub type PFN_vkCmdExecuteCommands = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        command_buffer_count: u32,
        p_command_buffers: *const CommandBuffer,
    );
}
pub struct EntryFn {
    create_instance: PFN_vkCreateInstance,
    enumerate_instance_extension_properties: PFN_vkEnumerateInstanceExtensionProperties,
    enumerate_instance_layer_properties: PFN_vkEnumerateInstanceLayerProperties,
}
impl EntryFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_instance: transmute(
                    load(c"vkCreateInstance").ok_or(MissingEntryPointError)?,
                ),
                enumerate_instance_extension_properties: transmute(
                    load(c"vkEnumerateInstanceExtensionProperties")
                        .ok_or(MissingEntryPointError)?,
                ),
                enumerate_instance_layer_properties: transmute(
                    load(c"vkEnumerateInstanceLayerProperties").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl EntryFn {
    pub unsafe fn create_instance(
        &self,
        create_info: &InstanceCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<Instance> {
        unsafe {
            let mut instance = core::mem::MaybeUninit::uninit();
            let result =
                (self.create_instance)(create_info, allocator.to_raw_ptr(), instance.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(instance.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn enumerate_instance_extension_properties(
        &self,
        layer_name: Option<&CStr>,
        mut properties: impl ExtendUninit<ExtensionProperties>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |property_count, properties| {
                let result = (self.enumerate_instance_extension_properties)(
                    layer_name.to_raw_ptr(),
                    property_count,
                    properties as _,
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
            let properties_buf = properties.reserve(capacity);
            len = properties_buf.len().try_into().unwrap();
            let result = call(&mut len, properties_buf.as_mut_ptr() as *mut _)?;
            properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
    pub unsafe fn enumerate_instance_layer_properties(
        &self,
        mut properties: impl ExtendUninit<LayerProperties>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |property_count, properties| {
                let result =
                    (self.enumerate_instance_layer_properties)(property_count, properties as _);

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let properties_buf = properties.reserve(capacity);
            len = properties_buf.len().try_into().unwrap();
            let result = call(&mut len, properties_buf.as_mut_ptr() as *mut _)?;
            properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}
pub struct InstanceFn {
    destroy_instance: PFN_vkDestroyInstance,
    enumerate_physical_devices: PFN_vkEnumeratePhysicalDevices,
    get_physical_device_features: PFN_vkGetPhysicalDeviceFeatures,
    get_physical_device_format_properties: PFN_vkGetPhysicalDeviceFormatProperties,
    get_physical_device_image_format_properties: PFN_vkGetPhysicalDeviceImageFormatProperties,
    get_physical_device_properties: PFN_vkGetPhysicalDeviceProperties,
    get_physical_device_queue_family_properties: PFN_vkGetPhysicalDeviceQueueFamilyProperties,
    get_physical_device_memory_properties: PFN_vkGetPhysicalDeviceMemoryProperties,
    get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    create_device: PFN_vkCreateDevice,
    enumerate_device_extension_properties: PFN_vkEnumerateDeviceExtensionProperties,
    enumerate_device_layer_properties: PFN_vkEnumerateDeviceLayerProperties,
    get_physical_device_sparse_image_format_properties:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                destroy_instance: transmute(
                    load(c"vkDestroyInstance").ok_or(MissingEntryPointError)?,
                ),
                enumerate_physical_devices: transmute(
                    load(c"vkEnumeratePhysicalDevices").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_features: transmute(
                    load(c"vkGetPhysicalDeviceFeatures").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_format_properties: transmute(
                    load(c"vkGetPhysicalDeviceFormatProperties").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_image_format_properties: transmute(
                    load(c"vkGetPhysicalDeviceImageFormatProperties")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_properties: transmute(
                    load(c"vkGetPhysicalDeviceProperties").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_queue_family_properties: transmute(
                    load(c"vkGetPhysicalDeviceQueueFamilyProperties")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_memory_properties: transmute(
                    load(c"vkGetPhysicalDeviceMemoryProperties").ok_or(MissingEntryPointError)?,
                ),
                get_instance_proc_addr: transmute(
                    load(c"vkGetInstanceProcAddr").ok_or(MissingEntryPointError)?,
                ),
                create_device: transmute(load(c"vkCreateDevice").ok_or(MissingEntryPointError)?),
                enumerate_device_extension_properties: transmute(
                    load(c"vkEnumerateDeviceExtensionProperties").ok_or(MissingEntryPointError)?,
                ),
                enumerate_device_layer_properties: transmute(
                    load(c"vkEnumerateDeviceLayerProperties").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_sparse_image_format_properties: transmute(
                    load(c"vkGetPhysicalDeviceSparseImageFormatProperties")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn destroy_instance(
        &self,
        instance: Instance,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_instance)(instance, allocator.to_raw_ptr()) }
    }
    pub unsafe fn enumerate_physical_devices(
        &self,
        instance: Instance,
        mut physical_devices: impl ExtendUninit<PhysicalDevice>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |physical_device_count, physical_devices| {
                let result = (self.enumerate_physical_devices)(
                    instance,
                    physical_device_count,
                    physical_devices as _,
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
            let physical_devices_buf = physical_devices.reserve(capacity);
            len = physical_devices_buf.len().try_into().unwrap();
            let result = call(&mut len, physical_devices_buf.as_mut_ptr() as *mut _)?;
            physical_devices.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceFeatures {
        unsafe {
            let mut features = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_features)(physical_device, features.as_mut_ptr());
            features.assume_init()
        }
    }
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
    ) -> FormatProperties {
        unsafe {
            let mut format_properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_format_properties)(
                physical_device,
                format,
                format_properties.as_mut_ptr(),
            );
            format_properties.assume_init()
        }
    }
    pub unsafe fn get_physical_device_image_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
    ) -> crate::Result<ImageFormatProperties> {
        unsafe {
            let mut image_format_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_physical_device_image_format_properties)(
                physical_device,
                format,
                ty,
                tiling,
                usage,
                flags,
                image_format_properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(image_format_properties.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceProperties {
        unsafe {
            let mut properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_properties)(physical_device, properties.as_mut_ptr());
            properties.assume_init()
        }
    }
    pub unsafe fn get_physical_device_queue_family_properties(
        &self,
        physical_device: PhysicalDevice,
        mut queue_family_properties: impl ExtendUninit<QueueFamilyProperties>,
    ) {
        unsafe {
            let call = |queue_family_property_count, queue_family_properties| {
                (self.get_physical_device_queue_family_properties)(
                    physical_device,
                    queue_family_property_count,
                    queue_family_properties as _,
                )
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut());
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let queue_family_properties_buf = queue_family_properties.reserve(capacity);
            len = queue_family_properties_buf.len().try_into().unwrap();
            call(&mut len, queue_family_properties_buf.as_mut_ptr() as *mut _);
            queue_family_properties.set_len(len.try_into().unwrap());
        }
    }
    pub unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceMemoryProperties {
        unsafe {
            let mut memory_properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_memory_properties)(
                physical_device,
                memory_properties.as_mut_ptr(),
            );
            memory_properties.assume_init()
        }
    }
    pub unsafe fn get_instance_proc_addr(
        &self,
        instance: Instance,
        name: &CStr,
    ) -> PFN_vkVoidFunction {
        unsafe { (self.get_instance_proc_addr)(instance, name.as_ptr() as _) }
    }
    pub unsafe fn create_device(
        &self,
        physical_device: PhysicalDevice,
        create_info: &DeviceCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<Device> {
        unsafe {
            let mut device = core::mem::MaybeUninit::uninit();
            let result = (self.create_device)(
                physical_device,
                create_info,
                allocator.to_raw_ptr(),
                device.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(device.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn enumerate_device_extension_properties(
        &self,
        physical_device: PhysicalDevice,
        layer_name: Option<&CStr>,
        mut properties: impl ExtendUninit<ExtensionProperties>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |property_count, properties| {
                let result = (self.enumerate_device_extension_properties)(
                    physical_device,
                    layer_name.to_raw_ptr(),
                    property_count,
                    properties as _,
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
            let properties_buf = properties.reserve(capacity);
            len = properties_buf.len().try_into().unwrap();
            let result = call(&mut len, properties_buf.as_mut_ptr() as *mut _)?;
            properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
    pub unsafe fn enumerate_device_layer_properties(
        &self,
        physical_device: PhysicalDevice,
        mut properties: impl ExtendUninit<LayerProperties>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |property_count, properties| {
                let result = (self.enumerate_device_layer_properties)(
                    physical_device,
                    property_count,
                    properties as _,
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
            let properties_buf = properties.reserve(capacity);
            len = properties_buf.len().try_into().unwrap();
            let result = call(&mut len, properties_buf.as_mut_ptr() as *mut _)?;
            properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        samples: SampleCountFlagBits,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
        mut properties: impl ExtendUninit<SparseImageFormatProperties>,
    ) {
        unsafe {
            let call = |property_count, properties| {
                (self.get_physical_device_sparse_image_format_properties)(
                    physical_device,
                    format,
                    ty,
                    samples,
                    usage,
                    tiling,
                    property_count,
                    properties as _,
                )
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut());
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let properties_buf = properties.reserve(capacity);
            len = properties_buf.len().try_into().unwrap();
            call(&mut len, properties_buf.as_mut_ptr() as *mut _);
            properties.set_len(len.try_into().unwrap());
        }
    }
}
pub struct DeviceFn {
    get_device_proc_addr: PFN_vkGetDeviceProcAddr,
    destroy_device: PFN_vkDestroyDevice,
    get_device_queue: PFN_vkGetDeviceQueue,
    queue_submit: PFN_vkQueueSubmit,
    queue_wait_idle: PFN_vkQueueWaitIdle,
    device_wait_idle: PFN_vkDeviceWaitIdle,
    allocate_memory: PFN_vkAllocateMemory,
    free_memory: PFN_vkFreeMemory,
    map_memory: PFN_vkMapMemory,
    unmap_memory: PFN_vkUnmapMemory,
    flush_mapped_memory_ranges: PFN_vkFlushMappedMemoryRanges,
    invalidate_mapped_memory_ranges: PFN_vkInvalidateMappedMemoryRanges,
    get_device_memory_commitment: PFN_vkGetDeviceMemoryCommitment,
    bind_buffer_memory: PFN_vkBindBufferMemory,
    bind_image_memory: PFN_vkBindImageMemory,
    get_buffer_memory_requirements: PFN_vkGetBufferMemoryRequirements,
    get_image_memory_requirements: PFN_vkGetImageMemoryRequirements,
    get_image_sparse_memory_requirements: PFN_vkGetImageSparseMemoryRequirements,
    queue_bind_sparse: PFN_vkQueueBindSparse,
    create_fence: PFN_vkCreateFence,
    destroy_fence: PFN_vkDestroyFence,
    reset_fences: PFN_vkResetFences,
    get_fence_status: PFN_vkGetFenceStatus,
    wait_for_fences: PFN_vkWaitForFences,
    create_semaphore: PFN_vkCreateSemaphore,
    destroy_semaphore: PFN_vkDestroySemaphore,
    create_query_pool: PFN_vkCreateQueryPool,
    destroy_query_pool: PFN_vkDestroyQueryPool,
    get_query_pool_results: PFN_vkGetQueryPoolResults,
    create_buffer: PFN_vkCreateBuffer,
    destroy_buffer: PFN_vkDestroyBuffer,
    create_image: PFN_vkCreateImage,
    destroy_image: PFN_vkDestroyImage,
    get_image_subresource_layout: PFN_vkGetImageSubresourceLayout,
    create_image_view: PFN_vkCreateImageView,
    destroy_image_view: PFN_vkDestroyImageView,
    create_command_pool: PFN_vkCreateCommandPool,
    destroy_command_pool: PFN_vkDestroyCommandPool,
    reset_command_pool: PFN_vkResetCommandPool,
    allocate_command_buffers: PFN_vkAllocateCommandBuffers,
    free_command_buffers: PFN_vkFreeCommandBuffers,
    begin_command_buffer: PFN_vkBeginCommandBuffer,
    end_command_buffer: PFN_vkEndCommandBuffer,
    reset_command_buffer: PFN_vkResetCommandBuffer,
    cmd_copy_buffer: PFN_vkCmdCopyBuffer,
    cmd_copy_image: PFN_vkCmdCopyImage,
    cmd_copy_buffer_to_image: PFN_vkCmdCopyBufferToImage,
    cmd_copy_image_to_buffer: PFN_vkCmdCopyImageToBuffer,
    cmd_update_buffer: PFN_vkCmdUpdateBuffer,
    cmd_fill_buffer: PFN_vkCmdFillBuffer,
    cmd_pipeline_barrier: PFN_vkCmdPipelineBarrier,
    cmd_begin_query: PFN_vkCmdBeginQuery,
    cmd_end_query: PFN_vkCmdEndQuery,
    cmd_reset_query_pool: PFN_vkCmdResetQueryPool,
    cmd_write_timestamp: PFN_vkCmdWriteTimestamp,
    cmd_copy_query_pool_results: PFN_vkCmdCopyQueryPoolResults,
    cmd_execute_commands: PFN_vkCmdExecuteCommands,
    create_event: PFN_vkCreateEvent,
    destroy_event: PFN_vkDestroyEvent,
    get_event_status: PFN_vkGetEventStatus,
    set_event: PFN_vkSetEvent,
    reset_event: PFN_vkResetEvent,
    create_buffer_view: PFN_vkCreateBufferView,
    destroy_buffer_view: PFN_vkDestroyBufferView,
    create_shader_module: PFN_vkCreateShaderModule,
    destroy_shader_module: PFN_vkDestroyShaderModule,
    create_pipeline_cache: PFN_vkCreatePipelineCache,
    destroy_pipeline_cache: PFN_vkDestroyPipelineCache,
    get_pipeline_cache_data: PFN_vkGetPipelineCacheData,
    merge_pipeline_caches: PFN_vkMergePipelineCaches,
    create_compute_pipelines: PFN_vkCreateComputePipelines,
    destroy_pipeline: PFN_vkDestroyPipeline,
    create_pipeline_layout: PFN_vkCreatePipelineLayout,
    destroy_pipeline_layout: PFN_vkDestroyPipelineLayout,
    create_sampler: PFN_vkCreateSampler,
    destroy_sampler: PFN_vkDestroySampler,
    create_descriptor_set_layout: PFN_vkCreateDescriptorSetLayout,
    destroy_descriptor_set_layout: PFN_vkDestroyDescriptorSetLayout,
    create_descriptor_pool: PFN_vkCreateDescriptorPool,
    destroy_descriptor_pool: PFN_vkDestroyDescriptorPool,
    reset_descriptor_pool: PFN_vkResetDescriptorPool,
    allocate_descriptor_sets: PFN_vkAllocateDescriptorSets,
    free_descriptor_sets: PFN_vkFreeDescriptorSets,
    update_descriptor_sets: PFN_vkUpdateDescriptorSets,
    cmd_bind_pipeline: PFN_vkCmdBindPipeline,
    cmd_bind_descriptor_sets: PFN_vkCmdBindDescriptorSets,
    cmd_clear_color_image: PFN_vkCmdClearColorImage,
    cmd_dispatch: PFN_vkCmdDispatch,
    cmd_dispatch_indirect: PFN_vkCmdDispatchIndirect,
    cmd_set_event: PFN_vkCmdSetEvent,
    cmd_reset_event: PFN_vkCmdResetEvent,
    cmd_wait_events: PFN_vkCmdWaitEvents,
    cmd_push_constants: PFN_vkCmdPushConstants,
    create_graphics_pipelines: PFN_vkCreateGraphicsPipelines,
    create_framebuffer: PFN_vkCreateFramebuffer,
    destroy_framebuffer: PFN_vkDestroyFramebuffer,
    create_render_pass: PFN_vkCreateRenderPass,
    destroy_render_pass: PFN_vkDestroyRenderPass,
    get_render_area_granularity: PFN_vkGetRenderAreaGranularity,
    cmd_set_viewport: PFN_vkCmdSetViewport,
    cmd_set_scissor: PFN_vkCmdSetScissor,
    cmd_set_line_width: PFN_vkCmdSetLineWidth,
    cmd_set_depth_bias: PFN_vkCmdSetDepthBias,
    cmd_set_blend_constants: PFN_vkCmdSetBlendConstants,
    cmd_set_depth_bounds: PFN_vkCmdSetDepthBounds,
    cmd_set_stencil_compare_mask: PFN_vkCmdSetStencilCompareMask,
    cmd_set_stencil_write_mask: PFN_vkCmdSetStencilWriteMask,
    cmd_set_stencil_reference: PFN_vkCmdSetStencilReference,
    cmd_bind_index_buffer: PFN_vkCmdBindIndexBuffer,
    cmd_bind_vertex_buffers: PFN_vkCmdBindVertexBuffers,
    cmd_draw: PFN_vkCmdDraw,
    cmd_draw_indexed: PFN_vkCmdDrawIndexed,
    cmd_draw_indirect: PFN_vkCmdDrawIndirect,
    cmd_draw_indexed_indirect: PFN_vkCmdDrawIndexedIndirect,
    cmd_blit_image: PFN_vkCmdBlitImage,
    cmd_clear_depth_stencil_image: PFN_vkCmdClearDepthStencilImage,
    cmd_clear_attachments: PFN_vkCmdClearAttachments,
    cmd_resolve_image: PFN_vkCmdResolveImage,
    cmd_begin_render_pass: PFN_vkCmdBeginRenderPass,
    cmd_next_subpass: PFN_vkCmdNextSubpass,
    cmd_end_render_pass: PFN_vkCmdEndRenderPass,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_device_proc_addr: transmute(
                    load(c"vkGetDeviceProcAddr").ok_or(MissingEntryPointError)?,
                ),
                destroy_device: transmute(load(c"vkDestroyDevice").ok_or(MissingEntryPointError)?),
                get_device_queue: transmute(
                    load(c"vkGetDeviceQueue").ok_or(MissingEntryPointError)?,
                ),
                queue_submit: transmute(load(c"vkQueueSubmit").ok_or(MissingEntryPointError)?),
                queue_wait_idle: transmute(load(c"vkQueueWaitIdle").ok_or(MissingEntryPointError)?),
                device_wait_idle: transmute(
                    load(c"vkDeviceWaitIdle").ok_or(MissingEntryPointError)?,
                ),
                allocate_memory: transmute(
                    load(c"vkAllocateMemory").ok_or(MissingEntryPointError)?,
                ),
                free_memory: transmute(load(c"vkFreeMemory").ok_or(MissingEntryPointError)?),
                map_memory: transmute(load(c"vkMapMemory").ok_or(MissingEntryPointError)?),
                unmap_memory: transmute(load(c"vkUnmapMemory").ok_or(MissingEntryPointError)?),
                flush_mapped_memory_ranges: transmute(
                    load(c"vkFlushMappedMemoryRanges").ok_or(MissingEntryPointError)?,
                ),
                invalidate_mapped_memory_ranges: transmute(
                    load(c"vkInvalidateMappedMemoryRanges").ok_or(MissingEntryPointError)?,
                ),
                get_device_memory_commitment: transmute(
                    load(c"vkGetDeviceMemoryCommitment").ok_or(MissingEntryPointError)?,
                ),
                bind_buffer_memory: transmute(
                    load(c"vkBindBufferMemory").ok_or(MissingEntryPointError)?,
                ),
                bind_image_memory: transmute(
                    load(c"vkBindImageMemory").ok_or(MissingEntryPointError)?,
                ),
                get_buffer_memory_requirements: transmute(
                    load(c"vkGetBufferMemoryRequirements").ok_or(MissingEntryPointError)?,
                ),
                get_image_memory_requirements: transmute(
                    load(c"vkGetImageMemoryRequirements").ok_or(MissingEntryPointError)?,
                ),
                get_image_sparse_memory_requirements: transmute(
                    load(c"vkGetImageSparseMemoryRequirements").ok_or(MissingEntryPointError)?,
                ),
                queue_bind_sparse: transmute(
                    load(c"vkQueueBindSparse").ok_or(MissingEntryPointError)?,
                ),
                create_fence: transmute(load(c"vkCreateFence").ok_or(MissingEntryPointError)?),
                destroy_fence: transmute(load(c"vkDestroyFence").ok_or(MissingEntryPointError)?),
                reset_fences: transmute(load(c"vkResetFences").ok_or(MissingEntryPointError)?),
                get_fence_status: transmute(
                    load(c"vkGetFenceStatus").ok_or(MissingEntryPointError)?,
                ),
                wait_for_fences: transmute(load(c"vkWaitForFences").ok_or(MissingEntryPointError)?),
                create_semaphore: transmute(
                    load(c"vkCreateSemaphore").ok_or(MissingEntryPointError)?,
                ),
                destroy_semaphore: transmute(
                    load(c"vkDestroySemaphore").ok_or(MissingEntryPointError)?,
                ),
                create_query_pool: transmute(
                    load(c"vkCreateQueryPool").ok_or(MissingEntryPointError)?,
                ),
                destroy_query_pool: transmute(
                    load(c"vkDestroyQueryPool").ok_or(MissingEntryPointError)?,
                ),
                get_query_pool_results: transmute(
                    load(c"vkGetQueryPoolResults").ok_or(MissingEntryPointError)?,
                ),
                create_buffer: transmute(load(c"vkCreateBuffer").ok_or(MissingEntryPointError)?),
                destroy_buffer: transmute(load(c"vkDestroyBuffer").ok_or(MissingEntryPointError)?),
                create_image: transmute(load(c"vkCreateImage").ok_or(MissingEntryPointError)?),
                destroy_image: transmute(load(c"vkDestroyImage").ok_or(MissingEntryPointError)?),
                get_image_subresource_layout: transmute(
                    load(c"vkGetImageSubresourceLayout").ok_or(MissingEntryPointError)?,
                ),
                create_image_view: transmute(
                    load(c"vkCreateImageView").ok_or(MissingEntryPointError)?,
                ),
                destroy_image_view: transmute(
                    load(c"vkDestroyImageView").ok_or(MissingEntryPointError)?,
                ),
                create_command_pool: transmute(
                    load(c"vkCreateCommandPool").ok_or(MissingEntryPointError)?,
                ),
                destroy_command_pool: transmute(
                    load(c"vkDestroyCommandPool").ok_or(MissingEntryPointError)?,
                ),
                reset_command_pool: transmute(
                    load(c"vkResetCommandPool").ok_or(MissingEntryPointError)?,
                ),
                allocate_command_buffers: transmute(
                    load(c"vkAllocateCommandBuffers").ok_or(MissingEntryPointError)?,
                ),
                free_command_buffers: transmute(
                    load(c"vkFreeCommandBuffers").ok_or(MissingEntryPointError)?,
                ),
                begin_command_buffer: transmute(
                    load(c"vkBeginCommandBuffer").ok_or(MissingEntryPointError)?,
                ),
                end_command_buffer: transmute(
                    load(c"vkEndCommandBuffer").ok_or(MissingEntryPointError)?,
                ),
                reset_command_buffer: transmute(
                    load(c"vkResetCommandBuffer").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_buffer: transmute(load(c"vkCmdCopyBuffer").ok_or(MissingEntryPointError)?),
                cmd_copy_image: transmute(load(c"vkCmdCopyImage").ok_or(MissingEntryPointError)?),
                cmd_copy_buffer_to_image: transmute(
                    load(c"vkCmdCopyBufferToImage").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_image_to_buffer: transmute(
                    load(c"vkCmdCopyImageToBuffer").ok_or(MissingEntryPointError)?,
                ),
                cmd_update_buffer: transmute(
                    load(c"vkCmdUpdateBuffer").ok_or(MissingEntryPointError)?,
                ),
                cmd_fill_buffer: transmute(load(c"vkCmdFillBuffer").ok_or(MissingEntryPointError)?),
                cmd_pipeline_barrier: transmute(
                    load(c"vkCmdPipelineBarrier").ok_or(MissingEntryPointError)?,
                ),
                cmd_begin_query: transmute(load(c"vkCmdBeginQuery").ok_or(MissingEntryPointError)?),
                cmd_end_query: transmute(load(c"vkCmdEndQuery").ok_or(MissingEntryPointError)?),
                cmd_reset_query_pool: transmute(
                    load(c"vkCmdResetQueryPool").ok_or(MissingEntryPointError)?,
                ),
                cmd_write_timestamp: transmute(
                    load(c"vkCmdWriteTimestamp").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_query_pool_results: transmute(
                    load(c"vkCmdCopyQueryPoolResults").ok_or(MissingEntryPointError)?,
                ),
                cmd_execute_commands: transmute(
                    load(c"vkCmdExecuteCommands").ok_or(MissingEntryPointError)?,
                ),
                create_event: transmute(load(c"vkCreateEvent").ok_or(MissingEntryPointError)?),
                destroy_event: transmute(load(c"vkDestroyEvent").ok_or(MissingEntryPointError)?),
                get_event_status: transmute(
                    load(c"vkGetEventStatus").ok_or(MissingEntryPointError)?,
                ),
                set_event: transmute(load(c"vkSetEvent").ok_or(MissingEntryPointError)?),
                reset_event: transmute(load(c"vkResetEvent").ok_or(MissingEntryPointError)?),
                create_buffer_view: transmute(
                    load(c"vkCreateBufferView").ok_or(MissingEntryPointError)?,
                ),
                destroy_buffer_view: transmute(
                    load(c"vkDestroyBufferView").ok_or(MissingEntryPointError)?,
                ),
                create_shader_module: transmute(
                    load(c"vkCreateShaderModule").ok_or(MissingEntryPointError)?,
                ),
                destroy_shader_module: transmute(
                    load(c"vkDestroyShaderModule").ok_or(MissingEntryPointError)?,
                ),
                create_pipeline_cache: transmute(
                    load(c"vkCreatePipelineCache").ok_or(MissingEntryPointError)?,
                ),
                destroy_pipeline_cache: transmute(
                    load(c"vkDestroyPipelineCache").ok_or(MissingEntryPointError)?,
                ),
                get_pipeline_cache_data: transmute(
                    load(c"vkGetPipelineCacheData").ok_or(MissingEntryPointError)?,
                ),
                merge_pipeline_caches: transmute(
                    load(c"vkMergePipelineCaches").ok_or(MissingEntryPointError)?,
                ),
                create_compute_pipelines: transmute(
                    load(c"vkCreateComputePipelines").ok_or(MissingEntryPointError)?,
                ),
                destroy_pipeline: transmute(
                    load(c"vkDestroyPipeline").ok_or(MissingEntryPointError)?,
                ),
                create_pipeline_layout: transmute(
                    load(c"vkCreatePipelineLayout").ok_or(MissingEntryPointError)?,
                ),
                destroy_pipeline_layout: transmute(
                    load(c"vkDestroyPipelineLayout").ok_or(MissingEntryPointError)?,
                ),
                create_sampler: transmute(load(c"vkCreateSampler").ok_or(MissingEntryPointError)?),
                destroy_sampler: transmute(
                    load(c"vkDestroySampler").ok_or(MissingEntryPointError)?,
                ),
                create_descriptor_set_layout: transmute(
                    load(c"vkCreateDescriptorSetLayout").ok_or(MissingEntryPointError)?,
                ),
                destroy_descriptor_set_layout: transmute(
                    load(c"vkDestroyDescriptorSetLayout").ok_or(MissingEntryPointError)?,
                ),
                create_descriptor_pool: transmute(
                    load(c"vkCreateDescriptorPool").ok_or(MissingEntryPointError)?,
                ),
                destroy_descriptor_pool: transmute(
                    load(c"vkDestroyDescriptorPool").ok_or(MissingEntryPointError)?,
                ),
                reset_descriptor_pool: transmute(
                    load(c"vkResetDescriptorPool").ok_or(MissingEntryPointError)?,
                ),
                allocate_descriptor_sets: transmute(
                    load(c"vkAllocateDescriptorSets").ok_or(MissingEntryPointError)?,
                ),
                free_descriptor_sets: transmute(
                    load(c"vkFreeDescriptorSets").ok_or(MissingEntryPointError)?,
                ),
                update_descriptor_sets: transmute(
                    load(c"vkUpdateDescriptorSets").ok_or(MissingEntryPointError)?,
                ),
                cmd_bind_pipeline: transmute(
                    load(c"vkCmdBindPipeline").ok_or(MissingEntryPointError)?,
                ),
                cmd_bind_descriptor_sets: transmute(
                    load(c"vkCmdBindDescriptorSets").ok_or(MissingEntryPointError)?,
                ),
                cmd_clear_color_image: transmute(
                    load(c"vkCmdClearColorImage").ok_or(MissingEntryPointError)?,
                ),
                cmd_dispatch: transmute(load(c"vkCmdDispatch").ok_or(MissingEntryPointError)?),
                cmd_dispatch_indirect: transmute(
                    load(c"vkCmdDispatchIndirect").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_event: transmute(load(c"vkCmdSetEvent").ok_or(MissingEntryPointError)?),
                cmd_reset_event: transmute(load(c"vkCmdResetEvent").ok_or(MissingEntryPointError)?),
                cmd_wait_events: transmute(load(c"vkCmdWaitEvents").ok_or(MissingEntryPointError)?),
                cmd_push_constants: transmute(
                    load(c"vkCmdPushConstants").ok_or(MissingEntryPointError)?,
                ),
                create_graphics_pipelines: transmute(
                    load(c"vkCreateGraphicsPipelines").ok_or(MissingEntryPointError)?,
                ),
                create_framebuffer: transmute(
                    load(c"vkCreateFramebuffer").ok_or(MissingEntryPointError)?,
                ),
                destroy_framebuffer: transmute(
                    load(c"vkDestroyFramebuffer").ok_or(MissingEntryPointError)?,
                ),
                create_render_pass: transmute(
                    load(c"vkCreateRenderPass").ok_or(MissingEntryPointError)?,
                ),
                destroy_render_pass: transmute(
                    load(c"vkDestroyRenderPass").ok_or(MissingEntryPointError)?,
                ),
                get_render_area_granularity: transmute(
                    load(c"vkGetRenderAreaGranularity").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_viewport: transmute(
                    load(c"vkCmdSetViewport").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_scissor: transmute(load(c"vkCmdSetScissor").ok_or(MissingEntryPointError)?),
                cmd_set_line_width: transmute(
                    load(c"vkCmdSetLineWidth").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_bias: transmute(
                    load(c"vkCmdSetDepthBias").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_blend_constants: transmute(
                    load(c"vkCmdSetBlendConstants").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_bounds: transmute(
                    load(c"vkCmdSetDepthBounds").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_stencil_compare_mask: transmute(
                    load(c"vkCmdSetStencilCompareMask").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_stencil_write_mask: transmute(
                    load(c"vkCmdSetStencilWriteMask").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_stencil_reference: transmute(
                    load(c"vkCmdSetStencilReference").ok_or(MissingEntryPointError)?,
                ),
                cmd_bind_index_buffer: transmute(
                    load(c"vkCmdBindIndexBuffer").ok_or(MissingEntryPointError)?,
                ),
                cmd_bind_vertex_buffers: transmute(
                    load(c"vkCmdBindVertexBuffers").ok_or(MissingEntryPointError)?,
                ),
                cmd_draw: transmute(load(c"vkCmdDraw").ok_or(MissingEntryPointError)?),
                cmd_draw_indexed: transmute(
                    load(c"vkCmdDrawIndexed").ok_or(MissingEntryPointError)?,
                ),
                cmd_draw_indirect: transmute(
                    load(c"vkCmdDrawIndirect").ok_or(MissingEntryPointError)?,
                ),
                cmd_draw_indexed_indirect: transmute(
                    load(c"vkCmdDrawIndexedIndirect").ok_or(MissingEntryPointError)?,
                ),
                cmd_blit_image: transmute(load(c"vkCmdBlitImage").ok_or(MissingEntryPointError)?),
                cmd_clear_depth_stencil_image: transmute(
                    load(c"vkCmdClearDepthStencilImage").ok_or(MissingEntryPointError)?,
                ),
                cmd_clear_attachments: transmute(
                    load(c"vkCmdClearAttachments").ok_or(MissingEntryPointError)?,
                ),
                cmd_resolve_image: transmute(
                    load(c"vkCmdResolveImage").ok_or(MissingEntryPointError)?,
                ),
                cmd_begin_render_pass: transmute(
                    load(c"vkCmdBeginRenderPass").ok_or(MissingEntryPointError)?,
                ),
                cmd_next_subpass: transmute(
                    load(c"vkCmdNextSubpass").ok_or(MissingEntryPointError)?,
                ),
                cmd_end_render_pass: transmute(
                    load(c"vkCmdEndRenderPass").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_device_proc_addr(&self, device: Device, name: &CStr) -> PFN_vkVoidFunction {
        unsafe { (self.get_device_proc_addr)(device, name.as_ptr() as _) }
    }
    pub unsafe fn destroy_device(
        &self,
        device: Device,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_device)(device, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_device_queue(
        &self,
        device: Device,
        queue_family_index: u32,
        queue_index: u32,
    ) -> Queue {
        unsafe {
            let mut queue = core::mem::MaybeUninit::uninit();
            (self.get_device_queue)(device, queue_family_index, queue_index, queue.as_mut_ptr());
            queue.assume_init()
        }
    }
    pub unsafe fn queue_submit(
        &self,
        queue: Queue,
        submits: &[SubmitInfo<'_>],
        fence: Fence,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.queue_submit)(
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
    pub unsafe fn queue_wait_idle(&self, queue: Queue) -> crate::Result<()> {
        unsafe {
            let result = (self.queue_wait_idle)(queue);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn device_wait_idle(&self, device: Device) -> crate::Result<()> {
        unsafe {
            let result = (self.device_wait_idle)(device);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn allocate_memory(
        &self,
        device: Device,
        allocate_info: &MemoryAllocateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<DeviceMemory> {
        unsafe {
            let mut memory = core::mem::MaybeUninit::uninit();
            let result = (self.allocate_memory)(
                device,
                allocate_info,
                allocator.to_raw_ptr(),
                memory.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(memory.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn free_memory(
        &self,
        device: Device,
        memory: DeviceMemory,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.free_memory)(device, memory, allocator.to_raw_ptr()) }
    }
    pub unsafe fn map_memory(
        &self,
        device: Device,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
    ) -> crate::Result<*mut c_void> {
        unsafe {
            let mut data = core::mem::MaybeUninit::uninit();
            let result = (self.map_memory)(device, memory, offset, size, flags, data.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(data.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn unmap_memory(&self, device: Device, memory: DeviceMemory) {
        unsafe { (self.unmap_memory)(device, memory) }
    }
    pub unsafe fn flush_mapped_memory_ranges(
        &self,
        device: Device,
        memory_ranges: &[MappedMemoryRange<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.flush_mapped_memory_ranges)(
                device,
                memory_ranges.len().try_into().unwrap(),
                memory_ranges.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn invalidate_mapped_memory_ranges(
        &self,
        device: Device,
        memory_ranges: &[MappedMemoryRange<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.invalidate_mapped_memory_ranges)(
                device,
                memory_ranges.len().try_into().unwrap(),
                memory_ranges.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_device_memory_commitment(
        &self,
        device: Device,
        memory: DeviceMemory,
    ) -> DeviceSize {
        unsafe {
            let mut committed_memory_in_bytes = core::mem::MaybeUninit::uninit();
            (self.get_device_memory_commitment)(
                device,
                memory,
                committed_memory_in_bytes.as_mut_ptr(),
            );
            committed_memory_in_bytes.assume_init()
        }
    }
    pub unsafe fn bind_buffer_memory(
        &self,
        device: Device,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.bind_buffer_memory)(device, buffer, memory, memory_offset);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn bind_image_memory(
        &self,
        device: Device,
        image: Image,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.bind_image_memory)(device, image, memory, memory_offset);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_buffer_memory_requirements(
        &self,
        device: Device,
        buffer: Buffer,
    ) -> MemoryRequirements {
        unsafe {
            let mut memory_requirements = core::mem::MaybeUninit::uninit();
            (self.get_buffer_memory_requirements)(device, buffer, memory_requirements.as_mut_ptr());
            memory_requirements.assume_init()
        }
    }
    pub unsafe fn get_image_memory_requirements(
        &self,
        device: Device,
        image: Image,
    ) -> MemoryRequirements {
        unsafe {
            let mut memory_requirements = core::mem::MaybeUninit::uninit();
            (self.get_image_memory_requirements)(device, image, memory_requirements.as_mut_ptr());
            memory_requirements.assume_init()
        }
    }
    pub unsafe fn get_image_sparse_memory_requirements(
        &self,
        device: Device,
        image: Image,
        mut sparse_memory_requirements: impl ExtendUninit<SparseImageMemoryRequirements>,
    ) {
        unsafe {
            let call = |sparse_memory_requirement_count, sparse_memory_requirements| {
                (self.get_image_sparse_memory_requirements)(
                    device,
                    image,
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
    pub unsafe fn queue_bind_sparse(
        &self,
        queue: Queue,
        bind_info: &[BindSparseInfo<'_>],
        fence: Fence,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.queue_bind_sparse)(
                queue,
                bind_info.len().try_into().unwrap(),
                bind_info.as_ptr() as _,
                fence,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn create_fence(
        &self,
        device: Device,
        create_info: &FenceCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<Fence> {
        unsafe {
            let mut fence = core::mem::MaybeUninit::uninit();
            let result = (self.create_fence)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                fence.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(fence.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_fence(
        &self,
        device: Device,
        fence: Fence,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_fence)(device, fence, allocator.to_raw_ptr()) }
    }
    pub unsafe fn reset_fences(&self, device: Device, fences: &[Fence]) -> crate::Result<()> {
        unsafe {
            let result = (self.reset_fences)(
                device,
                fences.len().try_into().unwrap(),
                fences.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_fence_status(&self, device: Device, fence: Fence) -> crate::Result<()> {
        unsafe {
            let result = (self.get_fence_status)(device, fence);

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::NOT_READY => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn wait_for_fences(
        &self,
        device: Device,
        fences: &[Fence],
        wait_all: Bool32,
        timeout: u64,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.wait_for_fences)(
                device,
                fences.len().try_into().unwrap(),
                fences.as_ptr() as _,
                wait_all,
                timeout,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::TIMEOUT => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn create_semaphore(
        &self,
        device: Device,
        create_info: &SemaphoreCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<Semaphore> {
        unsafe {
            let mut semaphore = core::mem::MaybeUninit::uninit();
            let result = (self.create_semaphore)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                semaphore.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(semaphore.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_semaphore(
        &self,
        device: Device,
        semaphore: Semaphore,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_semaphore)(device, semaphore, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_query_pool(
        &self,
        device: Device,
        create_info: &QueryPoolCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<QueryPool> {
        unsafe {
            let mut query_pool = core::mem::MaybeUninit::uninit();
            let result = (self.create_query_pool)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                query_pool.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(query_pool.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_query_pool(
        &self,
        device: Device,
        query_pool: QueryPool,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_query_pool)(device, query_pool, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_query_pool_results(
        &self,
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data: &mut [u8],
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_query_pool_results)(
                device,
                query_pool,
                first_query,
                query_count,
                data.len().try_into().unwrap(),
                data.as_mut_ptr() as _,
                stride,
                flags,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::NOT_READY => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn create_buffer(
        &self,
        device: Device,
        create_info: &BufferCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<Buffer> {
        unsafe {
            let mut buffer = core::mem::MaybeUninit::uninit();
            let result = (self.create_buffer)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                buffer.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(buffer.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_buffer(
        &self,
        device: Device,
        buffer: Buffer,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_buffer)(device, buffer, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_image(
        &self,
        device: Device,
        create_info: &ImageCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<Image> {
        unsafe {
            let mut image = core::mem::MaybeUninit::uninit();
            let result = (self.create_image)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                image.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(image.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_image(
        &self,
        device: Device,
        image: Image,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_image)(device, image, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_image_subresource_layout(
        &self,
        device: Device,
        image: Image,
        subresource: &ImageSubresource,
    ) -> SubresourceLayout {
        unsafe {
            let mut layout = core::mem::MaybeUninit::uninit();
            (self.get_image_subresource_layout)(device, image, subresource, layout.as_mut_ptr());
            layout.assume_init()
        }
    }
    pub unsafe fn create_image_view(
        &self,
        device: Device,
        create_info: &ImageViewCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<ImageView> {
        unsafe {
            let mut view = core::mem::MaybeUninit::uninit();
            let result = (self.create_image_view)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                view.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(view.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_image_view(
        &self,
        device: Device,
        image_view: ImageView,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_image_view)(device, image_view, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_command_pool(
        &self,
        device: Device,
        create_info: &CommandPoolCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<CommandPool> {
        unsafe {
            let mut command_pool = core::mem::MaybeUninit::uninit();
            let result = (self.create_command_pool)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                command_pool.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(command_pool.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_command_pool)(device, command_pool, allocator.to_raw_ptr()) }
    }
    pub unsafe fn reset_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.reset_command_pool)(device, command_pool, flags);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn allocate_command_buffers(
        &self,
        device: Device,
        allocate_info: &CommandBufferAllocateInfo<'_>,
        command_buffers: &mut [CommandBuffer],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.allocate_command_buffers)(
                device,
                allocate_info,
                command_buffers.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn free_command_buffers(
        &self,
        device: Device,
        command_pool: CommandPool,
        command_buffers: &[CommandBuffer],
    ) {
        unsafe {
            (self.free_command_buffers)(
                device,
                command_pool,
                command_buffers.len().try_into().unwrap(),
                command_buffers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn begin_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        begin_info: &CommandBufferBeginInfo<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.begin_command_buffer)(command_buffer, begin_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn end_command_buffer(&self, command_buffer: CommandBuffer) -> crate::Result<()> {
        unsafe {
            let result = (self.end_command_buffer)(command_buffer);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn reset_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        flags: CommandBufferResetFlags,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.reset_command_buffer)(command_buffer, flags);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn cmd_copy_buffer(
        &self,
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        regions: &[BufferCopy],
    ) {
        unsafe {
            (self.cmd_copy_buffer)(
                command_buffer,
                src_buffer,
                dst_buffer,
                regions.len().try_into().unwrap(),
                regions.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_copy_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageCopy],
    ) {
        unsafe {
            (self.cmd_copy_image)(
                command_buffer,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions.len().try_into().unwrap(),
                regions.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_copy_buffer_to_image(
        &self,
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[BufferImageCopy],
    ) {
        unsafe {
            (self.cmd_copy_buffer_to_image)(
                command_buffer,
                src_buffer,
                dst_image,
                dst_image_layout,
                regions.len().try_into().unwrap(),
                regions.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_copy_image_to_buffer(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        regions: &[BufferImageCopy],
    ) {
        unsafe {
            (self.cmd_copy_image_to_buffer)(
                command_buffer,
                src_image,
                src_image_layout,
                dst_buffer,
                regions.len().try_into().unwrap(),
                regions.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_update_buffer(
        &self,
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        data: &[u8],
    ) {
        unsafe {
            (self.cmd_update_buffer)(
                command_buffer,
                dst_buffer,
                dst_offset,
                data.len().try_into().unwrap(),
                data.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_fill_buffer(
        &self,
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: u32,
    ) {
        unsafe { (self.cmd_fill_buffer)(command_buffer, dst_buffer, dst_offset, size, data) }
    }
    pub unsafe fn cmd_pipeline_barrier(
        &self,
        command_buffer: CommandBuffer,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barriers: &[MemoryBarrier<'_>],
        buffer_memory_barriers: &[BufferMemoryBarrier<'_>],
        image_memory_barriers: &[ImageMemoryBarrier<'_>],
    ) {
        unsafe {
            (self.cmd_pipeline_barrier)(
                command_buffer,
                src_stage_mask,
                dst_stage_mask,
                dependency_flags,
                memory_barriers.len().try_into().unwrap(),
                memory_barriers.as_ptr() as _,
                buffer_memory_barriers.len().try_into().unwrap(),
                buffer_memory_barriers.as_ptr() as _,
                image_memory_barriers.len().try_into().unwrap(),
                image_memory_barriers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_begin_query(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
    ) {
        unsafe { (self.cmd_begin_query)(command_buffer, query_pool, query, flags) }
    }
    pub unsafe fn cmd_end_query(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
    ) {
        unsafe { (self.cmd_end_query)(command_buffer, query_pool, query) }
    }
    pub unsafe fn cmd_reset_query_pool(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        unsafe { (self.cmd_reset_query_pool)(command_buffer, query_pool, first_query, query_count) }
    }
    pub unsafe fn cmd_write_timestamp(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlagBits,
        query_pool: QueryPool,
        query: u32,
    ) {
        unsafe { (self.cmd_write_timestamp)(command_buffer, pipeline_stage, query_pool, query) }
    }
    pub unsafe fn cmd_copy_query_pool_results(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) {
        unsafe {
            (self.cmd_copy_query_pool_results)(
                command_buffer,
                query_pool,
                first_query,
                query_count,
                dst_buffer,
                dst_offset,
                stride,
                flags,
            )
        }
    }
    pub unsafe fn cmd_execute_commands(
        &self,
        command_buffer: CommandBuffer,
        command_buffers: &[CommandBuffer],
    ) {
        unsafe {
            (self.cmd_execute_commands)(
                command_buffer,
                command_buffers.len().try_into().unwrap(),
                command_buffers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn create_event(
        &self,
        device: Device,
        create_info: &EventCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<Event> {
        unsafe {
            let mut event = core::mem::MaybeUninit::uninit();
            let result = (self.create_event)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                event.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(event.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_event(
        &self,
        device: Device,
        event: Event,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_event)(device, event, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_event_status(&self, device: Device, event: Event) -> crate::Result<()> {
        unsafe {
            let result = (self.get_event_status)(device, event);

            match result {
                VkResult::EVENT_SET => Ok(()),
                VkResult::EVENT_RESET => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn set_event(&self, device: Device, event: Event) -> crate::Result<()> {
        unsafe {
            let result = (self.set_event)(device, event);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn reset_event(&self, device: Device, event: Event) -> crate::Result<()> {
        unsafe {
            let result = (self.reset_event)(device, event);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn create_buffer_view(
        &self,
        device: Device,
        create_info: &BufferViewCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<BufferView> {
        unsafe {
            let mut view = core::mem::MaybeUninit::uninit();
            let result = (self.create_buffer_view)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                view.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(view.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_buffer_view(
        &self,
        device: Device,
        buffer_view: BufferView,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_buffer_view)(device, buffer_view, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_shader_module(
        &self,
        device: Device,
        create_info: &ShaderModuleCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<ShaderModule> {
        unsafe {
            let mut shader_module = core::mem::MaybeUninit::uninit();
            let result = (self.create_shader_module)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                shader_module.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(shader_module.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_shader_module(
        &self,
        device: Device,
        shader_module: ShaderModule,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_shader_module)(device, shader_module, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_pipeline_cache(
        &self,
        device: Device,
        create_info: &PipelineCacheCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<PipelineCache> {
        unsafe {
            let mut pipeline_cache = core::mem::MaybeUninit::uninit();
            let result = (self.create_pipeline_cache)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                pipeline_cache.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(pipeline_cache.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_pipeline_cache(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_pipeline_cache)(device, pipeline_cache, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_pipeline_cache_data(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        mut data: impl ExtendUninit<u8>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |data_size, data| {
                let result =
                    (self.get_pipeline_cache_data)(device, pipeline_cache, data_size, data as _);

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let data_buf = data.reserve(capacity);
            len = data_buf.len().try_into().unwrap();
            let result = call(&mut len, data_buf.as_mut_ptr() as *mut _)?;
            data.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
    pub unsafe fn merge_pipeline_caches(
        &self,
        device: Device,
        dst_cache: PipelineCache,
        src_caches: &[PipelineCache],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.merge_pipeline_caches)(
                device,
                dst_cache,
                src_caches.len().try_into().unwrap(),
                src_caches.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn create_compute_pipelines(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_infos: &[ComputePipelineCreateInfo<'_>],
        allocator: Option<&AllocationCallbacks<'_>>,
        pipelines: &mut [Pipeline],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.create_compute_pipelines)(
                device,
                pipeline_cache,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                pipelines.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::PIPELINE_COMPILE_REQUIRED_EXT => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_pipeline(
        &self,
        device: Device,
        pipeline: Pipeline,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_pipeline)(device, pipeline, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_pipeline_layout(
        &self,
        device: Device,
        create_info: &PipelineLayoutCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<PipelineLayout> {
        unsafe {
            let mut pipeline_layout = core::mem::MaybeUninit::uninit();
            let result = (self.create_pipeline_layout)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                pipeline_layout.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(pipeline_layout.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_pipeline_layout(
        &self,
        device: Device,
        pipeline_layout: PipelineLayout,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_pipeline_layout)(device, pipeline_layout, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_sampler(
        &self,
        device: Device,
        create_info: &SamplerCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<Sampler> {
        unsafe {
            let mut sampler = core::mem::MaybeUninit::uninit();
            let result = (self.create_sampler)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                sampler.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(sampler.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_sampler(
        &self,
        device: Device,
        sampler: Sampler,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_sampler)(device, sampler, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_descriptor_set_layout(
        &self,
        device: Device,
        create_info: &DescriptorSetLayoutCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<DescriptorSetLayout> {
        unsafe {
            let mut set_layout = core::mem::MaybeUninit::uninit();
            let result = (self.create_descriptor_set_layout)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                set_layout.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(set_layout.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_descriptor_set_layout(
        &self,
        device: Device,
        descriptor_set_layout: DescriptorSetLayout,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_descriptor_set_layout)(
                device,
                descriptor_set_layout,
                allocator.to_raw_ptr(),
            )
        }
    }
    pub unsafe fn create_descriptor_pool(
        &self,
        device: Device,
        create_info: &DescriptorPoolCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<DescriptorPool> {
        unsafe {
            let mut descriptor_pool = core::mem::MaybeUninit::uninit();
            let result = (self.create_descriptor_pool)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                descriptor_pool.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(descriptor_pool.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_descriptor_pool(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_descriptor_pool)(device, descriptor_pool, allocator.to_raw_ptr()) }
    }
    pub unsafe fn reset_descriptor_pool(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.reset_descriptor_pool)(device, descriptor_pool, flags);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn allocate_descriptor_sets(
        &self,
        device: Device,
        allocate_info: &DescriptorSetAllocateInfo<'_>,
        descriptor_sets: &mut [DescriptorSet],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.allocate_descriptor_sets)(
                device,
                allocate_info,
                descriptor_sets.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn free_descriptor_sets(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        descriptor_sets: &[DescriptorSet],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.free_descriptor_sets)(
                device,
                descriptor_pool,
                descriptor_sets.len().try_into().unwrap(),
                descriptor_sets.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn update_descriptor_sets(
        &self,
        device: Device,
        descriptor_writes: &[WriteDescriptorSet<'_>],
        descriptor_copies: &[CopyDescriptorSet<'_>],
    ) {
        unsafe {
            (self.update_descriptor_sets)(
                device,
                descriptor_writes.len().try_into().unwrap(),
                descriptor_writes.as_ptr() as _,
                descriptor_copies.len().try_into().unwrap(),
                descriptor_copies.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_bind_pipeline(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) {
        unsafe { (self.cmd_bind_pipeline)(command_buffer, pipeline_bind_point, pipeline) }
    }
    pub unsafe fn cmd_bind_descriptor_sets(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_sets: &[DescriptorSet],
        dynamic_offsets: &[u32],
    ) {
        unsafe {
            (self.cmd_bind_descriptor_sets)(
                command_buffer,
                pipeline_bind_point,
                layout,
                first_set,
                descriptor_sets.len().try_into().unwrap(),
                descriptor_sets.as_ptr() as _,
                dynamic_offsets.len().try_into().unwrap(),
                dynamic_offsets.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_clear_color_image(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        color: &ClearColorValue,
        ranges: &[ImageSubresourceRange],
    ) {
        unsafe {
            (self.cmd_clear_color_image)(
                command_buffer,
                image,
                image_layout,
                color,
                ranges.len().try_into().unwrap(),
                ranges.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_dispatch(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe { (self.cmd_dispatch)(command_buffer, group_count_x, group_count_y, group_count_z) }
    }
    pub unsafe fn cmd_dispatch_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
    ) {
        unsafe { (self.cmd_dispatch_indirect)(command_buffer, buffer, offset) }
    }
    pub unsafe fn cmd_set_event(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) {
        unsafe { (self.cmd_set_event)(command_buffer, event, stage_mask) }
    }
    pub unsafe fn cmd_reset_event(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) {
        unsafe { (self.cmd_reset_event)(command_buffer, event, stage_mask) }
    }
    pub unsafe fn cmd_wait_events(
        &self,
        command_buffer: CommandBuffer,
        events: &[Event],
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barriers: &[MemoryBarrier<'_>],
        buffer_memory_barriers: &[BufferMemoryBarrier<'_>],
        image_memory_barriers: &[ImageMemoryBarrier<'_>],
    ) {
        unsafe {
            (self.cmd_wait_events)(
                command_buffer,
                events.len().try_into().unwrap(),
                events.as_ptr() as _,
                src_stage_mask,
                dst_stage_mask,
                memory_barriers.len().try_into().unwrap(),
                memory_barriers.as_ptr() as _,
                buffer_memory_barriers.len().try_into().unwrap(),
                buffer_memory_barriers.as_ptr() as _,
                image_memory_barriers.len().try_into().unwrap(),
                image_memory_barriers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_push_constants(
        &self,
        command_buffer: CommandBuffer,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        values: &[u8],
    ) {
        unsafe {
            (self.cmd_push_constants)(
                command_buffer,
                layout,
                stage_flags,
                offset,
                values.len().try_into().unwrap(),
                values.as_ptr() as _,
            )
        }
    }
    pub unsafe fn create_graphics_pipelines(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_infos: &[GraphicsPipelineCreateInfo<'_>],
        allocator: Option<&AllocationCallbacks<'_>>,
        pipelines: &mut [Pipeline],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.create_graphics_pipelines)(
                device,
                pipeline_cache,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                pipelines.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::PIPELINE_COMPILE_REQUIRED_EXT => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn create_framebuffer(
        &self,
        device: Device,
        create_info: &FramebufferCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<Framebuffer> {
        unsafe {
            let mut framebuffer = core::mem::MaybeUninit::uninit();
            let result = (self.create_framebuffer)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                framebuffer.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(framebuffer.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_framebuffer(
        &self,
        device: Device,
        framebuffer: Framebuffer,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_framebuffer)(device, framebuffer, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_render_pass(
        &self,
        device: Device,
        create_info: &RenderPassCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<RenderPass> {
        unsafe {
            let mut render_pass = core::mem::MaybeUninit::uninit();
            let result = (self.create_render_pass)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                render_pass.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(render_pass.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_render_pass(
        &self,
        device: Device,
        render_pass: RenderPass,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_render_pass)(device, render_pass, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_render_area_granularity(
        &self,
        device: Device,
        render_pass: RenderPass,
    ) -> Extent2D {
        unsafe {
            let mut granularity = core::mem::MaybeUninit::uninit();
            (self.get_render_area_granularity)(device, render_pass, granularity.as_mut_ptr());
            granularity.assume_init()
        }
    }
    pub unsafe fn cmd_set_viewport(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewports: &[Viewport],
    ) {
        unsafe {
            (self.cmd_set_viewport)(
                command_buffer,
                first_viewport,
                viewports.len().try_into().unwrap(),
                viewports.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_scissor(
        &self,
        command_buffer: CommandBuffer,
        first_scissor: u32,
        scissors: &[Rect2D],
    ) {
        unsafe {
            (self.cmd_set_scissor)(
                command_buffer,
                first_scissor,
                scissors.len().try_into().unwrap(),
                scissors.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_line_width(&self, command_buffer: CommandBuffer, line_width: f32) {
        unsafe { (self.cmd_set_line_width)(command_buffer, line_width) }
    }
    pub unsafe fn cmd_set_depth_bias(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) {
        unsafe {
            (self.cmd_set_depth_bias)(
                command_buffer,
                depth_bias_constant_factor,
                depth_bias_clamp,
                depth_bias_slope_factor,
            )
        }
    }
    pub unsafe fn cmd_set_blend_constants(
        &self,
        command_buffer: CommandBuffer,
        blend_constants: &[f32; 4],
    ) {
        unsafe { (self.cmd_set_blend_constants)(command_buffer, blend_constants) }
    }
    pub unsafe fn cmd_set_depth_bounds(
        &self,
        command_buffer: CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) {
        unsafe { (self.cmd_set_depth_bounds)(command_buffer, min_depth_bounds, max_depth_bounds) }
    }
    pub unsafe fn cmd_set_stencil_compare_mask(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: u32,
    ) {
        unsafe { (self.cmd_set_stencil_compare_mask)(command_buffer, face_mask, compare_mask) }
    }
    pub unsafe fn cmd_set_stencil_write_mask(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: u32,
    ) {
        unsafe { (self.cmd_set_stencil_write_mask)(command_buffer, face_mask, write_mask) }
    }
    pub unsafe fn cmd_set_stencil_reference(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: u32,
    ) {
        unsafe { (self.cmd_set_stencil_reference)(command_buffer, face_mask, reference) }
    }
    pub unsafe fn cmd_bind_index_buffer(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        index_type: IndexType,
    ) {
        unsafe { (self.cmd_bind_index_buffer)(command_buffer, buffer, offset, index_type) }
    }
    pub unsafe fn cmd_bind_vertex_buffers(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
    ) {
        unsafe {
            (self.cmd_bind_vertex_buffers)(
                command_buffer,
                first_binding,
                buffers.len().try_into().unwrap(),
                buffers.as_ptr() as _,
                offsets.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_draw(
        &self,
        command_buffer: CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            (self.cmd_draw)(
                command_buffer,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
        }
    }
    pub unsafe fn cmd_draw_indexed(
        &self,
        command_buffer: CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        unsafe {
            (self.cmd_draw_indexed)(
                command_buffer,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            )
        }
    }
    pub unsafe fn cmd_draw_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        unsafe { (self.cmd_draw_indirect)(command_buffer, buffer, offset, draw_count, stride) }
    }
    pub unsafe fn cmd_draw_indexed_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_indexed_indirect)(command_buffer, buffer, offset, draw_count, stride)
        }
    }
    pub unsafe fn cmd_blit_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageBlit],
        filter: Filter,
    ) {
        unsafe {
            (self.cmd_blit_image)(
                command_buffer,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions.len().try_into().unwrap(),
                regions.as_ptr() as _,
                filter,
            )
        }
    }
    pub unsafe fn cmd_clear_depth_stencil_image(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        depth_stencil: &ClearDepthStencilValue,
        ranges: &[ImageSubresourceRange],
    ) {
        unsafe {
            (self.cmd_clear_depth_stencil_image)(
                command_buffer,
                image,
                image_layout,
                depth_stencil,
                ranges.len().try_into().unwrap(),
                ranges.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_clear_attachments(
        &self,
        command_buffer: CommandBuffer,
        attachments: &[ClearAttachment],
        rects: &[ClearRect],
    ) {
        unsafe {
            (self.cmd_clear_attachments)(
                command_buffer,
                attachments.len().try_into().unwrap(),
                attachments.as_ptr() as _,
                rects.len().try_into().unwrap(),
                rects.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_resolve_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageResolve],
    ) {
        unsafe {
            (self.cmd_resolve_image)(
                command_buffer,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions.len().try_into().unwrap(),
                regions.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_begin_render_pass(
        &self,
        command_buffer: CommandBuffer,
        render_pass_begin: &RenderPassBeginInfo<'_>,
        contents: SubpassContents,
    ) {
        unsafe { (self.cmd_begin_render_pass)(command_buffer, render_pass_begin, contents) }
    }
    pub unsafe fn cmd_next_subpass(
        &self,
        command_buffer: CommandBuffer,
        contents: SubpassContents,
    ) {
        unsafe { (self.cmd_next_subpass)(command_buffer, contents) }
    }
    pub unsafe fn cmd_end_render_pass(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_end_render_pass)(command_buffer) }
    }
}
