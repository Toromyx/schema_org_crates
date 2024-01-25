/// <https://schema.org/MenuSection>
pub const MENU_SECTION_IRI_HTTP: &str = "http://schema.org/MenuSection";
/// <https://schema.org/MenuSection>
pub const MENU_SECTION_IRI_HTTPS: &str = "https://schema.org/MenuSection";
/// <https://schema.org/MenuSection>
pub const MENU_SECTION_LABEL: &str = "MenuSection";
pub struct MenuSectionIri;
impl PartialEq<&str> for MenuSectionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MENU_SECTION_IRI_HTTP || *other == MENU_SECTION_IRI_HTTPS
	}
}
impl PartialEq<MenuSectionIri> for &str {
	fn eq(&self, other: &MenuSectionIri) -> bool {
		*self == MENU_SECTION_IRI_HTTP || *self == MENU_SECTION_IRI_HTTPS
	}
}
pub struct MenuSectionIriOrLabel;
impl PartialEq<&str> for MenuSectionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MenuSectionIri || *other == MENU_SECTION_LABEL
	}
}
impl PartialEq<MenuSectionIriOrLabel> for &str {
	fn eq(&self, other: &MenuSectionIriOrLabel) -> bool {
		*self == MenuSectionIri || *self == MENU_SECTION_LABEL
	}
}
