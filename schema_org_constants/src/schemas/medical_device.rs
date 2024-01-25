/// <https://schema.org/MedicalDevice>
pub const MEDICAL_DEVICE_IRI_HTTP: &str = "http://schema.org/MedicalDevice";
/// <https://schema.org/MedicalDevice>
pub const MEDICAL_DEVICE_IRI_HTTPS: &str = "https://schema.org/MedicalDevice";
/// <https://schema.org/MedicalDevice>
pub const MEDICAL_DEVICE_LABEL: &str = "MedicalDevice";
pub struct MedicalDeviceIri;
impl PartialEq<&str> for MedicalDeviceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_DEVICE_IRI_HTTP || *other == MEDICAL_DEVICE_IRI_HTTPS
	}
}
impl PartialEq<MedicalDeviceIri> for &str {
	fn eq(&self, other: &MedicalDeviceIri) -> bool {
		*self == MEDICAL_DEVICE_IRI_HTTP || *self == MEDICAL_DEVICE_IRI_HTTPS
	}
}
pub struct MedicalDeviceIriOrLabel;
impl PartialEq<&str> for MedicalDeviceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalDeviceIri || *other == MEDICAL_DEVICE_LABEL
	}
}
impl PartialEq<MedicalDeviceIriOrLabel> for &str {
	fn eq(&self, other: &MedicalDeviceIriOrLabel) -> bool {
		*self == MedicalDeviceIri || *self == MEDICAL_DEVICE_LABEL
	}
}
