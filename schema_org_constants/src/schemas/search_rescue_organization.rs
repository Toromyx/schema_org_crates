/// <https://schema.org/SearchRescueOrganization>
pub const SEARCH_RESCUE_ORGANIZATION_IRI_HTTP: &str = "http://schema.org/SearchRescueOrganization";
/// <https://schema.org/SearchRescueOrganization>
pub const SEARCH_RESCUE_ORGANIZATION_IRI_HTTPS: &str =
	"https://schema.org/SearchRescueOrganization";
/// <https://schema.org/SearchRescueOrganization>
pub const SEARCH_RESCUE_ORGANIZATION_LABEL: &str = "SearchRescueOrganization";
pub struct SearchRescueOrganizationIri;
impl PartialEq<&str> for SearchRescueOrganizationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEARCH_RESCUE_ORGANIZATION_IRI_HTTP
			|| *other == SEARCH_RESCUE_ORGANIZATION_IRI_HTTPS
	}
}
impl PartialEq<SearchRescueOrganizationIri> for &str {
	fn eq(&self, other: &SearchRescueOrganizationIri) -> bool {
		*self == SEARCH_RESCUE_ORGANIZATION_IRI_HTTP
			|| *self == SEARCH_RESCUE_ORGANIZATION_IRI_HTTPS
	}
}
pub struct SearchRescueOrganizationIriOrLabel;
impl PartialEq<&str> for SearchRescueOrganizationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SearchRescueOrganizationIri || *other == SEARCH_RESCUE_ORGANIZATION_LABEL
	}
}
impl PartialEq<SearchRescueOrganizationIriOrLabel> for &str {
	fn eq(&self, other: &SearchRescueOrganizationIriOrLabel) -> bool {
		*self == SearchRescueOrganizationIri || *self == SEARCH_RESCUE_ORGANIZATION_LABEL
	}
}
