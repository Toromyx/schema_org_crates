/// <https://schema.org/MedicalClinic>
pub const MEDICAL_CLINIC_IRI_HTTP: &str = "http://schema.org/MedicalClinic";
/// <https://schema.org/MedicalClinic>
pub const MEDICAL_CLINIC_IRI_HTTPS: &str = "https://schema.org/MedicalClinic";
/// <https://schema.org/MedicalClinic>
pub const MEDICAL_CLINIC_LABEL: &str = "MedicalClinic";
pub struct MedicalClinicIri;
impl PartialEq<&str> for MedicalClinicIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_CLINIC_IRI_HTTP || *other == MEDICAL_CLINIC_IRI_HTTPS
	}
}
impl PartialEq<MedicalClinicIri> for &str {
	fn eq(&self, other: &MedicalClinicIri) -> bool {
		*self == MEDICAL_CLINIC_IRI_HTTP || *self == MEDICAL_CLINIC_IRI_HTTPS
	}
}
pub struct MedicalClinicIriOrLabel;
impl PartialEq<&str> for MedicalClinicIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalClinicIri || *other == MEDICAL_CLINIC_LABEL
	}
}
impl PartialEq<MedicalClinicIriOrLabel> for &str {
	fn eq(&self, other: &MedicalClinicIriOrLabel) -> bool {
		*self == MedicalClinicIri || *self == MEDICAL_CLINIC_LABEL
	}
}
