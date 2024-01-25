/// <https://schema.org/Accommodation>
pub const ACCOMMODATION_IRI_HTTP: &str = "http://schema.org/Accommodation";
/// <https://schema.org/Accommodation>
pub const ACCOMMODATION_IRI_HTTPS: &str = "https://schema.org/Accommodation";
/// <https://schema.org/Accommodation>
pub const ACCOMMODATION_LABEL: &str = "Accommodation";
pub struct AccommodationIri;
impl PartialEq<&str> for AccommodationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCOMMODATION_IRI_HTTP || *other == ACCOMMODATION_IRI_HTTPS
	}
}
impl PartialEq<AccommodationIri> for &str {
	fn eq(&self, other: &AccommodationIri) -> bool {
		*self == ACCOMMODATION_IRI_HTTP || *self == ACCOMMODATION_IRI_HTTPS
	}
}
pub struct AccommodationIriOrLabel;
impl PartialEq<&str> for AccommodationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccommodationIri || *other == ACCOMMODATION_LABEL
	}
}
impl PartialEq<AccommodationIriOrLabel> for &str {
	fn eq(&self, other: &AccommodationIriOrLabel) -> bool {
		*self == AccommodationIri || *self == ACCOMMODATION_LABEL
	}
}
