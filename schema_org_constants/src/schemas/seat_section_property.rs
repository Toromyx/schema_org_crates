/// <https://schema.org/seatSection>
pub const SEAT_SECTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/seatSection";
/// <https://schema.org/seatSection>
pub const SEAT_SECTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/seatSection";
/// <https://schema.org/seatSection>
pub const SEAT_SECTION_PROPERTY_LABEL: &str = "seatSection";
pub struct SeatSectionPropertyIri;
impl PartialEq<&str> for SeatSectionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEAT_SECTION_PROPERTY_IRI_HTTP || *other == SEAT_SECTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SeatSectionPropertyIri> for &str {
	fn eq(&self, other: &SeatSectionPropertyIri) -> bool {
		*self == SEAT_SECTION_PROPERTY_IRI_HTTP || *self == SEAT_SECTION_PROPERTY_IRI_HTTPS
	}
}
pub struct SeatSectionPropertyIriOrLabel;
impl PartialEq<&str> for SeatSectionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeatSectionPropertyIri || *other == SEAT_SECTION_PROPERTY_LABEL
	}
}
impl PartialEq<SeatSectionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SeatSectionPropertyIriOrLabel) -> bool {
		*self == SeatSectionPropertyIri || *self == SEAT_SECTION_PROPERTY_LABEL
	}
}
