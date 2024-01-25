/// <https://schema.org/Flight>
pub const FLIGHT_IRI_HTTP: &str = "http://schema.org/Flight";
/// <https://schema.org/Flight>
pub const FLIGHT_IRI_HTTPS: &str = "https://schema.org/Flight";
/// <https://schema.org/Flight>
pub const FLIGHT_LABEL: &str = "Flight";
pub struct FlightIri;
impl PartialEq<&str> for FlightIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FLIGHT_IRI_HTTP || *other == FLIGHT_IRI_HTTPS
	}
}
impl PartialEq<FlightIri> for &str {
	fn eq(&self, other: &FlightIri) -> bool {
		*self == FLIGHT_IRI_HTTP || *self == FLIGHT_IRI_HTTPS
	}
}
pub struct FlightIriOrLabel;
impl PartialEq<&str> for FlightIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FlightIri || *other == FLIGHT_LABEL
	}
}
impl PartialEq<FlightIriOrLabel> for &str {
	fn eq(&self, other: &FlightIriOrLabel) -> bool {
		*self == FlightIri || *self == FLIGHT_LABEL
	}
}
