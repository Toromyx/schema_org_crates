/// <https://schema.org/medicalAudience>
pub const MEDICAL_AUDIENCE_PROPERTY_IRI_HTTP: &str = "http://schema.org/medicalAudience";
/// <https://schema.org/medicalAudience>
pub const MEDICAL_AUDIENCE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/medicalAudience";
/// <https://schema.org/medicalAudience>
pub const MEDICAL_AUDIENCE_PROPERTY_LABEL: &str = "medicalAudience";
pub struct MedicalAudiencePropertyIri;
impl PartialEq<&str> for MedicalAudiencePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_AUDIENCE_PROPERTY_IRI_HTTP
			|| *other == MEDICAL_AUDIENCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MedicalAudiencePropertyIri> for &str {
	fn eq(&self, other: &MedicalAudiencePropertyIri) -> bool {
		*self == MEDICAL_AUDIENCE_PROPERTY_IRI_HTTP || *self == MEDICAL_AUDIENCE_PROPERTY_IRI_HTTPS
	}
}
pub struct MedicalAudiencePropertyIriOrLabel;
impl PartialEq<&str> for MedicalAudiencePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalAudiencePropertyIri || *other == MEDICAL_AUDIENCE_PROPERTY_LABEL
	}
}
impl PartialEq<MedicalAudiencePropertyIriOrLabel> for &str {
	fn eq(&self, other: &MedicalAudiencePropertyIriOrLabel) -> bool {
		*self == MedicalAudiencePropertyIri || *self == MEDICAL_AUDIENCE_PROPERTY_LABEL
	}
}
