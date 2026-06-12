use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use crate::crypto::XorReader;

pub fn unpack(archive_path: &PathBuf, password: Option<&str>) -> io::Result<()> {
    let file = File::open(archive_path).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to open archive: {}", e))
    })?;
    
    let reader = BufReader::new(file);
    let mut xor_reader = XorReader::new(reader, password);

    let mut num_files_buf = [0u8; 4];
    xor_reader.read_exact(&mut num_files_buf).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "Corrupted archive: Cannot read number of files. Incorrect password?")
    })?;
    let num_files = u32::from_le_bytes(num_files_buf);

    struct FileEntry {
        name: String,
        size: u64,
    }

    let mut entries = Vec::new();


    for _ in 0..num_files {
        let mut len_buf = [0u8; 4];
        xor_reader.read_exact(&mut len_buf).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Corrupted archive header (filename length).")
        })?;
        let name_len = u32::from_le_bytes(len_buf);

        if name_len == 0 || name_len > 4096 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Archive is corrupted or wrong password provided.",
            ));
        }

        let mut name_buf = vec![0u8; name_len as usize];
        xor_reader.read_exact(&mut name_buf).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Corrupted archive header (filename).")
        })?;
        let name = String::from_utf8(name_buf).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Corrupted archive: Invalid UTF-8 in filename.")
        })?;

        let mut size_buf = [0u8; 8];
        xor_reader.read_exact(&mut size_buf).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Corrupted archive header (file size).")
        })?;
        let size = u64::from_le_bytes(size_buf);

        entries.push(FileEntry { name, size });
    }

    for entry in &entries {
        let out_file = File::create(&entry.name)?;
        let mut out_writer = BufWriter::new(out_file);
        
        let mut remaining = entry.size;
        let mut buffer = [0u8; 8192];

        while remaining > 0 {
            let to_read = std::cmp::min(remaining, buffer.len() as u64) as usize;
            xor_reader.read_exact(&mut buffer[..to_read]).map_err(|_| {
                io::Error::new(io::ErrorKind::UnexpectedEof, format!("Unexpected EOF reading '{}'.", entry.name))
            })?;
            out_writer.write_all(&buffer[..to_read])?;
            remaining -= to_read as u64;
        }
        out_writer.flush()?;
    }

    let file_names: Vec<String> = entries.into_iter().map(|e| e.name).collect();
    println!("Unpacked {} files: {}", num_files, file_names.join(", "));

    Ok(())
}
