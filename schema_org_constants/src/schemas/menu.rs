/// <https://schema.org/Menu>
pub const MENU_IRI_HTTP: &str = "http://schema.org/Menu";
/// <https://schema.org/Menu>
pub const MENU_IRI_HTTPS: &str = "https://schema.org/Menu";
/// <https://schema.org/Menu>
pub const MENU_LABEL: &str = "Menu";
pub struct MenuIri;
impl PartialEq<&str> for MenuIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MENU_IRI_HTTP || *other == MENU_IRI_HTTPS
	}
}
impl PartialEq<MenuIri> for &str {
	fn eq(&self, other: &MenuIri) -> bool {
		*self == MENU_IRI_HTTP || *self == MENU_IRI_HTTPS
	}
}
pub struct MenuIriOrLabel;
impl PartialEq<&str> for MenuIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MenuIri || *other == MENU_LABEL
	}
}
impl PartialEq<MenuIriOrLabel> for &str {
	fn eq(&self, other: &MenuIriOrLabel) -> bool {
		*self == MenuIri || *self == MENU_LABEL
	}
}
