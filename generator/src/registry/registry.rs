use crate::collections::OrdMap;
use crate::registry::VkFeature;

use super::VkPlatform;
use super::VkType;
use super::VkVendor;

/// Holds information about the Vulkan registry in a vk.xml file.
pub struct VkRegistry<'a> {
	pub platforms: OrdMap<&'a str, VkPlatform<'a>>,
	pub vendors: OrdMap<&'a str, VkVendor<'a>>,
	pub types: OrdMap<&'a str, VkType<'a>>,
	pub features: OrdMap<&'a str, VkFeature<'a>>,
}

impl<'a> VkRegistry<'a> {
	pub fn new() -> Self {
		Self {
			platforms: OrdMap::new(),
			vendors: OrdMap::new(),
			types: OrdMap::new(),
			features: OrdMap::new(),
		}
	}
}
