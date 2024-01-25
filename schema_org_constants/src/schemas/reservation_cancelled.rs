/// <https://schema.org/ReservationCancelled>
pub const RESERVATION_CANCELLED_IRI_HTTP: &str = "http://schema.org/ReservationCancelled";
/// <https://schema.org/ReservationCancelled>
pub const RESERVATION_CANCELLED_IRI_HTTPS: &str = "https://schema.org/ReservationCancelled";
/// <https://schema.org/ReservationCancelled>
pub const RESERVATION_CANCELLED_LABEL: &str = "ReservationCancelled";
pub struct ReservationCancelledIri;
impl PartialEq<&str> for ReservationCancelledIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESERVATION_CANCELLED_IRI_HTTP || *other == RESERVATION_CANCELLED_IRI_HTTPS
	}
}
impl PartialEq<ReservationCancelledIri> for &str {
	fn eq(&self, other: &ReservationCancelledIri) -> bool {
		*self == RESERVATION_CANCELLED_IRI_HTTP || *self == RESERVATION_CANCELLED_IRI_HTTPS
	}
}
pub struct ReservationCancelledIriOrLabel;
impl PartialEq<&str> for ReservationCancelledIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReservationCancelledIri || *other == RESERVATION_CANCELLED_LABEL
	}
}
impl PartialEq<ReservationCancelledIriOrLabel> for &str {
	fn eq(&self, other: &ReservationCancelledIriOrLabel) -> bool {
		*self == ReservationCancelledIri || *self == RESERVATION_CANCELLED_LABEL
	}
}
