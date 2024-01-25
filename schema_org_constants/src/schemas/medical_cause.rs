/// <https://schema.org/MedicalCause>
pub const MEDICAL_CAUSE_IRI_HTTP: &str = "http://schema.org/MedicalCause";
/// <https://schema.org/MedicalCause>
pub const MEDICAL_CAUSE_IRI_HTTPS: &str = "https://schema.org/MedicalCause";
/// <https://schema.org/MedicalCause>
pub const MEDICAL_CAUSE_LABEL: &str = "MedicalCause";
pub struct MedicalCauseIri;
impl PartialEq<&str> for MedicalCauseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_CAUSE_IRI_HTTP || *other == MEDICAL_CAUSE_IRI_HTTPS
	}
}
impl PartialEq<MedicalCauseIri> for &str {
	fn eq(&self, other: &MedicalCauseIri) -> bool {
		*self == MEDICAL_CAUSE_IRI_HTTP || *self == MEDICAL_CAUSE_IRI_HTTPS
	}
}
pub struct MedicalCauseIriOrLabel;
impl PartialEq<&str> for MedicalCauseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalCauseIri || *other == MEDICAL_CAUSE_LABEL
	}
}
impl PartialEq<MedicalCauseIriOrLabel> for &str {
	fn eq(&self, other: &MedicalCauseIriOrLabel) -> bool {
		*self == MedicalCauseIri || *self == MEDICAL_CAUSE_LABEL
	}
}
