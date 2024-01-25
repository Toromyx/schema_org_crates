/// <https://schema.org/ComicIssue>
pub const COMIC_ISSUE_IRI_HTTP: &str = "http://schema.org/ComicIssue";
/// <https://schema.org/ComicIssue>
pub const COMIC_ISSUE_IRI_HTTPS: &str = "https://schema.org/ComicIssue";
/// <https://schema.org/ComicIssue>
pub const COMIC_ISSUE_LABEL: &str = "ComicIssue";
pub struct ComicIssueIri;
impl PartialEq<&str> for ComicIssueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMIC_ISSUE_IRI_HTTP || *other == COMIC_ISSUE_IRI_HTTPS
	}
}
impl PartialEq<ComicIssueIri> for &str {
	fn eq(&self, other: &ComicIssueIri) -> bool {
		*self == COMIC_ISSUE_IRI_HTTP || *self == COMIC_ISSUE_IRI_HTTPS
	}
}
pub struct ComicIssueIriOrLabel;
impl PartialEq<&str> for ComicIssueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ComicIssueIri || *other == COMIC_ISSUE_LABEL
	}
}
impl PartialEq<ComicIssueIriOrLabel> for &str {
	fn eq(&self, other: &ComicIssueIriOrLabel) -> bool {
		*self == ComicIssueIri || *self == COMIC_ISSUE_LABEL
	}
}
