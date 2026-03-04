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
    handle_nondispatchable!(DebugReportCallbackEXT, DEBUG_REPORT_CALLBACK_EXT, doc = "");
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DebugReportCallbackCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DebugReportFlagsEXT,
        pub pfn_callback: Option<PFN_vkDebugReportCallbackEXT>,
        pub p_user_data: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DebugReportCallbackCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT;
    }
    unsafe impl<'a> Extends<InstanceCreateInfo<'a>> for DebugReportCallbackCreateInfoEXT<'a> {}
    impl Default for DebugReportCallbackCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                pfn_callback: Default::default(),
                p_user_data: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DebugReportCallbackCreateInfoEXT<'a> {
        pub fn flags(mut self, flags: DebugReportFlagsEXT) -> Self {
            self.flags = flags;
            self
        }
        pub fn pfn_callback(mut self, pfn_callback: PFN_vkDebugReportCallbackEXT) -> Self {
            self.pfn_callback = Some(pfn_callback);
            self
        }
        pub fn user_data(mut self, user_data: &'a mut c_void) -> Self {
            self.p_user_data = user_data;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DebugReportObjectTypeEXT(i32);
    impl DebugReportObjectTypeEXT {
        pub const UNKNOWN_EXT: Self = Self(0);
        pub const INSTANCE_EXT: Self = Self(1);
        pub const PHYSICAL_DEVICE_EXT: Self = Self(2);
        pub const DEVICE_EXT: Self = Self(3);
        pub const QUEUE_EXT: Self = Self(4);
        pub const SEMAPHORE_EXT: Self = Self(5);
        pub const COMMAND_BUFFER_EXT: Self = Self(6);
        pub const FENCE_EXT: Self = Self(7);
        pub const DEVICE_MEMORY_EXT: Self = Self(8);
        pub const BUFFER_EXT: Self = Self(9);
        pub const IMAGE_EXT: Self = Self(10);
        pub const EVENT_EXT: Self = Self(11);
        pub const QUERY_POOL_EXT: Self = Self(12);
        pub const BUFFER_VIEW_EXT: Self = Self(13);
        pub const IMAGE_VIEW_EXT: Self = Self(14);
        pub const SHADER_MODULE_EXT: Self = Self(15);
        pub const PIPELINE_CACHE_EXT: Self = Self(16);
        pub const PIPELINE_LAYOUT_EXT: Self = Self(17);
        pub const RENDER_PASS_EXT: Self = Self(18);
        pub const PIPELINE_EXT: Self = Self(19);
        pub const DESCRIPTOR_SET_LAYOUT_EXT: Self = Self(20);
        pub const SAMPLER_EXT: Self = Self(21);
        pub const DESCRIPTOR_POOL_EXT: Self = Self(22);
        pub const DESCRIPTOR_SET_EXT: Self = Self(23);
        pub const FRAMEBUFFER_EXT: Self = Self(24);
        pub const COMMAND_POOL_EXT: Self = Self(25);
        pub const SURFACE_KHR_EXT: Self = Self(26);
        pub const SWAPCHAIN_KHR_EXT: Self = Self(27);
        pub const DEBUG_REPORT_CALLBACK_EXT_EXT: Self = Self(28);
        pub const DISPLAY_KHR_EXT: Self = Self(29);
        pub const DISPLAY_MODE_KHR_EXT: Self = Self(30);
        pub const VALIDATION_CACHE_EXT_EXT: Self = Self(33);
        pub const ACCELERATION_STRUCTURE_KHR_EXT: Self = Self(1000150000);
        pub const ACCELERATION_STRUCTURE_NV_EXT: Self = Self(1000165000);
        pub const BUFFER_COLLECTION_FUCHSIA_EXT: Self = Self(1000366000);
        pub const CUDA_FUNCTION_NV_EXT: Self = Self(1000307001);
        pub const CUDA_MODULE_NV_EXT: Self = Self(1000307000);
        pub const CU_FUNCTION_NVX_EXT: Self = Self(1000029001);
        pub const CU_MODULE_NVX_EXT: Self = Self(1000029000);
        pub const DESCRIPTOR_UPDATE_TEMPLATE_EXT: Self = Self(1000085000);
        pub const SAMPLER_YCBCR_CONVERSION_EXT: Self = Self(1000156000);
        pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT: Self = Self::DESCRIPTOR_UPDATE_TEMPLATE_EXT;
        pub const SAMPLER_YCBCR_CONVERSION_KHR_EXT: Self = Self::SAMPLER_YCBCR_CONVERSION_EXT;
    }
    impl fmt::Debug for DebugReportObjectTypeEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::UNKNOWN_EXT => Some("UNKNOWN_EXT"),
                Self::INSTANCE_EXT => Some("INSTANCE_EXT"),
                Self::PHYSICAL_DEVICE_EXT => Some("PHYSICAL_DEVICE_EXT"),
                Self::DEVICE_EXT => Some("DEVICE_EXT"),
                Self::QUEUE_EXT => Some("QUEUE_EXT"),
                Self::SEMAPHORE_EXT => Some("SEMAPHORE_EXT"),
                Self::COMMAND_BUFFER_EXT => Some("COMMAND_BUFFER_EXT"),
                Self::FENCE_EXT => Some("FENCE_EXT"),
                Self::DEVICE_MEMORY_EXT => Some("DEVICE_MEMORY_EXT"),
                Self::BUFFER_EXT => Some("BUFFER_EXT"),
                Self::IMAGE_EXT => Some("IMAGE_EXT"),
                Self::EVENT_EXT => Some("EVENT_EXT"),
                Self::QUERY_POOL_EXT => Some("QUERY_POOL_EXT"),
                Self::BUFFER_VIEW_EXT => Some("BUFFER_VIEW_EXT"),
                Self::IMAGE_VIEW_EXT => Some("IMAGE_VIEW_EXT"),
                Self::SHADER_MODULE_EXT => Some("SHADER_MODULE_EXT"),
                Self::PIPELINE_CACHE_EXT => Some("PIPELINE_CACHE_EXT"),
                Self::PIPELINE_LAYOUT_EXT => Some("PIPELINE_LAYOUT_EXT"),
                Self::RENDER_PASS_EXT => Some("RENDER_PASS_EXT"),
                Self::PIPELINE_EXT => Some("PIPELINE_EXT"),
                Self::DESCRIPTOR_SET_LAYOUT_EXT => Some("DESCRIPTOR_SET_LAYOUT_EXT"),
                Self::SAMPLER_EXT => Some("SAMPLER_EXT"),
                Self::DESCRIPTOR_POOL_EXT => Some("DESCRIPTOR_POOL_EXT"),
                Self::DESCRIPTOR_SET_EXT => Some("DESCRIPTOR_SET_EXT"),
                Self::FRAMEBUFFER_EXT => Some("FRAMEBUFFER_EXT"),
                Self::COMMAND_POOL_EXT => Some("COMMAND_POOL_EXT"),
                Self::SURFACE_KHR_EXT => Some("SURFACE_KHR_EXT"),
                Self::SWAPCHAIN_KHR_EXT => Some("SWAPCHAIN_KHR_EXT"),
                Self::DEBUG_REPORT_CALLBACK_EXT_EXT => Some("DEBUG_REPORT_CALLBACK_EXT_EXT"),
                Self::DISPLAY_KHR_EXT => Some("DISPLAY_KHR_EXT"),
                Self::DISPLAY_MODE_KHR_EXT => Some("DISPLAY_MODE_KHR_EXT"),
                Self::VALIDATION_CACHE_EXT_EXT => Some("VALIDATION_CACHE_EXT_EXT"),
                Self::ACCELERATION_STRUCTURE_KHR_EXT => Some("ACCELERATION_STRUCTURE_KHR_EXT"),
                Self::ACCELERATION_STRUCTURE_NV_EXT => Some("ACCELERATION_STRUCTURE_NV_EXT"),
                Self::BUFFER_COLLECTION_FUCHSIA_EXT => Some("BUFFER_COLLECTION_FUCHSIA_EXT"),
                Self::CUDA_FUNCTION_NV_EXT => Some("CUDA_FUNCTION_NV_EXT"),
                Self::CUDA_MODULE_NV_EXT => Some("CUDA_MODULE_NV_EXT"),
                Self::CU_FUNCTION_NVX_EXT => Some("CU_FUNCTION_NVX_EXT"),
                Self::CU_MODULE_NVX_EXT => Some("CU_MODULE_NVX_EXT"),
                Self::DESCRIPTOR_UPDATE_TEMPLATE_EXT => Some("DESCRIPTOR_UPDATE_TEMPLATE_EXT"),
                Self::SAMPLER_YCBCR_CONVERSION_EXT => Some("SAMPLER_YCBCR_CONVERSION_EXT"),
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
    pub struct DebugReportFlagsEXT(Flags);
    vk_bitflags_wrapped!(DebugReportFlagsEXT, Flags);
    impl DebugReportFlagsEXT {
        pub const INFORMATION_EXT: Self = Self(DebugReportFlagBitsEXT::INFORMATION_EXT.0);
        pub const WARNING_EXT: Self = Self(DebugReportFlagBitsEXT::WARNING_EXT.0);
        pub const PERFORMANCE_WARNING_EXT: Self =
            Self(DebugReportFlagBitsEXT::PERFORMANCE_WARNING_EXT.0);
        pub const ERROR_EXT: Self = Self(DebugReportFlagBitsEXT::ERROR_EXT.0);
        pub const DEBUG_EXT: Self = Self(DebugReportFlagBitsEXT::DEBUG_EXT.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DebugReportFlagBitsEXT(u32);
    impl DebugReportFlagBitsEXT {
        pub const INFORMATION_EXT: Self = Self(1 << 0);
        pub const WARNING_EXT: Self = Self(1 << 1);
        pub const PERFORMANCE_WARNING_EXT: Self = Self(1 << 2);
        pub const ERROR_EXT: Self = Self(1 << 3);
        pub const DEBUG_EXT: Self = Self(1 << 4);
    }
    pub type PFN_vkDebugReportCallbackEXT = unsafe extern "system" fn(
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        p_layer_prefix: *const c_char,
        p_message: *const c_char,
        p_user_data: *mut c_void,
    ) -> Bool32;
    pub type PFN_vkCreateDebugReportCallbackEXT = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const DebugReportCallbackCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_callback: *mut DebugReportCallbackEXT,
    ) -> vk::Result;
    pub type PFN_vkDestroyDebugReportCallbackEXT = unsafe extern "system" fn(
        instance: Instance,
        callback: DebugReportCallbackEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkDebugReportMessageEXT = unsafe extern "system" fn(
        instance: Instance,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        p_layer_prefix: *const c_char,
        p_message: *const c_char,
    );
}
pub struct InstanceFn {
    create_debug_report_callback_ext: PFN_vkCreateDebugReportCallbackEXT,
    destroy_debug_report_callback_ext: PFN_vkDestroyDebugReportCallbackEXT,
    debug_report_message_ext: PFN_vkDebugReportMessageEXT,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_debug_report_callback_ext: transmute(
                    load(c"vkCreateDebugReportCallbackEXT").ok_or(MissingEntryPointError)?,
                ),
                destroy_debug_report_callback_ext: transmute(
                    load(c"vkDestroyDebugReportCallbackEXT").ok_or(MissingEntryPointError)?,
                ),
                debug_report_message_ext: transmute(
                    load(c"vkDebugReportMessageEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn create_debug_report_callback_ext(
        &self,
        instance: Instance,
        create_info: &DebugReportCallbackCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<DebugReportCallbackEXT> {
        unsafe {
            let mut callback = core::mem::MaybeUninit::uninit();
            let result = (self.create_debug_report_callback_ext)(
                instance,
                create_info,
                allocator.to_raw_ptr(),
                callback.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(callback.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_debug_report_callback_ext(
        &self,
        instance: Instance,
        callback: DebugReportCallbackEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_debug_report_callback_ext)(instance, callback, allocator.to_raw_ptr())
        }
    }
    pub unsafe fn debug_report_message_ext(
        &self,
        instance: Instance,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        layer_prefix: &CStr,
        message: &CStr,
    ) {
        unsafe {
            (self.debug_report_message_ext)(
                instance,
                flags,
                object_type,
                object,
                location,
                message_code,
                layer_prefix.as_ptr() as _,
                message.as_ptr() as _,
            )
        }
    }
}
