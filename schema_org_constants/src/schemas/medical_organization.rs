/// <https://schema.org/MedicalOrganization>
pub const MEDICAL_ORGANIZATION_IRI_HTTP: &str = "http://schema.org/MedicalOrganization";
/// <https://schema.org/MedicalOrganization>
pub const MEDICAL_ORGANIZATION_IRI_HTTPS: &str = "https://schema.org/MedicalOrganization";
/// <https://schema.org/MedicalOrganization>
pub const MEDICAL_ORGANIZATION_LABEL: &str = "MedicalOrganization";
pub struct MedicalOrganizationIri;
impl PartialEq<&str> for MedicalOrganizationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_ORGANIZATION_IRI_HTTP || *other == MEDICAL_ORGANIZATION_IRI_HTTPS
	}
}
impl PartialEq<MedicalOrganizationIri> for &str {
	fn eq(&self, other: &MedicalOrganizationIri) -> bool {
		*self == MEDICAL_ORGANIZATION_IRI_HTTP || *self == MEDICAL_ORGANIZATION_IRI_HTTPS
	}
}
pub struct MedicalOrganizationIriOrLabel;
impl PartialEq<&str> for MedicalOrganizationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalOrganizationIri || *other == MEDICAL_ORGANIZATION_LABEL
	}
}
impl PartialEq<MedicalOrganizationIriOrLabel> for &str {
	fn eq(&self, other: &MedicalOrganizationIriOrLabel) -> bool {
		*self == MedicalOrganizationIri || *self == MEDICAL_ORGANIZATION_LABEL
	}
}
