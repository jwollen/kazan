pub struct Device {}
impl Device {
    pub fn vk_create_swapchain_khr(&self);
    pub fn vk_destroy_swapchain_khr(&self);
    pub fn vk_get_swapchain_images_khr(&self);
    pub fn vk_acquire_next_image_khr(&self);
    pub fn vk_queue_present_khr(&self);
    pub fn vk_get_device_group_present_capabilities_khr(&self);
    pub fn vk_get_device_group_surface_present_modes_khr(&self);
    pub fn vk_acquire_next_image2_khr(&self);
}
