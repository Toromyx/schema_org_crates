/// <https://schema.org/issueNumber>
pub const ISSUE_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/issueNumber";
/// <https://schema.org/issueNumber>
pub const ISSUE_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/issueNumber";
/// <https://schema.org/issueNumber>
pub const ISSUE_NUMBER_PROPERTY_LABEL: &str = "issueNumber";
pub struct IssueNumberPropertyIri;
impl PartialEq<&str> for IssueNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ISSUE_NUMBER_PROPERTY_IRI_HTTP || *other == ISSUE_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IssueNumberPropertyIri> for &str {
	fn eq(&self, other: &IssueNumberPropertyIri) -> bool {
		*self == ISSUE_NUMBER_PROPERTY_IRI_HTTP || *self == ISSUE_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct IssueNumberPropertyIriOrLabel;
impl PartialEq<&str> for IssueNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IssueNumberPropertyIri || *other == ISSUE_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<IssueNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IssueNumberPropertyIriOrLabel) -> bool {
		*self == IssueNumberPropertyIri || *self == ISSUE_NUMBER_PROPERTY_LABEL
	}
}
