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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceTilePropertiesFeaturesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceTilePropertiesFeaturesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub tile_properties: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTilePropertiesFeaturesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceTilePropertiesFeaturesQCOM<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceTilePropertiesFeaturesQCOM<'a> {}
    impl Default for PhysicalDeviceTilePropertiesFeaturesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                tile_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceTilePropertiesFeaturesQCOM<'a> {
        pub fn tile_properties(mut self, tile_properties: bool) -> Self {
            self.tile_properties = tile_properties.into();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTilePropertiesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct TilePropertiesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub tile_size: Extent3D,
        pub apron_size: Extent2D,
        pub origin: Offset2D,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for TilePropertiesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TILE_PROPERTIES_QCOM;
    }
    impl Default for TilePropertiesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                tile_size: Default::default(),
                apron_size: Default::default(),
                origin: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> TilePropertiesQCOM<'a> {
        pub fn tile_size(mut self, tile_size: Extent3D) -> Self {
            self.tile_size = tile_size;
            self
        }
        pub fn apron_size(mut self, apron_size: Extent2D) -> Self {
            self.apron_size = apron_size;
            self
        }
        pub fn origin(mut self, origin: Offset2D) -> Self {
            self.origin = origin;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetFramebufferTilePropertiesQCOM.html>
    pub type PFN_vkGetFramebufferTilePropertiesQCOM = unsafe extern "system" fn(
        device: Device,
        framebuffer: Framebuffer,
        p_properties_count: *mut u32,
        p_properties: *mut TilePropertiesQCOM<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDynamicRenderingTilePropertiesQCOM.html>
    pub type PFN_vkGetDynamicRenderingTilePropertiesQCOM = unsafe extern "system" fn(
        device: Device,
        p_rendering_info: *const RenderingInfo<'_>,
        p_properties: *mut TilePropertiesQCOM<'_>,
    )
        -> vk::Result;
}
pub struct DeviceFn {
    get_framebuffer_tile_properties_qcom: PFN_vkGetFramebufferTilePropertiesQCOM,
    get_dynamic_rendering_tile_properties_qcom: PFN_vkGetDynamicRenderingTilePropertiesQCOM,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_framebuffer_tile_properties_qcom: transmute(
                    load(c"vkGetFramebufferTilePropertiesQCOM").ok_or(MissingEntryPointError)?,
                ),
                get_dynamic_rendering_tile_properties_qcom: transmute(
                    load(c"vkGetDynamicRenderingTilePropertiesQCOM")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetFramebufferTilePropertiesQCOM.html>
    pub unsafe fn get_framebuffer_tile_properties_qcom<'a>(
        &self,
        device: Device,
        framebuffer: Framebuffer,
        mut properties: impl ExtendUninit<TilePropertiesQCOM<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |properties_count, properties| {
                let result = (self.get_framebuffer_tile_properties_qcom)(
                    device,
                    framebuffer,
                    properties_count,
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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDynamicRenderingTilePropertiesQCOM.html>
    pub unsafe fn get_dynamic_rendering_tile_properties_qcom(
        &self,
        device: Device,
        rendering_info: &RenderingInfo<'_>,
        properties: &mut TilePropertiesQCOM<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_dynamic_rendering_tile_properties_qcom)(
                device,
                rendering_info,
                properties,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
