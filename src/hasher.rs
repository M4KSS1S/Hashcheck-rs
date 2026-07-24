use anyhow::{Context, Result};
use sha2::{Sha256, Digest};
use std::io::{self, Read};
use std::fs::File;

pub fn is_valid_hash(hash: &str) -> bool {
    let clean = hash.trim();
    !clean.is_empty() 
        && clean.len() % 2 == 0 
        && clean.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn hash_file(path : &str, algo : &crate::cli::Algorithm) -> Result<String>
{

	let mut file = File::open(&path).with_context(|| format!("can't open file : {}",path))?;
	hash_reader(&mut file, algo)
}

pub fn  hash_stdin(algo: &crate::cli::Algorithm) -> Result<String>
{
	let stdiin = io::stdin();
	let mut handle = stdiin.lock();
	hash_reader(&mut handle, algo)
}

fn hash_reader<R : Read>(reader: &mut R,algo : &crate::cli::Algorithm) -> Result<String>
{
	match algo
	{
		crate::cli::Algorithm::Sha256 => {
			let mut hasher = Sha256::new();
			let mut buffer = [0u8; 8192];
			loop
			{
				let bytes_read = reader.read(&mut buffer).context("failed to read file")?;
				if bytes_read == 0 { break;}
				hasher.update(&buffer[..bytes_read]);
			}
			Ok(hex::encode(hasher.finalize()))
		},
		crate::cli::Algorithm::Blake3 => {
			let mut hasher = blake3::Hasher::new();
			let mut buffer = [0u8; 8192];
			loop
			{
				let bytes_read = reader.read(&mut buffer).context("failed to read file")?;
				if bytes_read == 0 { break;}
				hasher.update(&buffer[..bytes_read]);
			}
			Ok(hasher.finalize().to_hex().to_string())
		}
	}	
}