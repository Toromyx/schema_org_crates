/// <https://schema.org/TaxiReservation>
pub const TAXI_RESERVATION_IRI_HTTP: &str = "http://schema.org/TaxiReservation";
/// <https://schema.org/TaxiReservation>
pub const TAXI_RESERVATION_IRI_HTTPS: &str = "https://schema.org/TaxiReservation";
/// <https://schema.org/TaxiReservation>
pub const TAXI_RESERVATION_LABEL: &str = "TaxiReservation";
pub struct TaxiReservationIri;
impl PartialEq<&str> for TaxiReservationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TAXI_RESERVATION_IRI_HTTP || *other == TAXI_RESERVATION_IRI_HTTPS
	}
}
impl PartialEq<TaxiReservationIri> for &str {
	fn eq(&self, other: &TaxiReservationIri) -> bool {
		*self == TAXI_RESERVATION_IRI_HTTP || *self == TAXI_RESERVATION_IRI_HTTPS
	}
}
pub struct TaxiReservationIriOrLabel;
impl PartialEq<&str> for TaxiReservationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TaxiReservationIri || *other == TAXI_RESERVATION_LABEL
	}
}
impl PartialEq<TaxiReservationIriOrLabel> for &str {
	fn eq(&self, other: &TaxiReservationIriOrLabel) -> bool {
		*self == TaxiReservationIri || *self == TAXI_RESERVATION_LABEL
	}
}
