use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap, HashSet},
    fs::{self, File},
    io::Write,
};

use heck::ToSnakeCase;

use crate::{
    analysis::Analysis,
    cdecl::CType,
    command::generate_commands,
    enums::{write_bitmask, write_enum},
    module::{Module, ModuleName},
    structs::write_struct,
    xml::Constant,
};

mod analysis;
mod cdecl;
mod command;
mod ctype_rust;
mod enums;
mod handle;
mod module;
mod overrides;
mod structs;
mod xml;

fn main() {
    let analysis = analysis::Analysis::new("crates/generator/external/Vulkan-Headers");

    generate(&analysis);
}

fn generate(analysis: &analysis::Analysis) {
    let registry = analysis.registry();

    let output_dir = "crates/kazan/src/generated/vk";

    let _ = fs::remove_dir_all(output_dir);

    struct ModuleEntry {
        name: String,
        provisional: bool,
    }

    let mut vendor_modules: BTreeMap<Option<String>, Vec<ModuleEntry>> = BTreeMap::new();

    // Build ownership map: for each item, determine which module should define it.
    // Prefer the module whose vendor tag matches the item's vendor suffix.
    let modules: Vec<_> = Module::from_registry(registry).collect();
    let item_owner = compute_item_owners(registry, &modules);

    for (module_index, module) in modules.iter().enumerate() {
        let requires: Vec<_> = module.requires();

        let required_types = requires.iter().map(|r| &r.types).flatten();

        let required_commands = requires.iter().map(|r| &r.commands).flatten();

        let required_api_constants =
            requires
                .iter()
                .map(|r| &r.constants)
                .flatten()
                .filter_map(|constant| {
                    let global_api_constant = registry
                        .constants
                        .iter()
                        .find(|api_constant| api_constant.name == constant.name);

                    if let Some(global_api_constant) = global_api_constant {
                        Some(global_api_constant.clone())
                    } else if let (Some(ty), Some(value)) = (constant.ty, constant.value) {
                        Some(Constant {
                            name: constant.name,
                            ty,
                            value,
                        })
                    } else {
                        None
                    }
                });

        let new_items = required_types
            .map(|ty| ty.name)
            .chain(required_commands.clone().map(|cmd| cmd.name))
            .chain(required_api_constants.clone().map(|constant| constant.name))
            .filter(|name| item_owner.get(name) == Some(&module_index))
            .collect::<HashSet<_>>();

        let new_commands = registry
            .commands
            .iter()
            .filter(|cmd| new_items.contains(cmd.name));

        let ModuleName {
            vendor,
            name: module_name,
        } = module.name();

        let provisional = matches!(module, Module::Extension(ext) if ext.provisional);
        vendor_modules
            .entry(vendor.clone())
            .or_insert_with(Vec::new)
            .push(ModuleEntry { name: module_name.clone(), provisional });

        let vendor_path = match vendor {
            Some(ref vendor) => format!("{}/{}", output_dir, vendor),
            None => output_dir.to_string(),
        };

        fs::create_dir_all(&vendor_path).unwrap();
        let mut file = File::create(format!("{}/{}.rs", &vendor_path, module_name)).unwrap();

        if provisional {
            writeln!(file, "#![cfg(feature = \"provisional\")]").unwrap();
        }

        if let Module::Extension(extension) = module {
            writeln!(
                file,
                "//! <https://registry.khronos.org/vulkan/specs/latest/man/html/{}.html>",
                extension.name
            )
            .unwrap();
        }

        writeln!(
            file,
            "#![allow(unused_imports)]
            use core::ffi::{{c_char, c_int, c_void, CStr}};
            use core::mem::transmute;
            use core::ptr;
            use crate::{{*, vk::*, vk::Result as VkResult}};
            "
        )
        .unwrap();

        if let Module::Extension(extension) = module {
            writeln!(
                file,
                "pub const EXTENSION_NAME: &CStr = c\"{}\";\n",
                extension.name
            )
            .unwrap();
        }

        writeln!(file, "pub(super) mod defs {{").unwrap();

        if !new_items.is_empty() {
            writeln!(
                file,
                "#![allow(non_camel_case_types, unused_imports)]
                use core::ffi::{{c_char, c_int, c_void, CStr}};
                use core::fmt;
                use core::marker::PhantomData;
                use core::ptr;
                use crate::{{*, vk::*}};
                "
            )
            .unwrap();

            generate_api_constants(&mut file, analysis, &new_items, required_api_constants);

            generate_basetypes(&mut file, analysis, &new_items);

            generate_handles(&mut file, analysis, &new_items);

            generate_type_aliases(&mut file, analysis, &new_items);

            generate_structs(&mut file, analysis, &new_items);

            generate_unions(&mut file, analysis, &new_items);

            generate_enum_types(&mut file, analysis, &new_items);

            generate_bitmask_types(&mut file, analysis, &new_items);

            generate_funcpointers(&mut file, analysis, &new_items);

            generate_functions(&mut file, analysis, new_commands.clone());
        }
        writeln!(file, "}}\n").unwrap();

        if required_commands.clone().next().is_some() {
            generate_commands(&mut file, analysis, &requires);
        }
    }

    fs::create_dir_all(output_dir).unwrap();
    let mut mod_file = File::create(format!("{}/mod.rs", output_dir)).unwrap();

    for (vendor, entries) in &vendor_modules {
        if let Some(vendor) = vendor {
            writeln!(mod_file, "pub mod {};", vendor).unwrap();
            writeln!(mod_file, "pub use {}::defs::*;", vendor).unwrap();

            fs::create_dir_all(format!("{}/{}", output_dir, vendor)).unwrap();
            let mut file = File::create(format!("{}/{}/mod.rs", output_dir, vendor)).unwrap();

            for entry in entries {
                let cfg = if entry.provisional { "#[cfg(feature = \"provisional\")]\n" } else { "" };
                write!(file, "pub mod {};\n", entry.name).unwrap();
            }

            writeln!(file, "pub(super) mod defs {{").unwrap();
            writeln!(file, "use super::*;").unwrap();
            for entry in entries {
                let cfg = if entry.provisional { "#[cfg(feature = \"provisional\")]\n" } else { "" };
                write!(file, "{}pub use {}::defs::*;\n", cfg, entry.name).unwrap();
            }
            writeln!(file, "}}").unwrap();
        } else {
            for entry in entries {
                writeln!(mod_file, "pub mod {};", entry.name).unwrap();
                writeln!(mod_file, "pub use {}::defs::*;", entry.name).unwrap();
            }
        }
    }

    std::process::Command::new("rustfmt")
        .arg(format!("{}/mod.rs", output_dir))
        .arg("--edition=2024")
        .output()
        .unwrap();
}

/// For each item (type, command, constant) required by any module, determines which
/// module should own its definition. If the item has a vendor suffix that matches
/// a specific module's vendor tag, that module is preferred. Otherwise, the first
/// module that requires the item wins.
fn compute_item_owners(
    registry: &xml::Registry,
    modules: &[Module],
) -> HashMap<&'static str, usize> {
    // Collect all items required by each module, along with the module index.
    let mut first_requirer: HashMap<&'static str, usize> = HashMap::new();
    let mut vendor_requirer: HashMap<&'static str, usize> = HashMap::new();

    for (index, module) in modules.iter().enumerate() {
        let module_vendor = match module {
            Module::Extension(ext) => registry.extension_vendor(ext.name),
            Module::Version(_) => None,
        };

        for require in module.requires() {
            let item_names = require
                .types
                .iter()
                .map(|t| t.name)
                .chain(require.commands.iter().map(|c| c.name))
                .chain(require.constants.iter().filter_map(|c| {
                    // Only include constants that actually resolve to a definition
                    let exists_global = registry
                        .constants
                        .iter()
                        .any(|api_constant| api_constant.name == c.name);
                    let has_inline_def = c.ty.is_some() && c.value.is_some();
                    if exists_global || has_inline_def {
                        Some(c.name)
                    } else {
                        None
                    }
                }));

            for name in item_names {
                first_requirer.entry(name).or_insert(index);

                if let Some(item_vendor) = registry.vendor_suffix(name) {
                    if module_vendor == Some(item_vendor) {
                        vendor_requirer.entry(name).or_insert(index);
                    }
                }
            }
        }
    }

    // For each item, prefer the vendor-matched module; fall back to first requirer.
    let mut owners = HashMap::new();
    for (name, first) in &first_requirer {
        let owner = vendor_requirer.get(name).unwrap_or(first);
        owners.insert(*name, *owner);
    }
    owners
}

fn generate_api_constants<'a>(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    new_items: &HashSet<&str>,
    required_api_constants: impl Iterator<Item = xml::Constant>,
) {
    let constants = required_api_constants.filter(|constant| new_items.contains(constant.name));

    for constant in constants {
        writeln!(
            file,
            "pub const {}: {} = {};",
            normalize_const_name(constant.name),
            ctype_to_rust_type_str(constant.ty),
            convert_c_expr(constant.value),
        )
        .unwrap();
    }
    writeln!(file).unwrap();
}

fn generate_basetypes(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    new_items: &HashSet<&str>,
) {
    let basetypes = analysis
        .registry()
        .basetypes
        .iter()
        .filter(|ty| new_items.contains(ty.name));

    for ty in basetypes {
        writeln!(
            file,
            "pub type {} = {};",
            normalize_ty_name(ty.name),
            ctype_to_rust_type_str(ty.ty.unwrap_or("*const c_void"))
        )
        .unwrap();
    }
    writeln!(file).unwrap();
}

fn generate_handles(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    new_items: &HashSet<&str>,
) {
    let handles = analysis
        .registry()
        .handles
        .iter()
        .filter(|ty| new_items.contains(ty.name));

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
        )
        .unwrap();
    }
    writeln!(file).unwrap();
}

fn generate_type_aliases(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    new_items: &HashSet<&str>,
) {
    let registry = analysis.registry();
    let aliases = registry
        .enum_aliases
        .iter()
        .clone()
        .filter(|alias| {
            registry
                .enums
                .iter()
                .find(|ty| ty.name == alias.alias)
                .is_some()
        })
        .chain(registry.handle_aliases.iter())
        .chain(registry.struct_aliases.iter())
        .chain(registry.bitmask_aliases.iter())
        .filter(|alias| new_items.contains(alias.name));

    for alias in aliases {
        write_doc_link(file, alias.name);
        writeln!(
            file,
            "pub type {} = {};",
            type_name_with_lifetime(analysis, alias.name, Some("a")),
            type_name_with_lifetime(analysis, alias.alias, Some("a"))
        )
        .unwrap();
    }

    let command_aliases = registry
        .command_aliases
        .iter()
        .filter(|alias| new_items.contains(alias.name));

    for ty in command_aliases {
        writeln!(
            file,
            "pub type PFN_{} = PFN_{};",
            normalize_ty_name(ty.name),
            normalize_ty_name(ty.alias)
        )
        .unwrap();
    }
    writeln!(file).unwrap();
}

fn generate_structs(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    new_items: &HashSet<&str>,
) {
    let new_structs = analysis
        .registry()
        .structs
        .iter()
        .filter(|ty| new_items.contains(ty.name));

    for ty in new_structs {
        write_struct(file, analysis, ty);
    }
}

fn generate_unions(file: &mut impl std::io::Write, analysis: &Analysis, new_items: &HashSet<&str>) {
    let unions = analysis
        .registry()
        .unions
        .iter()
        .filter(|ty| new_items.contains(ty.name));
    for ty in unions {
        let name = normalize_ty_name(ty.name);
        let type_info = analysis.get_base_type_info(ty.name).unwrap();
        write_doc_link(file, ty.name);
        writeln!(
            file,
            "#[repr(C)]
            #[derive(Copy, Clone)]
            pub union {}{} {{",
            name,
            if type_info.lifetime_param { "<'a>" } else { "" }
        )
        .unwrap();
        for member in &ty.members {
            let field_ty = ctype_to_rust_type(analysis, &member.c_decl.ty, Some("a"));
            writeln!(
                file,
                "pub {}: {},",
                normalize_name(member.c_decl.name),
                field_ty
            )
            .unwrap();
        }
        if type_info.lifetime_param {
            writeln!(file, "pub _marker: PhantomData<&'a ()>,",).unwrap();
        }
        writeln!(file, "}}\n").unwrap();
        let anon = if type_info.lifetime_param { "<'_>" } else { "" };
        writeln!(
            file,
            "#[cfg(feature = \"debug\")]
            impl fmt::Debug for {name}{anon} {{
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
                    f.debug_struct(\"{name}\").finish()
                }}
            }}\n"
        )
        .unwrap();
        writeln!(
            file,
            "impl Default for {name}{anon} {{
                fn default() -> Self {{
                    unsafe {{ core::mem::zeroed() }}
                }}
            }}\n"
        )
        .unwrap();
    }
}

fn generate_enum_types(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    new_items: &HashSet<&str>,
) {
    let enums = analysis
        .registry()
        .enums
        .iter()
        .filter(|ty| new_items.contains(ty.name));

    for ty in enums {
        write_enum(file, analysis, ty);
    }
}

fn generate_bitmask_types(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    new_items: &HashSet<&str>,
) {
    let bitmask_types = analysis
        .registry()
        .bitmask_types
        .iter()
        .clone()
        .filter(|ty| new_items.contains(ty.name));

    for ty in bitmask_types {
        let bitmask = ty.bitvalues.or(ty.requires).and_then(|b| {
            analysis
                .registry()
                .bitmasks
                .iter()
                .find(|bitmask| bitmask.name == b)
        });

        write_bitmask(file, analysis, ty, bitmask, analysis.req_enum_data());
    }
}

fn generate_funcpointers(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    new_items: &HashSet<&str>,
) {
    let funcpointers = analysis
        .registry()
        .funcpointers
        .iter()
        .clone()
        .filter(|ty| new_items.contains(ty.name));

    for ty in funcpointers {
        write_doc_link(file, ty.name);
        writeln!(file, "pub type {} = unsafe extern \"system\" fn(", ty.name).unwrap();
        for param in &ty.params {
            writeln!(
                file,
                "    {}: {},",
                normalize_name(param.c_decl.name),
                ctype_to_rust_type(analysis, &param.c_decl.ty, None)
            )
            .unwrap();
        }
        if let Some(ref return_type) = ty.return_type {
            writeln!(
                file,
                ") -> {};",
                ctype_to_rust_type(analysis, &return_type, None)
            )
            .unwrap();
        } else {
            writeln!(file, ");").unwrap();
        }
    }
    writeln!(file).unwrap();
}

fn generate_functions<'a>(
    file: &mut impl std::io::Write,
    analysis: &Analysis,
    new_commands: impl Iterator<Item = &'a xml::Command>,
) {
    let mut count = 0;
    for command in new_commands {
        write_doc_link(file, command.name);
        writeln!(
            file,
            "pub type PFN_{} = unsafe extern \"system\" fn(",
            command.name
        )
        .unwrap();
        for param in &command.params {
            writeln!(
                file,
                "    {}: {},",
                normalize_name(param.c_decl.name),
                ctype_to_rust_type(analysis, &param.c_decl.ty, None)
            )
            .unwrap();
        }
        if let Some(ref return_type) = command.return_type {
            writeln!(
                file,
                ") -> {};",
                ctype_to_rust_type(analysis, &return_type, None)
            )
            .unwrap();
        } else {
            writeln!(file, ");").unwrap();
        }
    }
    writeln!(file).unwrap();
}

fn convert_c_expr<'a>(expr: &'a str) -> Cow<'a, str> {
    let expr = expr
        .strip_prefix('(')
        .and_then(|expr| expr.strip_suffix(')'))
        .unwrap_or(expr);

    let expr = expr
        .strip_suffix('f')
        .or_else(|| expr.strip_suffix('F'))
        .or_else(|| expr.strip_suffix("ULL"))
        .or_else(|| expr.strip_suffix("LL"))
        .or_else(|| expr.strip_suffix('U'))
        .unwrap_or(expr);

    if let Some(expr) = expr.strip_prefix("~") {
        Cow::Owned(format!("!{}", expr))
    } else {
        Cow::Borrowed(expr)
    }
}

pub(crate) fn doc_url(name: &str) -> String {
    format!("https://registry.khronos.org/vulkan/specs/latest/man/html/{name}.html")
}

/// Write a doc comment linking to the Vulkan spec for the given Vk-prefixed name.
pub(crate) fn write_doc_link(file: &mut impl std::io::Write, name: &str) {
    let url = doc_url(name);
    writeln!(file, "/// <{url}>").unwrap();
}

fn normalize_name(name: &str) -> String {
    match name {
        "type" => "ty".to_string(),
        _ => name.to_snake_case(),
    }
}

pub(crate) fn normalize_const_name(name: &str) -> &str {
    name.strip_prefix("VK_").unwrap_or(name)
}

pub(crate) fn normalize_ty_name(name: &str) -> &str {
    if name == "VkResult" {
        "vk::Result"
    } else {
        name.strip_prefix("Vk").unwrap_or(name)
    }
}

fn type_name_with_lifetime(analysis: &Analysis, name: &str, lifetime: Option<&str>) -> String {
    let type_info = analysis.get_base_type_info(name).unwrap();
    let name = ctype_to_rust_type_str(name);
    if type_info.lifetime_param {
        format!("{}<'{}>", name, lifetime.unwrap_or("_"))
    } else {
        name.to_string()
    }
}

// TODO: Replace occurances with type_name_with_lifetime
pub(crate) fn ctype_to_rust_type_str(name: &str) -> &str {
    match name {
        "int8_t" => "i8",
        "uint8_t" => "u8",
        "int16_t" => "i16",
        "uint16_t" => "u16",
        "int32_t" => "i32",
        "uint32_t" => "u32",
        "int64_t" => "i64",
        "uint64_t" => "u64",
        "size_t" => "usize",
        "float" => "f32",
        "double" => "f64",
        "void" => "c_void",
        "char" => "c_char",
        "int" => "c_int",
        _ => normalize_ty_name(name),
    }
}

fn ctype_to_rust_type(analysis: &Analysis, ty: &CType, lifetime: Option<&str>) -> String {
    match ty {
        CType::Base(base) => type_name_with_lifetime(analysis, base.name, lifetime),
        CType::Ptr {
            pointee,
            implicit_for_decay,
            is_const,
            ..
        } => {
            let pointee = ctype_to_rust_type(analysis, pointee.as_ref(), lifetime);
            if *is_const {
                format!("*const {}", pointee).to_string()
            } else {
                format!("*mut {}", pointee).to_string()
            }
        }
        CType::Array { element, len } => {
            let element_ty = ctype_to_rust_type(analysis, element.as_ref(), lifetime);
            match len {
                cdecl::CArrayLen::Named(name) => {
                    format!("[{}; {} as usize]", element_ty, normalize_const_name(name))
                }
                cdecl::CArrayLen::Literal(len) => format!("[{}; {}]", element_ty, len),
            }
        }
        CType::Func { .. } => todo!(),
    }
}

#[derive(Clone, Debug)]
enum LengthKind<'a> {
    NullTerminated,
    Literal(u32),
    Param {
        index: usize,
        c_decl: &'a cdecl::CDecl<'static>,
    },
    ParamField {
        index: usize,
        field: &'a xml::StructureMember,
    },
    Unknown(&'static str),
}

trait Param {
    fn c_decl(&self) -> &cdecl::CDecl<'static>;
}

impl Param for xml::CommandParam {
    fn c_decl(&self) -> &cdecl::CDecl<'static> {
        &self.c_decl
    }
}

impl Param for xml::StructureMember {
    fn c_decl(&self) -> &cdecl::CDecl<'static> {
        &self.c_decl
    }
}

fn get_param_index(params: &[impl Param], param_name: &str) -> Option<usize> {
    params.iter().enumerate().find_map(|(index, other_param)| {
        if other_param.c_decl().name == param_name {
            Some(index)
        } else {
            None
        }
    })
}

fn get_len_kind<'a>(
    analysis: &'a Analysis,
    params: &'a [impl Param],
    len: &'static str,
) -> LengthKind<'a> {
    if len == "null-terminated" {
        LengthKind::NullTerminated
    } else if let Ok(len) = len.parse() {
        LengthKind::Literal(len)
    } else if let Some((param_name, field_name)) = len.split_once("->")
        && let Some(index) = get_param_index(params, param_name)
    {
        let param = &params[index];
        let param_ty = &param.c_decl().ty;
        let CType::Ptr { pointee, .. } = param_ty else {
            panic!("expected pointer type, got {:?}", param_ty);
        };
        let CType::Base(base) = pointee.as_ref() else {
            panic!("expected base type, got {:?}", pointee);
        };

        let struct_ty = analysis
            .registry()
            .structs
            .iter()
            .find(|ty| ty.name == base.name)
            .unwrap_or_else(|| panic!("failed to find struct {}", base.name));

        let field = struct_ty
            .members
            .iter()
            .find(|field| field.c_decl.name == field_name)
            .unwrap_or_else(|| panic!("failed to find field {}", field_name));

        LengthKind::ParamField { index, field }
    } else if let Some(index) = get_param_index(params, len) {
        let param = &params[index];
        LengthKind::Param {
            index,
            c_decl: param.c_decl(),
        }
    } else {
        LengthKind::Unknown(len)
    }
}

fn is_opaque_type(ty: &str) -> bool {
    matches!(
        ty,
        "void"
            | "wl_display"
            | "wl_surface"
            | "Display"
            | "xcb_connection_t"
            | "ANativeWindow"
            | "AHardwareBuffer"
            | "CAMetalLayer"
            | "IDirectFB"
            | "IDirectFBSurface"
            | "_screen_buffer"
            | "_screen_context"
            | "_screen_window"
            | "SECURITY_ATTRIBUTES"
    )
}

pub fn normalize_param_name(name: &str) -> String {
    let name = normalize_name(name);

    name.strip_prefix("pp_")
        .or_else(|| name.strip_prefix("p_"))
        .unwrap_or(name.as_str())
        .to_string()
}

pub fn normalize_setter_param_name(name: &str) -> String {
    let name = normalize_name(name);

    let name = if name.starts_with("pp_") {
        format!("{}_ptrs", name)
    } else {
        name
    };

    name.strip_prefix("pp_")
        .or_else(|| name.strip_prefix("p_"))
        .unwrap_or(name.as_str())
        .to_string()
}

pub fn normalize_command_name(name: &str) -> String {
    name.strip_prefix("vk").unwrap().to_snake_case()
}
