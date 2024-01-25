/// <https://schema.org/candidate>
pub const CANDIDATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/candidate";
/// <https://schema.org/candidate>
pub const CANDIDATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/candidate";
/// <https://schema.org/candidate>
pub const CANDIDATE_PROPERTY_LABEL: &str = "candidate";
pub struct CandidatePropertyIri;
impl PartialEq<&str> for CandidatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CANDIDATE_PROPERTY_IRI_HTTP || *other == CANDIDATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CandidatePropertyIri> for &str {
	fn eq(&self, other: &CandidatePropertyIri) -> bool {
		*self == CANDIDATE_PROPERTY_IRI_HTTP || *self == CANDIDATE_PROPERTY_IRI_HTTPS
	}
}
pub struct CandidatePropertyIriOrLabel;
impl PartialEq<&str> for CandidatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CandidatePropertyIri || *other == CANDIDATE_PROPERTY_LABEL
	}
}
impl PartialEq<CandidatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CandidatePropertyIriOrLabel) -> bool {
		*self == CandidatePropertyIri || *self == CANDIDATE_PROPERTY_LABEL
	}
}
