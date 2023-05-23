use anyhow::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

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

pub fn read_lines(path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(path).map_err(|e| Error::msg(e.to_string()))?;

    let reader = BufReader::new(file);

    let mut lines = vec![];
    for line in reader.lines() {
        let line = line.map_err(|e| Error::msg(e.to_string()))?;
        lines.push(line);
    }
    Ok(lines)
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn convert_dir_path_to_absolute_path() {
        let current_dir = std::env::current_dir().unwrap();
        let current_dir = current_dir.to_str().unwrap();
        let dir = "./test";
        let dir = super::convert_dir_path_to_absolute_path(dir, current_dir).unwrap();
        assert_eq!(dir, format!("{}/test", current_dir));
    }

    #[test]
    fn read_lines() {
        let lines = super::read_lines("Cargo.toml").unwrap();
        assert_eq!(lines[0], "[package]");
    }
}
