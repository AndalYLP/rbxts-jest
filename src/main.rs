use std::time::Instant;

mod constants;
mod transform;
mod utils;

use crate::transform::{add_react_files, transform_dir};
use crate::utils::{build_rojo, install_jest};

fn main() {
    let start_main = Instant::now();

    install_jest::start();

    let start = Instant::now();

    transform_dir::start();
    add_react_files::start();

    let duration = start.elapsed();

    println!("Execution time: {:?}", duration);

    build_rojo::start();

    let duration_main = start_main.elapsed();

    println!("Build jest-lua in {:?}", duration_main);
}
