pub struct Device {}
impl Device {
    pub fn vk_create_render_pass2(&self);
    pub fn vk_cmd_begin_render_pass2(&self);
    pub fn vk_cmd_next_subpass2(&self);
    pub fn vk_cmd_end_render_pass2(&self);
    pub fn vk_cmd_draw_indirect_count(&self);
    pub fn vk_cmd_draw_indexed_indirect_count(&self);
}
