use super::file::RenamableFile;
use super::folder::RenamableFolder;
use crate::errors::Error;
use crate::errors::Result;
use std::path::Path;

#[derive(Debug)]
pub enum RenamableItem {
	FILE(RenamableFile),
	FOLDER(RenamableFolder),
}

impl<'a> TryFrom<&'a Path> for RenamableItem {
	type Error = Error;

	fn try_from(path: &'a Path) -> Result<Self> {
		let item = match path.is_file() {
			true => RenamableItem::FILE(RenamableFile::try_from(path)?),
			false => RenamableItem::FOLDER(RenamableFolder::try_from(path)?),
		};

		Ok(item)
	}
}
