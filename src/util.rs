use std::path::PathBuf;

use anyhow::Error;

pub fn convert_dir_path_to_absolute_path(dir: &str, current_dir: &str) -> Result<String, Error> {
    let current_dir = PathBuf::from(current_dir);
    if dir == "." || dir == "" || dir == "./" {
        return Ok(current_dir.to_str().unwrap().to_string());
    }
    if dir.starts_with("./") {
        return Ok(current_dir.join(&dir[2..]).to_str().unwrap().to_string());
    }
    if current_dir.join(dir).is_dir() {
        return Ok(current_dir.join(dir).to_str().unwrap().to_string());
    }
    Err(Error::msg("Invalid directory"))
}
