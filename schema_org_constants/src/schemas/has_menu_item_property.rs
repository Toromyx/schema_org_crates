/// <https://schema.org/hasMenuItem>
pub const HAS_MENU_ITEM_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasMenuItem";
/// <https://schema.org/hasMenuItem>
pub const HAS_MENU_ITEM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasMenuItem";
/// <https://schema.org/hasMenuItem>
pub const HAS_MENU_ITEM_PROPERTY_LABEL: &str = "hasMenuItem";
pub struct HasMenuItemPropertyIri;
impl PartialEq<&str> for HasMenuItemPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_MENU_ITEM_PROPERTY_IRI_HTTP || *other == HAS_MENU_ITEM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasMenuItemPropertyIri> for &str {
	fn eq(&self, other: &HasMenuItemPropertyIri) -> bool {
		*self == HAS_MENU_ITEM_PROPERTY_IRI_HTTP || *self == HAS_MENU_ITEM_PROPERTY_IRI_HTTPS
	}
}
pub struct HasMenuItemPropertyIriOrLabel;
impl PartialEq<&str> for HasMenuItemPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasMenuItemPropertyIri || *other == HAS_MENU_ITEM_PROPERTY_LABEL
	}
}
impl PartialEq<HasMenuItemPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasMenuItemPropertyIriOrLabel) -> bool {
		*self == HasMenuItemPropertyIri || *self == HAS_MENU_ITEM_PROPERTY_LABEL
	}
}
