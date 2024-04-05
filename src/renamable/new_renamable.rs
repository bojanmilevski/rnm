use super::abstract_renamable::AbstractRenamable;
use super::renamable::Renamable;
use crate::errors::Error;
use crate::errors::Result;
use std::path::PathBuf;

#[derive(Debug)]
pub struct NewRenamable {
	pub new_name: String,
	pub new_path: PathBuf,
}

impl Renamable for NewRenamable {}

impl TryFrom<&AbstractRenamable> for NewRenamable {
	type Error = Error;

	fn try_from(renamable: &AbstractRenamable) -> Result<Self> {
		let new_name = Self::rename_item(&renamable.name)?;
		let new_path = renamable.root.join(&new_name);
		Ok(Self { new_name, new_path })
	}
}
