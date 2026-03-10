//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_ARM_shader_instrumentation.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_ARM_shader_instrumentation";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    handle_nondispatchable!(
        ShaderInstrumentationARM,
        SHADER_INSTRUMENTATION_ARM,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkShaderInstrumentationARM.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderInstrumentationFeaturesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderInstrumentationFeaturesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_instrumentation: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderInstrumentationFeaturesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderInstrumentationFeaturesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_instrumentation", &self.shader_instrumentation)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderInstrumentationFeaturesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_FEATURES_ARM;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceShaderInstrumentationFeaturesARM<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceShaderInstrumentationFeaturesARM<'_> {}

    impl Default for PhysicalDeviceShaderInstrumentationFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_instrumentation: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderInstrumentationFeaturesARM<'a> {
        #[inline]
        pub fn shader_instrumentation(mut self, shader_instrumentation: bool) -> Self {
            self.shader_instrumentation = shader_instrumentation.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderInstrumentationPropertiesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderInstrumentationPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub num_metrics: u32,
        pub per_basic_block_granularity: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderInstrumentationPropertiesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderInstrumentationPropertiesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("num_metrics", &self.num_metrics)
                .field(
                    "per_basic_block_granularity",
                    &self.per_basic_block_granularity,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderInstrumentationPropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_PROPERTIES_ARM;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceShaderInstrumentationPropertiesARM<'_>
    {
    }

    impl Default for PhysicalDeviceShaderInstrumentationPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                num_metrics: Default::default(),
                per_basic_block_granularity: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderInstrumentationPropertiesARM<'a> {
        #[inline]
        pub fn num_metrics(mut self, num_metrics: u32) -> Self {
            self.num_metrics = num_metrics;
            self
        }

        #[inline]
        pub fn per_basic_block_granularity(mut self, per_basic_block_granularity: bool) -> Self {
            self.per_basic_block_granularity = per_basic_block_granularity.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkShaderInstrumentationCreateInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ShaderInstrumentationCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ShaderInstrumentationCreateInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ShaderInstrumentationCreateInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ShaderInstrumentationCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SHADER_INSTRUMENTATION_CREATE_INFO_ARM;
    }

    impl Default for ShaderInstrumentationCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ShaderInstrumentationCreateInfoARM<'a> {}

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkShaderInstrumentationMetricDescriptionARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ShaderInstrumentationMetricDescriptionARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
        pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ShaderInstrumentationMetricDescriptionARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ShaderInstrumentationMetricDescriptionARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("name", &wrap_c_str_slice_until_nul(&self.name))
                .field(
                    "description",
                    &wrap_c_str_slice_until_nul(&self.description),
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ShaderInstrumentationMetricDescriptionARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SHADER_INSTRUMENTATION_METRIC_DESCRIPTION_ARM;
    }

    impl Default for ShaderInstrumentationMetricDescriptionARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                name: [Default::default(); _],
                description: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ShaderInstrumentationMetricDescriptionARM<'a> {
        #[inline]
        pub fn name(
            mut self,
            name: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            write_c_str_slice_with_nul(&mut self.name, name)?;
            Ok(self)
        }

        #[inline]
        pub fn description(
            mut self,
            description: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            write_c_str_slice_with_nul(&mut self.description, description)?;
            Ok(self)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkShaderInstrumentationMetricDataHeaderARM.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct ShaderInstrumentationMetricDataHeaderARM {
        pub result_index: u32,
        pub result_sub_index: u32,
        pub stages: ShaderStageFlags,
        pub basic_block_index: u32,
    }

    impl ShaderInstrumentationMetricDataHeaderARM {
        #[inline]
        pub fn result_index(mut self, result_index: u32) -> Self {
            self.result_index = result_index;
            self
        }

        #[inline]
        pub fn result_sub_index(mut self, result_sub_index: u32) -> Self {
            self.result_sub_index = result_sub_index;
            self
        }

        #[inline]
        pub fn stages(mut self, stages: ShaderStageFlags) -> Self {
            self.stages = stages;
            self
        }

        #[inline]
        pub fn basic_block_index(mut self, basic_block_index: u32) -> Self {
            self.basic_block_index = basic_block_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkShaderInstrumentationValuesFlagsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ShaderInstrumentationValuesFlagsARM(Flags);
    vk_bitflags_wrapped!(ShaderInstrumentationValuesFlagsARM, Flags);

    impl fmt::Debug for ShaderInstrumentationValuesFlagsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM.html>
    pub type PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_description_count: *mut u32,
            p_descriptions: *mut ShaderInstrumentationMetricDescriptionARM<'_>,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateShaderInstrumentationARM.html>
    pub type PFN_vkCreateShaderInstrumentationARM = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ShaderInstrumentationCreateInfoARM<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_instrumentation: *mut ShaderInstrumentationARM,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyShaderInstrumentationARM.html>
    pub type PFN_vkDestroyShaderInstrumentationARM = unsafe extern "system" fn(
        device: Device,
        instrumentation: ShaderInstrumentationARM,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginShaderInstrumentationARM.html>
    pub type PFN_vkCmdBeginShaderInstrumentationARM = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        instrumentation: ShaderInstrumentationARM,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndShaderInstrumentationARM.html>
    pub type PFN_vkCmdEndShaderInstrumentationARM =
        unsafe extern "system" fn(command_buffer: CommandBuffer);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetShaderInstrumentationValuesARM.html>
    pub type PFN_vkGetShaderInstrumentationValuesARM = unsafe extern "system" fn(
        device: Device,
        instrumentation: ShaderInstrumentationARM,
        p_metric_block_count: *mut u32,
        p_metric_values: *mut c_void,
        flags: ShaderInstrumentationValuesFlagsARM,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkClearShaderInstrumentationMetricsARM.html>
    pub type PFN_vkClearShaderInstrumentationMetricsARM =
        unsafe extern "system" fn(device: Device, instrumentation: ShaderInstrumentationARM);
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkShaderInstrumentationARM = ShaderInstrumentationARM;
    pub type VkPhysicalDeviceShaderInstrumentationFeaturesARM =
        PhysicalDeviceShaderInstrumentationFeaturesARM<'static>;
    pub type VkPhysicalDeviceShaderInstrumentationPropertiesARM =
        PhysicalDeviceShaderInstrumentationPropertiesARM<'static>;
    pub type VkShaderInstrumentationCreateInfoARM = ShaderInstrumentationCreateInfoARM<'static>;
    pub type VkShaderInstrumentationMetricDescriptionARM =
        ShaderInstrumentationMetricDescriptionARM<'static>;
    pub type VkShaderInstrumentationMetricDataHeaderARM = ShaderInstrumentationMetricDataHeaderARM;
    pub type VkShaderInstrumentationValuesFlagsARM = ShaderInstrumentationValuesFlagsARM;
    impl PhysicalDeviceShaderInstrumentationFeaturesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderInstrumentationFeaturesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceShaderInstrumentationPropertiesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderInstrumentationPropertiesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ShaderInstrumentationCreateInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkShaderInstrumentationCreateInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ShaderInstrumentationMetricDescriptionARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkShaderInstrumentationMetricDescriptionARM {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    enumerate_physical_device_shader_instrumentation_metrics_arm:
        PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                enumerate_physical_device_shader_instrumentation_metrics_arm: transmute(
                    load(c"vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM.html>
    #[inline]
    pub unsafe fn enumerate_physical_device_shader_instrumentation_metrics_arm<'a>(
        &self,
        physical_device: PhysicalDevice,
        mut descriptions: impl ExtendUninit<ShaderInstrumentationMetricDescriptionARM<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |description_count, descriptions| {
                let result = (self.enumerate_physical_device_shader_instrumentation_metrics_arm)(
                    physical_device,
                    description_count,
                    descriptions as _,
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
            let descriptions_buf = descriptions.reserve(capacity);
            len = descriptions_buf.len().try_into().unwrap();
            let result = call(&mut len, descriptions_buf.as_mut_ptr() as *mut _)?;
            descriptions.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}

pub struct DeviceFn {
    create_shader_instrumentation_arm: PFN_vkCreateShaderInstrumentationARM,
    destroy_shader_instrumentation_arm: PFN_vkDestroyShaderInstrumentationARM,
    cmd_begin_shader_instrumentation_arm: PFN_vkCmdBeginShaderInstrumentationARM,
    cmd_end_shader_instrumentation_arm: PFN_vkCmdEndShaderInstrumentationARM,
    get_shader_instrumentation_values_arm: PFN_vkGetShaderInstrumentationValuesARM,
    clear_shader_instrumentation_metrics_arm: PFN_vkClearShaderInstrumentationMetricsARM,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_shader_instrumentation_arm: transmute(
                    load(c"vkCreateShaderInstrumentationARM").ok_or(MissingEntryPointError)?,
                ),
                destroy_shader_instrumentation_arm: transmute(
                    load(c"vkDestroyShaderInstrumentationARM").ok_or(MissingEntryPointError)?,
                ),
                cmd_begin_shader_instrumentation_arm: transmute(
                    load(c"vkCmdBeginShaderInstrumentationARM").ok_or(MissingEntryPointError)?,
                ),
                cmd_end_shader_instrumentation_arm: transmute(
                    load(c"vkCmdEndShaderInstrumentationARM").ok_or(MissingEntryPointError)?,
                ),
                get_shader_instrumentation_values_arm: transmute(
                    load(c"vkGetShaderInstrumentationValuesARM").ok_or(MissingEntryPointError)?,
                ),
                clear_shader_instrumentation_metrics_arm: transmute(
                    load(c"vkClearShaderInstrumentationMetricsARM")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateShaderInstrumentationARM.html>
    #[inline]
    pub unsafe fn create_shader_instrumentation_arm(
        &self,
        device: Device,
        create_info: &ShaderInstrumentationCreateInfoARM<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<ShaderInstrumentationARM> {
        unsafe {
            let mut instrumentation = core::mem::MaybeUninit::uninit();
            let result = (self.create_shader_instrumentation_arm)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                instrumentation.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(instrumentation.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyShaderInstrumentationARM.html>
    #[inline]
    pub unsafe fn destroy_shader_instrumentation_arm(
        &self,
        device: Device,
        instrumentation: ShaderInstrumentationARM,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_shader_instrumentation_arm)(
                device,
                instrumentation,
                allocator.to_raw_ptr(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginShaderInstrumentationARM.html>
    #[inline]
    pub unsafe fn cmd_begin_shader_instrumentation_arm(
        &self,
        command_buffer: CommandBuffer,
        instrumentation: ShaderInstrumentationARM,
    ) {
        unsafe { (self.cmd_begin_shader_instrumentation_arm)(command_buffer, instrumentation) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndShaderInstrumentationARM.html>
    #[inline]
    pub unsafe fn cmd_end_shader_instrumentation_arm(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_end_shader_instrumentation_arm)(command_buffer) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetShaderInstrumentationValuesARM.html>
    #[inline]
    pub unsafe fn get_shader_instrumentation_values_arm(
        &self,
        device: Device,
        instrumentation: ShaderInstrumentationARM,
        metric_values: *mut c_void,
        flags: ShaderInstrumentationValuesFlagsARM,
    ) -> crate::Result<u32> {
        unsafe {
            let mut metric_block_count = core::mem::MaybeUninit::uninit();
            let result = (self.get_shader_instrumentation_values_arm)(
                device,
                instrumentation,
                metric_block_count.as_mut_ptr(),
                metric_values,
                flags,
            );

            match result {
                VkResult::SUCCESS => Ok(metric_block_count.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkClearShaderInstrumentationMetricsARM.html>
    #[inline]
    pub unsafe fn clear_shader_instrumentation_metrics_arm(
        &self,
        device: Device,
        instrumentation: ShaderInstrumentationARM,
    ) {
        unsafe { (self.clear_shader_instrumentation_metrics_arm)(device, instrumentation) }
    }
}
