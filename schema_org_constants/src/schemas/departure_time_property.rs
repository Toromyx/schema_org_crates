/// <https://schema.org/departureTime>
pub const DEPARTURE_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/departureTime";
/// <https://schema.org/departureTime>
pub const DEPARTURE_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/departureTime";
/// <https://schema.org/departureTime>
pub const DEPARTURE_TIME_PROPERTY_LABEL: &str = "departureTime";
pub struct DepartureTimePropertyIri;
impl PartialEq<&str> for DepartureTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEPARTURE_TIME_PROPERTY_IRI_HTTP || *other == DEPARTURE_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DepartureTimePropertyIri> for &str {
	fn eq(&self, other: &DepartureTimePropertyIri) -> bool {
		*self == DEPARTURE_TIME_PROPERTY_IRI_HTTP || *self == DEPARTURE_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct DepartureTimePropertyIriOrLabel;
impl PartialEq<&str> for DepartureTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DepartureTimePropertyIri || *other == DEPARTURE_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<DepartureTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DepartureTimePropertyIriOrLabel) -> bool {
		*self == DepartureTimePropertyIri || *self == DEPARTURE_TIME_PROPERTY_LABEL
	}
}
