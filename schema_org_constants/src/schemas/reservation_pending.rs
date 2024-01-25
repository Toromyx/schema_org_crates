/// <https://schema.org/ReservationPending>
pub const RESERVATION_PENDING_IRI_HTTP: &str = "http://schema.org/ReservationPending";
/// <https://schema.org/ReservationPending>
pub const RESERVATION_PENDING_IRI_HTTPS: &str = "https://schema.org/ReservationPending";
/// <https://schema.org/ReservationPending>
pub const RESERVATION_PENDING_LABEL: &str = "ReservationPending";
pub struct ReservationPendingIri;
impl PartialEq<&str> for ReservationPendingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESERVATION_PENDING_IRI_HTTP || *other == RESERVATION_PENDING_IRI_HTTPS
	}
}
impl PartialEq<ReservationPendingIri> for &str {
	fn eq(&self, other: &ReservationPendingIri) -> bool {
		*self == RESERVATION_PENDING_IRI_HTTP || *self == RESERVATION_PENDING_IRI_HTTPS
	}
}
pub struct ReservationPendingIriOrLabel;
impl PartialEq<&str> for ReservationPendingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReservationPendingIri || *other == RESERVATION_PENDING_LABEL
	}
}
impl PartialEq<ReservationPendingIriOrLabel> for &str {
	fn eq(&self, other: &ReservationPendingIriOrLabel) -> bool {
		*self == ReservationPendingIri || *self == RESERVATION_PENDING_LABEL
	}
}
