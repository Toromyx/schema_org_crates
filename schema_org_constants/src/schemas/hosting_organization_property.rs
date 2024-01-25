/// <https://schema.org/hostingOrganization>
pub const HOSTING_ORGANIZATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/hostingOrganization";
/// <https://schema.org/hostingOrganization>
pub const HOSTING_ORGANIZATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hostingOrganization";
/// <https://schema.org/hostingOrganization>
pub const HOSTING_ORGANIZATION_PROPERTY_LABEL: &str = "hostingOrganization";
pub struct HostingOrganizationPropertyIri;
impl PartialEq<&str> for HostingOrganizationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOSTING_ORGANIZATION_PROPERTY_IRI_HTTP
			|| *other == HOSTING_ORGANIZATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HostingOrganizationPropertyIri> for &str {
	fn eq(&self, other: &HostingOrganizationPropertyIri) -> bool {
		*self == HOSTING_ORGANIZATION_PROPERTY_IRI_HTTP
			|| *self == HOSTING_ORGANIZATION_PROPERTY_IRI_HTTPS
	}
}
pub struct HostingOrganizationPropertyIriOrLabel;
impl PartialEq<&str> for HostingOrganizationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HostingOrganizationPropertyIri || *other == HOSTING_ORGANIZATION_PROPERTY_LABEL
	}
}
impl PartialEq<HostingOrganizationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HostingOrganizationPropertyIriOrLabel) -> bool {
		*self == HostingOrganizationPropertyIri || *self == HOSTING_ORGANIZATION_PROPERTY_LABEL
	}
}
