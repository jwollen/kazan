pub struct Device {}
impl Device {
    pub fn vk_get_execution_graph_pipeline_scratch_size_amdx(&self);
    pub fn vk_get_execution_graph_pipeline_node_index_amdx(&self);
    pub fn vk_create_execution_graph_pipelines_amdx(&self);
    pub fn vk_cmd_initialize_graph_scratch_memory_amdx(&self);
    pub fn vk_cmd_dispatch_graph_amdx(&self);
    pub fn vk_cmd_dispatch_graph_indirect_amdx(&self);
    pub fn vk_cmd_dispatch_graph_indirect_count_amdx(&self);
}
