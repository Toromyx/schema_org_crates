/// <https://schema.org/PublicationIssue>
pub const PUBLICATION_ISSUE_IRI_HTTP: &str = "http://schema.org/PublicationIssue";
/// <https://schema.org/PublicationIssue>
pub const PUBLICATION_ISSUE_IRI_HTTPS: &str = "https://schema.org/PublicationIssue";
/// <https://schema.org/PublicationIssue>
pub const PUBLICATION_ISSUE_LABEL: &str = "PublicationIssue";
pub struct PublicationIssueIri;
impl PartialEq<&str> for PublicationIssueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PUBLICATION_ISSUE_IRI_HTTP || *other == PUBLICATION_ISSUE_IRI_HTTPS
	}
}
impl PartialEq<PublicationIssueIri> for &str {
	fn eq(&self, other: &PublicationIssueIri) -> bool {
		*self == PUBLICATION_ISSUE_IRI_HTTP || *self == PUBLICATION_ISSUE_IRI_HTTPS
	}
}
pub struct PublicationIssueIriOrLabel;
impl PartialEq<&str> for PublicationIssueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PublicationIssueIri || *other == PUBLICATION_ISSUE_LABEL
	}
}
impl PartialEq<PublicationIssueIriOrLabel> for &str {
	fn eq(&self, other: &PublicationIssueIriOrLabel) -> bool {
		*self == PublicationIssueIri || *self == PUBLICATION_ISSUE_LABEL
	}
}
