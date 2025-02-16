use crate::constants::{REACT_INCLUDE, REACT_PATH, RESULT_DIR};
use std::{
    fs::{create_dir_all, File},
    io::Write,
};

pub fn start() {
    for (name, path) in REACT_INCLUDE {
        let _ = create_dir_all(format!("{}/{}", RESULT_DIR, name));

        let mut file = File::create(format!("{}/{}/init.lua", RESULT_DIR, name))
            .expect("[ERROR] Error creating excluded file.");

        let module_suffix: String = if path.is_empty() {
            "".to_string()
        } else {
            format!(".{}", path)
        };

        file.write_all(
            format!(
                "local module = require({}[\"{}\"]{})\nreturn module",
                REACT_PATH, name, module_suffix
            )
            .as_bytes(),
        )
        .expect("[ERROR] Error writing to file.");
    }
}
