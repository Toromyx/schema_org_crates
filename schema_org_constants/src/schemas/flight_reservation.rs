/// <https://schema.org/FlightReservation>
pub const FLIGHT_RESERVATION_IRI_HTTP: &str = "http://schema.org/FlightReservation";
/// <https://schema.org/FlightReservation>
pub const FLIGHT_RESERVATION_IRI_HTTPS: &str = "https://schema.org/FlightReservation";
/// <https://schema.org/FlightReservation>
pub const FLIGHT_RESERVATION_LABEL: &str = "FlightReservation";
pub struct FlightReservationIri;
impl PartialEq<&str> for FlightReservationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FLIGHT_RESERVATION_IRI_HTTP || *other == FLIGHT_RESERVATION_IRI_HTTPS
	}
}
impl PartialEq<FlightReservationIri> for &str {
	fn eq(&self, other: &FlightReservationIri) -> bool {
		*self == FLIGHT_RESERVATION_IRI_HTTP || *self == FLIGHT_RESERVATION_IRI_HTTPS
	}
}
pub struct FlightReservationIriOrLabel;
impl PartialEq<&str> for FlightReservationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FlightReservationIri || *other == FLIGHT_RESERVATION_LABEL
	}
}
impl PartialEq<FlightReservationIriOrLabel> for &str {
	fn eq(&self, other: &FlightReservationIriOrLabel) -> bool {
		*self == FlightReservationIri || *self == FLIGHT_RESERVATION_LABEL
	}
}
