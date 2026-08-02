use crate::hasher::hash_stdin;
use crate::cli::Algorithm;
use crate::hasher::is_valid_hash;

#[test]
fn valid_hash()
{
	assert!(is_valid_hash("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"));
}

#[test]
fn invalid_hash()
{
	assert!(!is_valid_hash("not-a-hash"));
}

#[test]
fn empty_str()
{
	assert!(!is_valid_hash(""));
}

#[test]
fn hash_with_more_bytes()
{
	assert!(!is_valid_hash("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b8557"));
}

#[test]
fn hash_less_more_bytes()
{
	assert!(!is_valid_hash("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b85"));
}

// stdin tests
#[test]
#[should_panic]
// ensure hash_stdin errors when stdin is closed/empty, to close it => exec 0<&-
fn empty_or_closed_stdin()
{
	let algo = Algorithm::Sha256;
	match hash_stdin(&algo)
	{
		Ok(str) => panic!("expected error when stdin is closed"),
		Err(err) => format!(""),
	};
	// we expect an error when stdin is closed/empty
}