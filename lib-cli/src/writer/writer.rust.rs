use super::parser::store::{ Store };
use super::parser::structs::{ Struct };
use super::parser::fields::{ Field };
use super::parser::groups::{ Group };

pub fn get_str(store: Store) -> String {
    let mut body = String::new();
    for strct in &store.structs {
        if strct.parent == 0 {
            body = format!("{}{}\n", body, render::structs(strct, 0)).to_string();
        }
    }
    for group in &store.groups {
        if group.parent == 0 {
            body = format!("{}{}\n", body, render::groups(group, &mut store.clone(), 0)).to_string();
        }
    }
    body
}

mod render {
    use super::{ Store, Struct, Field, Group };

    pub fn groups(group: &Group, store: &mut Store, level: u8) -> String {
        let mut body = format!("{}mod {} {{", spaces(level), group.name);
        body = format!("{}\n{}// id={}", body, spaces(level + 1), group.id);
        body = format!("{}\n{}// parent={}", body, spaces(level + 1), group.parent);
        for struct_id in &group.structs {
            if let Some(strct) = store.get_struct(*struct_id) {
                body = format!("{}\n{}", body, structs(&strct, level + 1));
            }
        }
        let childs = store.get_child_groups(group.id);
        for group in childs {
            body = format!("{}\n{}", body, groups(&group, &mut store.clone(), level + 1));
        }
        format!("{}\n{}}}\n", body, spaces(level))
    }
    pub fn structs(strct: &Struct, level: u8) -> String {
        let mut body = format!("{}struct {} {{", spaces(level), strct.name);
        body = format!("{}\n{}// id={}", body, spaces(level + 1), strct.id);
        body = format!("{}\n{}// parent={}", body, spaces(level + 1), strct.parent);
        for field in &strct.fields {
            body = format!("{}\n{}{}", body, spaces(level + 1), fields(field));
        }
        format!("{}\n{}}}\n", body, spaces(level))
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

    fn spaces(level: u8) -> String {
        "    ".repeat(level as usize)
    }

    fn optional(ref_type: String, opt: bool) -> String {
        if opt {
            format!("Option<{}>", ref_type)
        } else {
            ref_type
        }
    }

}