pub struct Device {}
impl Device {
    pub fn vk_get_descriptor_set_layout_size_ext(&self);
    pub fn vk_get_descriptor_set_layout_binding_offset_ext(&self);
    pub fn vk_get_descriptor_ext(&self);
    pub fn vk_cmd_bind_descriptor_buffers_ext(&self);
    pub fn vk_cmd_set_descriptor_buffer_offsets_ext(&self);
    pub fn vk_cmd_bind_descriptor_buffer_embedded_samplers_ext(&self);
    pub fn vk_get_buffer_opaque_capture_descriptor_data_ext(&self);
    pub fn vk_get_image_opaque_capture_descriptor_data_ext(&self);
    pub fn vk_get_image_view_opaque_capture_descriptor_data_ext(&self);
    pub fn vk_get_sampler_opaque_capture_descriptor_data_ext(&self);
    pub fn vk_get_acceleration_structure_opaque_capture_descriptor_data_ext(&self);
}
