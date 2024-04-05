mod cli;
mod errors;
mod renamable;

use clap::Parser;
use cli::Cli;
use errors::Result;
use renamable::item::RenamableItem;

#[tokio::main]
async fn main() -> Result<()> {
	let args = Cli::parse();

	for path in args.paths {
		for dir in walkdir::WalkDir::new(path) {
			let renamable = RenamableItem::try_from(dir?.path())?;
			dbg!(renamable);
			//std::fs::rename(&renamable.abs_path, &renamable.new_path)?;
		}
	}

	Ok(())
}
