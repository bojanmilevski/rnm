use crate::errors::Error;
use crate::errors::Result;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct AbstractRenamable {
	pub abs_path: PathBuf,
	pub root: PathBuf,
	pub name: String,
}

impl<'a> TryFrom<&'a Path> for AbstractRenamable {
	type Error = Error;

	fn try_from(path: &'a Path) -> Result<Self> {
		let abs_path = std::fs::canonicalize(path)?;
		let root = abs_path.parent().unwrap().to_owned();
		let name = abs_path.file_stem().unwrap().to_str().unwrap().to_owned();
		Ok(Self { abs_path, root, name })
	}
}
