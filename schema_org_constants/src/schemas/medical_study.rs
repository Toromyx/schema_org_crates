/// <https://schema.org/MedicalStudy>
pub const MEDICAL_STUDY_IRI_HTTP: &str = "http://schema.org/MedicalStudy";
/// <https://schema.org/MedicalStudy>
pub const MEDICAL_STUDY_IRI_HTTPS: &str = "https://schema.org/MedicalStudy";
/// <https://schema.org/MedicalStudy>
pub const MEDICAL_STUDY_LABEL: &str = "MedicalStudy";
pub struct MedicalStudyIri;
impl PartialEq<&str> for MedicalStudyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_STUDY_IRI_HTTP || *other == MEDICAL_STUDY_IRI_HTTPS
	}
}
impl PartialEq<MedicalStudyIri> for &str {
	fn eq(&self, other: &MedicalStudyIri) -> bool {
		*self == MEDICAL_STUDY_IRI_HTTP || *self == MEDICAL_STUDY_IRI_HTTPS
	}
}
pub struct MedicalStudyIriOrLabel;
impl PartialEq<&str> for MedicalStudyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalStudyIri || *other == MEDICAL_STUDY_LABEL
	}
}
impl PartialEq<MedicalStudyIriOrLabel> for &str {
	fn eq(&self, other: &MedicalStudyIriOrLabel) -> bool {
		*self == MedicalStudyIri || *self == MEDICAL_STUDY_LABEL
	}
}
