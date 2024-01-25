/// <https://schema.org/seatNumber>
pub const SEAT_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/seatNumber";
/// <https://schema.org/seatNumber>
pub const SEAT_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/seatNumber";
/// <https://schema.org/seatNumber>
pub const SEAT_NUMBER_PROPERTY_LABEL: &str = "seatNumber";
pub struct SeatNumberPropertyIri;
impl PartialEq<&str> for SeatNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEAT_NUMBER_PROPERTY_IRI_HTTP || *other == SEAT_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SeatNumberPropertyIri> for &str {
	fn eq(&self, other: &SeatNumberPropertyIri) -> bool {
		*self == SEAT_NUMBER_PROPERTY_IRI_HTTP || *self == SEAT_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct SeatNumberPropertyIriOrLabel;
impl PartialEq<&str> for SeatNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeatNumberPropertyIri || *other == SEAT_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<SeatNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SeatNumberPropertyIriOrLabel) -> bool {
		*self == SeatNumberPropertyIri || *self == SEAT_NUMBER_PROPERTY_LABEL
	}
}
