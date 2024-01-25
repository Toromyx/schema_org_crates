/// <https://schema.org/MedicalEnumeration>
pub const MEDICAL_ENUMERATION_IRI_HTTP: &str = "http://schema.org/MedicalEnumeration";
/// <https://schema.org/MedicalEnumeration>
pub const MEDICAL_ENUMERATION_IRI_HTTPS: &str = "https://schema.org/MedicalEnumeration";
/// <https://schema.org/MedicalEnumeration>
pub const MEDICAL_ENUMERATION_LABEL: &str = "MedicalEnumeration";
pub struct MedicalEnumerationIri;
impl PartialEq<&str> for MedicalEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_ENUMERATION_IRI_HTTP || *other == MEDICAL_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<MedicalEnumerationIri> for &str {
	fn eq(&self, other: &MedicalEnumerationIri) -> bool {
		*self == MEDICAL_ENUMERATION_IRI_HTTP || *self == MEDICAL_ENUMERATION_IRI_HTTPS
	}
}
pub struct MedicalEnumerationIriOrLabel;
impl PartialEq<&str> for MedicalEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalEnumerationIri || *other == MEDICAL_ENUMERATION_LABEL
	}
}
impl PartialEq<MedicalEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &MedicalEnumerationIriOrLabel) -> bool {
		*self == MedicalEnumerationIri || *self == MEDICAL_ENUMERATION_LABEL
	}
}
