/// <https://schema.org/ReservationHold>
pub const RESERVATION_HOLD_IRI_HTTP: &str = "http://schema.org/ReservationHold";
/// <https://schema.org/ReservationHold>
pub const RESERVATION_HOLD_IRI_HTTPS: &str = "https://schema.org/ReservationHold";
/// <https://schema.org/ReservationHold>
pub const RESERVATION_HOLD_LABEL: &str = "ReservationHold";
pub struct ReservationHoldIri;
impl PartialEq<&str> for ReservationHoldIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESERVATION_HOLD_IRI_HTTP || *other == RESERVATION_HOLD_IRI_HTTPS
	}
}
impl PartialEq<ReservationHoldIri> for &str {
	fn eq(&self, other: &ReservationHoldIri) -> bool {
		*self == RESERVATION_HOLD_IRI_HTTP || *self == RESERVATION_HOLD_IRI_HTTPS
	}
}
pub struct ReservationHoldIriOrLabel;
impl PartialEq<&str> for ReservationHoldIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReservationHoldIri || *other == RESERVATION_HOLD_LABEL
	}
}
impl PartialEq<ReservationHoldIriOrLabel> for &str {
	fn eq(&self, other: &ReservationHoldIriOrLabel) -> bool {
		*self == ReservationHoldIri || *self == RESERVATION_HOLD_LABEL
	}
}
