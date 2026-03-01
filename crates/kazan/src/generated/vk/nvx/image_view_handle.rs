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
    pub struct ImageViewHandleInfoNVX<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image_view: ImageView,
        pub descriptor_type: DescriptorType,
        pub sampler: Sampler,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageViewHandleInfoNVX<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_VIEW_HANDLE_INFO_NVX;
    }
    impl Default for ImageViewHandleInfoNVX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                image_view: Default::default(),
                descriptor_type: Default::default(),
                sampler: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageViewHandleInfoNVX<'a> {
        pub fn image_view(mut self, image_view: ImageView) -> Self {
            self.image_view = image_view;
            self
        }
        pub fn descriptor_type(mut self, descriptor_type: DescriptorType) -> Self {
            self.descriptor_type = descriptor_type;
            self
        }
        pub fn sampler(mut self, sampler: Sampler) -> Self {
            self.sampler = sampler;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageViewAddressPropertiesNVX<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub device_address: DeviceAddress,
        pub size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageViewAddressPropertiesNVX<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_VIEW_ADDRESS_PROPERTIES_NVX;
    }
    impl Default for ImageViewAddressPropertiesNVX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                device_address: Default::default(),
                size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageViewAddressPropertiesNVX<'a> {
        pub fn device_address(mut self, device_address: DeviceAddress) -> Self {
            self.device_address = device_address;
            self
        }
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }
    pub type PFN_vkGetImageViewHandleNVX =
        unsafe extern "system" fn(device: Device, p_info: *const ImageViewHandleInfoNVX<'_>) -> u32;
    pub type PFN_vkGetImageViewHandle64NVX =
        unsafe extern "system" fn(device: Device, p_info: *const ImageViewHandleInfoNVX<'_>) -> u64;
    pub type PFN_vkGetImageViewAddressNVX = unsafe extern "system" fn(
        device: Device,
        image_view: ImageView,
        p_properties: *mut ImageViewAddressPropertiesNVX<'_>,
    ) -> vk::Result;
    pub type PFN_vkGetDeviceCombinedImageSamplerIndexNVX =
        unsafe extern "system" fn(device: Device, image_view_index: u64, sampler_index: u64) -> u64;
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
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_image_view_handle_nvx: transmute(
                    load(c"vkGetImageViewHandleNVX").ok_or(LoadingError)?,
                ),
                get_image_view_handle64_nvx: transmute(
                    load(c"vkGetImageViewHandle64NVX").ok_or(LoadingError)?,
                ),
                get_image_view_address_nvx: transmute(
                    load(c"vkGetImageViewAddressNVX").ok_or(LoadingError)?,
                ),
                get_device_combined_image_sampler_index_nvx: transmute(
                    load(c"vkGetDeviceCombinedImageSamplerIndexNVX").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_image_view_handle_nvx(
        &self,
        device: Device,
        info: &ImageViewHandleInfoNVX<'_>,
    ) -> u32 {
        unsafe { (self.get_image_view_handle_nvx)(device, info) }
    }
    pub unsafe fn get_image_view_handle64_nvx(
        &self,
        device: Device,
        info: &ImageViewHandleInfoNVX<'_>,
    ) -> u64 {
        unsafe { (self.get_image_view_handle64_nvx)(device, info) }
    }
    pub unsafe fn get_image_view_address_nvx(
        &self,
        device: Device,
        image_view: ImageView,
    ) -> crate::Result<ImageViewAddressPropertiesNVX<'_>> {
        unsafe {
            let mut properties = core::mem::MaybeUninit::uninit();
            let result =
                (self.get_image_view_address_nvx)(device, image_view, properties.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(properties.assume_init()),
                err => Err(err),
            }
        }
    }
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
