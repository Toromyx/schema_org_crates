/// <https://schema.org/MedicalSymptom>
pub const MEDICAL_SYMPTOM_IRI_HTTP: &str = "http://schema.org/MedicalSymptom";
/// <https://schema.org/MedicalSymptom>
pub const MEDICAL_SYMPTOM_IRI_HTTPS: &str = "https://schema.org/MedicalSymptom";
/// <https://schema.org/MedicalSymptom>
pub const MEDICAL_SYMPTOM_LABEL: &str = "MedicalSymptom";
pub struct MedicalSymptomIri;
impl PartialEq<&str> for MedicalSymptomIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_SYMPTOM_IRI_HTTP || *other == MEDICAL_SYMPTOM_IRI_HTTPS
	}
}
impl PartialEq<MedicalSymptomIri> for &str {
	fn eq(&self, other: &MedicalSymptomIri) -> bool {
		*self == MEDICAL_SYMPTOM_IRI_HTTP || *self == MEDICAL_SYMPTOM_IRI_HTTPS
	}
}
pub struct MedicalSymptomIriOrLabel;
impl PartialEq<&str> for MedicalSymptomIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalSymptomIri || *other == MEDICAL_SYMPTOM_LABEL
	}
}
impl PartialEq<MedicalSymptomIriOrLabel> for &str {
	fn eq(&self, other: &MedicalSymptomIriOrLabel) -> bool {
		*self == MedicalSymptomIri || *self == MEDICAL_SYMPTOM_LABEL
	}
}
