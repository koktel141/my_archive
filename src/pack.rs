use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::path::PathBuf;
use crate::crypto::XorWriter;

pub fn pack(files: &[PathBuf], password: Option<&str>) -> io::Result<()> {
    // بررسی وجود تمامی فایل‌ها
    for file in files {
        if !file.exists() || !file.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Error: File {:?} not found or is not a readable file.", file),
            ));
        }
    }

    let output_name = "output.arch";
    let file = File::create(output_name)?;
    let writer = BufWriter::new(file);
    let mut xor_writer = XorWriter::new(writer, password);

    if password.is_some() {
        println!("Creating encrypted archive {} ...", output_name);
    } else {
        println!("Creating archive {} ...", output_name);
    }

    // نوشتن هدر (تعداد فایل‌ها)
    let num_files = files.len() as u32;
    xor_writer.write_all(&num_files.to_le_bytes())?;

    // نوشتن اطلاعات متادیتا برای هر فایل
    for file_path in files {
        let basename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name encountered."))?;
        
        let basename_bytes = basename.as_bytes();
        let name_len = basename_bytes.len() as u32;
        
        let metadata = file_path.metadata()?;
        let file_size = metadata.len() as u64;

        xor_writer.write_all(&name_len.to_le_bytes())?;
        xor_writer.write_all(basename_bytes)?;
        xor_writer.write_all(&file_size.to_le_bytes())?;
    }

    // نوشتن محتوای باینری فایل‌ها پشت سر هم
    for file_path in files {
        let mut f = File::open(file_path)?;
        let mut buffer = [0u8; 8192];
        loop {
            let n = f.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            xor_writer.write_all(&buffer[..n])?;
        }
    }

    xor_writer.flush()?;
    println!("Packed {} files successfully.", num_files);
    Ok(())
}
