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
    pub struct SubpassShadingPipelineCreateInfoHUAWEI<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub render_pass: RenderPass,
        pub subpass: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for SubpassShadingPipelineCreateInfoHUAWEI<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI,
                p_next: core::ptr::null_mut(),
                render_pass: Default::default(),
                subpass: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SubpassShadingPipelineCreateInfoHUAWEI<'a> {
        pub fn render_pass(mut self, render_pass: RenderPass) -> Self {
            self.render_pass = render_pass;
            self
        }
        pub fn subpass(mut self, subpass: u32) -> Self {
            self.subpass = subpass;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceSubpassShadingPropertiesHUAWEI<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_subpass_shading_workgroup_size_aspect_ratio: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceSubpassShadingPropertiesHUAWEI<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI,
                p_next: core::ptr::null_mut(),
                max_subpass_shading_workgroup_size_aspect_ratio: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceSubpassShadingPropertiesHUAWEI<'a> {
        pub fn max_subpass_shading_workgroup_size_aspect_ratio(
            mut self,
            max_subpass_shading_workgroup_size_aspect_ratio: u32,
        ) -> Self {
            self.max_subpass_shading_workgroup_size_aspect_ratio =
                max_subpass_shading_workgroup_size_aspect_ratio;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceSubpassShadingFeaturesHUAWEI<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub subpass_shading: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceSubpassShadingFeaturesHUAWEI<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI,
                p_next: core::ptr::null_mut(),
                subpass_shading: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceSubpassShadingFeaturesHUAWEI<'a> {
        pub fn subpass_shading(mut self, subpass_shading: Bool32) -> Self {
            self.subpass_shading = subpass_shading;
            self
        }
    }
    pub type PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI =
        unsafe extern "system" fn(
            device: Device,
            renderpass: RenderPass,
            p_max_workgroup_size: *mut Extent2D,
        ) -> vk::Result;
    pub type PFN_vkCmdSubpassShadingHUAWEI =
        unsafe extern "system" fn(command_buffer: CommandBuffer);
}
pub struct DeviceFn {
    get_device_subpass_shading_max_workgroup_size_huawei:
        PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI,
    cmd_subpass_shading_huawei: PFN_vkCmdSubpassShadingHUAWEI,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_device_subpass_shading_max_workgroup_size_huawei: transmute(
                    load(c"vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI").ok_or(LoadingError)?,
                ),
                cmd_subpass_shading_huawei: transmute(
                    load(c"vkCmdSubpassShadingHUAWEI").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_device_subpass_shading_max_workgroup_size_huawei(
        &self,
        device: Device,
        renderpass: RenderPass,
        max_workgroup_size: &mut Extent2D,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_device_subpass_shading_max_workgroup_size_huawei)(
                device,
                renderpass,
                max_workgroup_size,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn cmd_subpass_shading_huawei(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_subpass_shading_huawei)(command_buffer) }
    }
}
