/// <https://schema.org/DataFeedItem>
pub const DATA_FEED_ITEM_IRI_HTTP: &str = "http://schema.org/DataFeedItem";
/// <https://schema.org/DataFeedItem>
pub const DATA_FEED_ITEM_IRI_HTTPS: &str = "https://schema.org/DataFeedItem";
/// <https://schema.org/DataFeedItem>
pub const DATA_FEED_ITEM_LABEL: &str = "DataFeedItem";
pub struct DataFeedItemIri;
impl PartialEq<&str> for DataFeedItemIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATA_FEED_ITEM_IRI_HTTP || *other == DATA_FEED_ITEM_IRI_HTTPS
	}
}
impl PartialEq<DataFeedItemIri> for &str {
	fn eq(&self, other: &DataFeedItemIri) -> bool {
		*self == DATA_FEED_ITEM_IRI_HTTP || *self == DATA_FEED_ITEM_IRI_HTTPS
	}
}
pub struct DataFeedItemIriOrLabel;
impl PartialEq<&str> for DataFeedItemIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DataFeedItemIri || *other == DATA_FEED_ITEM_LABEL
	}
}
impl PartialEq<DataFeedItemIriOrLabel> for &str {
	fn eq(&self, other: &DataFeedItemIriOrLabel) -> bool {
		*self == DataFeedItemIri || *self == DATA_FEED_ITEM_LABEL
	}
}
