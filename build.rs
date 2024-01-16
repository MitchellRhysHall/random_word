use miniz_oxide::deflate::compress_to_vec;
use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
};
use unicase::UniCase;

fn main() -> io::Result<()> {
    let txt_folderpath = PathBuf::from("src/txt/");
    let gz_folderpath = PathBuf::from("src/gz/");

    let txt_file_paths = read_dir_filter_ext(&txt_folderpath, "txt")?;
    let gz_file_names = read_dir_filter_ext(&gz_folderpath, "gz")?
        .into_iter()
        .map(|path| path.file_stem().unwrap().to_str().unwrap().to_owned())
        .collect::<Vec<_>>();

    for txt_path in txt_file_paths {
        let stem = txt_path.file_stem().unwrap().to_str().unwrap();
        if !gz_file_names.contains(&stem.to_string()) {
            let output_path = gz_folderpath.join(format!("{}.gz", stem));
            compress_file(&txt_path, &output_path)?;
        }
    }

    Ok(())
}

fn read_dir_filter_ext(dir: &PathBuf, ext: &str) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some(OsStr::new(ext)) {
            paths.push(path);
        }
    }
    Ok(paths)
}

pub fn compress_file(path: &PathBuf, output_path: &PathBuf) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let lines: Vec<&str> = std::str::from_utf8(&contents).unwrap().lines().collect();
    let mut sorted_lines = lines.clone();
    sorted_lines.sort_by_key(|&w| UniCase::new(w));
    let compressed = compress_to_vec(sorted_lines.join("\n").as_bytes(), 6);
    let mut output_file = File::create(output_path)?;
    output_file.write_all(&compressed)?;

    Ok(())
}
