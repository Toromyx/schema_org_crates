/// <https://schema.org/LodgingReservation>
pub const LODGING_RESERVATION_IRI_HTTP: &str = "http://schema.org/LodgingReservation";
/// <https://schema.org/LodgingReservation>
pub const LODGING_RESERVATION_IRI_HTTPS: &str = "https://schema.org/LodgingReservation";
/// <https://schema.org/LodgingReservation>
pub const LODGING_RESERVATION_LABEL: &str = "LodgingReservation";
pub struct LodgingReservationIri;
impl PartialEq<&str> for LodgingReservationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LODGING_RESERVATION_IRI_HTTP || *other == LODGING_RESERVATION_IRI_HTTPS
	}
}
impl PartialEq<LodgingReservationIri> for &str {
	fn eq(&self, other: &LodgingReservationIri) -> bool {
		*self == LODGING_RESERVATION_IRI_HTTP || *self == LODGING_RESERVATION_IRI_HTTPS
	}
}
pub struct LodgingReservationIriOrLabel;
impl PartialEq<&str> for LodgingReservationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LodgingReservationIri || *other == LODGING_RESERVATION_LABEL
	}
}
impl PartialEq<LodgingReservationIriOrLabel> for &str {
	fn eq(&self, other: &LodgingReservationIriOrLabel) -> bool {
		*self == LodgingReservationIri || *self == LODGING_RESERVATION_LABEL
	}
}
