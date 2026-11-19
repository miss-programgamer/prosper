pub mod api;
pub mod command;
pub mod extension;
pub mod feature;
pub mod format;
pub mod global;
pub mod platform;
pub mod registry;
pub mod r#type;
pub mod vendor;

#[doc(inline)]
pub use api::*;

#[doc(inline)]
pub use command::*;

#[doc(inline)]
pub use extension::*;

#[doc(inline)]
pub use feature::*;

#[doc(inline)]
pub use format::*;

#[doc(inline)]
pub use global::*;

#[doc(inline)]
pub use platform::*;

#[doc(inline)]
pub use registry::*;

#[doc(inline)]
pub use r#type::*;

#[doc(inline)]
pub use vendor::*;

#[derive(Debug, Clone, Copy)]
pub enum VkObsolete {
	Deprecated,
	Aliased,
	Unused,
}

#[derive(Debug, Clone, Copy)]
pub enum VkOwner<'a> {
	Feature(&'a str),
	Extension(&'a str),
}
