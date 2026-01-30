use std::fs;
use std::path::Path;

pub fn move_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if fs::rename(src, dst).is_ok() {
        return Ok(());
    }
    if src.is_dir() {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            move_recursive(&entry.path(), &dst.join(entry.file_name()))?;
        }
        fs::remove_dir_all(src)?;
    } else {
        fs::copy(src, dst)?;
        fs::remove_file(src)?;
    }
    Ok(())
}

pub fn hoist(src: &Path, dst: &Path) -> std::io::Result<()> {
    let mut current = src.to_path_buf();
    loop {
        let entries: Vec<_> = fs::read_dir(&current)?.collect::<Result<Vec<_>, _>>()?;
        if entries.len() == 1 && entries[0].path().is_dir() {
            current = entries[0].path();
        } else {
            break;
        }
    }
    move_recursive(&current, dst)
}

pub fn open_path(path: &str) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer").arg(path).spawn()?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(path).spawn()?;
    }
    Ok(())
}

pub fn check_path_exists(path: &str) -> bool {
    Path::new(path).exists()
}
