pub struct Device {}
impl Device {
    pub fn vk_copy_memory_to_image(&self);
    pub fn vk_copy_image_to_memory(&self);
    pub fn vk_copy_image_to_image(&self);
    pub fn vk_transition_image_layout(&self);
    pub fn vk_get_image_subresource_layout2(&self);
    pub fn vk_get_device_image_subresource_layout(&self);
    pub fn vk_map_memory2(&self);
    pub fn vk_unmap_memory2(&self);
}
