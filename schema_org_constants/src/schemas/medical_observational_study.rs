/// <https://schema.org/MedicalObservationalStudy>
pub const MEDICAL_OBSERVATIONAL_STUDY_IRI_HTTP: &str =
	"http://schema.org/MedicalObservationalStudy";
/// <https://schema.org/MedicalObservationalStudy>
pub const MEDICAL_OBSERVATIONAL_STUDY_IRI_HTTPS: &str =
	"https://schema.org/MedicalObservationalStudy";
/// <https://schema.org/MedicalObservationalStudy>
pub const MEDICAL_OBSERVATIONAL_STUDY_LABEL: &str = "MedicalObservationalStudy";
pub struct MedicalObservationalStudyIri;
impl PartialEq<&str> for MedicalObservationalStudyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_OBSERVATIONAL_STUDY_IRI_HTTP
			|| *other == MEDICAL_OBSERVATIONAL_STUDY_IRI_HTTPS
	}
}
impl PartialEq<MedicalObservationalStudyIri> for &str {
	fn eq(&self, other: &MedicalObservationalStudyIri) -> bool {
		*self == MEDICAL_OBSERVATIONAL_STUDY_IRI_HTTP
			|| *self == MEDICAL_OBSERVATIONAL_STUDY_IRI_HTTPS
	}
}
pub struct MedicalObservationalStudyIriOrLabel;
impl PartialEq<&str> for MedicalObservationalStudyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalObservationalStudyIri || *other == MEDICAL_OBSERVATIONAL_STUDY_LABEL
	}
}
impl PartialEq<MedicalObservationalStudyIriOrLabel> for &str {
	fn eq(&self, other: &MedicalObservationalStudyIriOrLabel) -> bool {
		*self == MedicalObservationalStudyIri || *self == MEDICAL_OBSERVATIONAL_STUDY_LABEL
	}
}
