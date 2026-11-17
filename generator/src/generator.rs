use std::io::{BufWriter, Write};

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::registry::VkHandle;

///
pub struct VkGenerator<W: Write> {
	write: BufWriter<W>,
}

impl<W: Write> VkGenerator<W> {
	/// Constructs a new Vulkan code generator.
	pub fn new(write: W) -> Self {
		Self { write: BufWriter::new(write) }
	}

	/// Generates code for the given handle.
	pub fn gen_handle(&mut self, handle: VkHandle) {
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

	/// Writes the pretty-printed result of a quote macro to our writer.
	fn write(&mut self, tokens: TokenStream) {
		let tree = syn::parse2(tokens).unwrap();
		let source = prettyplease::unparse(&tree);
		writeln!(self.write, "{}", source).unwrap();
	}
}

/// Produces an identifier of the given name.
fn ident(name: &str) -> Ident {
	Ident::new(name, Span::call_site())
}
