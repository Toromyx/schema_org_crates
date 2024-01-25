/// <https://schema.org/location>
pub const LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/location";
/// <https://schema.org/location>
pub const LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/location";
/// <https://schema.org/location>
pub const LOCATION_PROPERTY_LABEL: &str = "location";
pub struct LocationPropertyIri;
impl PartialEq<&str> for LocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOCATION_PROPERTY_IRI_HTTP || *other == LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LocationPropertyIri> for &str {
	fn eq(&self, other: &LocationPropertyIri) -> bool {
		*self == LOCATION_PROPERTY_IRI_HTTP || *self == LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct LocationPropertyIriOrLabel;
impl PartialEq<&str> for LocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LocationPropertyIri || *other == LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<LocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LocationPropertyIriOrLabel) -> bool {
		*self == LocationPropertyIri || *self == LOCATION_PROPERTY_LABEL
	}
}
