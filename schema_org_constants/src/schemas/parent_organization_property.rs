/// <https://schema.org/parentOrganization>
pub const PARENT_ORGANIZATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/parentOrganization";
/// <https://schema.org/parentOrganization>
pub const PARENT_ORGANIZATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/parentOrganization";
/// <https://schema.org/parentOrganization>
pub const PARENT_ORGANIZATION_PROPERTY_LABEL: &str = "parentOrganization";
pub struct ParentOrganizationPropertyIri;
impl PartialEq<&str> for ParentOrganizationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PARENT_ORGANIZATION_PROPERTY_IRI_HTTP
			|| *other == PARENT_ORGANIZATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ParentOrganizationPropertyIri> for &str {
	fn eq(&self, other: &ParentOrganizationPropertyIri) -> bool {
		*self == PARENT_ORGANIZATION_PROPERTY_IRI_HTTP
			|| *self == PARENT_ORGANIZATION_PROPERTY_IRI_HTTPS
	}
}
pub struct ParentOrganizationPropertyIriOrLabel;
impl PartialEq<&str> for ParentOrganizationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ParentOrganizationPropertyIri || *other == PARENT_ORGANIZATION_PROPERTY_LABEL
	}
}
impl PartialEq<ParentOrganizationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ParentOrganizationPropertyIriOrLabel) -> bool {
		*self == ParentOrganizationPropertyIri || *self == PARENT_ORGANIZATION_PROPERTY_LABEL
	}
}
