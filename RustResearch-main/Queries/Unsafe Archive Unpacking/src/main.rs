extern crate tar;
use rocket::fs::FileName;
use std::error::Error;
use std::io::Write;
use std::io::{self, Read};
use std::{fs::File, path::Path, path::PathBuf};
use tar::Archive;
use tar::Builder;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use std::env::args_os;
use std::io::{copy, stdin, stdout};

const PATH_DST: &str = "C:\\Users\\orir\\OneDrive - Checkmarx\\Desktop\\SAST\\Rust\\repo\\RustResearch\\Queries\\Unsafe Archive Unpacking\\foo";
const PATH_ZIP: &str = "./zip-slip-win.zip";
const PATH_TAR: &str = "C:\\Users\\orir\\OneDrive - Checkmarx\\Desktop\\SAST\\Rust\\repo\\RustResearch\\Queries\\Unsafe Archive Unpacking\\src\\exp.tar";


fn zip_extract_vulnerable() -> Result<(), Box<dyn Error>> {
    let zip_file_path = Path::new(PATH_ZIP);
    let zip_file = File::open(zip_file_path)?;
    let mut archive = ZipArchive::new(zip_file)?; // SOURCE
    let extraction_dir = Path::new("extracted_files");

    // Create the directory if it does not exist.
    if !extraction_dir.exists() {
        std::fs::create_dir(extraction_dir)?;
    }

    // Iterate through the files in the ZIP archive.
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_owned();

        // Create the path to the extracted file in the destination directory.
        let target_path = extraction_dir.join(file_name);
        println!(
            "{}: path assigned for the file is: {}",
            i,
            target_path.display()
        );
        // Create the destination directory if it does not exist.
        if let Some(parent_dir) = target_path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }

        let mut output_file = File::create(&target_path)?; // RESULT - SINK without any sanitization

        // Read the contents of the file from the ZIP archive and write them to the destination file.
        io::copy(&mut file, &mut output_file)?;
        //println!("File extracted to: {}", output_file.display());
    }

    println!("Files successfully extracted to {:?}", extraction_dir);

    Ok(())
}
fn zip_extract_safe_path_traversal_sanitizer() -> Result<(), Box<dyn Error>> {
    let zip_file_path = Path::new(PATH_ZIP);
    let zip_file = File::open(zip_file_path)?;

    let mut archive = ZipArchive::new(zip_file)?; // SOURCE
    let extraction_dir = Path::new("extracted_files");

    // Create the directory if it does not exist.
    if !extraction_dir.exists() {
        std::fs::create_dir(extraction_dir)?;
    }

    // Iterate through the files in the ZIP archive.
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_owned();
        let inPath = FileName::new(&file_name); // SANITIZER - path traversal sanitizer
        let target_path = extraction_dir.join(inPath.as_str().unwrap()); //
        println!(
            "{}: path assigned for the file is: {}",
            i,
            target_path.display()
        );
        // Create the destination directory if it does not exist.
        if let Some(parent_dir) = target_path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }

        let mut output_file = File::create(&target_path)?; // SINK - this is safe, as the sanitizer is applied

        // Read the contents of the file from the ZIP archive and write them to the destination file.
        io::copy(&mut file, &mut output_file)?;
        //println!("File extracted to: {}", output_file.display());
    }

    println!("Files successfully extracted to {:?}", extraction_dir);

    Ok(())
}
fn zip_extract_safe_built_in_sanitizer(filename: &str) {
    let file = fs::File::open(fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap(); // SOURCE

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() // SANITIZER
        {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if file.is_dir() {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap(); // SINK - this is safe, as the sanitizer is applied
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}

fn tar_extract_vulnerable() {
    let mut ar = Archive::new(File::open(PATH_TAR).unwrap()); // SOURCE
    for file in ar.entries().unwrap() {
        let mut f = file.unwrap();
        
        let mut output_file = File::create(&f.path().unwrap()).unwrap(); // RESULT - SINK without any sanitization        
        copy(&mut f, &mut output_file).unwrap();
    }
}

fn tar_extract_safe() {
    // SAFE - no sink, thus no result
    let mut ar = Archive::new(File::open(PATH_TAR).unwrap()); // SOURCE
    ar.unpack(PATH_DST).unwrap(); // NOT A SINK
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    zip_extract_vulnerable()?;
    zip_extract_safe_path_traversal_sanitizer()?;
    tar_extract_vulnerable();
    tar_extract_safe();
    Ok(())
}
