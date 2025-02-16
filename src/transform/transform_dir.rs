use futures::future::join_all;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use tokio::fs::{read_dir, rename, DirEntry};

use crate::constants::{JEST_INCLUDE, REACT_EXCLUDE, REACT_PATH, RESULT_DIR};
use crate::transform::add_export_file;

//use crate::transform::transform_import;

async fn move_to_result_dir(dir: DirEntry) {
    let binding = dir.file_name();
    let file_name = binding.to_string_lossy();

    let real_name = file_name
        .split("jsdotlua_")
        .nth(1)
        .and_then(|s| s.split('@').next())
        .expect("[ERROR] Couldn't change result file name.");

    if file_name.contains("jest") || JEST_INCLUDE.contains(&file_name.as_ref()) {
        add_export_file::start(real_name)
    }

    let dest_path = Path::new(RESULT_DIR).join(&real_name);

    if let Some(&(_, path)) = REACT_EXCLUDE.iter().find(|&&(name, _)| name == real_name) {
        let _ = create_dir_all(
            dest_path
                .to_str()
                .expect("[ERROR] Invalid destination path."),
        )
        .expect("[ERROR] Error creating REACT_EXCLUDE dir.");

        let mut file = File::create(format!(
            "{}/init.lua",
            dest_path
                .to_str()
                .expect("[ERROR] Invalid destination path.")
        ))
        .expect("[ERROR] Error creating excluded file.");

        let module_suffix: String = if path.is_empty() {
            "".to_string()
        } else {
            format!(".{}", path)
        };

        file.write_all(
            format!(
                "local module = require({}[\"{}\"]{})\nreturn module",
                REACT_PATH, real_name, module_suffix
            )
            .as_bytes(),
        )
        .expect("[ERROR] Error writing to file.");

        return;
    }

    let src_path = dir.path().join(&real_name);

    let _ = rename(src_path, &dest_path)
        .await
        .expect("[ERROR] error while moving file to dest dir.");

    //transform_import::start(dest_path.join("src"));
}

#[tokio::main]
pub async fn start() {
    assert!(
        create_dir_all(RESULT_DIR).is_ok(),
        "[ERROR] Error creating result dir."
    );

    let mut result = read_dir("./DevPackages/_Index")
        .await
        .expect("[FATAL] Error reading JestLua dir.");

    let mut tasks = vec![];

    while let Some(entry) = result
        .next_entry()
        .await
        .expect("[ERROR] Error reading dir entry.")
    {
        tasks.push(move_to_result_dir(entry));
    }

    join_all(tasks).await;
}
