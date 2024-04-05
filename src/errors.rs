use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
	#[error("walkdir error.")]
	WalkDir(#[from] walkdir::Error),

	#[error("IO")]
	IO(#[from] std::io::Error),

	#[error("Regex error.")]
	Regex(#[from] lazy_regex::regex::Error),
}
