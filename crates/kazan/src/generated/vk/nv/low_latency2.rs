pub struct Device {}
impl Device {
    pub fn vk_set_latency_sleep_mode_nv(&self);
    pub fn vk_latency_sleep_nv(&self);
    pub fn vk_set_latency_marker_nv(&self);
    pub fn vk_get_latency_timings_nv(&self);
    pub fn vk_queue_notify_out_of_band_nv(&self);
}
