/// <https://schema.org/CompleteDataFeed>
pub const COMPLETE_DATA_FEED_IRI_HTTP: &str = "http://schema.org/CompleteDataFeed";
/// <https://schema.org/CompleteDataFeed>
pub const COMPLETE_DATA_FEED_IRI_HTTPS: &str = "https://schema.org/CompleteDataFeed";
/// <https://schema.org/CompleteDataFeed>
pub const COMPLETE_DATA_FEED_LABEL: &str = "CompleteDataFeed";
pub struct CompleteDataFeedIri;
impl PartialEq<&str> for CompleteDataFeedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMPLETE_DATA_FEED_IRI_HTTP || *other == COMPLETE_DATA_FEED_IRI_HTTPS
	}
}
impl PartialEq<CompleteDataFeedIri> for &str {
	fn eq(&self, other: &CompleteDataFeedIri) -> bool {
		*self == COMPLETE_DATA_FEED_IRI_HTTP || *self == COMPLETE_DATA_FEED_IRI_HTTPS
	}
}
pub struct CompleteDataFeedIriOrLabel;
impl PartialEq<&str> for CompleteDataFeedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CompleteDataFeedIri || *other == COMPLETE_DATA_FEED_LABEL
	}
}
impl PartialEq<CompleteDataFeedIriOrLabel> for &str {
	fn eq(&self, other: &CompleteDataFeedIriOrLabel) -> bool {
		*self == CompleteDataFeedIri || *self == COMPLETE_DATA_FEED_LABEL
	}
}
