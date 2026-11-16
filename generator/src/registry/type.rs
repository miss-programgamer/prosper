use crate::collections::OrdMap;
use crate::registry::VkApi;

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

	pub fn name(&self) -> &'a str {
		self.base().name
	}

	pub fn api(&self) -> VkApi {
		self.base().api
	}
}

#[derive(Debug, Clone, Copy)]
pub struct VkTypeBase<'a> {
	pub api: VkApi,
	pub name: &'a str,
	pub requires: Option<&'a str>,
	pub obsolete: Obsolete,
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

#[derive(Debug, Clone, Copy)]
pub struct VkBitmask<'a> {
	pub base: VkTypeBase<'a>,
}

#[derive(Debug, Clone, Copy)]
pub struct VkDefine<'a> {
	pub base: VkTypeBase<'a>,
}

#[derive(Debug, Clone, Copy)]
pub struct VkEnum<'a> {
	pub base: VkTypeBase<'a>,
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
	pub is_result: bool,
	pub allow_dup: bool,
	pub members: OrdMap<&'a str, VkMember<'a>>,
}

#[derive(Debug, Clone, Copy)]
pub struct VkMember<'a> {
	pub name: &'a str,
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

#[derive(Debug, Clone, Copy)]
pub enum Obsolete {
	False,
	Deprecated,
	Aliased,
}
