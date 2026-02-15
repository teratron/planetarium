use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=assets");

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(&manifest_dir).parent().unwrap().parent().unwrap();

    let target_dir = workspace_root
        .join("target")
        .join("assets")
        .join("framework")
        .join("launcher");

    if Path::new("assets").exists() {
        fs::create_dir_all(&target_dir).unwrap();
        copy_dir_recursive("assets", &target_dir).unwrap();
        println!("cargo:warning=Copied launcher assets to {:?}", target_dir);
    }
}

fn copy_dir_recursive(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.as_ref().join(entry.file_name());

        if ty.is_dir() {
            copy_dir_recursive(entry.path(), dst_path)?;
        } else {
            fs::copy(entry.path(), dst_path)?;
        }
    }

    Ok(())
}
