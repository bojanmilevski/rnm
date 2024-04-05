use super::abstract_renamable::AbstractRenamable;
use super::new_renamable::NewRenamable;
use crate::errors::Error;
use crate::errors::Result;
use std::path::Path;

#[derive(Debug)]
pub struct RenamableFolder {
	renamable: AbstractRenamable,
	new_renamable: NewRenamable,
}

impl<'a> TryFrom<&'a Path> for RenamableFolder {
	type Error = Error;

	fn try_from(path: &'a Path) -> Result<Self> {
		let renamable = AbstractRenamable::try_from(path)?;
		let new_renamable = NewRenamable::try_from(&renamable)?;
		Ok(Self { renamable, new_renamable })
	}
}
