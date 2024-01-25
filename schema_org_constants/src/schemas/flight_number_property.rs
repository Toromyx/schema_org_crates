/// <https://schema.org/flightNumber>
pub const FLIGHT_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/flightNumber";
/// <https://schema.org/flightNumber>
pub const FLIGHT_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/flightNumber";
/// <https://schema.org/flightNumber>
pub const FLIGHT_NUMBER_PROPERTY_LABEL: &str = "flightNumber";
pub struct FlightNumberPropertyIri;
impl PartialEq<&str> for FlightNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FLIGHT_NUMBER_PROPERTY_IRI_HTTP || *other == FLIGHT_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FlightNumberPropertyIri> for &str {
	fn eq(&self, other: &FlightNumberPropertyIri) -> bool {
		*self == FLIGHT_NUMBER_PROPERTY_IRI_HTTP || *self == FLIGHT_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct FlightNumberPropertyIriOrLabel;
impl PartialEq<&str> for FlightNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FlightNumberPropertyIri || *other == FLIGHT_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<FlightNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FlightNumberPropertyIriOrLabel) -> bool {
		*self == FlightNumberPropertyIri || *self == FLIGHT_NUMBER_PROPERTY_LABEL
	}
}
