/// <https://schema.org/occupationLocation>
pub const OCCUPATION_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/occupationLocation";
/// <https://schema.org/occupationLocation>
pub const OCCUPATION_LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/occupationLocation";
/// <https://schema.org/occupationLocation>
pub const OCCUPATION_LOCATION_PROPERTY_LABEL: &str = "occupationLocation";
pub struct OccupationLocationPropertyIri;
impl PartialEq<&str> for OccupationLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OCCUPATION_LOCATION_PROPERTY_IRI_HTTP
			|| *other == OCCUPATION_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OccupationLocationPropertyIri> for &str {
	fn eq(&self, other: &OccupationLocationPropertyIri) -> bool {
		*self == OCCUPATION_LOCATION_PROPERTY_IRI_HTTP
			|| *self == OCCUPATION_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct OccupationLocationPropertyIriOrLabel;
impl PartialEq<&str> for OccupationLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OccupationLocationPropertyIri || *other == OCCUPATION_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<OccupationLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OccupationLocationPropertyIriOrLabel) -> bool {
		*self == OccupationLocationPropertyIri || *self == OCCUPATION_LOCATION_PROPERTY_LABEL
	}
}
