pub struct Device {}
impl Device {
    pub fn vk_create_shaders_ext(&self);
    pub fn vk_destroy_shader_ext(&self);
    pub fn vk_get_shader_binary_data_ext(&self);
    pub fn vk_cmd_bind_shaders_ext(&self);
    pub fn vk_cmd_set_depth_clamp_range_ext(&self);
}
