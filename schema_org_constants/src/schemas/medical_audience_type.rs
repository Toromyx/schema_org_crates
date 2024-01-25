/// <https://schema.org/MedicalAudienceType>
pub const MEDICAL_AUDIENCE_TYPE_IRI_HTTP: &str = "http://schema.org/MedicalAudienceType";
/// <https://schema.org/MedicalAudienceType>
pub const MEDICAL_AUDIENCE_TYPE_IRI_HTTPS: &str = "https://schema.org/MedicalAudienceType";
/// <https://schema.org/MedicalAudienceType>
pub const MEDICAL_AUDIENCE_TYPE_LABEL: &str = "MedicalAudienceType";
pub struct MedicalAudienceTypeIri;
impl PartialEq<&str> for MedicalAudienceTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_AUDIENCE_TYPE_IRI_HTTP || *other == MEDICAL_AUDIENCE_TYPE_IRI_HTTPS
	}
}
impl PartialEq<MedicalAudienceTypeIri> for &str {
	fn eq(&self, other: &MedicalAudienceTypeIri) -> bool {
		*self == MEDICAL_AUDIENCE_TYPE_IRI_HTTP || *self == MEDICAL_AUDIENCE_TYPE_IRI_HTTPS
	}
}
pub struct MedicalAudienceTypeIriOrLabel;
impl PartialEq<&str> for MedicalAudienceTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalAudienceTypeIri || *other == MEDICAL_AUDIENCE_TYPE_LABEL
	}
}
impl PartialEq<MedicalAudienceTypeIriOrLabel> for &str {
	fn eq(&self, other: &MedicalAudienceTypeIriOrLabel) -> bool {
		*self == MedicalAudienceTypeIri || *self == MEDICAL_AUDIENCE_TYPE_LABEL
	}
}
