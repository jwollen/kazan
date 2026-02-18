pub struct Device {}
impl Device {
    pub fn vk_get_device_buffer_memory_requirements(&self);
    pub fn vk_get_device_image_memory_requirements(&self);
    pub fn vk_get_device_image_sparse_memory_requirements(&self);
    pub fn vk_create_private_data_slot(&self);
    pub fn vk_destroy_private_data_slot(&self);
    pub fn vk_set_private_data(&self);
    pub fn vk_get_private_data(&self);
    pub fn vk_cmd_copy_buffer2(&self);
    pub fn vk_cmd_copy_image2(&self);
    pub fn vk_cmd_copy_buffer_to_image2(&self);
    pub fn vk_cmd_copy_image_to_buffer2(&self);
    pub fn vk_cmd_pipeline_barrier2(&self);
    pub fn vk_queue_submit2(&self);
    pub fn vk_cmd_write_timestamp2(&self);
}
