/// <https://schema.org/MedicalCode>
pub const MEDICAL_CODE_IRI_HTTP: &str = "http://schema.org/MedicalCode";
/// <https://schema.org/MedicalCode>
pub const MEDICAL_CODE_IRI_HTTPS: &str = "https://schema.org/MedicalCode";
/// <https://schema.org/MedicalCode>
pub const MEDICAL_CODE_LABEL: &str = "MedicalCode";
pub struct MedicalCodeIri;
impl PartialEq<&str> for MedicalCodeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_CODE_IRI_HTTP || *other == MEDICAL_CODE_IRI_HTTPS
	}
}
impl PartialEq<MedicalCodeIri> for &str {
	fn eq(&self, other: &MedicalCodeIri) -> bool {
		*self == MEDICAL_CODE_IRI_HTTP || *self == MEDICAL_CODE_IRI_HTTPS
	}
}
pub struct MedicalCodeIriOrLabel;
impl PartialEq<&str> for MedicalCodeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalCodeIri || *other == MEDICAL_CODE_LABEL
	}
}
impl PartialEq<MedicalCodeIriOrLabel> for &str {
	fn eq(&self, other: &MedicalCodeIriOrLabel) -> bool {
		*self == MedicalCodeIri || *self == MEDICAL_CODE_LABEL
	}
}
