/// <https://schema.org/GovernmentOrganization>
pub const GOVERNMENT_ORGANIZATION_IRI_HTTP: &str = "http://schema.org/GovernmentOrganization";
/// <https://schema.org/GovernmentOrganization>
pub const GOVERNMENT_ORGANIZATION_IRI_HTTPS: &str = "https://schema.org/GovernmentOrganization";
/// <https://schema.org/GovernmentOrganization>
pub const GOVERNMENT_ORGANIZATION_LABEL: &str = "GovernmentOrganization";
pub struct GovernmentOrganizationIri;
impl PartialEq<&str> for GovernmentOrganizationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GOVERNMENT_ORGANIZATION_IRI_HTTP || *other == GOVERNMENT_ORGANIZATION_IRI_HTTPS
	}
}
impl PartialEq<GovernmentOrganizationIri> for &str {
	fn eq(&self, other: &GovernmentOrganizationIri) -> bool {
		*self == GOVERNMENT_ORGANIZATION_IRI_HTTP || *self == GOVERNMENT_ORGANIZATION_IRI_HTTPS
	}
}
pub struct GovernmentOrganizationIriOrLabel;
impl PartialEq<&str> for GovernmentOrganizationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GovernmentOrganizationIri || *other == GOVERNMENT_ORGANIZATION_LABEL
	}
}
impl PartialEq<GovernmentOrganizationIriOrLabel> for &str {
	fn eq(&self, other: &GovernmentOrganizationIriOrLabel) -> bool {
		*self == GovernmentOrganizationIri || *self == GOVERNMENT_ORGANIZATION_LABEL
	}
}
