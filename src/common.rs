use std::fs;
use std::io;
use std::path::Path;

pub fn limpar_output() -> io::Result<()> {
    let dir = Path::new("output");
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            if name == ".gitkeep" {
                continue;
            }
        }
        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }
    Ok(())
}
