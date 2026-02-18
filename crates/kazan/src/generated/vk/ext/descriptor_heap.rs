pub struct Device {}
impl Device {
    pub fn vk_write_sampler_descriptors_ext(&self);
    pub fn vk_write_resource_descriptors_ext(&self);
    pub fn vk_cmd_bind_sampler_heap_ext(&self);
    pub fn vk_cmd_bind_resource_heap_ext(&self);
    pub fn vk_cmd_push_data_ext(&self);
    pub fn vk_register_custom_border_color_ext(&self);
    pub fn vk_unregister_custom_border_color_ext(&self);
    pub fn vk_get_image_opaque_capture_data_ext(&self);
    pub fn vk_get_tensor_opaque_capture_data_arm(&self);
}
