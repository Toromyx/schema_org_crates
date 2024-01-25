/// <https://schema.org/ReservationConfirmed>
pub const RESERVATION_CONFIRMED_IRI_HTTP: &str = "http://schema.org/ReservationConfirmed";
/// <https://schema.org/ReservationConfirmed>
pub const RESERVATION_CONFIRMED_IRI_HTTPS: &str = "https://schema.org/ReservationConfirmed";
/// <https://schema.org/ReservationConfirmed>
pub const RESERVATION_CONFIRMED_LABEL: &str = "ReservationConfirmed";
pub struct ReservationConfirmedIri;
impl PartialEq<&str> for ReservationConfirmedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESERVATION_CONFIRMED_IRI_HTTP || *other == RESERVATION_CONFIRMED_IRI_HTTPS
	}
}
impl PartialEq<ReservationConfirmedIri> for &str {
	fn eq(&self, other: &ReservationConfirmedIri) -> bool {
		*self == RESERVATION_CONFIRMED_IRI_HTTP || *self == RESERVATION_CONFIRMED_IRI_HTTPS
	}
}
pub struct ReservationConfirmedIriOrLabel;
impl PartialEq<&str> for ReservationConfirmedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReservationConfirmedIri || *other == RESERVATION_CONFIRMED_LABEL
	}
}
impl PartialEq<ReservationConfirmedIriOrLabel> for &str {
	fn eq(&self, other: &ReservationConfirmedIriOrLabel) -> bool {
		*self == ReservationConfirmedIri || *self == RESERVATION_CONFIRMED_LABEL
	}
}
