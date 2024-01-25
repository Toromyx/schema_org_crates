/// <https://schema.org/Apartment>
pub const APARTMENT_IRI_HTTP: &str = "http://schema.org/Apartment";
/// <https://schema.org/Apartment>
pub const APARTMENT_IRI_HTTPS: &str = "https://schema.org/Apartment";
/// <https://schema.org/Apartment>
pub const APARTMENT_LABEL: &str = "Apartment";
pub struct ApartmentIri;
impl PartialEq<&str> for ApartmentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APARTMENT_IRI_HTTP || *other == APARTMENT_IRI_HTTPS
	}
}
impl PartialEq<ApartmentIri> for &str {
	fn eq(&self, other: &ApartmentIri) -> bool {
		*self == APARTMENT_IRI_HTTP || *self == APARTMENT_IRI_HTTPS
	}
}
pub struct ApartmentIriOrLabel;
impl PartialEq<&str> for ApartmentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ApartmentIri || *other == APARTMENT_LABEL
	}
}
impl PartialEq<ApartmentIriOrLabel> for &str {
	fn eq(&self, other: &ApartmentIriOrLabel) -> bool {
		*self == ApartmentIri || *self == APARTMENT_LABEL
	}
}
