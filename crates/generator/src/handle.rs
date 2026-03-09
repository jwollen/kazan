use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::Write,
};

use anyhow::Result;

use crate::{doc_url, normalize_ty_name, xml};

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

pub fn generate_handles(
    file: &mut impl Write,
    analysis: &crate::analysis::Analysis,
    owned: &HashSet<&str>,
) -> Result<()> {
    let handles = analysis
        .registry()
        .handles
        .iter()
        .filter(|ty| owned.contains(ty.name));

    for handle in handles {
        let macro_name = match handle.ty {
            "VK_DEFINE_HANDLE" => "define_handle",
            "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => "handle_nondispatchable",
            _ => panic!(),
        };

        let name = normalize_ty_name(handle.name);
        let obj_type = handle.objtypeenum.strip_prefix("VK_OBJECT_TYPE_").unwrap();

        let doc_url = doc_url(handle.name);
        writeln!(
            file,
            "{macro_name}!({name}, {obj_type}, doc = \"<{doc_url}>\");"
        )?;
    }
    writeln!(file)?;
    Ok(())
}
