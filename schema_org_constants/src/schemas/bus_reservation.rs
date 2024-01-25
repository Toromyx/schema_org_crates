/// <https://schema.org/BusReservation>
pub const BUS_RESERVATION_IRI_HTTP: &str = "http://schema.org/BusReservation";
/// <https://schema.org/BusReservation>
pub const BUS_RESERVATION_IRI_HTTPS: &str = "https://schema.org/BusReservation";
/// <https://schema.org/BusReservation>
pub const BUS_RESERVATION_LABEL: &str = "BusReservation";
pub struct BusReservationIri;
impl PartialEq<&str> for BusReservationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUS_RESERVATION_IRI_HTTP || *other == BUS_RESERVATION_IRI_HTTPS
	}
}
impl PartialEq<BusReservationIri> for &str {
	fn eq(&self, other: &BusReservationIri) -> bool {
		*self == BUS_RESERVATION_IRI_HTTP || *self == BUS_RESERVATION_IRI_HTTPS
	}
}
pub struct BusReservationIriOrLabel;
impl PartialEq<&str> for BusReservationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BusReservationIri || *other == BUS_RESERVATION_LABEL
	}
}
impl PartialEq<BusReservationIriOrLabel> for &str {
	fn eq(&self, other: &BusReservationIriOrLabel) -> bool {
		*self == BusReservationIri || *self == BUS_RESERVATION_LABEL
	}
}
