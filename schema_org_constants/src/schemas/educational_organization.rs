/// <https://schema.org/EducationalOrganization>
pub const EDUCATIONAL_ORGANIZATION_IRI_HTTP: &str = "http://schema.org/EducationalOrganization";
/// <https://schema.org/EducationalOrganization>
pub const EDUCATIONAL_ORGANIZATION_IRI_HTTPS: &str = "https://schema.org/EducationalOrganization";
/// <https://schema.org/EducationalOrganization>
pub const EDUCATIONAL_ORGANIZATION_LABEL: &str = "EducationalOrganization";
pub struct EducationalOrganizationIri;
impl PartialEq<&str> for EducationalOrganizationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDUCATIONAL_ORGANIZATION_IRI_HTTP || *other == EDUCATIONAL_ORGANIZATION_IRI_HTTPS
	}
}
impl PartialEq<EducationalOrganizationIri> for &str {
	fn eq(&self, other: &EducationalOrganizationIri) -> bool {
		*self == EDUCATIONAL_ORGANIZATION_IRI_HTTP || *self == EDUCATIONAL_ORGANIZATION_IRI_HTTPS
	}
}
pub struct EducationalOrganizationIriOrLabel;
impl PartialEq<&str> for EducationalOrganizationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EducationalOrganizationIri || *other == EDUCATIONAL_ORGANIZATION_LABEL
	}
}
impl PartialEq<EducationalOrganizationIriOrLabel> for &str {
	fn eq(&self, other: &EducationalOrganizationIriOrLabel) -> bool {
		*self == EducationalOrganizationIri || *self == EDUCATIONAL_ORGANIZATION_LABEL
	}
}
