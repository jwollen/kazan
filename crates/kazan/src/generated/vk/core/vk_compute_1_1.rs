pub struct Device {}
impl Device {
    pub fn vk_cmd_dispatch_base(&self);
    pub fn vk_create_descriptor_update_template(&self);
    pub fn vk_destroy_descriptor_update_template(&self);
    pub fn vk_update_descriptor_set_with_template(&self);
    pub fn vk_create_sampler_ycbcr_conversion(&self);
    pub fn vk_destroy_sampler_ycbcr_conversion(&self);
    pub fn vk_get_descriptor_set_layout_support(&self);
}
