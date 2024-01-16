use brotli::{enc::backward_references::BrotliEncoderParams, CompressorWriter};
use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};
use unicase::UniCase;

fn main() -> io::Result<()> {
    let txt_folderpath = PathBuf::from("src/txt/");
    let br_folderpath = PathBuf::from("src/br/");

    let txt_file_paths = read_dir_filter_ext(&txt_folderpath, "txt")?;

    let br_file_names = read_dir_filter_ext(&br_folderpath, "br")?
        .into_iter()
        .map(|path| path.file_stem().unwrap().to_str().unwrap().to_owned())
        .collect::<Vec<_>>();

    for txt_path in &txt_file_paths {
        let stem = txt_path.file_stem().unwrap().to_str().unwrap();
        if !br_file_names.contains(&stem.to_string()) {
            let br_output_path = br_folderpath.join(format!("{}.br", stem));
            compress_file_brotli(&txt_path, &br_output_path)?;
        }
    }

    Ok(())
}

fn compress_file_brotli(path: &PathBuf, output_path: &PathBuf) -> io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut lines = Vec::new();
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        lines.push(line.clone());
        line.clear();
    }

    lines.sort_by_key(|l| UniCase::new(l.clone()));

    let output_file = File::create(output_path)?;
    let writer = BufWriter::new(output_file);
    let params = BrotliEncoderParams::default();
    let mut compressor =
        CompressorWriter::new(writer, 4096, params.quality as u32, params.lgwin as u32);

    for line in lines {
        compressor.write_all(line.as_bytes())?;
    }

    compressor.flush()?;
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
