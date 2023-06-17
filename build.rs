use std::{fs::{File, self}, io::{Read, Write, self}};

use miniz_oxide::deflate::compress_to_vec;

fn main()-> io::Result<()> {
    let csv_folderpath = "src/db/csv/";
    let gz_folderpath = "src/db/gz/";

    for entry in fs::read_dir(csv_folderpath)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("csv")) {
            let filename = path.file_stem().unwrap().to_string_lossy();
            compress_file(path.to_str().unwrap(), &format!("{}{}.gz", gz_folderpath, filename))?;
        }
    }

    Ok(())
}

pub fn compress_file(path: &str, output_path: &str) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    let compressed = compress_to_vec(&contents, 6);

    let mut output_file = File::create(output_path)?;

    output_file.write_all(&compressed)?;

    Ok(())
}