use std::fs::remove_dir_all;
use std::process::Command;

fn clean_dir() {
    let _ = remove_dir_all("./DevPackages");
    let _ = remove_dir_all("./JestLua");
}

pub fn start() {
    clean_dir();

    let output = Command::new("wally")
        .arg("install")
        .output()
        .expect("Error installing with wally");
    assert!(output.status.success(), "[FATAL] error installing JestLua");
}
