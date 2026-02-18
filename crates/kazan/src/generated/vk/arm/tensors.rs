pub struct Device {}
impl Device {
    pub fn vk_create_tensor_arm(&self);
    pub fn vk_destroy_tensor_arm(&self);
    pub fn vk_create_tensor_view_arm(&self);
    pub fn vk_destroy_tensor_view_arm(&self);
    pub fn vk_get_tensor_memory_requirements_arm(&self);
    pub fn vk_bind_tensor_memory_arm(&self);
    pub fn vk_get_device_tensor_memory_requirements_arm(&self);
    pub fn vk_cmd_copy_tensor_arm(&self);
    pub fn vk_get_tensor_opaque_capture_descriptor_data_arm(&self);
    pub fn vk_get_tensor_view_opaque_capture_descriptor_data_arm(&self);
}
