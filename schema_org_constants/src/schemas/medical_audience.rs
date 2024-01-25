/// <https://schema.org/MedicalAudience>
pub const MEDICAL_AUDIENCE_IRI_HTTP: &str = "http://schema.org/MedicalAudience";
/// <https://schema.org/MedicalAudience>
pub const MEDICAL_AUDIENCE_IRI_HTTPS: &str = "https://schema.org/MedicalAudience";
/// <https://schema.org/MedicalAudience>
pub const MEDICAL_AUDIENCE_LABEL: &str = "MedicalAudience";
pub struct MedicalAudienceIri;
impl PartialEq<&str> for MedicalAudienceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_AUDIENCE_IRI_HTTP || *other == MEDICAL_AUDIENCE_IRI_HTTPS
	}
}
impl PartialEq<MedicalAudienceIri> for &str {
	fn eq(&self, other: &MedicalAudienceIri) -> bool {
		*self == MEDICAL_AUDIENCE_IRI_HTTP || *self == MEDICAL_AUDIENCE_IRI_HTTPS
	}
}
pub struct MedicalAudienceIriOrLabel;
impl PartialEq<&str> for MedicalAudienceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalAudienceIri || *other == MEDICAL_AUDIENCE_LABEL
	}
}
impl PartialEq<MedicalAudienceIriOrLabel> for &str {
	fn eq(&self, other: &MedicalAudienceIriOrLabel) -> bool {
		*self == MedicalAudienceIri || *self == MEDICAL_AUDIENCE_LABEL
	}
}
