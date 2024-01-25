/// <https://schema.org/aircraft>
pub const AIRCRAFT_PROPERTY_IRI_HTTP: &str = "http://schema.org/aircraft";
/// <https://schema.org/aircraft>
pub const AIRCRAFT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/aircraft";
/// <https://schema.org/aircraft>
pub const AIRCRAFT_PROPERTY_LABEL: &str = "aircraft";
pub struct AircraftPropertyIri;
impl PartialEq<&str> for AircraftPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AIRCRAFT_PROPERTY_IRI_HTTP || *other == AIRCRAFT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AircraftPropertyIri> for &str {
	fn eq(&self, other: &AircraftPropertyIri) -> bool {
		*self == AIRCRAFT_PROPERTY_IRI_HTTP || *self == AIRCRAFT_PROPERTY_IRI_HTTPS
	}
}
pub struct AircraftPropertyIriOrLabel;
impl PartialEq<&str> for AircraftPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AircraftPropertyIri || *other == AIRCRAFT_PROPERTY_LABEL
	}
}
impl PartialEq<AircraftPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AircraftPropertyIriOrLabel) -> bool {
		*self == AircraftPropertyIri || *self == AIRCRAFT_PROPERTY_LABEL
	}
}
