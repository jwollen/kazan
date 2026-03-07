#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_validation_flags";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkValidationFlagsEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ValidationFlagsEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub disabled_validation_check_count: u32,
        pub p_disabled_validation_checks: *const ValidationCheckEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ValidationFlagsEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ValidationFlagsEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "disabled_validation_check_count",
                    &self.disabled_validation_check_count,
                )
                .field(
                    "p_disabled_validation_checks",
                    &self.p_disabled_validation_checks,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ValidationFlagsEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VALIDATION_FLAGS_EXT;
    }

    unsafe impl<'a> Extends<InstanceCreateInfo<'a>> for ValidationFlagsEXT<'a> {}

    impl Default for ValidationFlagsEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                disabled_validation_check_count: Default::default(),
                p_disabled_validation_checks: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ValidationFlagsEXT<'a> {
        #[inline]
        pub fn disabled_validation_checks(
            mut self,
            disabled_validation_checks: &'a [ValidationCheckEXT],
        ) -> Self {
            self.disabled_validation_check_count =
                disabled_validation_checks.len().try_into().unwrap();
            self.p_disabled_validation_checks = disabled_validation_checks.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkValidationCheckEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ValidationCheckEXT(i32);

    impl ValidationCheckEXT {
        pub const ALL_EXT: Self = Self(0);
        pub const SHADERS_EXT: Self = Self(1);
    }

    impl fmt::Debug for ValidationCheckEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ALL_EXT => Some("ALL_EXT"),
                Self::SHADERS_EXT => Some("SHADERS_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
}
