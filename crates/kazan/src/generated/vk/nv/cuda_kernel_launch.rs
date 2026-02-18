pub struct Device {}
impl Device {
    pub fn vk_create_cuda_module_nv(&self);
    pub fn vk_get_cuda_module_cache_nv(&self);
    pub fn vk_create_cuda_function_nv(&self);
    pub fn vk_destroy_cuda_module_nv(&self);
    pub fn vk_destroy_cuda_function_nv(&self);
    pub fn vk_cmd_cuda_launch_kernel_nv(&self);
}
