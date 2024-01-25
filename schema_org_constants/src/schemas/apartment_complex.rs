/// <https://schema.org/ApartmentComplex>
pub const APARTMENT_COMPLEX_IRI_HTTP: &str = "http://schema.org/ApartmentComplex";
/// <https://schema.org/ApartmentComplex>
pub const APARTMENT_COMPLEX_IRI_HTTPS: &str = "https://schema.org/ApartmentComplex";
/// <https://schema.org/ApartmentComplex>
pub const APARTMENT_COMPLEX_LABEL: &str = "ApartmentComplex";
pub struct ApartmentComplexIri;
impl PartialEq<&str> for ApartmentComplexIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APARTMENT_COMPLEX_IRI_HTTP || *other == APARTMENT_COMPLEX_IRI_HTTPS
	}
}
impl PartialEq<ApartmentComplexIri> for &str {
	fn eq(&self, other: &ApartmentComplexIri) -> bool {
		*self == APARTMENT_COMPLEX_IRI_HTTP || *self == APARTMENT_COMPLEX_IRI_HTTPS
	}
}
pub struct ApartmentComplexIriOrLabel;
impl PartialEq<&str> for ApartmentComplexIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ApartmentComplexIri || *other == APARTMENT_COMPLEX_LABEL
	}
}
impl PartialEq<ApartmentComplexIriOrLabel> for &str {
	fn eq(&self, other: &ApartmentComplexIriOrLabel) -> bool {
		*self == ApartmentComplexIri || *self == APARTMENT_COMPLEX_LABEL
	}
}
