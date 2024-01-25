/// <https://schema.org/medicalSpecialty>
pub const MEDICAL_SPECIALTY_PROPERTY_IRI_HTTP: &str = "http://schema.org/medicalSpecialty";
/// <https://schema.org/medicalSpecialty>
pub const MEDICAL_SPECIALTY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/medicalSpecialty";
/// <https://schema.org/medicalSpecialty>
pub const MEDICAL_SPECIALTY_PROPERTY_LABEL: &str = "medicalSpecialty";
pub struct MedicalSpecialtyPropertyIri;
impl PartialEq<&str> for MedicalSpecialtyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_SPECIALTY_PROPERTY_IRI_HTTP
			|| *other == MEDICAL_SPECIALTY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MedicalSpecialtyPropertyIri> for &str {
	fn eq(&self, other: &MedicalSpecialtyPropertyIri) -> bool {
		*self == MEDICAL_SPECIALTY_PROPERTY_IRI_HTTP
			|| *self == MEDICAL_SPECIALTY_PROPERTY_IRI_HTTPS
	}
}
pub struct MedicalSpecialtyPropertyIriOrLabel;
impl PartialEq<&str> for MedicalSpecialtyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalSpecialtyPropertyIri || *other == MEDICAL_SPECIALTY_PROPERTY_LABEL
	}
}
impl PartialEq<MedicalSpecialtyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MedicalSpecialtyPropertyIriOrLabel) -> bool {
		*self == MedicalSpecialtyPropertyIri || *self == MEDICAL_SPECIALTY_PROPERTY_LABEL
	}
}
