use clap::Parser;
use clap::ValueHint;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
	#[arg(short, long, num_args = 1.., default_value = ".", value_hint = ValueHint::AnyPath)]
	pub paths: Vec<PathBuf>,

	#[arg(short = 'D', long)]
	pub dry_run: bool,

	#[arg(short, long, conflicts_with = "uppercase")]
	pub lowercase: bool,

	#[arg(short, long, conflicts_with = "lowercase")]
	pub uppercase: bool,

	#[arg(short, long, default_value = "_")]
	pub delimiter: char,

	#[arg(short, long)]
	pub recursive: bool,
}
