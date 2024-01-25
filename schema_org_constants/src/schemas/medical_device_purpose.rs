/// <https://schema.org/MedicalDevicePurpose>
pub const MEDICAL_DEVICE_PURPOSE_IRI_HTTP: &str = "http://schema.org/MedicalDevicePurpose";
/// <https://schema.org/MedicalDevicePurpose>
pub const MEDICAL_DEVICE_PURPOSE_IRI_HTTPS: &str = "https://schema.org/MedicalDevicePurpose";
/// <https://schema.org/MedicalDevicePurpose>
pub const MEDICAL_DEVICE_PURPOSE_LABEL: &str = "MedicalDevicePurpose";
pub struct MedicalDevicePurposeIri;
impl PartialEq<&str> for MedicalDevicePurposeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_DEVICE_PURPOSE_IRI_HTTP || *other == MEDICAL_DEVICE_PURPOSE_IRI_HTTPS
	}
}
impl PartialEq<MedicalDevicePurposeIri> for &str {
	fn eq(&self, other: &MedicalDevicePurposeIri) -> bool {
		*self == MEDICAL_DEVICE_PURPOSE_IRI_HTTP || *self == MEDICAL_DEVICE_PURPOSE_IRI_HTTPS
	}
}
pub struct MedicalDevicePurposeIriOrLabel;
impl PartialEq<&str> for MedicalDevicePurposeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalDevicePurposeIri || *other == MEDICAL_DEVICE_PURPOSE_LABEL
	}
}
impl PartialEq<MedicalDevicePurposeIriOrLabel> for &str {
	fn eq(&self, other: &MedicalDevicePurposeIriOrLabel) -> bool {
		*self == MedicalDevicePurposeIri || *self == MEDICAL_DEVICE_PURPOSE_LABEL
	}
}
