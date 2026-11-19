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
		let mut parser = VkParser { registry: VkRegistry::new(), api };

		let registry_el = document.root().first_element_child().unwrap();
		for child in registry_el.children().filter(is_noncomment_el) {
			match child.tag_name().name() {
				"platforms" => parser.parse_platforms(child),
				"tags" => parser.parse_vendors(child),
				"types" => parser.parse_types(child),
				"enums" => parser.parse_enums(child),
				"commands" => parser.parse_commands(child),
				"feature" => parser.parse_feature(child),
				"extensions" => parser.parse_extensions(child),
				"formats" => parser.parse_formats(child),
				"dynamicstates" => {}
				"spirvextensions" => {}
				"spirvcapabilities" => {}
				"sync" => {}
				"videocodecs" => {}
				tag => panic!("unknown registry element '{tag}'"),
			}
		}

		Ok(parser.registry)
	}

	fn parse_platforms(&mut self, element: Node<'a, 'a>) {
		for child in element.children().filter(Node::is_element) {
			let platform = VkPlatform { name: child.attribute("name").unwrap() };
			self.registry.platforms.insert(platform.name, platform);
		}
	}

	fn parse_vendors(&mut self, element: Node<'a, 'a>) {
		for child in element.children().filter(Node::is_element) {
			let vendor = VkVendor {
				ext: child.attribute("name").unwrap(),
				author: child.attribute("author").unwrap(),
				contact: child.attribute("contact").unwrap(),
			};
			self.registry.vendors.insert(vendor.ext, vendor);
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
					category => panic!("unknown category '{category}'"),
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
		VkType::Bitmask(VkBitmask { base: self.parse_type_base(element), members: OrdMap::new() })
	}

	fn parse_type_define(&mut self, element: Node<'a, 'a>) -> VkType<'a> {
		VkType::Define(VkDefine { base: self.parse_type_base(element) })
	}

	fn parse_type_enum(&mut self, element: Node<'a, 'a>) -> VkType<'a> {
		VkType::Enum(VkEnum { base: self.parse_type_base(element), members: OrdMap::new() })
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
			members: {
				let mut members = OrdMap::new();
				for child in element.children().filter(is_noncomment_el) {
					let member = VkStructMember {
						name: get_name_or_name(child).unwrap(),
						r#type: get_type_or_type(child).unwrap(),
						obsolete: parse_deprecated(child.attribute("deprecated")),
						featurelink: child.attribute("featurelink"),
						objecttype: child.attribute("objecttype"),
						optional: parse_optional(child.attribute("optional")),
						values: child.attribute("values").map(|v| v.split(",").collect()),
						extern_sync: parse_member_externsync(child.attribute("externsync")),
					};
					members.insert(member.name, member);
				}
				members
			},
			is_result: element.has_attribute("returnedonly"),
			allow_dup: element.has_attribute("allowduplicate"),
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
			owner: None,
		}
	}

	fn parse_enums(&mut self, element: Node<'a, 'a>) {
		let name = element.attribute("name").unwrap();
		match element.attribute("type").unwrap() {
			"enum" => {
				let r#enum = match &mut self.registry.types[name] {
					VkType::Enum(r#enum) => r#enum,
					_ => panic!("enum {name} not found"),
				};

				for child in element.children().filter(is_enum_el) {
					let member = VkEnumMember { name: get_name_or_name(child).unwrap() };
					r#enum.members.insert(member.name, member);
				}
			}
			"bitmask" => {
				match &mut self.registry.types[name] {
					// Bitmask values are allowed to populate enums because it'll be an enum of
					// that bitmask's possible bit values. Don't think about it too hard.
					VkType::Enum(r#enum) => {
						for child in element.children().filter(is_enum_el) {
							let member = VkEnumMember { name: get_name_or_name(child).unwrap() };
							r#enum.members.insert(member.name, member);
						}
					}
					VkType::Bitmask(bitmask) => {
						for child in element.children().filter(is_enum_el) {
							let member = VkBitmaskMember { name: get_name_or_name(child).unwrap() };
							bitmask.members.insert(member.name, member);
						}
					}
					_ => panic!("bitmask {name} not found"),
				};
			}
			"constants" => {
				for child in element.children().filter(is_enum_el) {
					let member = VkGlobal { name: get_name_or_name(child).unwrap() };
					self.registry.globals.insert(member.name, member);
				}
			}
			r#type => panic!("unknown enum type '{type}'"),
		}
	}

	fn parse_commands(&mut self, element: Node<'a, 'a>) {
		for child in element.children().filter(is_noncomment_el) {
			if let Some(alias) = element.attribute("alias") {
				let name = get_name_or_name(child).unwrap();
				let command = VkCommandAlias { name, alias, owner: None };
				self.registry.commands.insert(command.name, command.into());
			} else {
				let name = get_name_or_name(child).unwrap();
				let command = VkCommandConcrete { name, owner: None };
				self.registry.commands.insert(command.name, command.into());
			}
		}
	}

	fn parse_feature(&mut self, element: Node<'a, 'a>) {
		let mut requirements: OrdMap<&str, &str> = OrdMap::new();
		let mut removals: OrdMap<&str, &str> = OrdMap::new();
		let mut deprecations: OrdMap<&str, &str> = OrdMap::new();

		let feature_name = get_name_or_name(element).unwrap();
		let owner = Some(VkOwner::Feature(feature_name));

		for child in element.children().filter(is_noncomment_el) {
			match child.tag_name().name() {
				"require" => {
					for child in child.children().filter(is_noncomment_el) {
						let name = get_name_or_name(child).unwrap();
						requirements.insert(name, name);
						match child.tag_name().name() {
							"command" => self.registry.commands[name].set_owner(owner),
							"enum" => {}
							"type" => self.registry.types[name].set_owner(owner),
							"feature" => {}
							tag => panic!("unknown requirement '{tag}'"),
						}
					}
				}
				"remove" => {
					for child in child.children().filter(is_noncomment_el) {
						let name = get_name_or_name(child).unwrap();
						removals.insert(name, name);
					}
				}
				"deprecate" => {
					for child in child.children().filter(is_noncomment_el) {
						let name = get_name_or_name(child).unwrap();
						deprecations.insert(name, name);
					}
				}
				_ => panic!(),
			}
		}

		let feature = VkFeature {
			api: parse_api(element.attribute("api")),
			name: feature_name,
			public: element
				.attribute("apitype")
				.map(|api| match api {
					"internal" => false,
					"public" => true,
					api => panic!("unknown api type '{api}'"),
				})
				.unwrap_or(false),
			requirements,
			removals,
			deprecations,
		};

		self.registry.features.insert(feature.name, feature);
	}

	fn parse_extensions(&mut self, element: Node<'a, 'a>) {
		for child in element.children().filter(is_noncomment_el) {
			self.parse_extension(child);
		}
	}

	fn parse_extension(&mut self, element: Node<'a, 'a>) {
		let ext_name = get_name_or_name(element).unwrap();
		let extension = VkExtension { name: ext_name };
		self.registry.extensions.insert(extension.name, extension);
	}

	fn parse_formats(&mut self, element: Node<'a, 'a>) {
		for child in element.children().filter(is_noncomment_el) {
			let format = VkFormat { name: get_name_or_name(child).unwrap() };
			self.registry.formats.insert(format.name, format);
		}
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
fn parse_deprecated(deprecated: Option<&str>) -> Option<VkObsolete> {
	deprecated.map_or(None, |d| match d {
		"true" => Some(VkObsolete::Deprecated),
		"aliased" => Some(VkObsolete::Aliased),
		"unused" => Some(VkObsolete::Unused),
		dep => panic!("unknown deprecation '{dep}'"),
	})
}

///
fn parse_member_externsync(externsync: Option<&str>) -> bool {
	externsync.map_or(false, |at| match at {
		"true" => true,
		// This isn't supposed to happen, but it does so we handle it. Thanks vk.xml.
		"maybe" => {
			eprintln!("encountered 'maybe' while parsing externsync attribute");
			false
		}
		at => panic!("unknown externsync value '{at}'"),
	})
}

/// Parse an optional attribute on a member.
///
/// Because this can be a comma-separated list of values for each indirection of a pointer, this
/// returns a vector of booleans. I hate it too.
fn parse_optional(optional: Option<&str>) -> Vec<bool> {
	optional.map_or_default(|opt| {
		opt.split(",")
			.map(|opt| match opt {
				"true" => true,
				"false" => false,
				opt => panic!("unknown fragment of optional attribute '{opt}'"),
			})
			.collect()
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

/// Checks whether the given node is an enum element.
fn is_enum_el(node: &Node) -> bool {
	node.is_element() && node.has_tag_name("enum")
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
