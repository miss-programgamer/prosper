use std::fs;

use argh::{self, FromArgs};
use regex::RegexBuilder;

use prosper_gen::Document;
use prosper_gen::parser::VkParser;
use prosper_gen::registry::*;

/// Dump information about vk.xml
#[derive(FromArgs)]
struct CliArgs {
	/// where vk.xml is located
	#[argh(positional, default = "String::from(\"vk.xml\")")]
	pub filename: String,

	/// query string for search
	#[argh(option, short = 's')]
	pub search: Option<String>,

	/// whether searches are case-sensitive
	#[argh(switch, short = 'x')]
	pub case_sensitive: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let CliArgs { filename, search, case_sensitive, .. } = argh::from_env();
	let mut search = RegexBuilder::new(&search.unwrap_or_default());
	let search = search.case_insensitive(!case_sensitive).build()?;

	let source = fs::read_to_string(filename)?;
	let document = Document::parse(&source)?;
	let registry = VkParser::parse(&document, VkApi::VULKAN)?;
	let VkRegistry { types, globals, commands, formats, .. } = &registry;

	let mut match_count: usize = 0;

	for (n, m) in types.into_iter().filter_map(|(n, _)| search.find(n).map(|m| (n, m))) {
		let (pre, it, post) = (&n[0..m.start()], m.as_str(), &n[m.end()..]);
		println!("\x1b[92m{}\x1b[7m{}\x1b[92;27m{}\x1b[0m", pre, it, post);
		match_count += 1;
	}

	for (n, m) in globals.into_iter().filter_map(|(n, _)| search.find(n).map(|m| (n, m))) {
		let (pre, it, post) = (&n[0..m.start()], m.as_str(), &n[m.end()..]);
		println!("\x1b[96m{}\x1b[7m{}\x1b[96;27m{}\x1b[0m", pre, it, post);
		match_count += 1;
	}

	for (n, m) in commands.into_iter().filter_map(|(n, _)| search.find(n).map(|m| (n, m))) {
		let (pre, it, post) = (&n[0..m.start()], m.as_str(), &n[m.end()..]);
		println!("\x1b[93m{}\x1b[7m{}\x1b[93;27m{}\x1b[0m", pre, it, post);
		match_count += 1;
	}

	for (n, m) in formats.into_iter().filter_map(|(n, _)| search.find(n).map(|m| (n, m))) {
		let (pre, it, post) = (&n[0..m.start()], m.as_str(), &n[m.end()..]);
		println!("\x1b[95m{}\x1b[7m{}\x1b[95;27m{}\x1b[0m", pre, it, post);
		match_count += 1;
	}

	if match_count == 0 {
		eprintln!("\x1b[2m{}\x1b[0m", "no matches found")
	}

	Ok(())
}
