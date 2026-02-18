pub struct Device {}
impl Device {
    pub fn vk_initialize_performance_api_intel(&self);
    pub fn vk_uninitialize_performance_api_intel(&self);
    pub fn vk_cmd_set_performance_marker_intel(&self);
    pub fn vk_cmd_set_performance_stream_marker_intel(&self);
    pub fn vk_cmd_set_performance_override_intel(&self);
    pub fn vk_acquire_performance_configuration_intel(&self);
    pub fn vk_release_performance_configuration_intel(&self);
    pub fn vk_queue_set_performance_configuration_intel(&self);
    pub fn vk_get_performance_parameter_intel(&self);
}
