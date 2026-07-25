use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command (name="hashcheck")]
#[command (author="Created by M4KSS1S, github here : https://github.com/M4KSS1S :) enjoy!")]
#[command (about="Hash files, verify checksums")]
#[command (override_usage="Examples = ./hashcheck file.txt --algo sha256 -v (FILE_HASH), ./hashcheck --stdin -a blake3")]
#[command(group = clap::ArgGroup::new("input")
    .required(true)
    .args(&["file", "stdin"])
)]
pub struct Args{
	
	/// File to hash
	pub file: Option<String>, // with no #[arg()] this argument shoudl be written without a flag

	/// Hashing Algorithm to use
	#[arg(short, long, value_enum, default_value="sha256")]
	pub algo: Algorithm,

	/// Verify file checksum
	#[arg(short, long)]
	pub verify: Option<String>,
	
	/// Read from stdin !!(use ctrl+d or cmd+d if you want to end the listening)
	#[arg(short, long)]
	pub stdin: bool,
}

#[derive(Clone, ValueEnum)]
pub enum Algorithm{
	Sha256,
	Blake3,
}