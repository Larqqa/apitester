use std::{fs, path::Path, io::Result};

pub async fn write_to_file(content: String, path: &Path) -> Result<()> {
    println!("writing result to {}", path.display());

    if let Some(p) = path.parent() {
        fs::create_dir_all(p)?
    };

    fs::write(path, content).expect("Unable to write to file.");
    Ok(())
}
