use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let libs_dir = Path::new("libs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).ancestors().nth(3).unwrap().join("libs");

    if target_dir.exists() {
        fs::remove_dir_all(&target_dir).unwrap();
    }

    copy_dir_recursive(libs_dir, &target_dir).unwrap();

    println!("cargo:rerun-if-changed=libs");
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}
