/// <https://schema.org/BoatReservation>
pub const BOAT_RESERVATION_IRI_HTTP: &str = "http://schema.org/BoatReservation";
/// <https://schema.org/BoatReservation>
pub const BOAT_RESERVATION_IRI_HTTPS: &str = "https://schema.org/BoatReservation";
/// <https://schema.org/BoatReservation>
pub const BOAT_RESERVATION_LABEL: &str = "BoatReservation";
pub struct BoatReservationIri;
impl PartialEq<&str> for BoatReservationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOAT_RESERVATION_IRI_HTTP || *other == BOAT_RESERVATION_IRI_HTTPS
	}
}
impl PartialEq<BoatReservationIri> for &str {
	fn eq(&self, other: &BoatReservationIri) -> bool {
		*self == BOAT_RESERVATION_IRI_HTTP || *self == BOAT_RESERVATION_IRI_HTTPS
	}
}
pub struct BoatReservationIriOrLabel;
impl PartialEq<&str> for BoatReservationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BoatReservationIri || *other == BOAT_RESERVATION_LABEL
	}
}
impl PartialEq<BoatReservationIriOrLabel> for &str {
	fn eq(&self, other: &BoatReservationIriOrLabel) -> bool {
		*self == BoatReservationIri || *self == BOAT_RESERVATION_LABEL
	}
}
