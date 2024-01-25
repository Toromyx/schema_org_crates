/// <https://schema.org/cookingMethod>
pub const COOKING_METHOD_PROPERTY_IRI_HTTP: &str = "http://schema.org/cookingMethod";
/// <https://schema.org/cookingMethod>
pub const COOKING_METHOD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cookingMethod";
/// <https://schema.org/cookingMethod>
pub const COOKING_METHOD_PROPERTY_LABEL: &str = "cookingMethod";
pub struct CookingMethodPropertyIri;
impl PartialEq<&str> for CookingMethodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COOKING_METHOD_PROPERTY_IRI_HTTP || *other == COOKING_METHOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CookingMethodPropertyIri> for &str {
	fn eq(&self, other: &CookingMethodPropertyIri) -> bool {
		*self == COOKING_METHOD_PROPERTY_IRI_HTTP || *self == COOKING_METHOD_PROPERTY_IRI_HTTPS
	}
}
pub struct CookingMethodPropertyIriOrLabel;
impl PartialEq<&str> for CookingMethodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CookingMethodPropertyIri || *other == COOKING_METHOD_PROPERTY_LABEL
	}
}
impl PartialEq<CookingMethodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CookingMethodPropertyIriOrLabel) -> bool {
		*self == CookingMethodPropertyIri || *self == COOKING_METHOD_PROPERTY_LABEL
	}
}
