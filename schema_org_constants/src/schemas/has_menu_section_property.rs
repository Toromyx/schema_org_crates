/// <https://schema.org/hasMenuSection>
pub const HAS_MENU_SECTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasMenuSection";
/// <https://schema.org/hasMenuSection>
pub const HAS_MENU_SECTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasMenuSection";
/// <https://schema.org/hasMenuSection>
pub const HAS_MENU_SECTION_PROPERTY_LABEL: &str = "hasMenuSection";
pub struct HasMenuSectionPropertyIri;
impl PartialEq<&str> for HasMenuSectionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_MENU_SECTION_PROPERTY_IRI_HTTP
			|| *other == HAS_MENU_SECTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasMenuSectionPropertyIri> for &str {
	fn eq(&self, other: &HasMenuSectionPropertyIri) -> bool {
		*self == HAS_MENU_SECTION_PROPERTY_IRI_HTTP || *self == HAS_MENU_SECTION_PROPERTY_IRI_HTTPS
	}
}
pub struct HasMenuSectionPropertyIriOrLabel;
impl PartialEq<&str> for HasMenuSectionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasMenuSectionPropertyIri || *other == HAS_MENU_SECTION_PROPERTY_LABEL
	}
}
impl PartialEq<HasMenuSectionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasMenuSectionPropertyIriOrLabel) -> bool {
		*self == HasMenuSectionPropertyIri || *self == HAS_MENU_SECTION_PROPERTY_LABEL
	}
}
