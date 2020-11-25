use super::parser::store::{ Store };
use super::parser::structs::{ Struct };
use super::parser::fields::{ Field };

pub fn get_str(store: Store) -> String {
    let mut body = String::new();
    for strct in &store.structs {
        body = format!("{}{}\n", body, render::structs(strct)).to_string();
    }
    body
}

mod render {
    use super::{ Struct, Field };

    pub fn structs(strct: &Struct) -> String {
        let mut body = format!("struct {} {{", strct.name);
        body = format!("{}\n{}// id={}", body, spaces(1), strct.id);
        body = format!("{}\n{}// parent={}", body, spaces(1), strct.parent);
        for field in &strct.fields {
            body = format!("{}\n{}{}", body, spaces(1), fields(field));
        }
        format!("{}\n}}\n", body)
    }

    pub fn fields(field: &Field) -> String {
        let mut body = format!("pub {}:", field.name);
        if field.repeated {
            body = format!("{} Vec<{}>;", body, optional(field.kind.clone(), field.optional));
        } else {
            body = format!("{} {};", body, optional(field.kind.clone(), field.optional));
        }
        body
    }

    fn spaces(level: usize) -> String {
        "    ".repeat(level)
    }

    fn optional(ref_type: String, opt: bool) -> String {
        if opt {
            format!("Option<{}>", ref_type)
        } else {
            ref_type
        }
    }

}