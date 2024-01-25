/// <https://schema.org/downvoteCount>
pub const DOWNVOTE_COUNT_PROPERTY_IRI_HTTP: &str = "http://schema.org/downvoteCount";
/// <https://schema.org/downvoteCount>
pub const DOWNVOTE_COUNT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/downvoteCount";
/// <https://schema.org/downvoteCount>
pub const DOWNVOTE_COUNT_PROPERTY_LABEL: &str = "downvoteCount";
pub struct DownvoteCountPropertyIri;
impl PartialEq<&str> for DownvoteCountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DOWNVOTE_COUNT_PROPERTY_IRI_HTTP || *other == DOWNVOTE_COUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DownvoteCountPropertyIri> for &str {
	fn eq(&self, other: &DownvoteCountPropertyIri) -> bool {
		*self == DOWNVOTE_COUNT_PROPERTY_IRI_HTTP || *self == DOWNVOTE_COUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct DownvoteCountPropertyIriOrLabel;
impl PartialEq<&str> for DownvoteCountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DownvoteCountPropertyIri || *other == DOWNVOTE_COUNT_PROPERTY_LABEL
	}
}
impl PartialEq<DownvoteCountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DownvoteCountPropertyIriOrLabel) -> bool {
		*self == DownvoteCountPropertyIri || *self == DOWNVOTE_COUNT_PROPERTY_LABEL
	}
}
