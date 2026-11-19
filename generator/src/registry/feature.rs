use crate::collections::OrdMap;
use crate::registry::VkApi;

pub struct VkFeature<'a> {
	pub api: VkApi,
	pub name: &'a str,
	pub public: bool,
	pub requirements: OrdMap<&'a str, &'a str>,
	pub removals: OrdMap<&'a str, &'a str>,
	pub deprecations: OrdMap<&'a str, &'a str>,
}
