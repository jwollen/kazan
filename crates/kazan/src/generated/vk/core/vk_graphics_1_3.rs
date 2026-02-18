pub struct Device {}
impl Device {
    pub fn vk_cmd_set_cull_mode(&self);
    pub fn vk_cmd_set_front_face(&self);
    pub fn vk_cmd_set_primitive_topology(&self);
    pub fn vk_cmd_set_viewport_with_count(&self);
    pub fn vk_cmd_set_scissor_with_count(&self);
    pub fn vk_cmd_bind_vertex_buffers2(&self);
    pub fn vk_cmd_set_depth_test_enable(&self);
    pub fn vk_cmd_set_depth_write_enable(&self);
    pub fn vk_cmd_set_depth_compare_op(&self);
    pub fn vk_cmd_set_depth_bounds_test_enable(&self);
    pub fn vk_cmd_set_stencil_test_enable(&self);
    pub fn vk_cmd_set_stencil_op(&self);
    pub fn vk_cmd_set_rasterizer_discard_enable(&self);
    pub fn vk_cmd_set_depth_bias_enable(&self);
    pub fn vk_cmd_set_primitive_restart_enable(&self);
    pub fn vk_cmd_blit_image2(&self);
    pub fn vk_cmd_resolve_image2(&self);
    pub fn vk_cmd_begin_rendering(&self);
    pub fn vk_cmd_end_rendering(&self);
}
