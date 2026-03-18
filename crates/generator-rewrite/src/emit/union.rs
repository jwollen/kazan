//! Emit layer: UnionDef → Rust source for union definitions.

use std::io::Write;

use anyhow::Result;

use crate::{analysis::Analysis, model::union as mu, write_doc_link, xml};

pub fn generate_unions(
    file: &mut impl Write,
    analysis: &Analysis,
    unions: &[&xml::Structure],
) -> Result<()> {
    for ty in unions {
        let model = crate::model::union::build_union(analysis, ty);
        emit_union(file, &model)?;
    }
    Ok(())
}

fn emit_union(file: &mut impl Write, u: &mu::UnionDef) -> Result<()> {
    let name = &u.name;
    let lifetime_spec = if u.has_lifetime { "<'a>" } else { "" };
    let lifetime_spec_anon = if u.has_lifetime { "<'_>" } else { "" };

    write_doc_link(file, u.c_name)?;
    writeln!(
        file,
        "#[repr(C)]
            #[derive(Copy, Clone)]
            pub union {name}{lifetime_spec} {{"
    )?;
    for field in &u.fields {
        writeln!(file, "pub {}: {},", field.name, field.ty.to_tokens())?;
    }
    if u.has_lifetime {
        writeln!(file, "pub _marker: PhantomData<&'a ()>,")?;
    }
    writeln!(file, "}}\n")?;

    writeln!(
        file,
        "#[cfg(feature = \"debug\")]
            impl fmt::Debug for {name}{lifetime_spec_anon} {{
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
                    f.debug_struct(\"{name}\").finish()
                }}
            }}\n"
    )?;
    writeln!(
        file,
        "impl Default for {name}{lifetime_spec_anon} {{
                fn default() -> Self {{
                    unsafe {{ core::mem::zeroed() }}
                }}
            }}\n"
    )?;
    Ok(())
}
