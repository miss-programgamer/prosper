use crate::registry::VkApi;

pub struct VkFeature<'a> {
	pub api: VkApi,
	pub name: &'a str,
	pub public: bool,
}
