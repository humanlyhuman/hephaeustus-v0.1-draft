// assembler/src/bin.rs
use std::fs::File;
use std::io::Write;

pub fn write_osl_bin(
    path: &str,
    text: &[u16],
    text_base: u64,
    data_base: u64,
) -> Result<(), String> {
    let text_bytes: Vec<u8> = text.iter().flat_map(|w| w.to_le_bytes()).collect();
    let entry      = text_base;
    let text_size  = text_bytes.len() as u64;   // ← BYTES, not instructions!
    let data_size  = 0u64;

    let mut f = File::create(path).map_err(|e| e.to_string())?;

    f.write_all(&entry.to_le_bytes())       .map_err(|_| "write error")?;
    f.write_all(&text_base.to_le_bytes())   .map_err(|_| "write error")?;
    f.write_all(&text_size.to_le_bytes())   .map_err(|_| "write error")?;
    f.write_all(&data_base.to_le_bytes())   .map_err(|_| "write error")?;
    f.write_all(&data_size.to_le_bytes())   .map_err(|_| "write error")?;
    f.write_all(&text_bytes)                .map_err(|_| "write error")?;

    Ok(())
}