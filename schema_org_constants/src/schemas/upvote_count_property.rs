/// <https://schema.org/upvoteCount>
pub const UPVOTE_COUNT_PROPERTY_IRI_HTTP: &str = "http://schema.org/upvoteCount";
/// <https://schema.org/upvoteCount>
pub const UPVOTE_COUNT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/upvoteCount";
/// <https://schema.org/upvoteCount>
pub const UPVOTE_COUNT_PROPERTY_LABEL: &str = "upvoteCount";
pub struct UpvoteCountPropertyIri;
impl PartialEq<&str> for UpvoteCountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UPVOTE_COUNT_PROPERTY_IRI_HTTP || *other == UPVOTE_COUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<UpvoteCountPropertyIri> for &str {
	fn eq(&self, other: &UpvoteCountPropertyIri) -> bool {
		*self == UPVOTE_COUNT_PROPERTY_IRI_HTTP || *self == UPVOTE_COUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct UpvoteCountPropertyIriOrLabel;
impl PartialEq<&str> for UpvoteCountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UpvoteCountPropertyIri || *other == UPVOTE_COUNT_PROPERTY_LABEL
	}
}
impl PartialEq<UpvoteCountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &UpvoteCountPropertyIriOrLabel) -> bool {
		*self == UpvoteCountPropertyIri || *self == UPVOTE_COUNT_PROPERTY_LABEL
	}
}
