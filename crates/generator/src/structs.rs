use crate::{ctype_to_rust_type, normalize_name, normalize_ty_name, xml};

// struct StructInfo<'a> {
//     ty: &'a xml::Structure,
// }

// struct MemberInfo<'a> {
//     name: String,
//     ty: String,
//     len: Option<LengthKind<'a>>,
// }

// fn analyze_struct<'a>(
//     ty: &'a xml::Structure,
// ) -> StructInfo<'a> {
//     todo!()
// }

pub fn write_struct(file: &mut impl std::io::Write, ty: &xml::Structure) {
    writeln!(file, "#[repr(C)]").unwrap();
    writeln!(file, "#[derive(Copy, Clone)]").unwrap();
    writeln!(file, "pub struct {} {{", normalize_ty_name(ty.name)).unwrap();
    for member in &ty.members {
        let field_ty = ctype_to_rust_type(&member.c_decl.ty);
        writeln!(
            file,
            "    pub {}: {},",
            normalize_name(member.c_decl.name),
            field_ty
        )
        .unwrap();
    }
    writeln!(file, "}}").unwrap();
}
