use std::{fs, path::Path};

use toml::Value;

const TEMPLATE: &str = include_str!("../Template.toml");

pub fn parse_toml(path: Option<String>, dir: String) {
    let mut toml_file = String::new();

    match path {
        Some(path) => {
            toml_file = fs::read_to_string(path).expect("Path not valid.");
        }

        None => {
            toml_file = TEMPLATE.to_string();
        }
    }

    let value: Value = toml::from_str(&toml_file).expect("Failed reading TOML file.");

    if let Some(table) = value.as_table() {
        process_table(table, Path::new(&dir));
    }
}

fn process_table(table: &toml::map::Map<String, Value>, base: &Path) {
    for (key, val) in table {
        // choose directory: top-level "files" stays at base, other keys create subdir
        let mut dir = base.to_path_buf();
        if key != "files" {
            dir = base.join(key);
        }
        fs::create_dir_all(&dir).expect("failed to create directory");

        // handle when the value is an array (this covers top-level `files = [...]`)
        if let Some(list) = val.as_array() {
            for item in list {
                if let Some(name) = item.as_str() {
                    let file_path = dir.join(name);
                    fs::write(file_path, "").expect("failed to write file");
                }
            }
        } else {
            // handle when the value is a table containing a "files" key (normal sections)
            if let Some(files) = val.get("files") {
                if let Some(list) = files.as_array() {
                    for item in list {
                        if let Some(name) = item.as_str() {
                            let file_path = dir.join(name);
                            fs::write(file_path, "").expect("failed to write file");
                        }
                    }
                }
            }
        }

        // skip recursion for the literal "files" entry
        if key == "files" {
            continue;
        }

        // recurse into any subtables
        if let Some(subtable) = val.as_table() {
            process_table(subtable, &dir);
        }
    }
}
