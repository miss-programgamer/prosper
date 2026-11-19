use std::io::{BufWriter, Write};

use convert_case::{Case::*, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Type;

use crate::registry::{VkHandle, VkRegistry, VkStruct, VkType};

///
pub struct VkGenerator<'a, W: Write> {
	write: BufWriter<W>,
	registry: &'a VkRegistry<'a>,
}

impl<'a, W: Write> VkGenerator<'a, W> {
	/// Constructs a new Vulkan code generator.
	pub fn new(write: W, registry: &'a VkRegistry<'a>) -> Self {
		Self { write: BufWriter::new(write), registry }
	}

	///
	pub fn generate(self) {
		//
	}

	///
	pub fn gen_type(&mut self, r#type: &VkType) {
		match r#type {
			VkType::Handle(handle) => self.gen_handle(handle),
			VkType::Struct(r#struct) => self.gen_struct(r#struct),
			_ => {}
		}
	}

	/// Generates code for the given handle.
	pub fn gen_handle(&mut self, handle: &VkHandle) {
		let name = ident(handle.base.name);

		self.write(quote! {
			#[repr(C)]
			#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
			pub struct #name(u64);

			impl #name {
				pub unsafe fn from_raw(raw: u64) -> Self {
					Self(raw)
				}

				pub fn into_raw(self) -> u64 {
					self.0
				}

				pub fn as_raw(&self) -> u64 {
					self.0
				}
			}
		});
	}

	///
	pub fn gen_struct(&mut self, r#struct: &VkStruct) {
		let name = ident(r#struct.base.name);
		let members = r#struct.members.iter().map(|(_, member)| {
			let name = ident(&member.name.to_case(Snake));
			let tn = self.typename(member.r#type);
			quote! { #name: #tn }
		});

		self.write(quote! {
			#[repr(C)]
			#[derive(Copy, Clone, Eq, PartialEq)]
			pub struct #name { #(pub #members,)* }
		});
	}

	/// Writes the pretty-printed result of a quote macro to our writer.
	fn write(&mut self, tokens: TokenStream) {
		let tree = syn::parse2(tokens).unwrap();
		let source = prettyplease::unparse(&tree);
		writeln!(self.write, "{}", source).unwrap();
	}

	///
	fn typename(&self, name: &str) -> Type {
		syn::parse_str(self.registry.resolve_typename(name)).unwrap()
	}
}

/// Produces an identifier of the given name, escaping keywords if necessary.
fn ident(name: &str) -> Ident {
	syn::parse_str(name).unwrap_or_else(|_| syn::parse_str(&format!("r#{name}")).unwrap())
}
