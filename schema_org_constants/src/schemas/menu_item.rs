/// <https://schema.org/MenuItem>
pub const MENU_ITEM_IRI_HTTP: &str = "http://schema.org/MenuItem";
/// <https://schema.org/MenuItem>
pub const MENU_ITEM_IRI_HTTPS: &str = "https://schema.org/MenuItem";
/// <https://schema.org/MenuItem>
pub const MENU_ITEM_LABEL: &str = "MenuItem";
pub struct MenuItemIri;
impl PartialEq<&str> for MenuItemIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MENU_ITEM_IRI_HTTP || *other == MENU_ITEM_IRI_HTTPS
	}
}
impl PartialEq<MenuItemIri> for &str {
	fn eq(&self, other: &MenuItemIri) -> bool {
		*self == MENU_ITEM_IRI_HTTP || *self == MENU_ITEM_IRI_HTTPS
	}
}
pub struct MenuItemIriOrLabel;
impl PartialEq<&str> for MenuItemIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MenuItemIri || *other == MENU_ITEM_LABEL
	}
}
impl PartialEq<MenuItemIriOrLabel> for &str {
	fn eq(&self, other: &MenuItemIriOrLabel) -> bool {
		*self == MenuItemIri || *self == MENU_ITEM_LABEL
	}
}
