#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_fragment_shading_rates_khr: PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_fragment_shading_rates_khr(
        &self,
        physical_device: PhysicalDevice,
        fragment_shading_rates: impl ExtendUninit<PhysicalDeviceFragmentShadingRateKHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(
                fragment_shading_rates,
                |fragment_shading_rate_count, fragment_shading_rates| {
                    (self.get_physical_device_fragment_shading_rates_khr)(
                        physical_device,
                        fragment_shading_rate_count,
                        fragment_shading_rates as _,
                    )
                },
            )
        }
    }
}
pub struct DeviceFn {
    cmd_set_fragment_shading_rate_khr: PFN_vkCmdSetFragmentShadingRateKHR,
}
impl DeviceFn {
    pub unsafe fn cmd_set_fragment_shading_rate_khr(
        &self,
        command_buffer: CommandBuffer,
        fragment_size: &Extent2D,
        combiner_ops: &[FragmentShadingRateCombinerOpKHR; 2],
    ) {
        unsafe {
            (self.cmd_set_fragment_shading_rate_khr)(command_buffer, fragment_size, combiner_ops)
        }
    }
}
