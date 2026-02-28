use std::collections::{HashMap, VecDeque};

use crate::xml;

pub type HandleCommandTypes = HashMap<&'static str, CommandType>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CommandType {
    Entry,
    Instance,
    Device,
}

pub fn collect_handle_command_types(registry: &xml::Registry) -> HandleCommandTypes {
    let mut handle_command_types = HashMap::new();
    let mut pending_handles: VecDeque<_> = registry.handles.iter().collect();

    while let Some(handle) = pending_handles.pop_front() {
        if handle.name == "VkInstance" {
            handle_command_types.insert(handle.name, CommandType::Instance);
        } else if handle.name == "VkDevice" {
            handle_command_types.insert(handle.name, CommandType::Device);
        } else {
            let parent = handle.parent.unwrap();
            if let Some(parent_command_type) = handle_command_types.get(parent) {
                handle_command_types.insert(handle.name, *parent_command_type);
            } else {
                pending_handles.push_back(handle);
            }
        }
    }

    handle_command_types
}
