/// <https://schema.org/reservationId>
pub const RESERVATION_ID_PROPERTY_IRI_HTTP: &str = "http://schema.org/reservationId";
/// <https://schema.org/reservationId>
pub const RESERVATION_ID_PROPERTY_IRI_HTTPS: &str = "https://schema.org/reservationId";
/// <https://schema.org/reservationId>
pub const RESERVATION_ID_PROPERTY_LABEL: &str = "reservationId";
pub struct ReservationIdPropertyIri;
impl PartialEq<&str> for ReservationIdPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESERVATION_ID_PROPERTY_IRI_HTTP || *other == RESERVATION_ID_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReservationIdPropertyIri> for &str {
	fn eq(&self, other: &ReservationIdPropertyIri) -> bool {
		*self == RESERVATION_ID_PROPERTY_IRI_HTTP || *self == RESERVATION_ID_PROPERTY_IRI_HTTPS
	}
}
pub struct ReservationIdPropertyIriOrLabel;
impl PartialEq<&str> for ReservationIdPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReservationIdPropertyIri || *other == RESERVATION_ID_PROPERTY_LABEL
	}
}
impl PartialEq<ReservationIdPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReservationIdPropertyIriOrLabel) -> bool {
		*self == ReservationIdPropertyIri || *self == RESERVATION_ID_PROPERTY_LABEL
	}
}
