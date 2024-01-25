/// <https://schema.org/hiringOrganization>
pub const HIRING_ORGANIZATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/hiringOrganization";
/// <https://schema.org/hiringOrganization>
pub const HIRING_ORGANIZATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hiringOrganization";
/// <https://schema.org/hiringOrganization>
pub const HIRING_ORGANIZATION_PROPERTY_LABEL: &str = "hiringOrganization";
pub struct HiringOrganizationPropertyIri;
impl PartialEq<&str> for HiringOrganizationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HIRING_ORGANIZATION_PROPERTY_IRI_HTTP
			|| *other == HIRING_ORGANIZATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HiringOrganizationPropertyIri> for &str {
	fn eq(&self, other: &HiringOrganizationPropertyIri) -> bool {
		*self == HIRING_ORGANIZATION_PROPERTY_IRI_HTTP
			|| *self == HIRING_ORGANIZATION_PROPERTY_IRI_HTTPS
	}
}
pub struct HiringOrganizationPropertyIriOrLabel;
impl PartialEq<&str> for HiringOrganizationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HiringOrganizationPropertyIri || *other == HIRING_ORGANIZATION_PROPERTY_LABEL
	}
}
impl PartialEq<HiringOrganizationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HiringOrganizationPropertyIriOrLabel) -> bool {
		*self == HiringOrganizationPropertyIri || *self == HIRING_ORGANIZATION_PROPERTY_LABEL
	}
}
