/// <https://schema.org/cookTime>
pub const COOK_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/cookTime";
/// <https://schema.org/cookTime>
pub const COOK_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cookTime";
/// <https://schema.org/cookTime>
pub const COOK_TIME_PROPERTY_LABEL: &str = "cookTime";
pub struct CookTimePropertyIri;
impl PartialEq<&str> for CookTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COOK_TIME_PROPERTY_IRI_HTTP || *other == COOK_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CookTimePropertyIri> for &str {
	fn eq(&self, other: &CookTimePropertyIri) -> bool {
		*self == COOK_TIME_PROPERTY_IRI_HTTP || *self == COOK_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct CookTimePropertyIriOrLabel;
impl PartialEq<&str> for CookTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CookTimePropertyIri || *other == COOK_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<CookTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CookTimePropertyIriOrLabel) -> bool {
		*self == CookTimePropertyIri || *self == COOK_TIME_PROPERTY_LABEL
	}
}
