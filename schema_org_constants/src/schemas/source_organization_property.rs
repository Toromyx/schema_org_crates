/// <https://schema.org/sourceOrganization>
pub const SOURCE_ORGANIZATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/sourceOrganization";
/// <https://schema.org/sourceOrganization>
pub const SOURCE_ORGANIZATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sourceOrganization";
/// <https://schema.org/sourceOrganization>
pub const SOURCE_ORGANIZATION_PROPERTY_LABEL: &str = "sourceOrganization";
pub struct SourceOrganizationPropertyIri;
impl PartialEq<&str> for SourceOrganizationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SOURCE_ORGANIZATION_PROPERTY_IRI_HTTP
			|| *other == SOURCE_ORGANIZATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SourceOrganizationPropertyIri> for &str {
	fn eq(&self, other: &SourceOrganizationPropertyIri) -> bool {
		*self == SOURCE_ORGANIZATION_PROPERTY_IRI_HTTP
			|| *self == SOURCE_ORGANIZATION_PROPERTY_IRI_HTTPS
	}
}
pub struct SourceOrganizationPropertyIriOrLabel;
impl PartialEq<&str> for SourceOrganizationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SourceOrganizationPropertyIri || *other == SOURCE_ORGANIZATION_PROPERTY_LABEL
	}
}
impl PartialEq<SourceOrganizationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SourceOrganizationPropertyIriOrLabel) -> bool {
		*self == SourceOrganizationPropertyIri || *self == SOURCE_ORGANIZATION_PROPERTY_LABEL
	}
}
