/// <https://schema.org/locationCreated>
pub const LOCATION_CREATED_PROPERTY_IRI_HTTP: &str = "http://schema.org/locationCreated";
/// <https://schema.org/locationCreated>
pub const LOCATION_CREATED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/locationCreated";
/// <https://schema.org/locationCreated>
pub const LOCATION_CREATED_PROPERTY_LABEL: &str = "locationCreated";
pub struct LocationCreatedPropertyIri;
impl PartialEq<&str> for LocationCreatedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOCATION_CREATED_PROPERTY_IRI_HTTP
			|| *other == LOCATION_CREATED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LocationCreatedPropertyIri> for &str {
	fn eq(&self, other: &LocationCreatedPropertyIri) -> bool {
		*self == LOCATION_CREATED_PROPERTY_IRI_HTTP || *self == LOCATION_CREATED_PROPERTY_IRI_HTTPS
	}
}
pub struct LocationCreatedPropertyIriOrLabel;
impl PartialEq<&str> for LocationCreatedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LocationCreatedPropertyIri || *other == LOCATION_CREATED_PROPERTY_LABEL
	}
}
impl PartialEq<LocationCreatedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LocationCreatedPropertyIriOrLabel) -> bool {
		*self == LocationCreatedPropertyIri || *self == LOCATION_CREATED_PROPERTY_LABEL
	}
}
