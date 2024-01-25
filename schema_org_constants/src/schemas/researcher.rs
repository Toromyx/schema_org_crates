/// <https://schema.org/Researcher>
pub const RESEARCHER_IRI_HTTP: &str = "http://schema.org/Researcher";
/// <https://schema.org/Researcher>
pub const RESEARCHER_IRI_HTTPS: &str = "https://schema.org/Researcher";
/// <https://schema.org/Researcher>
pub const RESEARCHER_LABEL: &str = "Researcher";
pub struct ResearcherIri;
impl PartialEq<&str> for ResearcherIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESEARCHER_IRI_HTTP || *other == RESEARCHER_IRI_HTTPS
	}
}
impl PartialEq<ResearcherIri> for &str {
	fn eq(&self, other: &ResearcherIri) -> bool {
		*self == RESEARCHER_IRI_HTTP || *self == RESEARCHER_IRI_HTTPS
	}
}
pub struct ResearcherIriOrLabel;
impl PartialEq<&str> for ResearcherIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ResearcherIri || *other == RESEARCHER_LABEL
	}
}
impl PartialEq<ResearcherIriOrLabel> for &str {
	fn eq(&self, other: &ResearcherIriOrLabel) -> bool {
		*self == ResearcherIri || *self == RESEARCHER_LABEL
	}
}
