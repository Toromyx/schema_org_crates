/// <https://schema.org/MedicalStudyStatus>
pub const MEDICAL_STUDY_STATUS_IRI_HTTP: &str = "http://schema.org/MedicalStudyStatus";
/// <https://schema.org/MedicalStudyStatus>
pub const MEDICAL_STUDY_STATUS_IRI_HTTPS: &str = "https://schema.org/MedicalStudyStatus";
/// <https://schema.org/MedicalStudyStatus>
pub const MEDICAL_STUDY_STATUS_LABEL: &str = "MedicalStudyStatus";
pub struct MedicalStudyStatusIri;
impl PartialEq<&str> for MedicalStudyStatusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_STUDY_STATUS_IRI_HTTP || *other == MEDICAL_STUDY_STATUS_IRI_HTTPS
	}
}
impl PartialEq<MedicalStudyStatusIri> for &str {
	fn eq(&self, other: &MedicalStudyStatusIri) -> bool {
		*self == MEDICAL_STUDY_STATUS_IRI_HTTP || *self == MEDICAL_STUDY_STATUS_IRI_HTTPS
	}
}
pub struct MedicalStudyStatusIriOrLabel;
impl PartialEq<&str> for MedicalStudyStatusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalStudyStatusIri || *other == MEDICAL_STUDY_STATUS_LABEL
	}
}
impl PartialEq<MedicalStudyStatusIriOrLabel> for &str {
	fn eq(&self, other: &MedicalStudyStatusIriOrLabel) -> bool {
		*self == MedicalStudyStatusIri || *self == MEDICAL_STUDY_STATUS_LABEL
	}
}
