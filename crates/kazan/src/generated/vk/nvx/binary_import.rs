pub struct Device {}
impl Device {
    pub fn vk_create_cu_module_nvx(&self);
    pub fn vk_create_cu_function_nvx(&self);
    pub fn vk_destroy_cu_module_nvx(&self);
    pub fn vk_destroy_cu_function_nvx(&self);
    pub fn vk_cmd_cu_launch_kernel_nvx(&self);
}
