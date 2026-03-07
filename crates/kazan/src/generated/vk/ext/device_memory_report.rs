#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_device_memory_report";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDeviceMemoryReportFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDeviceMemoryReportFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub device_memory_report: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceDeviceMemoryReportFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDeviceMemoryReportFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("device_memory_report", &self.device_memory_report)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDeviceMemoryReportFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDeviceMemoryReportFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceDeviceMemoryReportFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceDeviceMemoryReportFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                device_memory_report: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDeviceMemoryReportFeaturesEXT<'a> {
        pub fn device_memory_report(mut self, device_memory_report: bool) -> Self {
            self.device_memory_report = device_memory_report.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceDeviceMemoryReportCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceDeviceMemoryReportCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DeviceMemoryReportFlagsEXT,
        pub pfn_user_callback: Option<PFN_vkDeviceMemoryReportCallbackEXT>,
        pub p_user_data: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for DeviceDeviceMemoryReportCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceDeviceMemoryReportCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field(
                    "pfn_user_callback",
                    &self.pfn_user_callback.map(|f| f as *const ()),
                )
                .field("p_user_data", &self.p_user_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceDeviceMemoryReportCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for DeviceDeviceMemoryReportCreateInfoEXT<'a> {}

    impl Default for DeviceDeviceMemoryReportCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                pfn_user_callback: Default::default(),
                p_user_data: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceDeviceMemoryReportCreateInfoEXT<'a> {
        pub fn flags(mut self, flags: DeviceMemoryReportFlagsEXT) -> Self {
            self.flags = flags;
            self
        }

        pub fn pfn_user_callback(
            mut self,
            pfn_user_callback: PFN_vkDeviceMemoryReportCallbackEXT,
        ) -> Self {
            self.pfn_user_callback = Some(pfn_user_callback);
            self
        }

        pub fn user_data(mut self, user_data: *mut c_void) -> Self {
            self.p_user_data = user_data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceMemoryReportCallbackDataEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceMemoryReportCallbackDataEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: DeviceMemoryReportFlagsEXT,
        pub ty: DeviceMemoryReportEventTypeEXT,
        pub memory_object_id: u64,
        pub size: DeviceSize,
        pub object_type: ObjectType,
        pub object_handle: u64,
        pub heap_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for DeviceMemoryReportCallbackDataEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceMemoryReportCallbackDataEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("ty", &self.ty)
                .field("memory_object_id", &self.memory_object_id)
                .field("size", &self.size)
                .field("object_type", &self.object_type)
                .field("object_handle", &self.object_handle)
                .field("heap_index", &self.heap_index)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceMemoryReportCallbackDataEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT;
    }

    impl Default for DeviceMemoryReportCallbackDataEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                flags: Default::default(),
                ty: Default::default(),
                memory_object_id: Default::default(),
                size: Default::default(),
                object_type: Default::default(),
                object_handle: Default::default(),
                heap_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceMemoryReportCallbackDataEXT<'a> {
        pub fn flags(mut self, flags: DeviceMemoryReportFlagsEXT) -> Self {
            self.flags = flags;
            self
        }

        pub fn ty(mut self, ty: DeviceMemoryReportEventTypeEXT) -> Self {
            self.ty = ty;
            self
        }

        pub fn memory_object_id(mut self, memory_object_id: u64) -> Self {
            self.memory_object_id = memory_object_id;
            self
        }

        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }

        pub fn object_type(mut self, object_type: ObjectType) -> Self {
            self.object_type = object_type;
            self
        }

        pub fn object_handle(mut self, object_handle: u64) -> Self {
            self.object_handle = object_handle;
            self
        }

        pub fn heap_index(mut self, heap_index: u32) -> Self {
            self.heap_index = heap_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceMemoryReportEventTypeEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DeviceMemoryReportEventTypeEXT(i32);

    impl DeviceMemoryReportEventTypeEXT {
        pub const ALLOCATE_EXT: Self = Self(0);
        pub const FREE_EXT: Self = Self(1);
        pub const IMPORT_EXT: Self = Self(2);
        pub const UNIMPORT_EXT: Self = Self(3);
        pub const ALLOCATION_FAILED_EXT: Self = Self(4);
    }

    impl fmt::Debug for DeviceMemoryReportEventTypeEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ALLOCATE_EXT => Some("ALLOCATE_EXT"),
                Self::FREE_EXT => Some("FREE_EXT"),
                Self::IMPORT_EXT => Some("IMPORT_EXT"),
                Self::UNIMPORT_EXT => Some("UNIMPORT_EXT"),
                Self::ALLOCATION_FAILED_EXT => Some("ALLOCATION_FAILED_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceMemoryReportFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DeviceMemoryReportFlagsEXT(Flags);
    vk_bitflags_wrapped!(DeviceMemoryReportFlagsEXT, Flags);

    impl fmt::Debug for DeviceMemoryReportFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/PFN_vkDeviceMemoryReportCallbackEXT.html>
    pub type PFN_vkDeviceMemoryReportCallbackEXT = unsafe extern "system" fn(
        p_callback_data: *const DeviceMemoryReportCallbackDataEXT<'_>,
        p_user_data: *mut c_void,
    );
}
