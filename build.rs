use std::env;
use std::fs::{self, File};
use std::path::Path;

use prosper_gen::generator::VkGenerator;
use prosper_gen::parser::VkParser;
use prosper_gen::{Document, registry::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("cargo::rerun-if-changed={}", "build.rs");
	println!("cargo::rerun-if-changed={}", "vk.xml");

	let source = fs::read_to_string("vk.xml")?;
	let document = Document::parse(&source)?;
	let registry = VkParser::parse(&document, VkApi::VULKAN)?;

	let out = env::var_os("OUT_DIR").unwrap();
	let dest = Path::new(&out).join("vulkan.rs");
	let generator = VkGenerator::new(File::create(dest)?, &registry);
	generator.generate();

	Ok(())
}
