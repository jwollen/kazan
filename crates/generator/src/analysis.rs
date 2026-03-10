use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    path::Path,
};
use tracing::{debug, error_span};

use crate::{
    cdecl::CType,
    ctype_rust,
    enums::ReqEnumData,
    external,
    handle::{HandleCommandTypes, collect_handle_command_types},
    module::Module,
    xml,
};

/// Holds the analysis results for easy querying.
pub struct Analysis {
    module_items: Vec<ModuleItems<'static>>,
    registry: &'static xml::Registry,
    custom_types: CustomTypes,
    type_refs: TypeRefs,
    type_infos: BTreeMap<&'static str, TypeInfo>,
    handle_command_types: HandleCommandTypes,
    req_enum_data: ReqEnumData,
    provisional_types: HashSet<&'static str>,
    provisional_extensions: HashSet<&'static str>,
    opaque_types: HashSet<&'static str>,
}

impl Analysis {
    /// Analyse the provided copy of the
    /// [Vulkan-Headers](https://github.com/KhronosGroup/Vulkan-Headers) repo.
    pub fn new(vulkan_headers_path: impl AsRef<Path>) -> Analysis {
        let vulkan_headers_path = vulkan_headers_path.as_ref();

        let paths = [
            vulkan_headers_path.join("registry/vk.xml"),
            vulkan_headers_path.join("registry/video.xml"),
        ];

        let mut registry = xml::Registry::default();
        for path in paths {
            let xml = error_span!("xml", path = %path.display()).in_scope(|| {
                debug!("reading xml");
                // We leak the input string here for convenience, to avoid explicit lifetimes.
                let xml_input = Box::leak(fs::read_to_string(path).unwrap().into_boxed_str());
                debug!("parsing xml");
                xml::Registry::parse(xml_input, "vulkan")
            });
            registry.merge(xml);
        }

        // Leak the registry for convenience, like the XML input strings.
        let registry: &'static xml::Registry = Box::leak(Box::new(registry));

        let custom_types = external::external_types();
        let type_refs = build_type_refs(registry, &custom_types);

        // Assert every external type from vk.xml is resolvable (either a
        // primitive or defined in overrides::external_types).
        for ext in &registry.externals {
            assert!(
                type_refs.contains_key(ext.name),
                "External type {:?} from vk.xml has no type definition. \
                 Add it to external::external_types().",
                ext.name,
            );
        }
        let type_infos = compute_type_infos(registry, &custom_types, &type_refs);
        let handle_command_types = collect_handle_command_types(registry);
        let req_enum_data = ReqEnumData::from_registry(registry);
        let module_items = compute_module_items(registry);

        let provisional_extensions: HashSet<&'static str> = registry
            .extensions
            .iter()
            .filter(|ext| ext.provisional)
            .map(|ext| ext.name)
            .collect();

        let provisional_types: HashSet<&'static str> = registry
            .extensions
            .iter()
            .filter(|ext| ext.provisional)
            .flat_map(|ext| ext.requires.iter())
            .flat_map(|req| req.types.iter())
            .map(|ty| ty.name)
            .collect();

        // Opaque types: "void" itself, external types aliasing void, plus
        // basetypes with no underlying C type (forward-declared platform
        // structs like ANativeWindow).
        let opaque_types: HashSet<&'static str> = ["void"]
            .into_iter()
            .chain(
                custom_types
                    .iter()
                    .filter(|(_, ty)| *ty == CType::VOID)
                    .map(|(name, _)| *name),
            )
            .chain(
                registry
                    .basetypes
                    .iter()
                    .filter(|bt| bt.ty.is_none())
                    .map(|bt| bt.name),
            )
            .collect();

        Self {
            module_items,
            registry,
            custom_types,
            type_refs,
            type_infos,
            handle_command_types,
            req_enum_data,
            provisional_types,
            provisional_extensions,
            opaque_types,
        }
    }

    /// Get "raw" Vulkan XML registry.
    pub fn registry(&self) -> &'static xml::Registry {
        self.registry
    }

    pub fn types(&self) -> Types<'_> {
        Types {
            registry: self.registry,
            custom_types: &self.custom_types,
            type_refs: &self.type_refs,
        }
    }

    pub fn get_base_type_info(&self, name: &str) -> Option<TypeInfo> {
        get_base_type_info(&self.types(), &self.type_infos, name)
    }

    pub fn get_type_info(&self, ty: &CType<'_>) -> Option<TypeInfo> {
        get_type_info(&self.types(), &self.type_infos, ty)
    }

    pub fn handle_command_types(&self) -> &HandleCommandTypes {
        &self.handle_command_types
    }

    pub fn req_enum_data(&self) -> &ReqEnumData {
        &self.req_enum_data
    }

    pub fn module_items(&self, module_index: usize) -> &ModuleItems<'_> {
        &self.module_items[module_index]
    }

    /// Returns true if the named C type belongs to a provisional extension.
    pub fn is_provisional_type(&self, name: &str) -> bool {
        self.provisional_types.contains(name)
    }

    /// Returns true if the named extension is provisional.
    pub fn is_provisional_extension(&self, name: &str) -> bool {
        self.provisional_extensions.contains(name)
    }

    /// Returns true if `name` (a C type name) is an opaque type whose pointers
    /// should stay raw (`*const` / `*mut`) rather than being converted to Rust references.
    pub fn is_opaque_type_name(&self, name: &str) -> bool {
        self.opaque_types.contains(name)
    }

    /// Returns true if `name` (a C type name) is a bitmask/flags type.
    pub fn is_bitmask_type(&self, name: &str) -> bool {
        self.registry.bitmask_types.iter().any(|bt| bt.name == name)
            || self.registry.bitmask_aliases.iter().any(|a| a.name == name)
    }

    /// Like [`is_opaque_type_name`], but extracts the base name from a `CType`.
    pub fn is_opaque_type(&self, ty: &CType) -> bool {
        ctype_rust::base_name(ty).is_some_and(|name| self.opaque_types.contains(name))
    }

    /// Returns true if the named type is a struct with an `sType` member (i.e. extensible via pNext).
    pub fn is_extensible_struct(&self, type_name: &str) -> bool {
        let types = self.types();
        let mut name = type_name;
        loop {
            match types.get(name) {
                Some(TypeKind::Struct(s)) => {
                    return s.members.iter().any(|m| m.c_decl.name == "sType");
                }
                Some(TypeKind::Alias(a)) => {
                    name = a.alias;
                }
                _ => return false,
            }
        }
    }
}

/// Items owned by a module, pre-filtered and split by registry type category.
pub struct ModuleItems<'a> {
    pub api_constants: Vec<xml::Constant>,
    pub structs: Vec<&'a xml::Structure>,
    pub unions: Vec<&'a xml::Structure>,
    pub enums: Vec<&'a xml::Enum>,
    pub bitmask_types: Vec<&'a xml::BitMaskType>,
    pub handles: Vec<&'a xml::Handle>,
    pub basetypes: Vec<&'a xml::BaseType>,
    pub funcpointers: Vec<&'a xml::FuncPointer>,
    pub commands: Vec<&'a xml::Command>,
    pub type_aliases: Vec<&'a xml::Alias>,
    pub command_aliases: Vec<&'a xml::Alias>,
}

impl ModuleItems<'_> {
    /// Returns true if this module owns any items at all.
    pub fn is_empty(&self) -> bool {
        self.api_constants.is_empty()
            && self.structs.is_empty()
            && self.unions.is_empty()
            && self.enums.is_empty()
            && self.bitmask_types.is_empty()
            && self.handles.is_empty()
            && self.basetypes.is_empty()
            && self.funcpointers.is_empty()
            && self.commands.is_empty()
            && self.type_aliases.is_empty()
            && self.command_aliases.is_empty()
    }

    /// Returns true if this module has any types that would produce ffi aliases.
    pub fn has_ffi_types(&self) -> bool {
        !self.structs.is_empty()
            || !self.unions.is_empty()
            || !self.enums.is_empty()
            || !self.bitmask_types.is_empty()
            || !self.handles.is_empty()
            || self
                .basetypes
                .iter()
                .any(|ty| crate::normalize_ty_name(ty.name) != ty.name)
            || !self.type_aliases.is_empty()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum TypeKind<'a> {
    Struct(&'a xml::Structure),
    Union(&'a xml::Structure),
    EnumType(&'a xml::EnumType),
    BitmaskType(&'a xml::BitMaskType),
    Handle(&'a xml::Handle),
    BaseType(&'a xml::BaseType),
    FuncPointer(&'a xml::FuncPointer),

    Alias(&'a xml::Alias),

    Primitive(PrimitiveType),
    Custom(&'a CType<'static>),
}

#[derive(Debug)]
enum TypeKindRef {
    Struct(usize),
    Union(usize),
    EnumType(usize),
    BitmaskType(usize),
    Handle(usize),
    BaseType(usize),
    FuncPointer(usize),

    StructAlias(usize),
    //UnionAlias(usize),
    EnumTypeAlias(usize),
    BitmaskAlias(usize),
    HandleAlias(usize),

    Primitive(PrimitiveType),
    Custom(usize),
}

type TypeRefs = BTreeMap<&'static str, TypeKindRef>;

pub struct Types<'a> {
    registry: &'a xml::Registry,
    custom_types: &'a CustomTypes,
    type_refs: &'a TypeRefs,
}

impl<'a> Types<'a> {
    fn get(&self, name: &str) -> Option<TypeKind<'a>> {
        match *self.type_refs.get(name)? {
            TypeKindRef::Struct(index) => Some(TypeKind::Struct(&self.registry.structs[index])),
            TypeKindRef::Union(index) => Some(TypeKind::Union(&self.registry.unions[index])),
            TypeKindRef::EnumType(index) => {
                Some(TypeKind::EnumType(&self.registry.enum_types[index]))
            }
            TypeKindRef::BitmaskType(index) => {
                Some(TypeKind::BitmaskType(&self.registry.bitmask_types[index]))
            }
            TypeKindRef::Handle(index) => Some(TypeKind::Handle(&self.registry.handles[index])),
            TypeKindRef::BaseType(index) => {
                Some(TypeKind::BaseType(&self.registry.basetypes[index]))
            }
            TypeKindRef::FuncPointer(index) => {
                Some(TypeKind::FuncPointer(&self.registry.funcpointers[index]))
            }
            TypeKindRef::StructAlias(index) => {
                Some(TypeKind::Alias(&self.registry.struct_aliases[index]))
            }
            //TypeKindRef::UnionAlias(index) => Some(TypeKind::Alias(&self.registry.union_aliases[index])),
            TypeKindRef::EnumTypeAlias(index) => {
                Some(TypeKind::Alias(&self.registry.enum_aliases[index]))
            }
            TypeKindRef::BitmaskAlias(index) => {
                Some(TypeKind::Alias(&self.registry.bitmask_aliases[index]))
            }
            TypeKindRef::HandleAlias(index) => {
                Some(TypeKind::Alias(&self.registry.handle_aliases[index]))
            }
            TypeKindRef::Primitive(primitive) => Some(TypeKind::Primitive(primitive)),
            TypeKindRef::Custom(index) => Some(TypeKind::Custom(&self.custom_types[index].1)),
        }
    }
}

fn build_type_refs(registry: &xml::Registry, custom_types: &CustomTypes) -> TypeRefs {
    let mut types = BTreeMap::new();

    types.extend(
        registry
            .structs
            .iter()
            .enumerate()
            .map(|(index, ty)| (ty.name, TypeKindRef::Struct(index))),
    );
    types.extend(
        registry
            .unions
            .iter()
            .enumerate()
            .map(|(index, ty)| (ty.name, TypeKindRef::Union(index))),
    );
    types.extend(
        registry
            .enum_types
            .iter()
            .enumerate()
            .map(|(index, ty)| (ty.name, TypeKindRef::EnumType(index))),
    );
    types.extend(
        registry
            .bitmask_types
            .iter()
            .enumerate()
            .map(|(index, ty)| (ty.name, TypeKindRef::BitmaskType(index))),
    );
    types.extend(
        registry
            .handles
            .iter()
            .enumerate()
            .map(|(index, ty)| (ty.name, TypeKindRef::Handle(index))),
    );
    types.extend(
        registry
            .basetypes
            .iter()
            .enumerate()
            .map(|(index, ty)| (ty.name, TypeKindRef::BaseType(index))),
    );
    types.extend(
        registry
            .funcpointers
            .iter()
            .enumerate()
            .map(|(index, ty)| (ty.name, TypeKindRef::FuncPointer(index))),
    );

    types.extend(
        registry
            .struct_aliases
            .iter()
            .enumerate()
            .map(|(index, ty)| (ty.name, TypeKindRef::StructAlias(index))),
    );
    types.extend(
        registry
            .enum_aliases
            .iter()
            .enumerate()
            .map(|(index, ty)| (ty.name, TypeKindRef::EnumTypeAlias(index))),
    );
    types.extend(
        registry
            .bitmask_aliases
            .iter()
            .enumerate()
            .map(|(index, ty)| (ty.name, TypeKindRef::BitmaskAlias(index))),
    );
    types.extend(
        registry
            .handle_aliases
            .iter()
            .enumerate()
            .map(|(index, ty)| (ty.name, TypeKindRef::HandleAlias(index))),
    );

    types.extend(
        custom_types
            .iter()
            .enumerate()
            .map(|(index, ty)| (ty.0, TypeKindRef::Custom(index))),
    );

    types.extend([
        ("int8_t", TypeKindRef::Primitive(PrimitiveType::Int8)),
        ("uint8_t", TypeKindRef::Primitive(PrimitiveType::Uint8)),
        ("int16_t", TypeKindRef::Primitive(PrimitiveType::Int16)),
        ("uint16_t", TypeKindRef::Primitive(PrimitiveType::Uint16)),
        ("int32_t", TypeKindRef::Primitive(PrimitiveType::Int32)),
        ("uint32_t", TypeKindRef::Primitive(PrimitiveType::Uint32)),
        ("int64_t", TypeKindRef::Primitive(PrimitiveType::Int64)),
        ("uint64_t", TypeKindRef::Primitive(PrimitiveType::Uint64)),
        ("size_t", TypeKindRef::Primitive(PrimitiveType::Size)),
        ("isize_t", TypeKindRef::Primitive(PrimitiveType::Isize)),
        ("float", TypeKindRef::Primitive(PrimitiveType::Float)),
        ("double", TypeKindRef::Primitive(PrimitiveType::Double)),
        ("void", TypeKindRef::Primitive(PrimitiveType::Void)),
        ("char", TypeKindRef::Primitive(PrimitiveType::Char)),
        ("int", TypeKindRef::Primitive(PrimitiveType::Int)),
        ("unsigned int", TypeKindRef::Primitive(PrimitiveType::Uint)),
        (
            "unsigned long",
            TypeKindRef::Primitive(PrimitiveType::Ulong),
        ),
    ]);

    types
}

pub type TypeInfos = BTreeMap<&'static str, TypeInfo>;

#[derive(Debug, Clone, Copy)]
pub struct TypeInfo {
    pub default: bool,
    pub clone: bool,
    pub pod: bool,
    pub lifetime_param: bool,
    pub lifetime: bool,
    /// Whether a simple `#[derive(Debug)]` works for this type.
    /// False when the type contains pointers, function pointers, or `[c_char; N]` arrays.
    pub trivial_debug: bool,
}

impl TypeInfo {
    const POD: TypeInfo = TypeInfo {
        default: true,
        clone: true,
        pod: true,
        lifetime_param: false,
        lifetime: false,
        trivial_debug: true,
    };

    const OPAQUE: TypeInfo = TypeInfo {
        default: false,
        clone: false,
        pod: false,
        lifetime_param: false,
        lifetime: false,
        trivial_debug: false,
    };

    const FN_POINTER: TypeInfo = TypeInfo {
        default: true,
        clone: true,
        pod: true,
        lifetime_param: false,
        lifetime: false,
        trivial_debug: false,
    };

    const POINTER: TypeInfo = TypeInfo {
        default: false,
        clone: false,
        pod: false,
        lifetime_param: false,
        lifetime: true,
        trivial_debug: false,
    };

    fn empty_struct() -> Self {
        Self {
            default: true,
            clone: true,
            pod: true,
            lifetime_param: false,
            lifetime: false,
            trivial_debug: true,
        }
    }

    fn merge(&mut self, other: &TypeInfo) {
        self.default &= other.default;
        self.clone &= other.clone;
        self.pod &= other.pod;
        self.lifetime_param |= other.lifetime_param | other.lifetime;
        self.trivial_debug &= other.trivial_debug;
    }
}

fn compute_type_infos(
    registry: &xml::Registry,
    custom_types: &CustomTypes,
    type_refs: &TypeRefs,
) -> TypeInfos {
    let types = Types {
        registry,
        custom_types,
        type_refs,
    };

    let mut type_infos = BTreeMap::new();
    let mut pending_types = Vec::new();
    let mut visited_types = HashSet::new();

    for ty in registry.structs.iter().chain(registry.unions.iter()) {
        pending_types.push((ty.name, false));
        while let Some((ty_name, complete)) = pending_types.pop() {
            let ty = types.get(ty_name).unwrap();
            let (TypeKind::Struct(ty) | TypeKind::Union(ty)) = ty else {
                continue;
            };

            if type_infos.contains_key(ty_name) {
                continue;
            }

            if !complete {
                // Schedule this type again after its member types are processed
                pending_types.push((ty_name, true));

                for member in &ty.members {
                    if let Some(base_ty_name) = get_base_type(&member.c_decl.ty) {
                        // Detect cycles
                        if visited_types.insert(base_ty_name) {
                            pending_types.push((base_ty_name, false));
                        }
                    }
                }
            } else {
                let mut type_info = TypeInfo::empty_struct();

                for member in &ty.members {
                    let member_ty = &member.c_decl.ty;

                    // No TypeInfo means this is a circular type. Introduce a lifetime.
                    let member_ty_info =
                        get_type_info(&types, &type_infos, member_ty).unwrap_or(TypeInfo {
                            lifetime_param: true,
                            ..TypeInfo::POINTER
                        });

                    type_info.merge(&member_ty_info);
                }

                type_infos.insert(ty_name, type_info);
            }
        }
    }

    type_infos
}

fn get_base_type<'a>(ty: &CType<'a>) -> Option<&'a str> {
    match ty {
        CType::Base(base) => Some(base.name),
        CType::Array { element, .. } => get_base_type(element),
        CType::Ptr { pointee, .. } => get_base_type(pointee),
        CType::Func { .. } => todo!(),
    }
}

fn get_type_info<'a>(
    types: &Types<'a>,
    type_infos: &'a BTreeMap<&'static str, TypeInfo>,
    ty: &CType,
) -> Option<TypeInfo> {
    match ty {
        CType::Base(base) => get_base_type_info(types, type_infos, base.name),
        CType::Array { element, .. } => {
            let mut info = get_type_info(types, type_infos, element)?;
            info.default = false;
            // [c_char; N] arrays need custom Debug to display as strings
            if matches!(element.as_ref(), CType::Base(b) if b.name == "char") {
                info.trivial_debug = false;
            }
            Some(info)
        }
        CType::Ptr { pointee, .. } => {
            let pointee_info = get_type_info(types, type_infos, pointee)?;
            Some(TypeInfo {
                lifetime_param: pointee_info.lifetime_param,
                ..TypeInfo::POINTER
            })
        }
        CType::Func { .. } => todo!(),
    }
}

fn get_base_type_info<'a>(
    types: &Types<'a>,
    type_infos: &'a BTreeMap<&'static str, TypeInfo>,
    type_name: &str,
) -> Option<TypeInfo> {
    match &types.get(type_name)? {
        TypeKind::Alias(alias) => get_base_type_info(types, type_infos, alias.alias),
        TypeKind::BaseType(base) => match base.ty {
            Some(ty) => get_base_type_info(types, type_infos, &ty),
            None => Some(TypeInfo::OPAQUE),
        },
        TypeKind::Struct(_) | TypeKind::Union(_) => type_infos.get(type_name).copied(),
        TypeKind::EnumType(_) => Some(TypeInfo::POD),
        TypeKind::BitmaskType(_) => Some(TypeInfo::POD),
        TypeKind::Handle(_) => Some(TypeInfo::POD),
        TypeKind::FuncPointer(_) => Some(TypeInfo::FN_POINTER),
        TypeKind::Primitive(PrimitiveType::Void) => Some(TypeInfo::OPAQUE),
        TypeKind::Primitive(_) => Some(TypeInfo::POD),
        TypeKind::Custom(ty) => get_type_info(types, type_infos, ty),
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PrimitiveType {
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    Size,
    Isize,
    Float,
    Double,
    Void,
    Char,
    Int,
    Uint,
    Ulong,
}

type CustomTypes = external::ExternalTypes;

/// For each item (type, command, constant) required by any module, determines which
/// module should own its definition. If the item has a vendor suffix that matches
/// a specific module's vendor tag, that module is preferred. Otherwise, the first
/// module that requires the item wins.
fn compute_module_items(registry: &xml::Registry) -> Vec<ModuleItems<'_>> {
    let modules: Vec<_> = Module::from_registry(registry).collect();
    let module_count = modules.len();

    // For each item, track first requirer and vendor-matched requirer.
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

    // Build per-module item sets. Prefer vendor-matched module; fall back to first requirer.
    let mut ownership: Vec<HashSet<&'static str>> = vec![HashSet::new(); module_count];
    for (name, first) in &first_requirer {
        let owner = *vendor_requirer.get(name).unwrap_or(first);
        ownership[owner].insert(name);
    }

    // Build ModuleItems for each module from the ownership sets.
    modules
        .iter()
        .zip(&ownership)
        .map(|(module, owned)| {
            let requires = module.requires();

            let api_constants = requires
                .iter()
                .flat_map(|req| &req.constants)
                .filter(|c| owned.contains(c.name))
                .filter_map(|constant| {
                    let global = registry
                        .constants
                        .iter()
                        .find(|api_constant| api_constant.name == constant.name);

                    if let Some(global) = global {
                        Some(global.clone())
                    } else if let (Some(ty), Some(value)) = (constant.ty, constant.value) {
                        Some(xml::Constant {
                            name: constant.name,
                            ty,
                            value,
                        })
                    } else {
                        None
                    }
                })
                .collect();

            let items = ModuleItems {
                api_constants,
                structs: registry
                    .structs
                    .iter()
                    .filter(|ty| owned.contains(ty.name))
                    .collect(),
                unions: registry
                    .unions
                    .iter()
                    .filter(|ty| owned.contains(ty.name))
                    .collect(),
                enums: registry
                    .enums
                    .iter()
                    .filter(|ty| owned.contains(ty.name))
                    .collect(),
                bitmask_types: registry
                    .bitmask_types
                    .iter()
                    .filter(|ty| owned.contains(ty.name))
                    .collect(),
                handles: registry
                    .handles
                    .iter()
                    .filter(|ty| owned.contains(ty.name))
                    .collect(),
                basetypes: registry
                    .basetypes
                    .iter()
                    .filter(|ty| owned.contains(ty.name))
                    .collect(),
                funcpointers: registry
                    .funcpointers
                    .iter()
                    .filter(|ty| owned.contains(ty.name))
                    .collect(),
                commands: registry
                    .commands
                    .iter()
                    .filter(|cmd| owned.contains(cmd.name))
                    .collect(),
                type_aliases: registry
                    .enum_aliases
                    .iter()
                    .filter(|a| registry.enums.iter().any(|ty| ty.name == a.alias))
                    .chain(registry.handle_aliases.iter())
                    .chain(registry.struct_aliases.iter())
                    .chain(registry.bitmask_aliases.iter())
                    .filter(|alias| owned.contains(alias.name))
                    .collect(),
                command_aliases: registry
                    .command_aliases
                    .iter()
                    .filter(|alias| owned.contains(alias.name))
                    .collect(),
            };

            items
        })
        .collect()
}
