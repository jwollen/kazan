pub struct Device {}
impl Device {
    pub fn vk_reset_query_pool(&self);
    pub fn vk_get_semaphore_counter_value(&self);
    pub fn vk_wait_semaphores(&self);
    pub fn vk_signal_semaphore(&self);
    pub fn vk_get_buffer_opaque_capture_address(&self);
    pub fn vk_get_buffer_device_address(&self);
    pub fn vk_get_device_memory_opaque_capture_address(&self);
}
