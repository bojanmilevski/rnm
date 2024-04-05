use crate::errors::Result;
use lazy_regex::Regex;

pub trait Renamable: Sized {
	fn rename_item(value: &str) -> Result<String> {
		let regex = Regex::new(r"[^\w\d]+")?
			.replace_all(value, "_")
			.trim_matches('_')
			.trim_end_matches('_')
			.to_lowercase();

		let transliterated = deunicode::deunicode(&regex);

		Ok(transliterated)
	}
}
