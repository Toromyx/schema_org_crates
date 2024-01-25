/// <https://schema.org/Organization>
pub const ORGANIZATION_IRI_HTTP: &str = "http://schema.org/Organization";
/// <https://schema.org/Organization>
pub const ORGANIZATION_IRI_HTTPS: &str = "https://schema.org/Organization";
/// <https://schema.org/Organization>
pub const ORGANIZATION_LABEL: &str = "Organization";
pub struct OrganizationIri;
impl PartialEq<&str> for OrganizationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORGANIZATION_IRI_HTTP || *other == ORGANIZATION_IRI_HTTPS
	}
}
impl PartialEq<OrganizationIri> for &str {
	fn eq(&self, other: &OrganizationIri) -> bool {
		*self == ORGANIZATION_IRI_HTTP || *self == ORGANIZATION_IRI_HTTPS
	}
}
pub struct OrganizationIriOrLabel;
impl PartialEq<&str> for OrganizationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrganizationIri || *other == ORGANIZATION_LABEL
	}
}
impl PartialEq<OrganizationIriOrLabel> for &str {
	fn eq(&self, other: &OrganizationIriOrLabel) -> bool {
		*self == OrganizationIri || *self == ORGANIZATION_LABEL
	}
}
