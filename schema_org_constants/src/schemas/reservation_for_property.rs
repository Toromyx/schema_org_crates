/// <https://schema.org/reservationFor>
pub const RESERVATION_FOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/reservationFor";
/// <https://schema.org/reservationFor>
pub const RESERVATION_FOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/reservationFor";
/// <https://schema.org/reservationFor>
pub const RESERVATION_FOR_PROPERTY_LABEL: &str = "reservationFor";
pub struct ReservationForPropertyIri;
impl PartialEq<&str> for ReservationForPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESERVATION_FOR_PROPERTY_IRI_HTTP || *other == RESERVATION_FOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReservationForPropertyIri> for &str {
	fn eq(&self, other: &ReservationForPropertyIri) -> bool {
		*self == RESERVATION_FOR_PROPERTY_IRI_HTTP || *self == RESERVATION_FOR_PROPERTY_IRI_HTTPS
	}
}
pub struct ReservationForPropertyIriOrLabel;
impl PartialEq<&str> for ReservationForPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReservationForPropertyIri || *other == RESERVATION_FOR_PROPERTY_LABEL
	}
}
impl PartialEq<ReservationForPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReservationForPropertyIriOrLabel) -> bool {
		*self == ReservationForPropertyIri || *self == RESERVATION_FOR_PROPERTY_LABEL
	}
}
