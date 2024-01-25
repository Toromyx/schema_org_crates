/// <https://schema.org/seatRow>
pub const SEAT_ROW_PROPERTY_IRI_HTTP: &str = "http://schema.org/seatRow";
/// <https://schema.org/seatRow>
pub const SEAT_ROW_PROPERTY_IRI_HTTPS: &str = "https://schema.org/seatRow";
/// <https://schema.org/seatRow>
pub const SEAT_ROW_PROPERTY_LABEL: &str = "seatRow";
pub struct SeatRowPropertyIri;
impl PartialEq<&str> for SeatRowPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEAT_ROW_PROPERTY_IRI_HTTP || *other == SEAT_ROW_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SeatRowPropertyIri> for &str {
	fn eq(&self, other: &SeatRowPropertyIri) -> bool {
		*self == SEAT_ROW_PROPERTY_IRI_HTTP || *self == SEAT_ROW_PROPERTY_IRI_HTTPS
	}
}
pub struct SeatRowPropertyIriOrLabel;
impl PartialEq<&str> for SeatRowPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeatRowPropertyIri || *other == SEAT_ROW_PROPERTY_LABEL
	}
}
impl PartialEq<SeatRowPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SeatRowPropertyIriOrLabel) -> bool {
		*self == SeatRowPropertyIri || *self == SEAT_ROW_PROPERTY_LABEL
	}
}
