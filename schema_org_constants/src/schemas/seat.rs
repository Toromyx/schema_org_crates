/// <https://schema.org/Seat>
pub const SEAT_IRI_HTTP: &str = "http://schema.org/Seat";
/// <https://schema.org/Seat>
pub const SEAT_IRI_HTTPS: &str = "https://schema.org/Seat";
/// <https://schema.org/Seat>
pub const SEAT_LABEL: &str = "Seat";
pub struct SeatIri;
impl PartialEq<&str> for SeatIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEAT_IRI_HTTP || *other == SEAT_IRI_HTTPS
	}
}
impl PartialEq<SeatIri> for &str {
	fn eq(&self, other: &SeatIri) -> bool {
		*self == SEAT_IRI_HTTP || *self == SEAT_IRI_HTTPS
	}
}
pub struct SeatIriOrLabel;
impl PartialEq<&str> for SeatIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeatIri || *other == SEAT_LABEL
	}
}
impl PartialEq<SeatIriOrLabel> for &str {
	fn eq(&self, other: &SeatIriOrLabel) -> bool {
		*self == SeatIri || *self == SEAT_LABEL
	}
}
