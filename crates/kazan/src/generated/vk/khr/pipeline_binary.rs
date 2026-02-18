pub struct Device {}
impl Device {
    pub fn vk_create_pipeline_binaries_khr(&self);
    pub fn vk_destroy_pipeline_binary_khr(&self);
    pub fn vk_get_pipeline_key_khr(&self);
    pub fn vk_get_pipeline_binary_data_khr(&self);
    pub fn vk_release_captured_pipeline_data_khr(&self);
}
