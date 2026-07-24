use anyhow::Result;
use colored::Colorize;
use hashcheck::cli::{Args, Algorithm};
use hashcheck::hasher;
use clap::Parser;
fn main() -> Result<()> {
    let args = Args::parse();
    let computed_hash = if args.stdin{
        hasher::hash_stdin(&args.algo)?
    }
    else {
        hasher::hash_file(&args.file, &args.algo)?
    };

    let algo_name = match args.algo
    {
        Algorithm::Sha256 => "Sha256",
        Algorithm::Blake3 => "Blake3", 
    };

    if let Some(expected) = args.verify{
        if !hasher::is_valid_hash(&expected)
        {
            eprintln!("{}", "Error: invalid verify arg hash format (64 chars, all of them are hex chars)".red());
            std::process::exit(1);
        }

        let expected_hash = expected.trim().to_lowercase();
        let computed_verify = computed_hash.to_lowercase();

        println!("File {}",&args.file);
        println!("Algo name {}", algo_name);
        println!("Computed hash :({})",computed_verify);
        println!("Expected hash :({})",expected_hash);

        if expected_hash == computed_verify
        {
            println!("{}", "✓ VERIFICATION PASSED".green().bold());
        } else {
            println!("{}", "✗ VERIFICATION FAILED".red().bold());
            std::process::exit(1);
        }
    }
    else {
        println!("{} ({})", computed_hash, algo_name);
    }
    Ok(())
}
