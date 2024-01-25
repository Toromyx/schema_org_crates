/// <https://schema.org/toLocation>
pub const TO_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/toLocation";
/// <https://schema.org/toLocation>
pub const TO_LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/toLocation";
/// <https://schema.org/toLocation>
pub const TO_LOCATION_PROPERTY_LABEL: &str = "toLocation";
pub struct ToLocationPropertyIri;
impl PartialEq<&str> for ToLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TO_LOCATION_PROPERTY_IRI_HTTP || *other == TO_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ToLocationPropertyIri> for &str {
	fn eq(&self, other: &ToLocationPropertyIri) -> bool {
		*self == TO_LOCATION_PROPERTY_IRI_HTTP || *self == TO_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct ToLocationPropertyIriOrLabel;
impl PartialEq<&str> for ToLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ToLocationPropertyIri || *other == TO_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<ToLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ToLocationPropertyIriOrLabel) -> bool {
		*self == ToLocationPropertyIri || *self == TO_LOCATION_PROPERTY_LABEL
	}
}
