//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NVX_image_view_handle.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NVX_image_view_handle";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageViewHandleInfoNVX.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageViewHandleInfoNVX<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image_view: ImageView,
        pub descriptor_type: DescriptorType,
        pub sampler: Sampler,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageViewHandleInfoNVX<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageViewHandleInfoNVX")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image_view", &self.image_view)
                .field("descriptor_type", &self.descriptor_type)
                .field("sampler", &self.sampler)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageViewHandleInfoNVX<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_VIEW_HANDLE_INFO_NVX;
    }

    impl Default for ImageViewHandleInfoNVX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                image_view: Default::default(),
                descriptor_type: Default::default(),
                sampler: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageViewHandleInfoNVX<'a> {
        #[inline]
        pub fn image_view(mut self, image_view: ImageView) -> Self {
            self.image_view = image_view;
            self
        }

        #[inline]
        pub fn descriptor_type(mut self, descriptor_type: DescriptorType) -> Self {
            self.descriptor_type = descriptor_type;
            self
        }

        #[inline]
        pub fn sampler(mut self, sampler: Sampler) -> Self {
            self.sampler = sampler;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageViewAddressPropertiesNVX.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageViewAddressPropertiesNVX<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub device_address: DeviceAddress,
        pub size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageViewAddressPropertiesNVX<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageViewAddressPropertiesNVX")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("device_address", &self.device_address)
                .field("size", &self.size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageViewAddressPropertiesNVX<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_VIEW_ADDRESS_PROPERTIES_NVX;
    }

    impl Default for ImageViewAddressPropertiesNVX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                device_address: Default::default(),
                size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageViewAddressPropertiesNVX<'a> {
        #[inline]
        pub fn device_address(mut self, device_address: DeviceAddress) -> Self {
            self.device_address = device_address;
            self
        }

        #[inline]
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageViewHandleNVX.html>
    pub type PFN_vkGetImageViewHandleNVX =
        unsafe extern "system" fn(device: Device, p_info: *const ImageViewHandleInfoNVX<'_>) -> u32;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageViewHandle64NVX.html>
    pub type PFN_vkGetImageViewHandle64NVX =
        unsafe extern "system" fn(device: Device, p_info: *const ImageViewHandleInfoNVX<'_>) -> u64;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageViewAddressNVX.html>
    pub type PFN_vkGetImageViewAddressNVX = unsafe extern "system" fn(
        device: Device,
        image_view: ImageView,
        p_properties: *mut ImageViewAddressPropertiesNVX<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceCombinedImageSamplerIndexNVX.html>
    pub type PFN_vkGetDeviceCombinedImageSamplerIndexNVX =
        unsafe extern "system" fn(device: Device, image_view_index: u64, sampler_index: u64) -> u64;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkImageViewHandleInfoNVX = ImageViewHandleInfoNVX<'static>;
    pub type VkImageViewAddressPropertiesNVX = ImageViewAddressPropertiesNVX<'static>;
    impl ImageViewHandleInfoNVX<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageViewHandleInfoNVX {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageViewAddressPropertiesNVX<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageViewAddressPropertiesNVX {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_image_view_handle_nvx: PFN_vkGetImageViewHandleNVX,
    get_image_view_handle64_nvx: PFN_vkGetImageViewHandle64NVX,
    get_image_view_address_nvx: PFN_vkGetImageViewAddressNVX,
    get_device_combined_image_sampler_index_nvx: PFN_vkGetDeviceCombinedImageSamplerIndexNVX,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_image_view_handle_nvx: transmute(
                    load(c"vkGetImageViewHandleNVX").ok_or(MissingEntryPointError)?,
                ),
                get_image_view_handle64_nvx: transmute(
                    load(c"vkGetImageViewHandle64NVX").ok_or(MissingEntryPointError)?,
                ),
                get_image_view_address_nvx: transmute(
                    load(c"vkGetImageViewAddressNVX").ok_or(MissingEntryPointError)?,
                ),
                get_device_combined_image_sampler_index_nvx: transmute(
                    load(c"vkGetDeviceCombinedImageSamplerIndexNVX")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageViewHandleNVX.html>
    #[inline]
    pub unsafe fn get_image_view_handle_nvx(
        &self,
        device: Device,
        info: &ImageViewHandleInfoNVX<'_>,
    ) -> u32 {
        unsafe { (self.get_image_view_handle_nvx)(device, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageViewHandle64NVX.html>
    #[inline]
    pub unsafe fn get_image_view_handle64_nvx(
        &self,
        device: Device,
        info: &ImageViewHandleInfoNVX<'_>,
    ) -> u64 {
        unsafe { (self.get_image_view_handle64_nvx)(device, info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageViewAddressNVX.html>
    #[inline]
    pub unsafe fn get_image_view_address_nvx(
        &self,
        device: Device,
        image_view: ImageView,
        properties: &mut ImageViewAddressPropertiesNVX<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_image_view_address_nvx)(device, image_view, properties);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceCombinedImageSamplerIndexNVX.html>
    #[inline]
    pub unsafe fn get_device_combined_image_sampler_index_nvx(
        &self,
        device: Device,
        image_view_index: u64,
        sampler_index: u64,
    ) -> u64 {
        unsafe {
            (self.get_device_combined_image_sampler_index_nvx)(
                device,
                image_view_index,
                sampler_index,
            )
        }
    }
}
