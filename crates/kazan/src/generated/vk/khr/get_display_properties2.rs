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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DisplayProperties2KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub display_properties: DisplayPropertiesKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DisplayProperties2KHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DISPLAY_PROPERTIES_2_KHR;
    }
    impl Default for DisplayProperties2KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                display_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DisplayProperties2KHR<'a> {
        pub fn display_properties(mut self, display_properties: DisplayPropertiesKHR<'a>) -> Self {
            self.display_properties = display_properties;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DisplayPlaneProperties2KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub display_plane_properties: DisplayPlanePropertiesKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DisplayPlaneProperties2KHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DISPLAY_PLANE_PROPERTIES_2_KHR;
    }
    impl Default for DisplayPlaneProperties2KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                display_plane_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DisplayPlaneProperties2KHR<'a> {
        pub fn display_plane_properties(
            mut self,
            display_plane_properties: DisplayPlanePropertiesKHR,
        ) -> Self {
            self.display_plane_properties = display_plane_properties;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DisplayModeProperties2KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub display_mode_properties: DisplayModePropertiesKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DisplayModeProperties2KHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DISPLAY_MODE_PROPERTIES_2_KHR;
    }
    impl Default for DisplayModeProperties2KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                display_mode_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DisplayModeProperties2KHR<'a> {
        pub fn display_mode_properties(
            mut self,
            display_mode_properties: DisplayModePropertiesKHR,
        ) -> Self {
            self.display_mode_properties = display_mode_properties;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DisplayPlaneInfo2KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub mode: DisplayModeKHR,
        pub plane_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DisplayPlaneInfo2KHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DISPLAY_PLANE_INFO_2_KHR;
    }
    impl Default for DisplayPlaneInfo2KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                mode: Default::default(),
                plane_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DisplayPlaneInfo2KHR<'a> {
        pub fn mode(mut self, mode: DisplayModeKHR) -> Self {
            self.mode = mode;
            self
        }
        pub fn plane_index(mut self, plane_index: u32) -> Self {
            self.plane_index = plane_index;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DisplayPlaneCapabilities2KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub capabilities: DisplayPlaneCapabilitiesKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DisplayPlaneCapabilities2KHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DISPLAY_PLANE_CAPABILITIES_2_KHR;
    }
    impl Default for DisplayPlaneCapabilities2KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                capabilities: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DisplayPlaneCapabilities2KHR<'a> {
        pub fn capabilities(mut self, capabilities: DisplayPlaneCapabilitiesKHR) -> Self {
            self.capabilities = capabilities;
            self
        }
    }
    pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayProperties2KHR<'_>,
    )
        -> vk::Result;
    pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut DisplayPlaneProperties2KHR<'_>,
        ) -> vk::Result;
    pub type PFN_vkGetDisplayModeProperties2KHR = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_property_count: *mut u32,
        p_properties: *mut DisplayModeProperties2KHR<'_>,
    ) -> vk::Result;
    pub type PFN_vkGetDisplayPlaneCapabilities2KHR = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_display_plane_info: *const DisplayPlaneInfo2KHR<'_>,
        p_capabilities: *mut DisplayPlaneCapabilities2KHR<'_>,
    ) -> vk::Result;
}
pub struct InstanceFn {
    get_physical_device_display_properties2_khr: PFN_vkGetPhysicalDeviceDisplayProperties2KHR,
    get_physical_device_display_plane_properties2_khr:
        PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR,
    get_display_mode_properties2_khr: PFN_vkGetDisplayModeProperties2KHR,
    get_display_plane_capabilities2_khr: PFN_vkGetDisplayPlaneCapabilities2KHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_display_properties2_khr: transmute(
                    load(c"vkGetPhysicalDeviceDisplayProperties2KHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_display_plane_properties2_khr: transmute(
                    load(c"vkGetPhysicalDeviceDisplayPlaneProperties2KHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_display_mode_properties2_khr: transmute(
                    load(c"vkGetDisplayModeProperties2KHR").ok_or(MissingEntryPointError)?,
                ),
                get_display_plane_capabilities2_khr: transmute(
                    load(c"vkGetDisplayPlaneCapabilities2KHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_display_properties2_khr<'a>(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<DisplayProperties2KHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result = (self.get_physical_device_display_properties2_khr)(
                    physical_device,
                    property_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn get_physical_device_display_plane_properties2_khr<'a>(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<DisplayPlaneProperties2KHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result = (self.get_physical_device_display_plane_properties2_khr)(
                    physical_device,
                    property_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn get_display_mode_properties2_khr<'a>(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        properties: impl ExtendUninit<DisplayModeProperties2KHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result = (self.get_display_mode_properties2_khr)(
                    physical_device,
                    display,
                    property_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn get_display_plane_capabilities2_khr(
        &self,
        physical_device: PhysicalDevice,
        display_plane_info: &DisplayPlaneInfo2KHR<'_>,
    ) -> crate::Result<DisplayPlaneCapabilities2KHR<'_>> {
        unsafe {
            let mut capabilities = core::mem::MaybeUninit::uninit();
            let result = (self.get_display_plane_capabilities2_khr)(
                physical_device,
                display_plane_info,
                capabilities.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(capabilities.assume_init()),
                err => Err(err),
            }
        }
    }
}
