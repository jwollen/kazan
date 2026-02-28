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
    #[derive(Copy, Clone, Default)]
    pub struct OpticalFlowSessionNV(u64);
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceOpticalFlowFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub optical_flow: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceOpticalFlowFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV,
                p_next: core::ptr::null_mut(),
                optical_flow: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceOpticalFlowFeaturesNV<'a> {
        pub fn optical_flow(mut self, optical_flow: Bool32) -> Self {
            self.optical_flow = optical_flow;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceOpticalFlowPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub supported_output_grid_sizes: OpticalFlowGridSizeFlagsNV,
        pub supported_hint_grid_sizes: OpticalFlowGridSizeFlagsNV,
        pub hint_supported: Bool32,
        pub cost_supported: Bool32,
        pub bidirectional_flow_supported: Bool32,
        pub global_flow_supported: Bool32,
        pub min_width: u32,
        pub min_height: u32,
        pub max_width: u32,
        pub max_height: u32,
        pub max_num_regions_of_interest: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceOpticalFlowPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV,
                p_next: core::ptr::null_mut(),
                supported_output_grid_sizes: Default::default(),
                supported_hint_grid_sizes: Default::default(),
                hint_supported: Default::default(),
                cost_supported: Default::default(),
                bidirectional_flow_supported: Default::default(),
                global_flow_supported: Default::default(),
                min_width: Default::default(),
                min_height: Default::default(),
                max_width: Default::default(),
                max_height: Default::default(),
                max_num_regions_of_interest: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceOpticalFlowPropertiesNV<'a> {
        pub fn supported_output_grid_sizes(
            mut self,
            supported_output_grid_sizes: OpticalFlowGridSizeFlagsNV,
        ) -> Self {
            self.supported_output_grid_sizes = supported_output_grid_sizes;
            self
        }
        pub fn supported_hint_grid_sizes(
            mut self,
            supported_hint_grid_sizes: OpticalFlowGridSizeFlagsNV,
        ) -> Self {
            self.supported_hint_grid_sizes = supported_hint_grid_sizes;
            self
        }
        pub fn hint_supported(mut self, hint_supported: Bool32) -> Self {
            self.hint_supported = hint_supported;
            self
        }
        pub fn cost_supported(mut self, cost_supported: Bool32) -> Self {
            self.cost_supported = cost_supported;
            self
        }
        pub fn bidirectional_flow_supported(
            mut self,
            bidirectional_flow_supported: Bool32,
        ) -> Self {
            self.bidirectional_flow_supported = bidirectional_flow_supported;
            self
        }
        pub fn global_flow_supported(mut self, global_flow_supported: Bool32) -> Self {
            self.global_flow_supported = global_flow_supported;
            self
        }
        pub fn min_width(mut self, min_width: u32) -> Self {
            self.min_width = min_width;
            self
        }
        pub fn min_height(mut self, min_height: u32) -> Self {
            self.min_height = min_height;
            self
        }
        pub fn max_width(mut self, max_width: u32) -> Self {
            self.max_width = max_width;
            self
        }
        pub fn max_height(mut self, max_height: u32) -> Self {
            self.max_height = max_height;
            self
        }
        pub fn max_num_regions_of_interest(mut self, max_num_regions_of_interest: u32) -> Self {
            self.max_num_regions_of_interest = max_num_regions_of_interest;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct OpticalFlowImageFormatInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub usage: OpticalFlowUsageFlagsNV,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for OpticalFlowImageFormatInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV,
                p_next: core::ptr::null(),
                usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> OpticalFlowImageFormatInfoNV<'a> {
        pub fn usage(mut self, usage: OpticalFlowUsageFlagsNV) -> Self {
            self.usage = usage;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct OpticalFlowImageFormatPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub format: Format,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for OpticalFlowImageFormatPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV,
                p_next: core::ptr::null_mut(),
                format: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> OpticalFlowImageFormatPropertiesNV<'a> {
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct OpticalFlowSessionCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub width: u32,
        pub height: u32,
        pub image_format: Format,
        pub flow_vector_format: Format,
        pub cost_format: Format,
        pub output_grid_size: OpticalFlowGridSizeFlagsNV,
        pub hint_grid_size: OpticalFlowGridSizeFlagsNV,
        pub performance_level: OpticalFlowPerformanceLevelNV,
        pub flags: OpticalFlowSessionCreateFlagsNV,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for OpticalFlowSessionCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::OPTICAL_FLOW_SESSION_CREATE_INFO_NV,
                p_next: core::ptr::null_mut(),
                width: Default::default(),
                height: Default::default(),
                image_format: Default::default(),
                flow_vector_format: Default::default(),
                cost_format: Default::default(),
                output_grid_size: Default::default(),
                hint_grid_size: Default::default(),
                performance_level: Default::default(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> OpticalFlowSessionCreateInfoNV<'a> {
        pub fn width(mut self, width: u32) -> Self {
            self.width = width;
            self
        }
        pub fn height(mut self, height: u32) -> Self {
            self.height = height;
            self
        }
        pub fn image_format(mut self, image_format: Format) -> Self {
            self.image_format = image_format;
            self
        }
        pub fn flow_vector_format(mut self, flow_vector_format: Format) -> Self {
            self.flow_vector_format = flow_vector_format;
            self
        }
        pub fn cost_format(mut self, cost_format: Format) -> Self {
            self.cost_format = cost_format;
            self
        }
        pub fn output_grid_size(mut self, output_grid_size: OpticalFlowGridSizeFlagsNV) -> Self {
            self.output_grid_size = output_grid_size;
            self
        }
        pub fn hint_grid_size(mut self, hint_grid_size: OpticalFlowGridSizeFlagsNV) -> Self {
            self.hint_grid_size = hint_grid_size;
            self
        }
        pub fn performance_level(
            mut self,
            performance_level: OpticalFlowPerformanceLevelNV,
        ) -> Self {
            self.performance_level = performance_level;
            self
        }
        pub fn flags(mut self, flags: OpticalFlowSessionCreateFlagsNV) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct OpticalFlowSessionCreatePrivateDataInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub id: u32,
        pub size: u32,
        pub p_private_data: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for OpticalFlowSessionCreatePrivateDataInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV,
                p_next: core::ptr::null_mut(),
                id: Default::default(),
                size: Default::default(),
                p_private_data: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> OpticalFlowSessionCreatePrivateDataInfoNV<'a> {
        pub fn id(mut self, id: u32) -> Self {
            self.id = id;
            self
        }
        pub fn size(mut self, size: u32) -> Self {
            self.size = size;
            self
        }
        pub fn private_data(mut self, private_data: &'a c_void) -> Self {
            self.p_private_data = private_data;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct OpticalFlowExecuteInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: OpticalFlowExecuteFlagsNV,
        pub region_count: u32,
        pub p_regions: *const Rect2D,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for OpticalFlowExecuteInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::OPTICAL_FLOW_EXECUTE_INFO_NV,
                p_next: core::ptr::null_mut(),
                flags: Default::default(),
                region_count: Default::default(),
                p_regions: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> OpticalFlowExecuteInfoNV<'a> {
        pub fn flags(mut self, flags: OpticalFlowExecuteFlagsNV) -> Self {
            self.flags = flags;
            self
        }
        pub fn regions(mut self, regions: &'a [Rect2D]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr();
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OpticalFlowPerformanceLevelNV(i32);
    impl OpticalFlowPerformanceLevelNV {
        pub const UNKNOWN_NV: Self = Self(0);
        pub const SLOW_NV: Self = Self(1);
        pub const MEDIUM_NV: Self = Self(2);
        pub const FAST_NV: Self = Self(3);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OpticalFlowSessionBindingPointNV(i32);
    impl OpticalFlowSessionBindingPointNV {
        pub const UNKNOWN_NV: Self = Self(0);
        pub const INPUT_NV: Self = Self(1);
        pub const REFERENCE_NV: Self = Self(2);
        pub const HINT_NV: Self = Self(3);
        pub const FLOW_VECTOR_NV: Self = Self(4);
        pub const BACKWARD_FLOW_VECTOR_NV: Self = Self(5);
        pub const COST_NV: Self = Self(6);
        pub const BACKWARD_COST_NV: Self = Self(7);
        pub const GLOBAL_FLOW_NV: Self = Self(8);
    }
    bitflags! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct OpticalFlowGridSizeFlagsNV: Flags {
            const _1X1_NV = OpticalFlowGridSizeFlagBitsNV::_1X1_NV.0;
            const _2X2_NV = OpticalFlowGridSizeFlagBitsNV::_2X2_NV.0;
            const _4X4_NV = OpticalFlowGridSizeFlagBitsNV::_4X4_NV.0;
            const _8X8_NV = OpticalFlowGridSizeFlagBitsNV::_8X8_NV.0;
            const UNKNOWN = 0;
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OpticalFlowGridSizeFlagBitsNV(u32);
    impl OpticalFlowGridSizeFlagBitsNV {
        pub const _1X1_NV: Self = Self(1 << 0);
        pub const _2X2_NV: Self = Self(1 << 1);
        pub const _4X4_NV: Self = Self(1 << 2);
        pub const _8X8_NV: Self = Self(1 << 3);
    }
    bitflags! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct OpticalFlowUsageFlagsNV: Flags {
            const INPUT_NV = OpticalFlowUsageFlagBitsNV::INPUT_NV.0;
            const OUTPUT_NV = OpticalFlowUsageFlagBitsNV::OUTPUT_NV.0;
            const HINT_NV = OpticalFlowUsageFlagBitsNV::HINT_NV.0;
            const COST_NV = OpticalFlowUsageFlagBitsNV::COST_NV.0;
            const GLOBAL_FLOW_NV = OpticalFlowUsageFlagBitsNV::GLOBAL_FLOW_NV.0;
            const UNKNOWN = 0;
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OpticalFlowUsageFlagBitsNV(u32);
    impl OpticalFlowUsageFlagBitsNV {
        pub const INPUT_NV: Self = Self(1 << 0);
        pub const OUTPUT_NV: Self = Self(1 << 1);
        pub const HINT_NV: Self = Self(1 << 2);
        pub const COST_NV: Self = Self(1 << 3);
        pub const GLOBAL_FLOW_NV: Self = Self(1 << 4);
    }
    bitflags! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct OpticalFlowSessionCreateFlagsNV: Flags {
            const ENABLE_HINT_NV = OpticalFlowSessionCreateFlagBitsNV::ENABLE_HINT_NV.0;
            const ENABLE_COST_NV = OpticalFlowSessionCreateFlagBitsNV::ENABLE_COST_NV.0;
            const ENABLE_GLOBAL_FLOW_NV = OpticalFlowSessionCreateFlagBitsNV::ENABLE_GLOBAL_FLOW_NV.0;
            const ALLOW_REGIONS_NV = OpticalFlowSessionCreateFlagBitsNV::ALLOW_REGIONS_NV.0;
            const BOTH_DIRECTIONS_NV = OpticalFlowSessionCreateFlagBitsNV::BOTH_DIRECTIONS_NV.0;
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OpticalFlowSessionCreateFlagBitsNV(u32);
    impl OpticalFlowSessionCreateFlagBitsNV {
        pub const ENABLE_HINT_NV: Self = Self(1 << 0);
        pub const ENABLE_COST_NV: Self = Self(1 << 1);
        pub const ENABLE_GLOBAL_FLOW_NV: Self = Self(1 << 2);
        pub const ALLOW_REGIONS_NV: Self = Self(1 << 3);
        pub const BOTH_DIRECTIONS_NV: Self = Self(1 << 4);
    }
    bitflags! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct OpticalFlowExecuteFlagsNV: Flags {
            const DISABLE_TEMPORAL_HINTS_NV = OpticalFlowExecuteFlagBitsNV::DISABLE_TEMPORAL_HINTS_NV.0;
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OpticalFlowExecuteFlagBitsNV(u32);
    impl OpticalFlowExecuteFlagBitsNV {
        pub const DISABLE_TEMPORAL_HINTS_NV: Self = Self(1 << 0);
    }
    pub type PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_optical_flow_image_format_info: *const OpticalFlowImageFormatInfoNV<'_>,
            p_format_count: *mut u32,
            p_image_format_properties: *mut OpticalFlowImageFormatPropertiesNV<'_>,
        ) -> vk::Result;
    pub type PFN_vkCreateOpticalFlowSessionNV = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const OpticalFlowSessionCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_session: *mut OpticalFlowSessionNV,
    ) -> vk::Result;
    pub type PFN_vkDestroyOpticalFlowSessionNV = unsafe extern "system" fn(
        device: Device,
        session: OpticalFlowSessionNV,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkBindOpticalFlowSessionImageNV = unsafe extern "system" fn(
        device: Device,
        session: OpticalFlowSessionNV,
        binding_point: OpticalFlowSessionBindingPointNV,
        view: ImageView,
        layout: ImageLayout,
    ) -> vk::Result;
    pub type PFN_vkCmdOpticalFlowExecuteNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        session: OpticalFlowSessionNV,
        p_execute_info: *const OpticalFlowExecuteInfoNV<'_>,
    );
}
pub struct InstanceFn {
    get_physical_device_optical_flow_image_formats_nv:
        PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_optical_flow_image_formats_nv: transmute(
                    load(c"vkGetPhysicalDeviceOpticalFlowImageFormatsNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_optical_flow_image_formats_nv<'a>(
        &self,
        physical_device: PhysicalDevice,
        optical_flow_image_format_info: &OpticalFlowImageFormatInfoNV<'_>,
        image_format_properties: impl ExtendUninit<OpticalFlowImageFormatPropertiesNV<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(
                image_format_properties,
                |format_count, image_format_properties| {
                    let result = (self.get_physical_device_optical_flow_image_formats_nv)(
                        physical_device,
                        optical_flow_image_format_info,
                        format_count,
                        image_format_properties as _,
                    );

                    match result {
                        VkResult::SUCCESS => Ok(()),
                        VkResult::INCOMPLETE => Ok(()),
                        err => Err(err),
                    }
                },
            )
        }
    }
}
pub struct DeviceFn {
    create_optical_flow_session_nv: PFN_vkCreateOpticalFlowSessionNV,
    destroy_optical_flow_session_nv: PFN_vkDestroyOpticalFlowSessionNV,
    bind_optical_flow_session_image_nv: PFN_vkBindOpticalFlowSessionImageNV,
    cmd_optical_flow_execute_nv: PFN_vkCmdOpticalFlowExecuteNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_optical_flow_session_nv: transmute(
                    load(c"vkCreateOpticalFlowSessionNV").ok_or(LoadingError)?,
                ),
                destroy_optical_flow_session_nv: transmute(
                    load(c"vkDestroyOpticalFlowSessionNV").ok_or(LoadingError)?,
                ),
                bind_optical_flow_session_image_nv: transmute(
                    load(c"vkBindOpticalFlowSessionImageNV").ok_or(LoadingError)?,
                ),
                cmd_optical_flow_execute_nv: transmute(
                    load(c"vkCmdOpticalFlowExecuteNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_optical_flow_session_nv(
        &self,
        device: Device,
        create_info: &OpticalFlowSessionCreateInfoNV<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<OpticalFlowSessionNV> {
        unsafe {
            let mut session = core::mem::MaybeUninit::uninit();
            let result = (self.create_optical_flow_session_nv)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                session.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(session.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_optical_flow_session_nv(
        &self,
        device: Device,
        session: OpticalFlowSessionNV,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_optical_flow_session_nv)(device, session, allocator.to_raw_ptr()) }
    }
    pub unsafe fn bind_optical_flow_session_image_nv(
        &self,
        device: Device,
        session: OpticalFlowSessionNV,
        binding_point: OpticalFlowSessionBindingPointNV,
        view: ImageView,
        layout: ImageLayout,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.bind_optical_flow_session_image_nv)(
                device,
                session,
                binding_point,
                view,
                layout,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn cmd_optical_flow_execute_nv(
        &self,
        command_buffer: CommandBuffer,
        session: OpticalFlowSessionNV,
        execute_info: &OpticalFlowExecuteInfoNV<'_>,
    ) {
        unsafe { (self.cmd_optical_flow_execute_nv)(command_buffer, session, execute_info) }
    }
}
