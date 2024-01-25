/// <https://schema.org/DataFeed>
pub const DATA_FEED_IRI_HTTP: &str = "http://schema.org/DataFeed";
/// <https://schema.org/DataFeed>
pub const DATA_FEED_IRI_HTTPS: &str = "https://schema.org/DataFeed";
/// <https://schema.org/DataFeed>
pub const DATA_FEED_LABEL: &str = "DataFeed";
pub struct DataFeedIri;
impl PartialEq<&str> for DataFeedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATA_FEED_IRI_HTTP || *other == DATA_FEED_IRI_HTTPS
	}
}
impl PartialEq<DataFeedIri> for &str {
	fn eq(&self, other: &DataFeedIri) -> bool {
		*self == DATA_FEED_IRI_HTTP || *self == DATA_FEED_IRI_HTTPS
	}
}
pub struct DataFeedIriOrLabel;
impl PartialEq<&str> for DataFeedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DataFeedIri || *other == DATA_FEED_LABEL
	}
}
impl PartialEq<DataFeedIriOrLabel> for &str {
	fn eq(&self, other: &DataFeedIriOrLabel) -> bool {
		*self == DataFeedIri || *self == DATA_FEED_LABEL
	}
}
