//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_HUAWEI_subpass_shading.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_HUAWEI_subpass_shading";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubpassShadingPipelineCreateInfoHUAWEI.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SubpassShadingPipelineCreateInfoHUAWEI<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub render_pass: RenderPass,
        pub subpass: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SubpassShadingPipelineCreateInfoHUAWEI<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SubpassShadingPipelineCreateInfoHUAWEI")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("render_pass", &self.render_pass)
                .field("subpass", &self.subpass)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SubpassShadingPipelineCreateInfoHUAWEI<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI;
    }

    unsafe impl Extends<ComputePipelineCreateInfo<'_>> for SubpassShadingPipelineCreateInfoHUAWEI<'_> {}

    impl Default for SubpassShadingPipelineCreateInfoHUAWEI<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                render_pass: Default::default(),
                subpass: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SubpassShadingPipelineCreateInfoHUAWEI<'a> {
        #[inline]
        pub fn render_pass(mut self, render_pass: RenderPass) -> Self {
            self.render_pass = render_pass;
            self
        }

        #[inline]
        pub fn subpass(mut self, subpass: u32) -> Self {
            self.subpass = subpass;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSubpassShadingPropertiesHUAWEI.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceSubpassShadingPropertiesHUAWEI<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_subpass_shading_workgroup_size_aspect_ratio: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceSubpassShadingPropertiesHUAWEI<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceSubpassShadingPropertiesHUAWEI")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_subpass_shading_workgroup_size_aspect_ratio",
                    &self.max_subpass_shading_workgroup_size_aspect_ratio,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSubpassShadingPropertiesHUAWEI<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceSubpassShadingPropertiesHUAWEI<'_>
    {
    }

    impl Default for PhysicalDeviceSubpassShadingPropertiesHUAWEI<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_subpass_shading_workgroup_size_aspect_ratio: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceSubpassShadingPropertiesHUAWEI<'a> {
        #[inline]
        pub fn max_subpass_shading_workgroup_size_aspect_ratio(
            mut self,
            max_subpass_shading_workgroup_size_aspect_ratio: u32,
        ) -> Self {
            self.max_subpass_shading_workgroup_size_aspect_ratio =
                max_subpass_shading_workgroup_size_aspect_ratio;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSubpassShadingFeaturesHUAWEI.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceSubpassShadingFeaturesHUAWEI<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub subpass_shading: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceSubpassShadingFeaturesHUAWEI<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceSubpassShadingFeaturesHUAWEI")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("subpass_shading", &self.subpass_shading)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSubpassShadingFeaturesHUAWEI<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceSubpassShadingFeaturesHUAWEI<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceSubpassShadingFeaturesHUAWEI<'_> {}

    impl Default for PhysicalDeviceSubpassShadingFeaturesHUAWEI<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                subpass_shading: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceSubpassShadingFeaturesHUAWEI<'a> {
        #[inline]
        pub fn subpass_shading(mut self, subpass_shading: bool) -> Self {
            self.subpass_shading = subpass_shading.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html>
    pub type PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI =
        unsafe extern "system" fn(
            device: Device,
            renderpass: RenderPass,
            p_max_workgroup_size: *mut Extent2D,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSubpassShadingHUAWEI.html>
    pub type PFN_vkCmdSubpassShadingHUAWEI =
        unsafe extern "system" fn(command_buffer: CommandBuffer);
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkSubpassShadingPipelineCreateInfoHUAWEI =
        SubpassShadingPipelineCreateInfoHUAWEI<'static>;
    pub type VkPhysicalDeviceSubpassShadingPropertiesHUAWEI =
        PhysicalDeviceSubpassShadingPropertiesHUAWEI<'static>;
    pub type VkPhysicalDeviceSubpassShadingFeaturesHUAWEI =
        PhysicalDeviceSubpassShadingFeaturesHUAWEI<'static>;
    impl SubpassShadingPipelineCreateInfoHUAWEI<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSubpassShadingPipelineCreateInfoHUAWEI {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceSubpassShadingPropertiesHUAWEI<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceSubpassShadingPropertiesHUAWEI {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceSubpassShadingFeaturesHUAWEI<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceSubpassShadingFeaturesHUAWEI {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_device_subpass_shading_max_workgroup_size:
        PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI,
    cmd_subpass_shading: PFN_vkCmdSubpassShadingHUAWEI,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_device_subpass_shading_max_workgroup_size: transmute(
                    load(c"vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")
                        .ok_or(MissingEntryPointError)?,
                ),
                cmd_subpass_shading: transmute(
                    load(c"vkCmdSubpassShadingHUAWEI").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html>
    #[inline]
    pub unsafe fn get_device_subpass_shading_max_workgroup_size(
        &self,
        device: Device,
        renderpass: RenderPass,
        max_workgroup_size: &mut Extent2D,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_device_subpass_shading_max_workgroup_size)(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSubpassShadingHUAWEI.html>
    #[inline]
    pub unsafe fn cmd_subpass_shading(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_subpass_shading)(command_buffer) }
    }
}
