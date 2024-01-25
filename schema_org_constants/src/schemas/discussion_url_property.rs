/// <https://schema.org/discussionUrl>
pub const DISCUSSION_URL_PROPERTY_IRI_HTTP: &str = "http://schema.org/discussionUrl";
/// <https://schema.org/discussionUrl>
pub const DISCUSSION_URL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/discussionUrl";
/// <https://schema.org/discussionUrl>
pub const DISCUSSION_URL_PROPERTY_LABEL: &str = "discussionUrl";
pub struct DiscussionUrlPropertyIri;
impl PartialEq<&str> for DiscussionUrlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISCUSSION_URL_PROPERTY_IRI_HTTP || *other == DISCUSSION_URL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DiscussionUrlPropertyIri> for &str {
	fn eq(&self, other: &DiscussionUrlPropertyIri) -> bool {
		*self == DISCUSSION_URL_PROPERTY_IRI_HTTP || *self == DISCUSSION_URL_PROPERTY_IRI_HTTPS
	}
}
pub struct DiscussionUrlPropertyIriOrLabel;
impl PartialEq<&str> for DiscussionUrlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiscussionUrlPropertyIri || *other == DISCUSSION_URL_PROPERTY_LABEL
	}
}
impl PartialEq<DiscussionUrlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DiscussionUrlPropertyIriOrLabel) -> bool {
		*self == DiscussionUrlPropertyIri || *self == DISCUSSION_URL_PROPERTY_LABEL
	}
}
