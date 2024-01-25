/// <https://schema.org/MedicalTherapy>
pub const MEDICAL_THERAPY_IRI_HTTP: &str = "http://schema.org/MedicalTherapy";
/// <https://schema.org/MedicalTherapy>
pub const MEDICAL_THERAPY_IRI_HTTPS: &str = "https://schema.org/MedicalTherapy";
/// <https://schema.org/MedicalTherapy>
pub const MEDICAL_THERAPY_LABEL: &str = "MedicalTherapy";
pub struct MedicalTherapyIri;
impl PartialEq<&str> for MedicalTherapyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_THERAPY_IRI_HTTP || *other == MEDICAL_THERAPY_IRI_HTTPS
	}
}
impl PartialEq<MedicalTherapyIri> for &str {
	fn eq(&self, other: &MedicalTherapyIri) -> bool {
		*self == MEDICAL_THERAPY_IRI_HTTP || *self == MEDICAL_THERAPY_IRI_HTTPS
	}
}
pub struct MedicalTherapyIriOrLabel;
impl PartialEq<&str> for MedicalTherapyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalTherapyIri || *other == MEDICAL_THERAPY_LABEL
	}
}
impl PartialEq<MedicalTherapyIriOrLabel> for &str {
	fn eq(&self, other: &MedicalTherapyIriOrLabel) -> bool {
		*self == MedicalTherapyIri || *self == MEDICAL_THERAPY_LABEL
	}
}
