use crate::collections::OrdMap;
use crate::registry::{VkApi, VkObsolete, VkOwner};

/// Type descriptor from a Vulkan registry.
#[derive(Debug, Clone)]
pub enum VkType<'a> {
	Alias(VkAlias<'a>),
	Basetype(VkBasetype<'a>),
	Bitmask(VkBitmask<'a>),
	Define(VkDefine<'a>),
	Enum(VkEnum<'a>),
	FuncPtr(VkFuncPtr<'a>),
	Handle(VkHandle<'a>),
	Include(VkInclude<'a>),
	Struct(VkStruct<'a>),
	Union(VkUnion<'a>),
	Raw(VkRawType<'a>),
}

impl<'a> VkType<'a> {
	/// Sets the owner feature/extension of this type.
	pub fn set_owner(&mut self, owner: Option<VkOwner<'a>>) {
		self.base_mut().owner = owner;
	}

	/// Obtains the data common to all types.
	pub fn base(&self) -> &VkTypeBase<'a> {
		match self {
			Self::Alias(t) => &t.base,
			Self::Basetype(t) => &t.base,
			Self::Bitmask(t) => &t.base,
			Self::Define(t) => &t.base,
			Self::Enum(t) => &t.base,
			Self::FuncPtr(t) => &t.base,
			Self::Handle(t) => &t.base,
			Self::Include(t) => &t.base,
			Self::Struct(t) => &t.base,
			Self::Union(t) => &t.base,
			Self::Raw(t) => &t.base,
		}
	}

	/// Obtains the data common to all types as mutable.
	pub fn base_mut(&mut self) -> &mut VkTypeBase<'a> {
		match self {
			Self::Alias(t) => &mut t.base,
			Self::Basetype(t) => &mut t.base,
			Self::Bitmask(t) => &mut t.base,
			Self::Define(t) => &mut t.base,
			Self::Enum(t) => &mut t.base,
			Self::FuncPtr(t) => &mut t.base,
			Self::Handle(t) => &mut t.base,
			Self::Include(t) => &mut t.base,
			Self::Struct(t) => &mut t.base,
			Self::Union(t) => &mut t.base,
			Self::Raw(t) => &mut t.base,
		}
	}

	/// Obtains this type's name.
	pub fn name(&self) -> &'a str {
		self.base().name
	}

	/// Obtains this type's API.
	pub fn api(&self) -> VkApi {
		self.base().api
	}
}

#[derive(Debug, Clone, Copy)]
pub struct VkTypeBase<'a> {
	pub api: VkApi,
	pub name: &'a str,
	pub requires: Option<&'a str>,
	pub obsolete: Option<VkObsolete>,
	pub owner: Option<VkOwner<'a>>,
}

#[derive(Debug, Clone, Copy)]
pub struct VkAlias<'a> {
	pub base: VkTypeBase<'a>,
	pub alias: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct VkBasetype<'a> {
	pub base: VkTypeBase<'a>,
}

#[derive(Debug, Clone)]
pub struct VkBitmask<'a> {
	pub base: VkTypeBase<'a>,
	pub members: OrdMap<&'a str, VkBitmaskMember<'a>>,
}

#[derive(Debug, Clone, Copy)]
pub struct VkBitmaskMember<'a> {
	pub name: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct VkDefine<'a> {
	pub base: VkTypeBase<'a>,
}

#[derive(Debug, Clone)]
pub struct VkEnum<'a> {
	pub base: VkTypeBase<'a>,
	pub members: OrdMap<&'a str, VkEnumMember<'a>>,
}

#[derive(Debug, Clone, Copy)]
pub struct VkEnumMember<'a> {
	pub name: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct VkFuncPtr<'a> {
	pub base: VkTypeBase<'a>,
}

#[derive(Debug, Clone, Copy)]
pub struct VkHandle<'a> {
	pub base: VkTypeBase<'a>,
	pub parent: Option<&'a str>,
	pub object_type: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct VkInclude<'a> {
	pub base: VkTypeBase<'a>,
}

#[derive(Debug, Clone)]
pub struct VkStruct<'a> {
	pub base: VkTypeBase<'a>,
	pub extends: Vec<&'a str>,
	pub members: OrdMap<&'a str, VkStructMember<'a>>,
	pub is_result: bool,
	pub allow_dup: bool,
}

#[derive(Debug, Clone)]
pub struct VkStructMember<'a> {
	pub name: &'a str,
	pub r#type: &'a str,
	pub obsolete: Option<VkObsolete>,
	pub featurelink: Option<&'a str>,
	pub objecttype: Option<&'a str>,
	pub optional: Vec<bool>,
	pub values: Option<Vec<&'a str>>,
	pub extern_sync: bool,
}

impl<'a> VkStructMember<'a> {
	pub fn is_optional(&self) -> bool {
		self.optional[0]
	}
}

#[derive(Debug, Clone, Copy)]
pub struct VkUnion<'a> {
	pub base: VkTypeBase<'a>,
}

#[derive(Debug, Clone, Copy)]
pub struct VkRawType<'a> {
	pub base: VkTypeBase<'a>,
	pub tn: Option<&'a str>,
}
