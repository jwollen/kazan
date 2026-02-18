pub struct Device {}
impl Device {
    pub fn vk_trim_command_pool(&self);
    pub fn vk_get_device_group_peer_memory_features(&self);
    pub fn vk_bind_buffer_memory2(&self);
    pub fn vk_bind_image_memory2(&self);
    pub fn vk_cmd_set_device_mask(&self);
    pub fn vk_get_buffer_memory_requirements2(&self);
    pub fn vk_get_image_memory_requirements2(&self);
    pub fn vk_get_image_sparse_memory_requirements2(&self);
    pub fn vk_get_device_queue2(&self);
}
