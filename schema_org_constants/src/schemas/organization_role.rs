/// <https://schema.org/OrganizationRole>
pub const ORGANIZATION_ROLE_IRI_HTTP: &str = "http://schema.org/OrganizationRole";
/// <https://schema.org/OrganizationRole>
pub const ORGANIZATION_ROLE_IRI_HTTPS: &str = "https://schema.org/OrganizationRole";
/// <https://schema.org/OrganizationRole>
pub const ORGANIZATION_ROLE_LABEL: &str = "OrganizationRole";
pub struct OrganizationRoleIri;
impl PartialEq<&str> for OrganizationRoleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORGANIZATION_ROLE_IRI_HTTP || *other == ORGANIZATION_ROLE_IRI_HTTPS
	}
}
impl PartialEq<OrganizationRoleIri> for &str {
	fn eq(&self, other: &OrganizationRoleIri) -> bool {
		*self == ORGANIZATION_ROLE_IRI_HTTP || *self == ORGANIZATION_ROLE_IRI_HTTPS
	}
}
pub struct OrganizationRoleIriOrLabel;
impl PartialEq<&str> for OrganizationRoleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrganizationRoleIri || *other == ORGANIZATION_ROLE_LABEL
	}
}
impl PartialEq<OrganizationRoleIriOrLabel> for &str {
	fn eq(&self, other: &OrganizationRoleIriOrLabel) -> bool {
		*self == OrganizationRoleIri || *self == ORGANIZATION_ROLE_LABEL
	}
}
