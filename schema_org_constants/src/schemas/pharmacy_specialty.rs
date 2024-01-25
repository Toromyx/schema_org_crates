/// <https://schema.org/PharmacySpecialty>
pub const PHARMACY_SPECIALTY_IRI_HTTP: &str = "http://schema.org/PharmacySpecialty";
/// <https://schema.org/PharmacySpecialty>
pub const PHARMACY_SPECIALTY_IRI_HTTPS: &str = "https://schema.org/PharmacySpecialty";
/// <https://schema.org/PharmacySpecialty>
pub const PHARMACY_SPECIALTY_LABEL: &str = "PharmacySpecialty";
pub struct PharmacySpecialtyIri;
impl PartialEq<&str> for PharmacySpecialtyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PHARMACY_SPECIALTY_IRI_HTTP || *other == PHARMACY_SPECIALTY_IRI_HTTPS
	}
}
impl PartialEq<PharmacySpecialtyIri> for &str {
	fn eq(&self, other: &PharmacySpecialtyIri) -> bool {
		*self == PHARMACY_SPECIALTY_IRI_HTTP || *self == PHARMACY_SPECIALTY_IRI_HTTPS
	}
}
pub struct PharmacySpecialtyIriOrLabel;
impl PartialEq<&str> for PharmacySpecialtyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PharmacySpecialtyIri || *other == PHARMACY_SPECIALTY_LABEL
	}
}
impl PartialEq<PharmacySpecialtyIriOrLabel> for &str {
	fn eq(&self, other: &PharmacySpecialtyIriOrLabel) -> bool {
		*self == PharmacySpecialtyIri || *self == PHARMACY_SPECIALTY_LABEL
	}
}
