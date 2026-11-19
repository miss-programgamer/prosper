use crate::registry::VkOwner;

pub enum VkCommand<'a> {
	Concrete(VkCommandConcrete<'a>),
	Alias(VkCommandAlias<'a>),
}

impl<'a> VkCommand<'a> {
	/// Sets the owner feature/extension of this command.
	pub fn set_owner(&mut self, owner: Option<VkOwner<'a>>) {
		match self {
			Self::Alias(cmd) => cmd.owner = owner,
			Self::Concrete(cmd) => cmd.owner = owner,
		}
	}
}

pub struct VkCommandConcrete<'a> {
	pub name: &'a str,
	pub owner: Option<VkOwner<'a>>,
}

impl<'a> From<VkCommandConcrete<'a>> for VkCommand<'a> {
	fn from(value: VkCommandConcrete<'a>) -> Self {
		Self::Concrete(value)
	}
}

pub struct VkCommandAlias<'a> {
	pub name: &'a str,
	pub alias: &'a str,
	pub owner: Option<VkOwner<'a>>,
}

impl<'a> From<VkCommandAlias<'a>> for VkCommand<'a> {
	fn from(value: VkCommandAlias<'a>) -> Self {
		Self::Alias(value)
	}
}
