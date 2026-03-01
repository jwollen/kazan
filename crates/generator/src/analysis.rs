use std::{
    collections::{BTreeMap, HashSet},
    fs,
    path::Path,
};
use tracing::{debug, error_span};

use crate::{
    cdecl::CType,
    handle::{HandleCommandTypes, collect_handle_command_types},
    xml,
};

/// Holds the analysis results for easy querying.
#[derive(Debug)]
pub struct Analysis {
    registry: xml::Registry,
    custom_types: CustomTypes,
    type_refs: TypeRefs,
    type_infos: BTreeMap<&'static str, TypeInfo>,
    handle_command_types: HandleCommandTypes,
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

        let custom_types = custom_types();
        let type_refs = build_type_refs(&registry, &custom_types);
        let type_infos = compute_type_infos(&registry, &custom_types, &type_refs);
        let handle_command_types = collect_handle_command_types(&registry);

        Self {
            registry,
            custom_types,
            type_refs,
            type_infos,
            handle_command_types,
        }
    }

    /// Get "raw" Vulkan XML registry.
    pub fn registry(&self) -> &xml::Registry {
        &self.registry
    }

    pub fn types(&self) -> Types<'_> {
        Types {
            registry: &self.registry,
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
}

impl TypeInfo {
    const POD: TypeInfo = TypeInfo {
        default: true,
        clone: true,
        pod: true,
        lifetime_param: false,
        lifetime: false,
    };

    const OPAQUE: TypeInfo = TypeInfo {
        default: false,
        clone: false,
        pod: false,
        lifetime_param: false,
        lifetime: false,
    };

    const FN_POINTER: TypeInfo = TypeInfo {
        default: true,
        clone: true,
        pod: true,
        lifetime_param: false,
        lifetime: false,
    };

    const POINTER: TypeInfo = TypeInfo {
        default: false,
        clone: false,
        pod: false,
        lifetime_param: false,
        lifetime: true,
    };

    fn empty_struct() -> Self {
        Self {
            default: true,
            clone: true,
            pod: true,
            lifetime_param: false,
            lifetime: false,
        }
    }

    fn merge(&mut self, other: &TypeInfo) {
        self.default &= other.default;
        self.clone &= other.clone;
        self.pod &= other.pod;
        self.lifetime_param |= other.lifetime_param | other.lifetime;
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

const fn ctype(base: &str) -> CType<'_> {
    CType::Base(crate::cdecl::CBaseType {
        struct_tag: false,
        name: base,
    })
}

fn c_ptr_type(pointee: CType<'static>, is_const: bool) -> CType<'_> {
    CType::Ptr {
        implicit_for_decay: false,
        is_const,
        pointee: Box::new(pointee),
    }
}

impl CType<'_> {
    pub const UINT16_T: CType<'static> = ctype("uint16_t");
    pub const UINT32_T: CType<'static> = ctype("uint32_t");
    pub const ISIZE_T: CType<'static> = ctype("isize_t");
    pub const INT: CType<'static> = ctype("int");
    pub const UINT: CType<'static> = ctype("unsigned int");
    pub const ULONG: CType<'static> = ctype("unsigned long");
    pub const HANDLE: CType<'static> = ctype("HANDLE");
}

type CustomTypes = Vec<(&'static str, CType<'static>)>;

fn custom_types() -> CustomTypes {
    [
        ("VisualID", CType::UINT),
        ("Display", CType::VOID),
        ("Window", CType::ULONG),
        ("RROutput", CType::ULONG),
        ("wl_display", CType::VOID),
        ("wl_surface", CType::VOID),
        ("HANDLE", CType::ISIZE_T),
        ("HINSTANCE", CType::HANDLE),
        ("HWND", CType::HANDLE),
        ("HMONITOR", CType::HANDLE),
        ("DWORD", CType::ULONG),
        ("LPCWSTR", c_ptr_type(CType::UINT16_T, true)),
        ("xcb_connection_t", CType::VOID),
        ("xcb_window_t", CType::UINT32_T),
        ("xcb_visualid_t", CType::UINT32_T),
        ("SECURITY_ATTRIBUTES", CType::VOID),
        ("IDirectFB", CType::VOID),
        ("IDirectFBSurface", CType::VOID),
        ("zx_handle_t", CType::UINT32_T),
        ("GgpStreamDescriptor", CType::INT),
        ("GgpFrameToken", CType::INT),
        ("_screen_buffer", CType::VOID),
        ("_screen_context", CType::VOID),
        ("_screen_window", CType::VOID),
        ("NvSciSyncAttrList", c_ptr_type(CType::VOID, true)),
        ("NvSciSyncObj", c_ptr_type(CType::VOID, true)),
        ("NvSciSyncFence", c_ptr_type(CType::VOID, false)),
        ("NvSciBufAttrList", c_ptr_type(CType::VOID, true)),
        ("NvSciBufObj", c_ptr_type(CType::VOID, true)),
    ]
    .into_iter()
    .collect()
}
