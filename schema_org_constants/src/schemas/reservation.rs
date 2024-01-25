/// <https://schema.org/Reservation>
pub const RESERVATION_IRI_HTTP: &str = "http://schema.org/Reservation";
/// <https://schema.org/Reservation>
pub const RESERVATION_IRI_HTTPS: &str = "https://schema.org/Reservation";
/// <https://schema.org/Reservation>
pub const RESERVATION_LABEL: &str = "Reservation";
pub struct ReservationIri;
impl PartialEq<&str> for ReservationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESERVATION_IRI_HTTP || *other == RESERVATION_IRI_HTTPS
	}
}
impl PartialEq<ReservationIri> for &str {
	fn eq(&self, other: &ReservationIri) -> bool {
		*self == RESERVATION_IRI_HTTP || *self == RESERVATION_IRI_HTTPS
	}
}
pub struct ReservationIriOrLabel;
impl PartialEq<&str> for ReservationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReservationIri || *other == RESERVATION_LABEL
	}
}
impl PartialEq<ReservationIriOrLabel> for &str {
	fn eq(&self, other: &ReservationIriOrLabel) -> bool {
		*self == ReservationIri || *self == RESERVATION_LABEL
	}
}
