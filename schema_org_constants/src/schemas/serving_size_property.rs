/// <https://schema.org/servingSize>
pub const SERVING_SIZE_PROPERTY_IRI_HTTP: &str = "http://schema.org/servingSize";
/// <https://schema.org/servingSize>
pub const SERVING_SIZE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/servingSize";
/// <https://schema.org/servingSize>
pub const SERVING_SIZE_PROPERTY_LABEL: &str = "servingSize";
pub struct ServingSizePropertyIri;
impl PartialEq<&str> for ServingSizePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERVING_SIZE_PROPERTY_IRI_HTTP || *other == SERVING_SIZE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ServingSizePropertyIri> for &str {
	fn eq(&self, other: &ServingSizePropertyIri) -> bool {
		*self == SERVING_SIZE_PROPERTY_IRI_HTTP || *self == SERVING_SIZE_PROPERTY_IRI_HTTPS
	}
}
pub struct ServingSizePropertyIriOrLabel;
impl PartialEq<&str> for ServingSizePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ServingSizePropertyIri || *other == SERVING_SIZE_PROPERTY_LABEL
	}
}
impl PartialEq<ServingSizePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ServingSizePropertyIriOrLabel) -> bool {
		*self == ServingSizePropertyIri || *self == SERVING_SIZE_PROPERTY_LABEL
	}
}
