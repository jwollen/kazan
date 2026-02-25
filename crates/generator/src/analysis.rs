use std::{
    collections::{BTreeMap, HashSet},
    fs,
    path::Path,
};
use tracing::{debug, error_span};

use crate::{cdecl::CType, xml};

/// Holds the analysis results for easy querying.
#[derive(Debug)]
pub struct Analysis {
    registry: xml::Registry,
    type_refs: TypeRefs,
    type_infos: BTreeMap<&'static str, TypeInfo>,
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

        let type_refs = build_type_refs(&registry);
        let type_infos = compute_type_infos(&registry, &type_refs);

        Self {
            registry,
            type_refs,
            type_infos,
        }
    }

    /// Get "raw" Vulkan XML registry.
    pub fn registry(&self) -> &xml::Registry {
        &self.registry
    }

    pub fn types(&self) -> Types {
        Types {
            registry: &self.registry,
            type_refs: &self.type_refs,
        }
    }

    pub fn type_infos(&self) -> &TypeInfos {
        &self.type_infos
    }
}

#[derive(Debug)]
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
    Foreign(&'static str),
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
    Foreign(&'static str),
}

type TypeRefs = BTreeMap<&'static str, TypeKindRef>;

pub struct Types<'a> {
    registry: &'a xml::Registry,
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
            TypeKindRef::Foreign(name) => Some(TypeKind::Foreign(name)),
        }
    }
}

fn build_type_refs(registry: &xml::Registry) -> TypeRefs {
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
    ]);

    types.extend(
        FOREIGN_TYPES
            .iter()
            .copied()
            .map(|(name, _)| (name, TypeKindRef::Foreign(name))),
    );

    types
}

pub type TypeInfos = BTreeMap<&'static str, TypeInfo>;

#[derive(Debug, Clone, Copy)]
pub struct TypeInfo {
    default: bool,
    clone: bool,
    pod: bool,
    lifetime: bool,
}

impl TypeInfo {
    const POD: TypeInfo = TypeInfo {
        default: true,
        clone: true,
        pod: true,
        lifetime: false,
    };

    const FN_POINTER: TypeInfo = TypeInfo {
        default: true,
        clone: true,
        pod: true,
        lifetime: false,
    };

    const POINTER: TypeInfo = TypeInfo {
        default: false,
        clone: false,
        pod: false,
        lifetime: true,
    };

    fn empty_struct() -> Self {
        Self {
            default: true,
            clone: true,
            pod: true,
            lifetime: false,
        }
    }

    fn merge(&mut self, other: &TypeInfo) {
        self.default &= other.default;
        self.clone &= other.clone;
        self.pod &= other.pod;
        self.lifetime |= other.lifetime;
    }
}

fn compute_type_infos(registry: &xml::Registry, type_refs: &TypeRefs) -> TypeInfos {
    let types = Types {
        registry,
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

                if !visited_types.insert(ty_name) {
                    todo!("cycle detected in {}", ty_name);
                }

                for member in &ty.members {
                    if let Some(base_ty_name) = get_base_type(&member.c_decl.ty) {
                        pending_types.push((base_ty_name, false));
                    }
                }
            } else {
                let mut type_info = TypeInfo::empty_struct();

                for member in &ty.members {
                    let member_ty = &member.c_decl.ty;
                    let member_ty_info = get_type_info(&types, &type_infos, member_ty).unwrap();
                    type_info.merge(member_ty_info);
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
        CType::Ptr { .. } => None,
        CType::Func { .. } => todo!(),
    }
}

fn get_type_info<'a>(
    types: &Types<'a>,
    type_infos: &'a BTreeMap<&'static str, TypeInfo>,
    ty: &CType,
) -> Option<&'a TypeInfo> {
    match ty {
        CType::Base(base) => get_base_type_info(types, type_infos, base.name),
        CType::Array { element, .. } => get_type_info(types, type_infos, element),
        CType::Ptr { .. } => Some(&TypeInfo::POINTER),
        CType::Func { .. } => todo!(),
    }
}

fn get_base_type_info<'a>(
    types: &Types<'a>,
    type_infos: &'a BTreeMap<&'static str, TypeInfo>,
    type_name: &str,
) -> Option<&'a TypeInfo> {
    match &types.get(type_name)? {
        TypeKind::Alias(alias) => get_base_type_info(types, type_infos, alias.alias),
        TypeKind::Struct(_) | TypeKind::Union(_) => type_infos.get(type_name),
        TypeKind::EnumType(_) => Some(&TypeInfo::POD),
        TypeKind::BitmaskType(_) => Some(&TypeInfo::POD),
        TypeKind::Handle(_) => Some(&TypeInfo::POD),
        TypeKind::FuncPointer(_) => Some(&TypeInfo::FN_POINTER),
        TypeKind::BaseType(_) => Some(&TypeInfo::POD),
        TypeKind::Primitive(_) => Some(&TypeInfo::POD),
        TypeKind::Foreign(_) => Some(&TypeInfo::POD),
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
}

const FOREIGN_TYPES: &[(&'static str, &'static str)] = &[
    ("VisualID", "c_uint"),
    ("Display", "c_void"),
    ("Window", "c_ulong"),
    ("RROutput", "c_ulong"),
    ("wl_display", "c_void"),
    ("wl_surface", "c_void"),
    ("HANDLE", "isize"),
    ("HINSTANCE", "HANDLE"),
    ("HWND", "HANDLE"),
    ("HMONITOR", "HANDLE"),
    ("DWORD", "c_ulong"),
    ("LPCWSTR", "*const u16"),
    ("xcb_connection_t", "c_void"),
    ("xcb_window_t", "u32"),
    ("xcb_visualid_t", "u32"),
    ("SECURITY_ATTRIBUTES", "c_void"),
    ("IDirectFB", "c_void"),
    ("IDirectFBSurface", "c_void"),
    ("zx_handle_t", "u32"),
    ("GgpStreamDescriptor", "c_int"),
    ("GgpFrameToken", "c_int"),
    ("_screen_buffer", "c_void"),
    ("_screen_context", "c_void"),
    ("_screen_window", "c_void"),
    ("NvSciSyncAttrList", "*const c_void"),
    ("NvSciSyncObj", "*const c_void"),
    ("NvSciSyncFence", "*c_void"),
    ("NvSciBufAttrList", "*const c_void"),
    ("NvSciBufObj", "*const c_void"),
];
