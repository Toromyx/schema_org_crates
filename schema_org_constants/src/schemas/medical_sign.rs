/// <https://schema.org/MedicalSign>
pub const MEDICAL_SIGN_IRI_HTTP: &str = "http://schema.org/MedicalSign";
/// <https://schema.org/MedicalSign>
pub const MEDICAL_SIGN_IRI_HTTPS: &str = "https://schema.org/MedicalSign";
/// <https://schema.org/MedicalSign>
pub const MEDICAL_SIGN_LABEL: &str = "MedicalSign";
pub struct MedicalSignIri;
impl PartialEq<&str> for MedicalSignIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_SIGN_IRI_HTTP || *other == MEDICAL_SIGN_IRI_HTTPS
	}
}
impl PartialEq<MedicalSignIri> for &str {
	fn eq(&self, other: &MedicalSignIri) -> bool {
		*self == MEDICAL_SIGN_IRI_HTTP || *self == MEDICAL_SIGN_IRI_HTTPS
	}
}
pub struct MedicalSignIriOrLabel;
impl PartialEq<&str> for MedicalSignIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalSignIri || *other == MEDICAL_SIGN_LABEL
	}
}
impl PartialEq<MedicalSignIriOrLabel> for &str {
	fn eq(&self, other: &MedicalSignIriOrLabel) -> bool {
		*self == MedicalSignIri || *self == MEDICAL_SIGN_LABEL
	}
}
