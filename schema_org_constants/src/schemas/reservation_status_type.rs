/// <https://schema.org/ReservationStatusType>
pub const RESERVATION_STATUS_TYPE_IRI_HTTP: &str = "http://schema.org/ReservationStatusType";
/// <https://schema.org/ReservationStatusType>
pub const RESERVATION_STATUS_TYPE_IRI_HTTPS: &str = "https://schema.org/ReservationStatusType";
/// <https://schema.org/ReservationStatusType>
pub const RESERVATION_STATUS_TYPE_LABEL: &str = "ReservationStatusType";
pub struct ReservationStatusTypeIri;
impl PartialEq<&str> for ReservationStatusTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESERVATION_STATUS_TYPE_IRI_HTTP || *other == RESERVATION_STATUS_TYPE_IRI_HTTPS
	}
}
impl PartialEq<ReservationStatusTypeIri> for &str {
	fn eq(&self, other: &ReservationStatusTypeIri) -> bool {
		*self == RESERVATION_STATUS_TYPE_IRI_HTTP || *self == RESERVATION_STATUS_TYPE_IRI_HTTPS
	}
}
pub struct ReservationStatusTypeIriOrLabel;
impl PartialEq<&str> for ReservationStatusTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReservationStatusTypeIri || *other == RESERVATION_STATUS_TYPE_LABEL
	}
}
impl PartialEq<ReservationStatusTypeIriOrLabel> for &str {
	fn eq(&self, other: &ReservationStatusTypeIriOrLabel) -> bool {
		*self == ReservationStatusTypeIri || *self == RESERVATION_STATUS_TYPE_LABEL
	}
}
