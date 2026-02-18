pub struct Device {}
impl Device {
    pub fn vk_cmd_execute_generated_commands_ext(&self);
    pub fn vk_cmd_preprocess_generated_commands_ext(&self);
    pub fn vk_get_generated_commands_memory_requirements_ext(&self);
    pub fn vk_create_indirect_commands_layout_ext(&self);
    pub fn vk_destroy_indirect_commands_layout_ext(&self);
    pub fn vk_create_indirect_execution_set_ext(&self);
    pub fn vk_destroy_indirect_execution_set_ext(&self);
    pub fn vk_update_indirect_execution_set_pipeline_ext(&self);
    pub fn vk_update_indirect_execution_set_shader_ext(&self);
}
