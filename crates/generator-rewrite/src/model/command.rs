use super::rust_type::RustType;

/// A dispatch function table struct (`EntryFn`, `InstanceFn`, `DeviceFn`).
#[derive(Debug, Clone)]
pub struct DispatchStruct {
    pub name: String,
    pub dispatch_type: DispatchType,
    pub groups: Vec<DispatchGroup>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchType {
    Entry,
    Instance,
    Device,
}

/// A group of dispatch entries from a single `<require>` block.
#[derive(Debug, Clone)]
pub struct DispatchGroup {
    pub entries: Vec<DispatchEntry>,
}

/// A single entry in a dispatch table.
#[derive(Debug, Clone)]
pub struct DispatchEntry {
    /// snake_case wrapper name (e.g. `create_instance`).
    pub field_name: String,
    /// PFN type name (e.g. `PFN_vkCreateInstance`).
    pub pfn_type: String,
    /// C entry point (e.g. `vkCreateInstance`).
    pub c_entry_point: String,
    /// True when the entry point may be absent at runtime (conditional extension).
    pub conditional: bool,
    /// The wrapper function for this entry.
    pub wrapper: CommandWrapper,
}

/// A safe Rust wrapper around a Vulkan command.
#[derive(Debug, Clone)]
pub struct CommandWrapper {
    pub name: String,
    pub c_name: String,
    pub lifetime_param: Option<String>,
    pub params: Vec<WrapperParam>,
    pub return_type: CommandReturn,
    pub body: CommandBody,
}

/// A parameter in a wrapper function signature.
#[derive(Debug, Clone)]
pub struct WrapperParam {
    pub name: String,
    pub ty: RustType,
    pub mutable_binding: bool,
}

/// What the wrapper function returns.
#[derive(Debug, Clone)]
pub enum CommandReturn {
    /// Returns nothing (`()`).
    Void,
    /// Returns a single value directly.
    Value(RustType),
    /// Returns one or more output parameters (via MaybeUninit pattern).
    OutputParams(Vec<OutputParam>),
    /// Wraps inner return in `VkResult<_>`.
    Fallible(Box<CommandReturn>),
    /// Wraps inner return in `VkResult<(_, status)>` with multiple success codes.
    FallibleMultiSuccess {
        inner: Box<CommandReturn>,
        status_type: RustType,
    },
}

/// An output parameter extracted from the command's parameter list into the return type.
#[derive(Debug, Clone)]
pub struct OutputParam {
    pub name: String,
    pub ty: RustType,
    /// Whether to convert Bool32 → bool for this output.
    pub bool_convert: bool,
}

/// The body of a wrapper function.
#[derive(Debug, Clone)]
pub enum CommandBody {
    /// A direct FFI call (possibly with output params).
    Direct(DirectCall),
    /// The two-call enumeration pattern.
    Enumeration(EnumerationCall),
    /// A hand-written override body.
    Override(String),
}

/// A direct call to the FFI function.
#[derive(Debug, Clone)]
pub struct DirectCall {
    pub ffi_call: FfiCall,
    pub result_handling: ResultHandling,
}

/// The raw FFI call setup.
#[derive(Debug, Clone)]
pub struct FfiCall {
    /// Name of the function pointer field on the dispatch struct.
    pub fn_field: String,
    /// Whether the function pointer is Option<PFN_...> (conditional entry).
    pub conditional: bool,
    /// Arguments to pass to the FFI function.
    pub args: Vec<FfiArg>,
    /// Assertions to emit before the call (e.g. length consistency checks).
    pub pre_assertions: Vec<LengthAssertion>,
}

/// How to produce an FFI argument from wrapper parameters.
#[derive(Debug, Clone)]
pub enum FfiArg {
    /// Pass the parameter directly (e.g. a handle, enum value).
    Direct { param: String },
    /// Derive length from a slice parameter: `param.len() as _`.
    LenFromSlice { slice: String },
    /// Derive length from a SliceOrLen parameter.
    LenFromSliceOrLen { param: String, option_wrapped: bool },
    /// Pass slice as pointer: `param.as_ptr()`.
    SliceAsPtr {
        param: String,
        is_const: bool,
        optional: bool,
    },
    /// Pass SliceOrLen as pointer.
    SliceOrLenAsPtr { param: String },
    /// Output parameter: `&mut param` or `param.as_mut_ptr()`.
    OutputMutPtr { param: String },
    /// Optional pointer to raw: `param.map_or(ptr::null(), |p| p as *const _)`.
    OptionalPtrToRaw { param: String, is_const: bool },
    /// Enumeration buffer: `buf.as_mut_ptr()`.
    EnumerationBuf { param: String },
    /// Bool conversion: `param.into()`.
    BoolInto { param: String },
}

/// An assertion to emit before the FFI call.
#[derive(Debug, Clone)]
pub struct LengthAssertion {
    /// The expression for the primary array's length (e.g. `buffers.len()`).
    pub primary_len_expr: String,
    /// Each secondary array's assertion.
    pub assertions: Vec<ArrayLenAssertion>,
}

/// A single array length assertion against the primary.
#[derive(Debug, Clone)]
pub struct ArrayLenAssertion {
    pub array_name: String,
    /// If true, array is optional — use `is_none_or(|s| s.len() == ...)`.
    pub nullable: bool,
}

/// How to handle the FFI call's return value.
#[derive(Debug, Clone)]
pub enum ResultHandling {
    /// Ignore the return value / no return.
    None,
    /// Return the value directly (possibly with bool conversion).
    ReturnDirect { bool_convert: bool },
    /// Match on VkResult, map ok codes.
    MatchResult {
        ok_codes: Vec<String>,
        output_expr: Option<String>,
        expose_status: bool,
    },
    /// Initialize MaybeUninit, call, assume_init, return.
    OutputParams,
}

/// The two-call enumeration pattern body.
#[derive(Debug, Clone)]
pub struct EnumerationCall {
    /// Name of the length variable.
    pub len_param: String,
    /// Names of the array output parameters.
    pub array_params: Vec<String>,
    /// Extra non-array output parameters.
    pub extra_output_params: Vec<ExtraEnumParam>,
    /// The inner FFI call (used for both the length query and the fill call).
    pub inner_call: FfiCall,
    /// Whether the command returns VkResult.
    pub is_fallible: bool,
    /// Success codes accepted by the enumeration closure (e.g. SUCCESS, INCOMPLETE).
    pub closure_ok_codes: Vec<String>,
    /// Whether extra output params that are optional get `None` in the first (length-query) call.
    pub extra_optional_flags: Vec<bool>,
}

/// An extra output parameter in an enumeration command (not the array itself).
#[derive(Debug, Clone)]
pub struct ExtraEnumParam {
    pub name: String,
    pub ty: RustType,
}
