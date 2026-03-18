use super::rust_type::RustType;

/// Model for a generated Rust struct definition.
#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub c_name: &'static str,
    pub kind: StructKind,
    pub fields: Vec<StructField>,
    pub has_lifetime: bool,
    pub derives: Vec<String>,
    pub setters: Vec<Setter>,
    pub trait_impls: Vec<TraitImpl>,
    pub default_impl: Option<DefaultImpl>,
    pub debug_impl: Option<DebugImpl>,
}

/// Whether the struct is extensible (has sType/pNext) or plain.
#[derive(Debug, Clone)]
pub enum StructKind {
    Extensible { stype_suffix: &'static str },
    Plain,
}

/// A field in the generated struct.
#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub ty: RustType,
}

/// A setter method on the struct's builder.
#[derive(Debug, Clone)]
pub struct Setter {
    pub name: String,
    pub kind: SetterKind,
}

/// All the different shapes a setter can take.
#[derive(Debug, Clone)]
pub enum SetterKind {
    /// Simple value assignment: `self.field = value`.
    Value {
        param: SetterParam,
        assignment: ValueAssignment,
    },
    /// Array setter: sets both len field and pointer field(s).
    Array {
        len_field: usize,
        params: Vec<SetterParam>,
    },
    /// Bitfield setter: pack bits into a backing integer field.
    Bitfield {
        backing_field: usize,
        offset: u8,
        width: u8,
        param_ty: RustType,
        extract: BitfieldExtract,
    },
    /// Set a `*const c_char` field from a `&CStr`.
    CStrToPtr { param: SetterParam, field: usize },
    /// Copy a `&CStr` into a `[c_char; N]` array field.
    CStrToArray { param: SetterParam, field: usize },
}

/// How to transform the setter parameter value before assignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueAssignment {
    /// Assign directly: `self.field = value`.
    Direct,
    /// Wrap in Some: `self.field = Some(value)`.
    WrapInSome,
    /// Convert bool→Bool32: `self.field = value.into()`.
    BoolInto,
}

/// How to extract the value from a bitfield for the setter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitfieldExtract {
    /// Use the raw integer value.
    Direct,
    /// Call `.as_raw()` on an enum/flags type.
    AsRaw,
    /// Convert bool to 0/1.
    Bool,
}

/// A parameter for a setter method.
#[derive(Debug, Clone)]
pub struct SetterParam {
    pub name: String,
    /// Index of the struct field this parameter sets.
    pub field_index: usize,
    pub ty: RustType,
    pub optional: bool,
    pub array_assign: Option<ArrayAssignment>,
}

/// How an array setter copies data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayAssignment {
    /// `self.field.copy_from_slice(value)`.
    CopyFromSlice,
    /// `self.field = value.as_ptr()`.
    PtrFromSlice { is_const: bool },
    /// `self.field = value as *const _`.
    PtrFromRef { is_const: bool },
}

/// A trait impl to generate for the struct.
#[derive(Debug, Clone)]
pub enum TraitImpl {
    TaggedStructure {
        stype_suffix: &'static str,
    },
    Extends {
        target: &'static str,
        provisional: bool,
    },
}

/// The Default impl for the struct.
#[derive(Debug, Clone)]
pub struct DefaultImpl {
    pub field_defaults: Vec<FieldDefault>,
    pub has_phantom: bool,
}

/// How to default-initialize a single field.
#[derive(Debug, Clone)]
pub enum FieldDefault {
    /// `StructureType::FOO` for the sType field.
    StructureType,
    /// `ptr::null()` or `ptr::null_mut()`.
    Null { mutable: bool },
    /// `[default; N]` where default is the element's FieldDefault.
    ArrayFill(Box<FieldDefault>),
    /// `0` / `0.0` / `Default::default()`.
    Zero,
}

/// The Debug impl for the struct.
#[derive(Debug, Clone)]
pub struct DebugImpl {
    pub fields: Vec<DebugField>,
}

/// How to format a single field in the Debug impl.
#[derive(Debug, Clone)]
pub enum DebugField {
    /// Normal debug: `.field("name", &self.name)`.
    Normal(String),
    /// Pointer-to-CStr: display as string.
    CStrPtr(String),
    /// Function pointer: display as hex address.
    FuncPointer(String),
    /// Bitfield group: extract bits from a backing field for debug display.
    Bitfield {
        name: String,
        backing_field: String,
        offset: u8,
        width: u8,
    },
}
