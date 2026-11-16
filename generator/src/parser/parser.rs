use roxmltree::{Document, Node};

use crate::collections::OrdMap;
use crate::registry::*;

/// Parses `vk.xml` into a registry.
pub struct VkParser<'a> {
	registry: VkRegistry<'a>,
	api: VkApi,
}

impl<'a> VkParser<'a> {
	/// Parses the given `vk.xml` into a registry instance.
	pub fn parse(document: &'a Document<'a>, api: VkApi) -> Result<VkRegistry<'a>> {
		let mut parser: VkParser<'a> = VkParser { registry: VkRegistry::new(), api };

		let registry_node = document.root().first_element_child().unwrap();
		for child in registry_node.children().filter(is_noncomment_el) {
			match child.tag_name().name() {
				"platforms" => parser.parse_platforms(child),
				"tags" => parser.parse_vendors(child),
				"types" => parser.parse_types(child),
				"enums" => parser.parse_enums(child),
				"feature" => parser.parse_feature(child),
				tag => eprintln!("{tag}"),
			};
		}

		Ok(parser.registry)
	}

	fn parse_platforms(&mut self, element: Node<'a, 'a>) {
		for child in element.children().filter(Node::is_element) {
			let name = child.attribute("name").unwrap();
			self.registry.platforms.insert(name, VkPlatform { name });
		}
	}

	fn parse_vendors(&mut self, element: Node<'a, 'a>) {
		for child in element.children().filter(Node::is_element) {
			let name = child.attribute("name").unwrap();
			let author = child.attribute("author").unwrap();
			let contact = child.attribute("contact").unwrap();
			self.registry.vendors.insert(name, VkVendor { ext: name, author, contact });
		}
	}

	fn parse_types(&mut self, element: Node<'a, 'a>) {
		for child in element.children().filter(is_noncomment_el) {
			let r#type = if let Some(alias) = child.attribute("alias") {
				self.parse_type_alias(child, alias)
			} else {
				match child.attribute("category").unwrap_or_default() {
					"basetype" => self.parse_type_basetype(child),
					"bitmask" => self.parse_type_bitmask(child),
					"define" => self.parse_type_define(child),
					"enum" => self.parse_type_enum(child),
					"funcpointer" => self.parse_type_funcptr(child),
					"handle" => self.parse_type_handle(child),
					"include" => self.parse_type_include(child),
					"struct" => self.parse_type_struct(child),
					"union" => self.parse_type_union(child),
					"" => self.parse_type_raw(child),
					category => panic!("unexpected category '{category}'"),
				}
			};

			if (self.api & r#type.api()) != 0 {
				self.registry.types.insert(r#type.name(), r#type);
			}
		}
	}

	fn parse_type_basetype(&mut self, element: Node<'a, 'a>) -> VkType<'a> {
		VkType::Basetype(VkBasetype { base: self.parse_type_base(element) })
	}

	fn parse_type_bitmask(&mut self, element: Node<'a, 'a>) -> VkType<'a> {
		VkType::Bitmask(VkBitmask { base: self.parse_type_base(element) })
	}

	fn parse_type_define(&mut self, element: Node<'a, 'a>) -> VkType<'a> {
		VkType::Define(VkDefine { base: self.parse_type_base(element) })
	}

	fn parse_type_enum(&mut self, element: Node<'a, 'a>) -> VkType<'a> {
		VkType::Enum(VkEnum { base: self.parse_type_base(element) })
	}

	fn parse_type_funcptr(&mut self, element: Node<'a, 'a>) -> VkType<'a> {
		VkType::FuncPtr(VkFuncPtr { base: self.parse_type_base(element) })
	}

	fn parse_type_handle(&mut self, element: Node<'a, 'a>) -> VkType<'a> {
		VkType::Handle(VkHandle {
			base: self.parse_type_base(element),
			parent: element.attribute("parent"),
			object_type: element.attribute("objtypeenum").unwrap_or_default(),
		})
	}

	fn parse_type_include(&mut self, element: Node<'a, 'a>) -> VkType<'a> {
		VkType::Include(VkInclude { base: self.parse_type_base(element) })
	}

	fn parse_type_struct(&mut self, element: Node<'a, 'a>) -> VkType<'a> {
		VkType::Struct(VkStruct {
			base: self.parse_type_base(element),
			extends: comma_separated(element.attribute("structextends")),
			is_result: element.has_attribute("returnedonly"),
			allow_dup: element.has_attribute("allowduplicate"),
			members: {
				let mut members: OrdMap<&'a str, VkMember> = OrdMap::new();
				for child in element.children().filter(is_noncomment_el) {
					let name = get_name_or_name(child).unwrap();
					members.insert(name, VkMember { name });
				}
				members
			},
		})
	}

	fn parse_type_union(&mut self, element: Node<'a, 'a>) -> VkType<'a> {
		VkType::Union(VkUnion { base: self.parse_type_base(element) })
	}

	fn parse_type_alias(&mut self, element: Node<'a, 'a>, alias: &'a str) -> VkType<'a> {
		VkType::Alias(VkAlias { base: self.parse_type_base(element), alias: alias })
	}

	fn parse_type_raw(&mut self, element: Node<'a, 'a>) -> VkType<'a> {
		VkType::Raw(VkRawType {
			base: self.parse_type_base(element),
			tn: get_type_or_type(element),
		})
	}

	fn parse_type_base(&mut self, element: Node<'a, 'a>) -> VkTypeBase<'a> {
		VkTypeBase {
			api: parse_api(element.attribute("api")),
			name: get_name_or_name(element).unwrap(),
			requires: element.attribute("requires"),
			obsolete: parse_deprecated(element.attribute("deprecated")),
		}
	}

	fn parse_enums(&mut self, _element: Node<'a, 'a>) {
		// todo!("implement parse_enums");
	}

	fn parse_feature(&mut self, element: Node<'a, 'a>) {
		let feature = VkFeature {
			api: parse_api(element.attribute("api")),
			name: get_name_or_name(element).unwrap(),
			public: element
				.attribute("apitype")
				.map(|api| match api {
					"internal" => false,
					"public" => true,
					api => panic!("unexpected api type '{api}'"),
				})
				.unwrap_or(false),
		};

		self.registry.features.insert(feature.name, feature);
	}
}

/// Get the name attribute of the given element, or the contents of a descendent name tag.
fn get_name_or_name<'a>(element: Node<'a, 'a>) -> Option<&'a str> {
	element
		.attribute("name")
		.or_else(|| element.descendants().filter(is_name_el).next().and_then(|el| el.text()))
}

/// Get the type attribute of the given element, or the contents of a descendent type tag.
fn get_type_or_type<'a>(element: Node<'a, 'a>) -> Option<&'a str> {
	element
		.attribute("type")
		.or_else(|| element.descendants().filter(is_type_el).next().and_then(|el| el.text()))
}

/// Parse an API string into an API bitmask.
fn parse_api(api: Option<&str>) -> VkApi {
	match api {
		Some(api) => api
			.split(",")
			.map(|api| match api {
				"" => VkApi::NONE,
				"vulkan" => VkApi::VULKAN,
				"vulkansc" => VkApi::VULKAN_SC,
				"vulkanbase" => VkApi::VULKAN_BASE,
				api => panic!("unknown api '{api}'"),
			})
			.fold(VkApi::NONE, |acc, el| acc | el),
		None => VkApi::ALL,
	}
}

/// Parse a deprecation attribute on a type.
fn parse_deprecated(deprecated: Option<&str>) -> Obsolete {
	deprecated.map_or(Obsolete::False, |d| match d {
		"true" => Obsolete::Deprecated,
		"aliased" => Obsolete::Aliased,
		dep => panic!("unknown deprecation '{dep}'"),
	})
}

/// Checks whether the given node is a name element.
fn is_name_el(node: &Node) -> bool {
	node.is_element() && node.has_tag_name("name")
}

/// Checks whether the given node is a type element.
fn is_type_el(node: &Node) -> bool {
	node.is_element() && node.has_tag_name("type")
}

/// In their infinite wisdom, the authors of vk.xml used `<comment>` tags rather than proper xml
/// comments. This has become everyone's problem and I hate it.
fn is_noncomment_el(node: &Node) -> bool {
	node.is_element() && !node.has_tag_name("comment")
}

/// Splits the given string using commas and produces a vector of the parts.
fn comma_separated(str: Option<&str>) -> Vec<&str> {
	match str {
		Some(str) => str.split(",").collect(),
		None => vec![],
	}
}

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
