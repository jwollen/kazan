use crate::{
    LengthKind, cdecl::CType, ctype_to_rust_type, get_len_kind, normalize_name, normalize_ty_name,
    xml,
};

struct StructInfo<'a> {
    name: String,
    ty: &'a xml::Structure,
    members: Vec<MemberInfo<'a>>,
    tag: Option<&'static str>,
    has_default: bool,
}

struct MemberInfo<'a> {
    member: &'a xml::StructureMember,
    name: String,
    ty: String,
    len: Vec<LengthKind<'a>>,
}

fn analyze_struct<'a>(structs: &'a [xml::Structure], ty: &'a xml::Structure) -> StructInfo<'a> {
    let len_kinds: Vec<Vec<_>> = ty
        .members
        .iter()
        .map(|member| {
            member
                .len
                .iter()
                .map(|len| get_len_kind(&ty.members, structs, len))
                .collect()
        })
        .collect();

    let mut has_default = true;
    let mut tag = None;

    let mut members = Vec::new();
    for (index, member) in ty.members.iter().enumerate() {
        let name = normalize_name(member.c_decl.name);
        let ty = ctype_to_rust_type(&member.c_decl.ty);
        let len = len_kinds[index].clone();

        if member.c_decl.name == "sType" {
            if let Some(value) = member.values {
                tag = Some(value.strip_prefix("VK_STRUCTURE_TYPE_").unwrap());
            } else {
                has_default = false;
            }
        }

        members.push(MemberInfo {
            member,
            name,
            ty,
            len,
        });
    }

    let name = normalize_ty_name(ty.name).to_string();
    StructInfo {
        name,
        ty,
        members,
        tag,
        has_default,
    }
}

pub fn write_struct(
    file: &mut impl std::io::Write,
    structs: &[xml::Structure],
    ty: &xml::Structure,
) {
    let info = analyze_struct(structs, ty);

    writeln!(
        file,
        "#[repr(C)]
        pub struct {} {{",
        normalize_ty_name(ty.name)
    )
    .unwrap();
    for member in &ty.members {
        let name = normalize_name(member.c_decl.name);

        let field_ty = ctype_to_rust_type(&member.c_decl.ty);
        let field_ty = if let CType::Base(base) = &member.c_decl.ty
            && base.name.starts_with("PFN_")
        {
            format!("Option<{}>", field_ty)
        } else {
            field_ty
        };

        writeln!(file, "pub {}: {},", name, field_ty).unwrap();
    }
    writeln!(file, "}}").unwrap();

    // writeln!(
    //     file,
    //     "#[repr(C)]
    //     pub struct {} {{",
    //     ty.name
    // )
    // .unwrap();
    // for member in &ty.members {
    //     let name = normalize_name(member.c_decl.name);
    //     let field_ty = ctype_to_rust_type(&member.c_decl.ty);
    //     writeln!(file, "pub {}: {},", name, field_ty).unwrap();
    // }
    // writeln!(file, "}}").unwrap();

    // if info.has_default {
    //     writeln!(
    //         file,
    //         "impl Default for {} {{
    //     fn default() -> Self {{
    //     Self {{",
    //         info.name
    //     )
    //     .unwrap();
    //     for member in &info.members {
    //         write!(file, "{}: ", member.name).unwrap();
    //         if member.member.c_decl.name == "sType" {
    //             writeln!(file, "StructureType::{}", info.tag.unwrap()).unwrap()
    //         } else if let CType::Array { .. } = &member.member.c_decl.ty {
    //             write!(file, "[Default::default(); _]").unwrap();
    //         } else if let CType::Ptr { is_const, .. } = &member.member.c_decl.ty {
    //             if *is_const {
    //                 write!(file, "core::ptr::null()").unwrap();
    //             } else {
    //                 write!(file, "core::ptr::null_mut()").unwrap();
    //             }
    //         // } else if let CType::Base(base) = &member.member.c_decl.ty
    //         //     && base.name.starts_with("PFN_")
    //         // {
    //         //     write!(file, "core::ptr::null()").unwrap();
    //         } else {
    //             write!(file, "Default::default()").unwrap();
    //         }
    //         writeln!(file, ",").unwrap();
    //     }
    //     writeln!(file, "}} }} }}").unwrap();
    // }

    // writeln!(file, "impl {} {{", info.name).unwrap();
    // for member in &ty.members {
    //     let name = normalize_name(member.c_decl.name);
    //     let field_ty = ctype_to_rust_type(&member.c_decl.ty);

    //     writeln!(file, "fn {}(&mut self,", name).unwrap();

    //     writeln!(file, "{}: {},", name, field_ty).unwrap();

    //     writeln!(file, ") -> &mut Self {{").unwrap();

    //     if let CType::Base(base) = &member.c_decl.ty
    //         && base.name.starts_with("PFN_")
    //     {
    //         writeln!(file, "self.{} = Some({});", name, name).unwrap();
    //     } else {
    //         writeln!(file, "self.{} = {};", name, name).unwrap();
    //     };

    //     writeln!(file, "self").unwrap();
    //     writeln!(file, "}}").unwrap();
    // }
    // writeln!(file, "}}").unwrap();
}
