/// <https://schema.org/ResearchOrganization>
pub const RESEARCH_ORGANIZATION_IRI_HTTP: &str = "http://schema.org/ResearchOrganization";
/// <https://schema.org/ResearchOrganization>
pub const RESEARCH_ORGANIZATION_IRI_HTTPS: &str = "https://schema.org/ResearchOrganization";
/// <https://schema.org/ResearchOrganization>
pub const RESEARCH_ORGANIZATION_LABEL: &str = "ResearchOrganization";
pub struct ResearchOrganizationIri;
impl PartialEq<&str> for ResearchOrganizationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESEARCH_ORGANIZATION_IRI_HTTP || *other == RESEARCH_ORGANIZATION_IRI_HTTPS
	}
}
impl PartialEq<ResearchOrganizationIri> for &str {
	fn eq(&self, other: &ResearchOrganizationIri) -> bool {
		*self == RESEARCH_ORGANIZATION_IRI_HTTP || *self == RESEARCH_ORGANIZATION_IRI_HTTPS
	}
}
pub struct ResearchOrganizationIriOrLabel;
impl PartialEq<&str> for ResearchOrganizationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ResearchOrganizationIri || *other == RESEARCH_ORGANIZATION_LABEL
	}
}
impl PartialEq<ResearchOrganizationIriOrLabel> for &str {
	fn eq(&self, other: &ResearchOrganizationIriOrLabel) -> bool {
		*self == ResearchOrganizationIri || *self == RESEARCH_ORGANIZATION_LABEL
	}
}
