pub struct Device {}
impl Device {
    pub fn vk_set_debug_utils_object_name_ext(&self);
    pub fn vk_set_debug_utils_object_tag_ext(&self);
    pub fn vk_queue_begin_debug_utils_label_ext(&self);
    pub fn vk_queue_end_debug_utils_label_ext(&self);
    pub fn vk_queue_insert_debug_utils_label_ext(&self);
    pub fn vk_cmd_begin_debug_utils_label_ext(&self);
    pub fn vk_cmd_end_debug_utils_label_ext(&self);
    pub fn vk_cmd_insert_debug_utils_label_ext(&self);
}
