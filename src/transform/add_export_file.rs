use crate::utils::to_pascal_case;
use std::{
    fs::{read_to_string, File},
    io::Write,
};

pub fn start(name: &str) {
    let formatted_name = to_pascal_case::convert(&name);

    let mut file = File::create(format!("./JestLua/{}.lua", formatted_name))
        .expect("[ERROR] Error creating export file.");

    let definitions_file: String = read_to_string(format!("./exports/{}.lua", formatted_name))
        .unwrap_or_else(|_| {
            eprintln!("[WARN File definition not found.");
            "return module".to_string()
        });

    let _ = file.write_all(format!("{}\n{}",format!(
        "local module = require(script.Parent:WaitForChild(\"node_modules\"):WaitForChild(\"@jsdotlua\"):WaitForChild(\"{}\"))", name), definitions_file).as_bytes());
}
