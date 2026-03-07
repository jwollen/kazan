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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFeatures2KHR.html>
    pub type PhysicalDeviceFeatures2KHR<'a> = PhysicalDeviceFeatures2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceProperties2KHR.html>
    pub type PhysicalDeviceProperties2KHR<'a> = PhysicalDeviceProperties2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFormatProperties2KHR.html>
    pub type FormatProperties2KHR<'a> = FormatProperties2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageFormatProperties2KHR.html>
    pub type ImageFormatProperties2KHR<'a> = ImageFormatProperties2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImageFormatInfo2KHR.html>
    pub type PhysicalDeviceImageFormatInfo2KHR<'a> = PhysicalDeviceImageFormatInfo2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueFamilyProperties2KHR.html>
    pub type QueueFamilyProperties2KHR<'a> = QueueFamilyProperties2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMemoryProperties2KHR.html>
    pub type PhysicalDeviceMemoryProperties2KHR<'a> = PhysicalDeviceMemoryProperties2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSparseImageFormatProperties2KHR.html>
    pub type SparseImageFormatProperties2KHR<'a> = SparseImageFormatProperties2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSparseImageFormatInfo2KHR.html>
    pub type PhysicalDeviceSparseImageFormatInfo2KHR<'a> = PhysicalDeviceSparseImageFormatInfo2<'a>;
    pub type PFN_vkGetPhysicalDeviceFeatures2KHR = PFN_vkGetPhysicalDeviceFeatures2;
    pub type PFN_vkGetPhysicalDeviceProperties2KHR = PFN_vkGetPhysicalDeviceProperties2;
    pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR = PFN_vkGetPhysicalDeviceFormatProperties2;
    pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR =
        PFN_vkGetPhysicalDeviceImageFormatProperties2;
    pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR =
        PFN_vkGetPhysicalDeviceQueueFamilyProperties2;
    pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = PFN_vkGetPhysicalDeviceMemoryProperties2;
    pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR =
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties2;
}

pub struct InstanceFn {
    get_physical_device_features2_khr: PFN_vkGetPhysicalDeviceFeatures2,
    get_physical_device_properties2_khr: PFN_vkGetPhysicalDeviceProperties2,
    get_physical_device_format_properties2_khr: PFN_vkGetPhysicalDeviceFormatProperties2,
    get_physical_device_image_format_properties2_khr: PFN_vkGetPhysicalDeviceImageFormatProperties2,
    get_physical_device_queue_family_properties2_khr: PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
    get_physical_device_memory_properties2_khr: PFN_vkGetPhysicalDeviceMemoryProperties2,
    get_physical_device_sparse_image_format_properties2_khr:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_features2_khr: transmute(
                    load(c"vkGetPhysicalDeviceFeatures2KHR").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_properties2_khr: transmute(
                    load(c"vkGetPhysicalDeviceProperties2KHR").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_format_properties2_khr: transmute(
                    load(c"vkGetPhysicalDeviceFormatProperties2KHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_image_format_properties2_khr: transmute(
                    load(c"vkGetPhysicalDeviceImageFormatProperties2KHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_queue_family_properties2_khr: transmute(
                    load(c"vkGetPhysicalDeviceQueueFamilyProperties2KHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_memory_properties2_khr: transmute(
                    load(c"vkGetPhysicalDeviceMemoryProperties2KHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_sparse_image_format_properties2_khr: transmute(
                    load(c"vkGetPhysicalDeviceSparseImageFormatProperties2KHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceFeatures2KHR.html>
    pub unsafe fn get_physical_device_features2_khr(
        &self,
        physical_device: PhysicalDevice,
        features: &mut PhysicalDeviceFeatures2<'_>,
    ) {
        unsafe { (self.get_physical_device_features2_khr)(physical_device, features) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceProperties2KHR.html>
    pub unsafe fn get_physical_device_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        properties: &mut PhysicalDeviceProperties2<'_>,
    ) {
        unsafe { (self.get_physical_device_properties2_khr)(physical_device, properties) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceFormatProperties2KHR.html>
    pub unsafe fn get_physical_device_format_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        format_properties: &mut FormatProperties2<'_>,
    ) {
        unsafe {
            (self.get_physical_device_format_properties2_khr)(
                physical_device,
                format,
                format_properties,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceImageFormatProperties2KHR.html>
    pub unsafe fn get_physical_device_image_format_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        image_format_info: &PhysicalDeviceImageFormatInfo2<'_>,
        image_format_properties: &mut ImageFormatProperties2<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_physical_device_image_format_properties2_khr)(
                physical_device,
                image_format_info,
                image_format_properties,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html>
    pub unsafe fn get_physical_device_queue_family_properties2_khr<'a>(
        &self,
        physical_device: PhysicalDevice,
        mut queue_family_properties: impl ExtendUninit<QueueFamilyProperties2<'a>>,
    ) {
        unsafe {
            let call = |queue_family_property_count, queue_family_properties| {
                (self.get_physical_device_queue_family_properties2_khr)(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceMemoryProperties2KHR.html>
    pub unsafe fn get_physical_device_memory_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        memory_properties: &mut PhysicalDeviceMemoryProperties2<'_>,
    ) {
        unsafe {
            (self.get_physical_device_memory_properties2_khr)(physical_device, memory_properties)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html>
    pub unsafe fn get_physical_device_sparse_image_format_properties2_khr<'a>(
        &self,
        physical_device: PhysicalDevice,
        format_info: &PhysicalDeviceSparseImageFormatInfo2<'a>,
        mut properties: impl ExtendUninit<SparseImageFormatProperties2<'a>>,
    ) {
        unsafe {
            let call = |property_count, properties| {
                (self.get_physical_device_sparse_image_format_properties2_khr)(
                    physical_device,
                    format_info,
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
