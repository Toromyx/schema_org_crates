/// <https://schema.org/MedicalGuideline>
pub const MEDICAL_GUIDELINE_IRI_HTTP: &str = "http://schema.org/MedicalGuideline";
/// <https://schema.org/MedicalGuideline>
pub const MEDICAL_GUIDELINE_IRI_HTTPS: &str = "https://schema.org/MedicalGuideline";
/// <https://schema.org/MedicalGuideline>
pub const MEDICAL_GUIDELINE_LABEL: &str = "MedicalGuideline";
pub struct MedicalGuidelineIri;
impl PartialEq<&str> for MedicalGuidelineIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_GUIDELINE_IRI_HTTP || *other == MEDICAL_GUIDELINE_IRI_HTTPS
	}
}
impl PartialEq<MedicalGuidelineIri> for &str {
	fn eq(&self, other: &MedicalGuidelineIri) -> bool {
		*self == MEDICAL_GUIDELINE_IRI_HTTP || *self == MEDICAL_GUIDELINE_IRI_HTTPS
	}
}
pub struct MedicalGuidelineIriOrLabel;
impl PartialEq<&str> for MedicalGuidelineIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalGuidelineIri || *other == MEDICAL_GUIDELINE_LABEL
	}
}
impl PartialEq<MedicalGuidelineIriOrLabel> for &str {
	fn eq(&self, other: &MedicalGuidelineIriOrLabel) -> bool {
		*self == MedicalGuidelineIri || *self == MEDICAL_GUIDELINE_LABEL
	}
}
