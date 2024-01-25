/// <https://schema.org/subOrganization>
pub const SUB_ORGANIZATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/subOrganization";
/// <https://schema.org/subOrganization>
pub const SUB_ORGANIZATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/subOrganization";
/// <https://schema.org/subOrganization>
pub const SUB_ORGANIZATION_PROPERTY_LABEL: &str = "subOrganization";
pub struct SubOrganizationPropertyIri;
impl PartialEq<&str> for SubOrganizationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUB_ORGANIZATION_PROPERTY_IRI_HTTP
			|| *other == SUB_ORGANIZATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SubOrganizationPropertyIri> for &str {
	fn eq(&self, other: &SubOrganizationPropertyIri) -> bool {
		*self == SUB_ORGANIZATION_PROPERTY_IRI_HTTP || *self == SUB_ORGANIZATION_PROPERTY_IRI_HTTPS
	}
}
pub struct SubOrganizationPropertyIriOrLabel;
impl PartialEq<&str> for SubOrganizationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SubOrganizationPropertyIri || *other == SUB_ORGANIZATION_PROPERTY_LABEL
	}
}
impl PartialEq<SubOrganizationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SubOrganizationPropertyIriOrLabel) -> bool {
		*self == SubOrganizationPropertyIri || *self == SUB_ORGANIZATION_PROPERTY_LABEL
	}
}
