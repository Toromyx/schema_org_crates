/// <https://schema.org/menuAddOn>
pub const MENU_ADD_ON_PROPERTY_IRI_HTTP: &str = "http://schema.org/menuAddOn";
/// <https://schema.org/menuAddOn>
pub const MENU_ADD_ON_PROPERTY_IRI_HTTPS: &str = "https://schema.org/menuAddOn";
/// <https://schema.org/menuAddOn>
pub const MENU_ADD_ON_PROPERTY_LABEL: &str = "menuAddOn";
pub struct MenuAddOnPropertyIri;
impl PartialEq<&str> for MenuAddOnPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MENU_ADD_ON_PROPERTY_IRI_HTTP || *other == MENU_ADD_ON_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MenuAddOnPropertyIri> for &str {
	fn eq(&self, other: &MenuAddOnPropertyIri) -> bool {
		*self == MENU_ADD_ON_PROPERTY_IRI_HTTP || *self == MENU_ADD_ON_PROPERTY_IRI_HTTPS
	}
}
pub struct MenuAddOnPropertyIriOrLabel;
impl PartialEq<&str> for MenuAddOnPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MenuAddOnPropertyIri || *other == MENU_ADD_ON_PROPERTY_LABEL
	}
}
impl PartialEq<MenuAddOnPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MenuAddOnPropertyIriOrLabel) -> bool {
		*self == MenuAddOnPropertyIri || *self == MENU_ADD_ON_PROPERTY_LABEL
	}
}
