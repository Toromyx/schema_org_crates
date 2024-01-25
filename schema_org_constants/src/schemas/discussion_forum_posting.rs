/// <https://schema.org/DiscussionForumPosting>
pub const DISCUSSION_FORUM_POSTING_IRI_HTTP: &str = "http://schema.org/DiscussionForumPosting";
/// <https://schema.org/DiscussionForumPosting>
pub const DISCUSSION_FORUM_POSTING_IRI_HTTPS: &str = "https://schema.org/DiscussionForumPosting";
/// <https://schema.org/DiscussionForumPosting>
pub const DISCUSSION_FORUM_POSTING_LABEL: &str = "DiscussionForumPosting";
pub struct DiscussionForumPostingIri;
impl PartialEq<&str> for DiscussionForumPostingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISCUSSION_FORUM_POSTING_IRI_HTTP || *other == DISCUSSION_FORUM_POSTING_IRI_HTTPS
	}
}
impl PartialEq<DiscussionForumPostingIri> for &str {
	fn eq(&self, other: &DiscussionForumPostingIri) -> bool {
		*self == DISCUSSION_FORUM_POSTING_IRI_HTTP || *self == DISCUSSION_FORUM_POSTING_IRI_HTTPS
	}
}
pub struct DiscussionForumPostingIriOrLabel;
impl PartialEq<&str> for DiscussionForumPostingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiscussionForumPostingIri || *other == DISCUSSION_FORUM_POSTING_LABEL
	}
}
impl PartialEq<DiscussionForumPostingIriOrLabel> for &str {
	fn eq(&self, other: &DiscussionForumPostingIriOrLabel) -> bool {
		*self == DiscussionForumPostingIri || *self == DISCUSSION_FORUM_POSTING_LABEL
	}
}
