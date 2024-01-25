/// <https://schema.org/propertyID>
pub const PROPERTY_ID_PROPERTY_IRI_HTTP: &str = "http://schema.org/propertyID";
/// <https://schema.org/propertyID>
pub const PROPERTY_ID_PROPERTY_IRI_HTTPS: &str = "https://schema.org/propertyID";
/// <https://schema.org/propertyID>
pub const PROPERTY_ID_PROPERTY_LABEL: &str = "propertyID";
pub struct PropertyIdPropertyIri;
impl PartialEq<&str> for PropertyIdPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROPERTY_ID_PROPERTY_IRI_HTTP || *other == PROPERTY_ID_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PropertyIdPropertyIri> for &str {
	fn eq(&self, other: &PropertyIdPropertyIri) -> bool {
		*self == PROPERTY_ID_PROPERTY_IRI_HTTP || *self == PROPERTY_ID_PROPERTY_IRI_HTTPS
	}
}
pub struct PropertyIdPropertyIriOrLabel;
impl PartialEq<&str> for PropertyIdPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PropertyIdPropertyIri || *other == PROPERTY_ID_PROPERTY_LABEL
	}
}
impl PartialEq<PropertyIdPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PropertyIdPropertyIriOrLabel) -> bool {
		*self == PropertyIdPropertyIri || *self == PROPERTY_ID_PROPERTY_LABEL
	}
}
