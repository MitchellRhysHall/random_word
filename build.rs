use std::{fs::{File, self}, io::{Read, Write, self, BufRead, BufReader, BufWriter}, path::Path, ffi::OsStr};
use miniz_oxide::deflate::compress_to_vec;
use unicase::UniCase;

fn main()-> io::Result<()> {
    let txt_folderpath = "src/txt/";
    let gz_folderpath = "src/gz/";

    // for entry in fs::read_dir(txt_folderpath)? {
    //     let entry = entry?;
    //     let path = entry.path();
    //     if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("txt")) {
    //         let filename = path.file_stem().unwrap().to_string_lossy();
    //         compress_file(path.to_str().unwrap(), &format!("{}{}.gz", gz_folderpath, filename))?;
    //     }
    // }

    Ok(())
}

fn sort_files_in_dir(dir: &str) -> io::Result<()> {
    // Read the specified directory
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("txt")) {
            // Read the file
            let file = File::open(&path)?;
            let reader = BufReader::new(file);

            // Collect all lines
            let mut lines: Vec<String> = Vec::new();
            for line in reader.lines() {
                let line = line?;
                lines.push(line);
            }

            // Sort lines
            lines.sort_by_key(|w| UniCase::new(w.clone()));

            // Write sorted lines to a new file
            let new_path = path.with_file_name(format!("sorted_{}", path.file_name().unwrap().to_str().unwrap()));
            let file = File::create(new_path)?;
            let mut writer = BufWriter::new(file);
            for line in lines {
                writeln!(writer, "{}", line)?;
            }
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