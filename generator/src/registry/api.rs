use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

/// Bitmask of possible API values.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VkApi(u8);

impl VkApi {
	pub const NONE: VkApi = VkApi(0);
	pub const VULKAN: VkApi = VkApi(1);
	pub const VULKAN_SC: VkApi = VkApi(2);
	pub const VULKAN_BASE: VkApi = VkApi(4);
	pub const ALL: VkApi = VkApi(7);
}

impl PartialEq<u8> for VkApi {
	fn eq(&self, other: &u8) -> bool {
		self.0 == *other
	}
}

impl BitAnd for VkApi {
	type Output = VkApi;

	fn bitand(self, rhs: Self) -> Self::Output {
		Self(self.0 & rhs.0)
	}
}

impl BitAndAssign for VkApi {
	fn bitand_assign(&mut self, rhs: Self) {
		*self = *self & rhs;
	}
}

impl BitOr for VkApi {
	type Output = VkApi;

	fn bitor(self, rhs: Self) -> Self::Output {
		Self(self.0 | rhs.0)
	}
}

impl BitOrAssign for VkApi {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}
