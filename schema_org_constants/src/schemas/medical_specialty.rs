/// <https://schema.org/MedicalSpecialty>
pub const MEDICAL_SPECIALTY_IRI_HTTP: &str = "http://schema.org/MedicalSpecialty";
/// <https://schema.org/MedicalSpecialty>
pub const MEDICAL_SPECIALTY_IRI_HTTPS: &str = "https://schema.org/MedicalSpecialty";
/// <https://schema.org/MedicalSpecialty>
pub const MEDICAL_SPECIALTY_LABEL: &str = "MedicalSpecialty";
pub struct MedicalSpecialtyIri;
impl PartialEq<&str> for MedicalSpecialtyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_SPECIALTY_IRI_HTTP || *other == MEDICAL_SPECIALTY_IRI_HTTPS
	}
}
impl PartialEq<MedicalSpecialtyIri> for &str {
	fn eq(&self, other: &MedicalSpecialtyIri) -> bool {
		*self == MEDICAL_SPECIALTY_IRI_HTTP || *self == MEDICAL_SPECIALTY_IRI_HTTPS
	}
}
pub struct MedicalSpecialtyIriOrLabel;
impl PartialEq<&str> for MedicalSpecialtyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalSpecialtyIri || *other == MEDICAL_SPECIALTY_LABEL
	}
}
impl PartialEq<MedicalSpecialtyIriOrLabel> for &str {
	fn eq(&self, other: &MedicalSpecialtyIriOrLabel) -> bool {
		*self == MedicalSpecialtyIri || *self == MEDICAL_SPECIALTY_LABEL
	}
}
