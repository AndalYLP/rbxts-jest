use std::process::Command;

pub fn start() {
    let output = Command::new("rojo")
        .args(["build", "-o", "packages/jest-vendor/jest-lua.rbxm"])
        .output()
        .expect("Error building with wally");
    assert!(output.status.success(), "[FATAL] error building JestLua");
}
