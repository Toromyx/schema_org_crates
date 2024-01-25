/// <https://schema.org/requiredQuantity>
pub const REQUIRED_QUANTITY_PROPERTY_IRI_HTTP: &str = "http://schema.org/requiredQuantity";
/// <https://schema.org/requiredQuantity>
pub const REQUIRED_QUANTITY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/requiredQuantity";
/// <https://schema.org/requiredQuantity>
pub const REQUIRED_QUANTITY_PROPERTY_LABEL: &str = "requiredQuantity";
pub struct RequiredQuantityPropertyIri;
impl PartialEq<&str> for RequiredQuantityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REQUIRED_QUANTITY_PROPERTY_IRI_HTTP
			|| *other == REQUIRED_QUANTITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RequiredQuantityPropertyIri> for &str {
	fn eq(&self, other: &RequiredQuantityPropertyIri) -> bool {
		*self == REQUIRED_QUANTITY_PROPERTY_IRI_HTTP
			|| *self == REQUIRED_QUANTITY_PROPERTY_IRI_HTTPS
	}
}
pub struct RequiredQuantityPropertyIriOrLabel;
impl PartialEq<&str> for RequiredQuantityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RequiredQuantityPropertyIri || *other == REQUIRED_QUANTITY_PROPERTY_LABEL
	}
}
impl PartialEq<RequiredQuantityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RequiredQuantityPropertyIriOrLabel) -> bool {
		*self == RequiredQuantityPropertyIri || *self == REQUIRED_QUANTITY_PROPERTY_LABEL
	}
}
