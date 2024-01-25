/// <https://schema.org/menu>
#[deprecated = "This schema is superseded by <https://schema.org/hasMenu>."]
pub const MENU_PROPERTY_IRI_HTTP: &str = "http://schema.org/menu";
/// <https://schema.org/menu>
#[deprecated = "This schema is superseded by <https://schema.org/hasMenu>."]
pub const MENU_PROPERTY_IRI_HTTPS: &str = "https://schema.org/menu";
/// <https://schema.org/menu>
#[deprecated = "This schema is superseded by <https://schema.org/hasMenu>."]
pub const MENU_PROPERTY_LABEL: &str = "menu";
pub struct MenuPropertyIri;
impl PartialEq<&str> for MenuPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MENU_PROPERTY_IRI_HTTP || *other == MENU_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MenuPropertyIri> for &str {
	fn eq(&self, other: &MenuPropertyIri) -> bool {
		*self == MENU_PROPERTY_IRI_HTTP || *self == MENU_PROPERTY_IRI_HTTPS
	}
}
pub struct MenuPropertyIriOrLabel;
impl PartialEq<&str> for MenuPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MenuPropertyIri || *other == MENU_PROPERTY_LABEL
	}
}
impl PartialEq<MenuPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MenuPropertyIriOrLabel) -> bool {
		*self == MenuPropertyIri || *self == MENU_PROPERTY_LABEL
	}
}
