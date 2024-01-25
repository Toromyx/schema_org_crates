/// <https://schema.org/ResearchProject>
pub const RESEARCH_PROJECT_IRI_HTTP: &str = "http://schema.org/ResearchProject";
/// <https://schema.org/ResearchProject>
pub const RESEARCH_PROJECT_IRI_HTTPS: &str = "https://schema.org/ResearchProject";
/// <https://schema.org/ResearchProject>
pub const RESEARCH_PROJECT_LABEL: &str = "ResearchProject";
pub struct ResearchProjectIri;
impl PartialEq<&str> for ResearchProjectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESEARCH_PROJECT_IRI_HTTP || *other == RESEARCH_PROJECT_IRI_HTTPS
	}
}
impl PartialEq<ResearchProjectIri> for &str {
	fn eq(&self, other: &ResearchProjectIri) -> bool {
		*self == RESEARCH_PROJECT_IRI_HTTP || *self == RESEARCH_PROJECT_IRI_HTTPS
	}
}
pub struct ResearchProjectIriOrLabel;
impl PartialEq<&str> for ResearchProjectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ResearchProjectIri || *other == RESEARCH_PROJECT_LABEL
	}
}
impl PartialEq<ResearchProjectIriOrLabel> for &str {
	fn eq(&self, other: &ResearchProjectIriOrLabel) -> bool {
		*self == ResearchProjectIri || *self == RESEARCH_PROJECT_LABEL
	}
}
