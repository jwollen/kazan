pub struct Device {}
impl Device {
    pub fn vk_create_deferred_operation_khr(&self);
    pub fn vk_destroy_deferred_operation_khr(&self);
    pub fn vk_get_deferred_operation_max_concurrency_khr(&self);
    pub fn vk_get_deferred_operation_result_khr(&self);
    pub fn vk_deferred_operation_join_khr(&self);
}
