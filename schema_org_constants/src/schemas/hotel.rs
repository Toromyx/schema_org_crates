/// <https://schema.org/Hotel>
pub const HOTEL_IRI_HTTP: &str = "http://schema.org/Hotel";
/// <https://schema.org/Hotel>
pub const HOTEL_IRI_HTTPS: &str = "https://schema.org/Hotel";
/// <https://schema.org/Hotel>
pub const HOTEL_LABEL: &str = "Hotel";
pub struct HotelIri;
impl PartialEq<&str> for HotelIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOTEL_IRI_HTTP || *other == HOTEL_IRI_HTTPS
	}
}
impl PartialEq<HotelIri> for &str {
	fn eq(&self, other: &HotelIri) -> bool {
		*self == HOTEL_IRI_HTTP || *self == HOTEL_IRI_HTTPS
	}
}
pub struct HotelIriOrLabel;
impl PartialEq<&str> for HotelIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HotelIri || *other == HOTEL_LABEL
	}
}
impl PartialEq<HotelIriOrLabel> for &str {
	fn eq(&self, other: &HotelIriOrLabel) -> bool {
		*self == HotelIri || *self == HOTEL_LABEL
	}
}
