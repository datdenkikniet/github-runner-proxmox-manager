use std::io::BufReader;

pub(crate) mod raw;

pub(crate) mod path;
pub(crate) use path::{Path, PathElement};

pub(crate) mod generator;

use clap::Parser;
use raw::{flattened::Collection, TreeNode};

const RENAME_MAP: &[(&str, &str)] = &[
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

    for char in chars.by_ref() {
        if char == '.'
            || char == '{'
            || char == '}'
            || char == '['
            || char == ']'
            || char == '_'
            || char == '-'
        {
            continue;
        } else {
            new_name.push(char.to_ascii_uppercase());
            break;
        }
    }

    if let Some(v) = new_name.chars().next() {
        if v.is_numeric() {
            new_name = format!("_{new_name}");
        }
    }

    let mut prev_was_dash = false;
    for char in chars {
        if char == '}' || char == ']' {
            continue;
        } else if char == '-'
            || char == '_'
            || char == '{'
            || char == '['
            || char == '+'
            || char == '.'
        {
            prev_was_dash = true;
            continue;
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Style {
    Recursive,
    NonRecursive,
}

#[derive(Parser)]
pub enum Cli {
    Recursive { input_path: String, file: String },
    NonRecursive { input_path: String, file: String },
}

impl Cli {
    pub fn input_path(&self) -> &str {
        match self {
            Cli::Recursive { input_path, .. } => input_path,
            Cli::NonRecursive { input_path, .. } => input_path,
        }
    }
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let input_path = cli.input_path();

    let str = if input_path.ends_with(".xz") {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .open(input_path)
            .unwrap();

        let mut reader = BufReader::new(file);

        let mut output = Vec::new();
        lzma_rs::xz_decompress(&mut reader, &mut output).unwrap();

        String::from_utf8(output).unwrap()
    } else {
        std::fs::read_to_string(input_path).unwrap()
    };

    let tree: Vec<TreeNode> = serde_json::from_str(&str).unwrap();

    let collection = Collection::from_nodes(&tree);
    let generator = generator::Generator::new(&collection);

    match cli {
        Cli::Recursive { file, .. } => generator.generate_file_tree(file),
        Cli::NonRecursive { file, .. } => generator.generate_single_file(file),
    }
}
