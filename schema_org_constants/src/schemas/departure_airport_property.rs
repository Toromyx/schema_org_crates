/// <https://schema.org/departureAirport>
pub const DEPARTURE_AIRPORT_PROPERTY_IRI_HTTP: &str = "http://schema.org/departureAirport";
/// <https://schema.org/departureAirport>
pub const DEPARTURE_AIRPORT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/departureAirport";
/// <https://schema.org/departureAirport>
pub const DEPARTURE_AIRPORT_PROPERTY_LABEL: &str = "departureAirport";
pub struct DepartureAirportPropertyIri;
impl PartialEq<&str> for DepartureAirportPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEPARTURE_AIRPORT_PROPERTY_IRI_HTTP
			|| *other == DEPARTURE_AIRPORT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DepartureAirportPropertyIri> for &str {
	fn eq(&self, other: &DepartureAirportPropertyIri) -> bool {
		*self == DEPARTURE_AIRPORT_PROPERTY_IRI_HTTP
			|| *self == DEPARTURE_AIRPORT_PROPERTY_IRI_HTTPS
	}
}
pub struct DepartureAirportPropertyIriOrLabel;
impl PartialEq<&str> for DepartureAirportPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DepartureAirportPropertyIri || *other == DEPARTURE_AIRPORT_PROPERTY_LABEL
	}
}
impl PartialEq<DepartureAirportPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DepartureAirportPropertyIriOrLabel) -> bool {
		*self == DepartureAirportPropertyIri || *self == DEPARTURE_AIRPORT_PROPERTY_LABEL
	}
}
