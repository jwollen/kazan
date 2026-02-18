pub struct Device {}
impl Device {
    pub fn vk_create_video_session_khr(&self);
    pub fn vk_destroy_video_session_khr(&self);
    pub fn vk_create_video_session_parameters_khr(&self);
    pub fn vk_update_video_session_parameters_khr(&self);
    pub fn vk_destroy_video_session_parameters_khr(&self);
    pub fn vk_get_video_session_memory_requirements_khr(&self);
    pub fn vk_bind_video_session_memory_khr(&self);
    pub fn vk_cmd_begin_video_coding_khr(&self);
    pub fn vk_cmd_control_video_coding_khr(&self);
    pub fn vk_cmd_end_video_coding_khr(&self);
}
