/// <https://schema.org/flightDistance>
pub const FLIGHT_DISTANCE_PROPERTY_IRI_HTTP: &str = "http://schema.org/flightDistance";
/// <https://schema.org/flightDistance>
pub const FLIGHT_DISTANCE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/flightDistance";
/// <https://schema.org/flightDistance>
pub const FLIGHT_DISTANCE_PROPERTY_LABEL: &str = "flightDistance";
pub struct FlightDistancePropertyIri;
impl PartialEq<&str> for FlightDistancePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FLIGHT_DISTANCE_PROPERTY_IRI_HTTP || *other == FLIGHT_DISTANCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FlightDistancePropertyIri> for &str {
	fn eq(&self, other: &FlightDistancePropertyIri) -> bool {
		*self == FLIGHT_DISTANCE_PROPERTY_IRI_HTTP || *self == FLIGHT_DISTANCE_PROPERTY_IRI_HTTPS
	}
}
pub struct FlightDistancePropertyIriOrLabel;
impl PartialEq<&str> for FlightDistancePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FlightDistancePropertyIri || *other == FLIGHT_DISTANCE_PROPERTY_LABEL
	}
}
impl PartialEq<FlightDistancePropertyIriOrLabel> for &str {
	fn eq(&self, other: &FlightDistancePropertyIriOrLabel) -> bool {
		*self == FlightDistancePropertyIri || *self == FLIGHT_DISTANCE_PROPERTY_LABEL
	}
}
