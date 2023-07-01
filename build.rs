use miniz_oxide::deflate::compress_to_vec;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
};
use unicase::UniCase;

fn main() -> io::Result<()> {
    let txt_folderpath = "src/txt/";
    let gz_folderpath = "src/gz/";

    for entry in fs::read_dir(txt_folderpath)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("txt")) {
            let filename = path.file_stem().unwrap().to_string_lossy();
            compress_file(
                path.to_str().unwrap(),
                &format!("{}{}.gz", gz_folderpath, filename),
            )?;
        }
    }

    Ok(())
}
pub fn compress_file(path: &str, output_path: &str) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let mut lines: Vec<&str> = std::str::from_utf8(&contents).unwrap().lines().collect();
    lines.sort_by_key(|w| UniCase::new(w.clone()));

    let compressed = compress_to_vec(lines.join("\n").as_bytes(), 6);

    let mut output_file = File::create(output_path)?;

    output_file.write_all(&compressed)?;

    Ok(())
}
