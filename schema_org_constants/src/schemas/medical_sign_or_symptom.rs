/// <https://schema.org/MedicalSignOrSymptom>
pub const MEDICAL_SIGN_OR_SYMPTOM_IRI_HTTP: &str = "http://schema.org/MedicalSignOrSymptom";
/// <https://schema.org/MedicalSignOrSymptom>
pub const MEDICAL_SIGN_OR_SYMPTOM_IRI_HTTPS: &str = "https://schema.org/MedicalSignOrSymptom";
/// <https://schema.org/MedicalSignOrSymptom>
pub const MEDICAL_SIGN_OR_SYMPTOM_LABEL: &str = "MedicalSignOrSymptom";
pub struct MedicalSignOrSymptomIri;
impl PartialEq<&str> for MedicalSignOrSymptomIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_SIGN_OR_SYMPTOM_IRI_HTTP || *other == MEDICAL_SIGN_OR_SYMPTOM_IRI_HTTPS
	}
}
impl PartialEq<MedicalSignOrSymptomIri> for &str {
	fn eq(&self, other: &MedicalSignOrSymptomIri) -> bool {
		*self == MEDICAL_SIGN_OR_SYMPTOM_IRI_HTTP || *self == MEDICAL_SIGN_OR_SYMPTOM_IRI_HTTPS
	}
}
pub struct MedicalSignOrSymptomIriOrLabel;
impl PartialEq<&str> for MedicalSignOrSymptomIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalSignOrSymptomIri || *other == MEDICAL_SIGN_OR_SYMPTOM_LABEL
	}
}
impl PartialEq<MedicalSignOrSymptomIriOrLabel> for &str {
	fn eq(&self, other: &MedicalSignOrSymptomIriOrLabel) -> bool {
		*self == MedicalSignOrSymptomIri || *self == MEDICAL_SIGN_OR_SYMPTOM_LABEL
	}
}
