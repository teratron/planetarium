use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Rebuild if assets change
    println!("cargo:rerun-if-changed=assets");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = Path::new(&manifest_dir).parent().unwrap().parent().unwrap();

    let src_dir = Path::new(&manifest_dir).join("assets");
    let dst_dir = workspace_root
        .join("target")
        .join("assets")
        .join("framework")
        .join("menu");

    if src_dir.exists() {
        // Create destination
        fs::create_dir_all(&dst_dir).unwrap();

        // Copy assets
        match copy_assets(&src_dir, &dst_dir) {
            Ok(count) => {
                println!(
                    "cargo:warning=Copied {} menu assets to target/assets",
                    count
                );
            }
            Err(e) => {
                println!("cargo:warning=Failed to copy menu assets: {}", e);
            }
        }
    }
}

fn copy_assets(src: &Path, dst: &Path) -> std::io::Result<usize> {
    let mut count = 0;

    for entry in walkdir::WalkDir::new(src)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_file() {
            let relative = path.strip_prefix(src).unwrap();
            let dst_path = dst.join(relative);

            // Create parent directories
            if let Some(parent) = dst_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Copy file
            fs::copy(path, &dst_path)?;
            count += 1;
        }
    }

    Ok(count)
}
