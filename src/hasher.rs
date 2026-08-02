use anyhow::{Context, Result};
use sha2::{Sha256, Digest};
use std::io::{self, Read, ErrorKind};
use std::fs::File;
use std::error::Error;

pub fn is_valid_hash(hash: &str) -> bool {
    let clean = hash.trim();
    !clean.is_empty() 
        && clean.len() % 2 == 0 
        && clean.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn hash_file(path : &str, algo : &crate::cli::Algorithm) -> Result<String, Box<dyn Error>>
{
	let mut file = File::open(&path)?;
	hash_reader(&mut file, algo, false)
}

pub fn  hash_stdin(algo: &crate::cli::Algorithm) -> Result<String, Box<dyn Error>>
{
	let stdiin = io::stdin();
	let mut handle = stdiin.lock();
	match hash_reader(&mut handle, algo, true)
	{
		Ok(str) => Ok(str),
		Err(_err) => Err(std::io::Error::new(ErrorKind::UnexpectedEof, "stdin is closed or empty").into()),
	}
}

fn hash_reader<R : Read>(reader: &mut R, algo : &crate::cli::Algorithm, require_input: bool) -> Result<String, Box<dyn Error>>
{
	match algo
	{
		crate::cli::Algorithm::Sha256 => {
			let mut hasher = Sha256::new();
			let mut buffer = [0u8; 8192];
			let mut read_any = false;
			loop
			{
				let bytes_read = reader.read(&mut buffer)?;
				if bytes_read == 0 { break;}
				read_any = true;
				hasher.update(&buffer[..bytes_read]);
			}
			if require_input && !read_any {
				return Err(std::io::Error::new(ErrorKind::UnexpectedEof, "stdin is closed or empty").into());
			}
			Ok(hex::encode(hasher.finalize()))
		},
		crate::cli::Algorithm::Blake3 => {
			let mut hasher = blake3::Hasher::new();
			let mut buffer = [0u8; 8192];
			let mut read_any = false;
			loop
			{
				let bytes_read = reader.read(&mut buffer)?;
				if bytes_read == 0 { break;}
				read_any = true;
				hasher.update(&buffer[..bytes_read]);
			}
			if require_input && !read_any {
				return Err(std::io::Error::new(ErrorKind::UnexpectedEof, "stdin is closed or empty").into());
			}
			Ok(hasher.finalize().to_hex().to_string())
		}
	}	
}