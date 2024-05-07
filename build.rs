use fs_extra::dir::CopyOptions;
use std::env;

use std::path::{Path, PathBuf};

fn main() {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let profile = env::var("PROFILE").unwrap();

    let source_dir = project_root.join("simconnect");

    let dll_path = source_dir.join("SimConnect.dll");
    if !dll_path.exists() {
        println!("SimConnect.dll not found in project folder. Skipping copy.");
    }

    let target_dir = format!("./target/{}/", profile);

    let target_dll_path = Path::new(&target_dir).join("SimConnect.dll");
    if !target_dll_path.exists() {
        let source_files = vec![dll_path.to_str().unwrap().to_string()];
        let options = CopyOptions::new();
        if let Err(err) = fs_extra::copy_items(&source_files, &target_dir, &options) {
            eprintln!("Error copying DLL: {}", err);
        }
    }
}
