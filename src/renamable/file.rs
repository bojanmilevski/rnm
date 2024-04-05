use super::abstract_renamable::AbstractRenamable;
use super::new_renamable::NewRenamable;
use super::renamable::Renamable;
use crate::errors::Error;
use crate::errors::Result;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct RenamableFile {
	ext: Option<String>,
	new_ext: Option<String>,
	renamable: AbstractRenamable,
	new_renamable: NewRenamable,
}

impl Renamable for RenamableFile {}

impl<'a> TryFrom<&'a Path> for RenamableFile {
	type Error = Error;

	fn try_from(path: &'a Path) -> Result<Self> {
		let renamable = AbstractRenamable::try_from(path)?;
		let mut new_renamable = NewRenamable::try_from(&renamable)?;

		let ext = match renamable.abs_path.extension() {
			Some(ext) => Some(ext.to_str().unwrap().to_owned()),
			None => None,
		};

		let new_ext = match &ext {
			Some(ext) => Some(Self::rename_item(&ext)?),
			None => None,
		};

		new_renamable.new_path = match &new_ext {
			Some(ext) => PathBuf::from(format!("{}.{}", new_renamable.new_path.display(), ext)),
			None => new_renamable.new_path,
		};

		Ok(Self { renamable, new_renamable, ext, new_ext })
	}
}
