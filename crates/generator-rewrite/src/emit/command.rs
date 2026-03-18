use std::io::Write;

use anyhow::Result;
use itertools::Itertools;

use crate::{analysis::Analysis, write_doc_link, xml};

use crate::model::command as mc;

pub fn generate_commands(
    file: &mut impl Write,
    analysis: &Analysis,
    requires: &[&xml::Require],
) -> Result<()> {
    let dispatch_structs = crate::build::command::build_dispatch_structs(analysis, requires);
    for ds in &dispatch_structs {
        emit_dispatch_struct(file, ds)?;
    }
    Ok(())
}

fn emit_dispatch_struct(file: &mut impl Write, ds: &mc::DispatchStruct) -> Result<()> {
    let fn_type_name = match ds.dispatch_type {
        mc::DispatchType::Entry => "EntryFn",
        mc::DispatchType::Instance => "InstanceFn",
        mc::DispatchType::Device => "DeviceFn",
    };

    // Struct definition.
    writeln!(file, "pub struct {fn_type_name} {{")?;
    for group in &ds.groups {
        for entry in &group.entries {
            let ty = if entry.conditional {
                format!("Option<{}>", entry.pfn_type)
            } else {
                entry.pfn_type.clone()
            };
            writeln!(file, "{}: {ty},", entry.field_name)?;
        }
    }
    writeln!(file, "}}\n")?;

    // Load function.
    let load_trait = match ds.dispatch_type {
        mc::DispatchType::Instance => Some("LoadInstanceFn"),
        mc::DispatchType::Device => Some("LoadDeviceFn"),
        mc::DispatchType::Entry => None,
    };
    let (load_fn_name, impl_header) = match load_trait {
        Some(trait_name) => ("load_with", format!("impl {trait_name} for {fn_type_name}")),
        None => ("load", format!("impl {fn_type_name}")),
    };
    writeln!(file, "{impl_header} {{")?;
    writeln!(
        file,
        "{}unsafe fn {}(load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>) -> core::result::Result<Self, MissingEntryPointError> {{",
        if load_trait.is_some() { "" } else { "pub " },
        load_fn_name,
    )?;
    writeln!(file, "unsafe {{ Ok(Self {{")?;
    for group in &ds.groups {
        for entry in &group.entries {
            if entry.conditional {
                writeln!(
                    file,
                    "{}: transmute(load(c\"{}\")),",
                    entry.field_name, entry.c_entry_point
                )?;
            } else {
                writeln!(
                    file,
                    "{}: transmute(load(c\"{}\").ok_or(MissingEntryPointError)?),",
                    entry.field_name, entry.c_entry_point
                )?;
            }
        }
    }
    writeln!(file, "}}) }} }}\n}}\n")?;

    // Wrapper methods.
    writeln!(file, "impl {fn_type_name} {{")?;
    for group in &ds.groups {
        for entry in &group.entries {
            emit_command_wrapper(file, &entry.wrapper)?;
        }
    }
    writeln!(file, "}}\n")?;
    Ok(())
}

fn emit_command_wrapper(file: &mut impl Write, wrapper: &mc::CommandWrapper) -> Result<()> {
    // Doc link.
    write_doc_link(file, wrapper.c_name)?;

    // Signature.
    let lifetime = wrapper
        .lifetime_param
        .as_ref()
        .map(|l| format!("<'{l}>"))
        .unwrap_or_default();
    writeln!(file, "#[inline]")?;
    write!(file, "pub unsafe fn {}{lifetime}(&self, ", wrapper.name)?;
    for param in &wrapper.params {
        if param.mutable_binding {
            write!(file, "mut ")?;
        }
        write!(file, "{}: {}, ", param.name, param.ty.to_tokens())?;
    }
    write!(file, ")")?;
    if let Some(ret) = format_return_type(&wrapper.return_type) {
        writeln!(file, " -> {ret} {{")?;
    } else {
        writeln!(file, " {{")?;
    }

    // Body.
    writeln!(file, "unsafe {{")?;
    match &wrapper.body {
        mc::CommandBody::Direct(direct) => emit_direct_fn_body(file, wrapper, direct)?,
        mc::CommandBody::Enumeration(enumeration) => {
            emit_enumeration_fn_body(file, wrapper, enumeration)?
        }
        mc::CommandBody::Override(body) => write!(file, "{body}")?,
    }
    writeln!(file, "}} }}\n")?;
    Ok(())
}

fn format_return_type(return_type: &mc::CommandReturn) -> Option<String> {
    match return_type {
        mc::CommandReturn::Void => None,
        mc::CommandReturn::Value(ty) => Some(ty.to_tokens()),
        mc::CommandReturn::OutputParams(params) => {
            if params.len() == 1 {
                Some(params[0].ty.to_tokens())
            } else {
                Some(format!(
                    "({})",
                    params.iter().map(|p| p.ty.to_tokens()).join(", ")
                ))
            }
        }
        mc::CommandReturn::Fallible(inner) => {
            let inner_str = format_return_type(inner);
            let inner_str = inner_str.as_deref().unwrap_or("()");
            Some(format!("crate::Result<{inner_str}>"))
        }
        mc::CommandReturn::FallibleMultiSuccess {
            inner, status_type, ..
        } => {
            let inner_str = format_return_type(inner);
            let status = status_type.to_tokens();
            match inner_str {
                Some(s) => Some(format!("crate::Result<({s}, {status})>")),
                None => Some(format!("crate::Result<{status}>")),
            }
        }
    }
}

fn emit_direct_fn_body(
    file: &mut impl Write,
    wrapper: &mc::CommandWrapper,
    direct: &mc::DirectCall,
) -> Result<()> {
    // MaybeUninit for output params
    if let mc::CommandReturn::Fallible(inner)
    | mc::CommandReturn::FallibleMultiSuccess { inner, .. } = &wrapper.return_type
    {
        if let mc::CommandReturn::OutputParams(params) = inner.as_ref() {
            for param in params {
                writeln!(
                    file,
                    "let mut {} = core::mem::MaybeUninit::uninit();",
                    param.name
                )?;
            }
        }
    } else if let mc::CommandReturn::OutputParams(params) = &wrapper.return_type {
        for param in params {
            writeln!(
                file,
                "let mut {} = core::mem::MaybeUninit::uninit();",
                param.name
            )?;
        }
    }

    // Length assertions
    emit_length_assertions(file, &direct.ffi_call)?;

    // The FFI call
    let is_fallible = matches!(
        &wrapper.return_type,
        mc::CommandReturn::Fallible(_) | mc::CommandReturn::FallibleMultiSuccess { .. }
    );
    if is_fallible {
        writeln!(file, "let result = ")?;
    }
    emit_ffi_call(file, &direct.ffi_call)?;

    // Result handling
    match &direct.result_handling {
        mc::ResultHandling::None => {}
        mc::ResultHandling::ReturnDirect { bool_convert } => {
            if *bool_convert {
                writeln!(file, " != 0")?;
            }
        }
        mc::ResultHandling::MatchResult {
            ok_codes,
            output_expr,
            expose_status,
        } => {
            writeln!(file, ";\n")?;
            writeln!(file, "match result {{")?;
            if *expose_status {
                for (i, code) in ok_codes.iter().enumerate() {
                    let code_name = code.strip_prefix("VK_").unwrap_or(code);
                    let code_value = if i == 0 {
                        "false".to_string()
                    } else {
                        "true".to_string()
                    };
                    let ok_value = match output_expr.as_deref() {
                        Some(rv) => format!("({rv}, {code_value})"),
                        None => code_value,
                    };
                    writeln!(file, "VkResult::{code_name} => Ok({ok_value}),")?;
                }
            } else {
                let ok_value = output_expr.as_deref().unwrap_or("()");
                for code in ok_codes {
                    writeln!(
                        file,
                        "VkResult::{} => Ok({}),",
                        code.strip_prefix("VK_").unwrap_or(code),
                        ok_value,
                    )?;
                }
            }
            writeln!(file, "err => Err(err),")?;
            writeln!(file, "}}")?;
        }
        mc::ResultHandling::OutputParams => {
            emit_output_params_return(file, wrapper)?;
        }
    }
    Ok(())
}

fn emit_output_params_return(file: &mut impl Write, wrapper: &mc::CommandWrapper) -> Result<()> {
    let params = match &wrapper.return_type {
        mc::CommandReturn::OutputParams(params) => params,
        mc::CommandReturn::Fallible(inner)
        | mc::CommandReturn::FallibleMultiSuccess { inner, .. } => match inner.as_ref() {
            mc::CommandReturn::OutputParams(params) => params,
            _ => return Ok(()),
        },
        _ => return Ok(()),
    };

    writeln!(file, ";")?;
    let value = if params.len() == 1 {
        let p = &params[0];
        let init = format!("{}.assume_init()", p.name);
        if p.bool_convert {
            format!("{init} != 0")
        } else {
            init
        }
    } else {
        format!(
            "({})",
            params
                .iter()
                .map(|p| {
                    let init = format!("{}.assume_init()", p.name);
                    if p.bool_convert {
                        format!("{init} != 0")
                    } else {
                        init
                    }
                })
                .join(", ")
        )
    };

    let is_fallible = matches!(
        &wrapper.return_type,
        mc::CommandReturn::Fallible(_) | mc::CommandReturn::FallibleMultiSuccess { .. }
    );
    if is_fallible {
        writeln!(file, ";\n")?;
        writeln!(file, "match result {{")?;
        writeln!(file, "VkResult::SUCCESS => Ok({value}),")?;
        writeln!(file, "err => Err(err),")?;
        writeln!(file, "}}")?;
    } else {
        writeln!(file, "{value}")?;
    }
    Ok(())
}

fn emit_enumeration_fn_body(
    file: &mut impl Write,
    _wrapper: &mc::CommandWrapper,
    enumeration: &mc::EnumerationCall,
) -> Result<()> {
    let len_param = &enumeration.len_param;

    // Build the closure signature.
    writeln!(file, "let call = |{len_param}, ")?;
    for arr in &enumeration.array_params {
        writeln!(file, "{arr}, ")?;
    }
    for extra in &enumeration.extra_output_params {
        writeln!(file, "{}: {}, ", extra.name, extra.ty.to_tokens())?;
    }
    writeln!(file, "| {{ ")?;

    // Emit length assertions before the FFI call.
    emit_length_assertions(file, &enumeration.inner_call)?;

    // Emit the FFI call inside the closure.
    if enumeration.is_fallible {
        writeln!(file, "let result = ")?;
    }
    emit_ffi_call(file, &enumeration.inner_call)?;

    // Result handling in the enumeration closure.
    if enumeration.is_fallible {
        writeln!(file, ";\n")?;
        writeln!(file, "match result {{")?;
        for code in &enumeration.closure_ok_codes {
            writeln!(
                file,
                "VkResult::{} => Ok(()),",
                code.strip_prefix("VK_").unwrap_or(code)
            )?;
        }
        writeln!(file, "err => Err(err),")?;
        writeln!(file, "}}")?;
    }

    writeln!(file, "}};")?;

    // First call: query length.
    writeln!(file, "let mut len = 0; call(&mut len, ")?;
    for _ in &enumeration.array_params {
        writeln!(file, "std::ptr::null_mut(), ")?;
    }
    for (i, extra) in enumeration.extra_output_params.iter().enumerate() {
        let is_optional = enumeration
            .extra_optional_flags
            .get(i)
            .copied()
            .unwrap_or(false);
        if is_optional {
            writeln!(file, "None, ")?;
        } else {
            writeln!(file, "{}, ", extra.name)?;
        }
    }
    if enumeration.is_fallible {
        writeln!(file, ")?;")?;
    } else {
        writeln!(file, ");")?;
    }

    writeln!(
        file,
        "let capacity = len.try_into().expect(\"failed to convert `N` to usize\");"
    )?;

    // Reserve buffers.
    for (i, arr) in enumeration.array_params.iter().enumerate() {
        writeln!(file, "let {arr}_buf = {arr}.reserve(capacity);")?;
        if i == 0 {
            writeln!(file, "len = {arr}_buf.len().try_into().unwrap();")?;
        } else {
            let first = &enumeration.array_params[0];
            writeln!(file, "assert_eq!({arr}_buf.len(), {first}_buf.len());")?;
        }
    }

    // Second call: fill.
    if enumeration.is_fallible {
        writeln!(file, "let result = ")?;
    }
    writeln!(file, "call(&mut len, ")?;
    for arr in &enumeration.array_params {
        writeln!(file, "{arr}_buf.as_mut_ptr() as *mut _, ")?;
    }
    for extra in &enumeration.extra_output_params {
        writeln!(file, "{}, ", extra.name)?;
    }
    if enumeration.is_fallible {
        writeln!(file, ")?;")?;
    } else {
        writeln!(file, ");")?;
    }

    // Set lengths.
    for arr in &enumeration.array_params {
        writeln!(file, "{arr}.set_len(len.try_into().unwrap());")?;
    }
    if enumeration.is_fallible {
        writeln!(file, "Ok(result)")?;
    }
    Ok(())
}

fn emit_ffi_call(file: &mut impl Write, ffi_call: &mc::FfiCall) -> Result<()> {
    if ffi_call.conditional {
        writeln!(file, "(self.{}.unwrap())(", ffi_call.fn_field)?;
    } else {
        writeln!(file, "(self.{})(", ffi_call.fn_field)?;
    }

    for arg in &ffi_call.args {
        emit_ffi_arg(file, arg)?;
    }
    writeln!(file, ")")?;
    Ok(())
}

fn emit_ffi_arg(file: &mut impl Write, arg: &mc::FfiArg) -> Result<()> {
    match arg {
        mc::FfiArg::Direct { param } => {
            writeln!(file, "{param},")?;
        }
        mc::FfiArg::LenFromSlice { slice } => {
            writeln!(file, "{slice}.len().try_into().unwrap(),")?;
        }
        mc::FfiArg::LenFromSliceOrLen {
            param,
            option_wrapped,
        } => {
            if *option_wrapped {
                writeln!(
                    file,
                    "{param}.as_ref().map_or(0, SliceOrLen::len).try_into().unwrap(),"
                )?;
            } else {
                writeln!(file, "{param}.len().try_into().unwrap(),")?;
            }
        }
        mc::FfiArg::SliceAsPtr {
            param,
            is_const,
            optional,
        } => {
            if *optional {
                if *is_const {
                    writeln!(file, "{param}.to_raw_ptr(),")?;
                } else {
                    writeln!(file, "{param}.to_raw_mut_ptr(),")?;
                }
            } else if *is_const {
                writeln!(file, "{param}.as_ptr() as _,")?;
            } else {
                writeln!(file, "{param}.as_mut_ptr() as _,")?;
            }
        }
        mc::FfiArg::SliceOrLenAsPtr { param } => {
            writeln!(file, "{param}.to_raw_ptr(),")?;
        }
        mc::FfiArg::OutputMutPtr { param } => {
            writeln!(file, "{param}.as_mut_ptr(),")?;
        }
        mc::FfiArg::OptionalPtrToRaw { param, is_const } => {
            if *is_const {
                writeln!(file, "{param}.to_raw_ptr(),")?;
            } else {
                writeln!(file, "{param}.to_raw_mut_ptr(),")?;
            }
        }
        mc::FfiArg::EnumerationBuf { param } => {
            writeln!(file, "{param} as _,")?;
        }
        mc::FfiArg::BoolInto { param } => {
            writeln!(file, "{param}.into(),")?;
        }
    }
    Ok(())
}

fn emit_length_assertions(file: &mut impl Write, ffi_call: &mc::FfiCall) -> Result<()> {
    for assertion in &ffi_call.pre_assertions {
        for arr_assert in &assertion.assertions {
            if arr_assert.nullable {
                writeln!(
                    file,
                    "assert!({}.is_none_or(|s| s.len() == {}));",
                    arr_assert.array_name, assertion.primary_len_expr
                )?;
            } else {
                writeln!(
                    file,
                    "assert_eq!({}.len(), {});",
                    arr_assert.array_name, assertion.primary_len_expr
                )?;
            }
        }
    }
    Ok(())
}
