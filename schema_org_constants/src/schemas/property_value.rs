/// <https://schema.org/PropertyValue>
pub const PROPERTY_VALUE_IRI_HTTP: &str = "http://schema.org/PropertyValue";
/// <https://schema.org/PropertyValue>
pub const PROPERTY_VALUE_IRI_HTTPS: &str = "https://schema.org/PropertyValue";
/// <https://schema.org/PropertyValue>
pub const PROPERTY_VALUE_LABEL: &str = "PropertyValue";
pub struct PropertyValueIri;
impl PartialEq<&str> for PropertyValueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROPERTY_VALUE_IRI_HTTP || *other == PROPERTY_VALUE_IRI_HTTPS
	}
}
impl PartialEq<PropertyValueIri> for &str {
	fn eq(&self, other: &PropertyValueIri) -> bool {
		*self == PROPERTY_VALUE_IRI_HTTP || *self == PROPERTY_VALUE_IRI_HTTPS
	}
}
pub struct PropertyValueIriOrLabel;
impl PartialEq<&str> for PropertyValueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PropertyValueIri || *other == PROPERTY_VALUE_LABEL
	}
}
impl PartialEq<PropertyValueIriOrLabel> for &str {
	fn eq(&self, other: &PropertyValueIriOrLabel) -> bool {
		*self == PropertyValueIri || *self == PROPERTY_VALUE_LABEL
	}
}
