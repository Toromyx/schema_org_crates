/// <https://schema.org/reservationStatus>
pub const RESERVATION_STATUS_PROPERTY_IRI_HTTP: &str = "http://schema.org/reservationStatus";
/// <https://schema.org/reservationStatus>
pub const RESERVATION_STATUS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/reservationStatus";
/// <https://schema.org/reservationStatus>
pub const RESERVATION_STATUS_PROPERTY_LABEL: &str = "reservationStatus";
pub struct ReservationStatusPropertyIri;
impl PartialEq<&str> for ReservationStatusPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESERVATION_STATUS_PROPERTY_IRI_HTTP
			|| *other == RESERVATION_STATUS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReservationStatusPropertyIri> for &str {
	fn eq(&self, other: &ReservationStatusPropertyIri) -> bool {
		*self == RESERVATION_STATUS_PROPERTY_IRI_HTTP
			|| *self == RESERVATION_STATUS_PROPERTY_IRI_HTTPS
	}
}
pub struct ReservationStatusPropertyIriOrLabel;
impl PartialEq<&str> for ReservationStatusPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReservationStatusPropertyIri || *other == RESERVATION_STATUS_PROPERTY_LABEL
	}
}
impl PartialEq<ReservationStatusPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReservationStatusPropertyIriOrLabel) -> bool {
		*self == ReservationStatusPropertyIri || *self == RESERVATION_STATUS_PROPERTY_LABEL
	}
}
