pub mod raw;

mod path;
pub use path::{Path, PathElement};

mod generator;
pub use generator::Generator;

const RENAME_MAP: &'static [(&'static str, &'static str)] = &[
    ("type", "ty"),
    ("macro", "macro_def"),
    ("in", "in_name"),
    ("ref", "reference"),
];

fn check_rename(name: String) -> String {
    if let Some(to) = RENAME_MAP
        .iter()
        .find_map(|(from, to)| if from == &name { Some(to) } else { None })
    {
        to.to_string()
    } else {
        name
    }
}

pub(crate) fn name_to_underscore_name(name: &str) -> String {
    let name: String = name
        .chars()
        .map(|v| v.to_ascii_lowercase())
        .filter_map(|v| {
            if v == '-' || v == '.' || v == '[' || v == '{' {
                Some('_')
            } else if v == ']' || v == '}' {
                None
            } else {
                Some(v)
            }
        })
        .collect();

    check_rename(name)
}

pub(crate) fn name_to_ident(name: &str) -> String {
    let mut chars = name.chars();
    let mut new_name = String::new();

    if let Some(char) = chars.next() {
        new_name.push(char.to_ascii_uppercase());
    }

    let mut prev_was_dash = false;
    for char in chars {
        if char == '}' {
            continue;
        } else if char == '-' || char == '_' || char == '{' {
            prev_was_dash = true;
        } else if char == '+' || char == '.' {
            prev_was_dash = true;
            new_name.push('_')
        } else {
            if prev_was_dash {
                new_name.push(char.to_ascii_uppercase());
            } else {
                new_name.push(char);
            }

            prev_was_dash = false;
        }
    }

    check_rename(new_name)
}
