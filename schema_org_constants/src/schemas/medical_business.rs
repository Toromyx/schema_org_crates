/// <https://schema.org/MedicalBusiness>
pub const MEDICAL_BUSINESS_IRI_HTTP: &str = "http://schema.org/MedicalBusiness";
/// <https://schema.org/MedicalBusiness>
pub const MEDICAL_BUSINESS_IRI_HTTPS: &str = "https://schema.org/MedicalBusiness";
/// <https://schema.org/MedicalBusiness>
pub const MEDICAL_BUSINESS_LABEL: &str = "MedicalBusiness";
pub struct MedicalBusinessIri;
impl PartialEq<&str> for MedicalBusinessIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_BUSINESS_IRI_HTTP || *other == MEDICAL_BUSINESS_IRI_HTTPS
	}
}
impl PartialEq<MedicalBusinessIri> for &str {
	fn eq(&self, other: &MedicalBusinessIri) -> bool {
		*self == MEDICAL_BUSINESS_IRI_HTTP || *self == MEDICAL_BUSINESS_IRI_HTTPS
	}
}
pub struct MedicalBusinessIriOrLabel;
impl PartialEq<&str> for MedicalBusinessIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalBusinessIri || *other == MEDICAL_BUSINESS_LABEL
	}
}
impl PartialEq<MedicalBusinessIriOrLabel> for &str {
	fn eq(&self, other: &MedicalBusinessIriOrLabel) -> bool {
		*self == MedicalBusinessIri || *self == MEDICAL_BUSINESS_LABEL
	}
}
