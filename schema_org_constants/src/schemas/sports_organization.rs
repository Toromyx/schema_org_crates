/// <https://schema.org/SportsOrganization>
pub const SPORTS_ORGANIZATION_IRI_HTTP: &str = "http://schema.org/SportsOrganization";
/// <https://schema.org/SportsOrganization>
pub const SPORTS_ORGANIZATION_IRI_HTTPS: &str = "https://schema.org/SportsOrganization";
/// <https://schema.org/SportsOrganization>
pub const SPORTS_ORGANIZATION_LABEL: &str = "SportsOrganization";
pub struct SportsOrganizationIri;
impl PartialEq<&str> for SportsOrganizationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SPORTS_ORGANIZATION_IRI_HTTP || *other == SPORTS_ORGANIZATION_IRI_HTTPS
	}
}
impl PartialEq<SportsOrganizationIri> for &str {
	fn eq(&self, other: &SportsOrganizationIri) -> bool {
		*self == SPORTS_ORGANIZATION_IRI_HTTP || *self == SPORTS_ORGANIZATION_IRI_HTTPS
	}
}
pub struct SportsOrganizationIriOrLabel;
impl PartialEq<&str> for SportsOrganizationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SportsOrganizationIri || *other == SPORTS_ORGANIZATION_LABEL
	}
}
impl PartialEq<SportsOrganizationIriOrLabel> for &str {
	fn eq(&self, other: &SportsOrganizationIriOrLabel) -> bool {
		*self == SportsOrganizationIri || *self == SPORTS_ORGANIZATION_LABEL
	}
}
