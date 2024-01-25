/// <https://schema.org/ShoppingCenter>
pub const SHOPPING_CENTER_IRI_HTTP: &str = "http://schema.org/ShoppingCenter";
/// <https://schema.org/ShoppingCenter>
pub const SHOPPING_CENTER_IRI_HTTPS: &str = "https://schema.org/ShoppingCenter";
/// <https://schema.org/ShoppingCenter>
pub const SHOPPING_CENTER_LABEL: &str = "ShoppingCenter";
pub struct ShoppingCenterIri;
impl PartialEq<&str> for ShoppingCenterIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SHOPPING_CENTER_IRI_HTTP || *other == SHOPPING_CENTER_IRI_HTTPS
	}
}
impl PartialEq<ShoppingCenterIri> for &str {
	fn eq(&self, other: &ShoppingCenterIri) -> bool {
		*self == SHOPPING_CENTER_IRI_HTTP || *self == SHOPPING_CENTER_IRI_HTTPS
	}
}
pub struct ShoppingCenterIriOrLabel;
impl PartialEq<&str> for ShoppingCenterIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ShoppingCenterIri || *other == SHOPPING_CENTER_LABEL
	}
}
impl PartialEq<ShoppingCenterIriOrLabel> for &str {
	fn eq(&self, other: &ShoppingCenterIriOrLabel) -> bool {
		*self == ShoppingCenterIri || *self == SHOPPING_CENTER_LABEL
	}
}
