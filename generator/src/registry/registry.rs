use std::collections::HashMap;
use std::sync::LazyLock;

use crate::collections::OrdMap;

use super::VkCommand;
use super::VkExtension;
use super::VkFeature;
use super::VkFormat;
use super::VkGlobal;
use super::VkPlatform;
use super::VkType;
use super::VkVendor;

/// Holds information about the Vulkan registry in a vk.xml file.
pub struct VkRegistry<'a> {
	pub platforms: OrdMap<&'a str, VkPlatform<'a>>,
	pub vendors: OrdMap<&'a str, VkVendor<'a>>,
	pub types: OrdMap<&'a str, VkType<'a>>,
	pub globals: OrdMap<&'a str, VkGlobal<'a>>,
	pub commands: OrdMap<&'a str, VkCommand<'a>>,
	pub features: OrdMap<&'a str, VkFeature<'a>>,
	pub extensions: OrdMap<&'a str, VkExtension<'a>>,
	pub formats: OrdMap<&'a str, VkFormat<'a>>,
}

impl<'a> VkRegistry<'a> {
	pub fn new() -> Self {
		Self {
			platforms: OrdMap::new(),
			vendors: OrdMap::new(),
			types: OrdMap::new(),
			globals: OrdMap::new(),
			commands: OrdMap::new(),
			features: OrdMap::new(),
			extensions: OrdMap::new(),
			formats: OrdMap::new(),
		}
	}

	pub fn resolve_typename(&self, name: &str) -> &'a str {
		match &self.types[name] {
			VkType::Basetype(basetype) => {
				if basetype.base.requires == Some("vk_platform") {
					VK_PLATFORM_MAPPING
						.get(basetype.base.name)
						.expect(&format!("unknown vk_platform type '{}'", basetype.base.name))
				} else {
					basetype.base.name
				}
			}
			ty => ty.name(),
		}
	}
}

const VK_PLATFORM_MAPPING: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
	HashMap::from([
		("void", "()"),
		("char", "c_char"),
		("float", "f32"),
		("double", "f64"),
		("int8_t", "u8"),
		("uint8_t", "i8"),
		("int16_t", "u16"),
		("uint16_t", "i16"),
		("int32_t", "u32"),
		("uint32_t", "i32"),
		("int64_t", "u64"),
		("uint64_t", "i64"),
		("size_t", "usize"),
		("int", "c_int"),
	])
});
