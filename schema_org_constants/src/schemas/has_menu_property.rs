/// <https://schema.org/hasMenu>
pub const HAS_MENU_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasMenu";
/// <https://schema.org/hasMenu>
pub const HAS_MENU_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasMenu";
/// <https://schema.org/hasMenu>
pub const HAS_MENU_PROPERTY_LABEL: &str = "hasMenu";
pub struct HasMenuPropertyIri;
impl PartialEq<&str> for HasMenuPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_MENU_PROPERTY_IRI_HTTP || *other == HAS_MENU_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasMenuPropertyIri> for &str {
	fn eq(&self, other: &HasMenuPropertyIri) -> bool {
		*self == HAS_MENU_PROPERTY_IRI_HTTP || *self == HAS_MENU_PROPERTY_IRI_HTTPS
	}
}
pub struct HasMenuPropertyIriOrLabel;
impl PartialEq<&str> for HasMenuPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasMenuPropertyIri || *other == HAS_MENU_PROPERTY_LABEL
	}
}
impl PartialEq<HasMenuPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasMenuPropertyIriOrLabel) -> bool {
		*self == HasMenuPropertyIri || *self == HAS_MENU_PROPERTY_LABEL
	}
}
