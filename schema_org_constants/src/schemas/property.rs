/// <https://schema.org/Property>
pub const PROPERTY_IRI_HTTP: &str = "http://schema.org/Property";
/// <https://schema.org/Property>
pub const PROPERTY_IRI_HTTPS: &str = "https://schema.org/Property";
/// <https://schema.org/Property>
pub const PROPERTY_LABEL: &str = "Property";
pub struct PropertyIri;
impl PartialEq<&str> for PropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROPERTY_IRI_HTTP || *other == PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PropertyIri> for &str {
	fn eq(&self, other: &PropertyIri) -> bool {
		*self == PROPERTY_IRI_HTTP || *self == PROPERTY_IRI_HTTPS
	}
}
pub struct PropertyIriOrLabel;
impl PartialEq<&str> for PropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PropertyIri || *other == PROPERTY_LABEL
	}
}
impl PartialEq<PropertyIriOrLabel> for &str {
	fn eq(&self, other: &PropertyIriOrLabel) -> bool {
		*self == PropertyIri || *self == PROPERTY_LABEL
	}
}
