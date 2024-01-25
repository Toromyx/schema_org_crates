/// <https://schema.org/dataFeedElement>
pub const DATA_FEED_ELEMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/dataFeedElement";
/// <https://schema.org/dataFeedElement>
pub const DATA_FEED_ELEMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dataFeedElement";
/// <https://schema.org/dataFeedElement>
pub const DATA_FEED_ELEMENT_PROPERTY_LABEL: &str = "dataFeedElement";
pub struct DataFeedElementPropertyIri;
impl PartialEq<&str> for DataFeedElementPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATA_FEED_ELEMENT_PROPERTY_IRI_HTTP
			|| *other == DATA_FEED_ELEMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DataFeedElementPropertyIri> for &str {
	fn eq(&self, other: &DataFeedElementPropertyIri) -> bool {
		*self == DATA_FEED_ELEMENT_PROPERTY_IRI_HTTP
			|| *self == DATA_FEED_ELEMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct DataFeedElementPropertyIriOrLabel;
impl PartialEq<&str> for DataFeedElementPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DataFeedElementPropertyIri || *other == DATA_FEED_ELEMENT_PROPERTY_LABEL
	}
}
impl PartialEq<DataFeedElementPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DataFeedElementPropertyIriOrLabel) -> bool {
		*self == DataFeedElementPropertyIri || *self == DATA_FEED_ELEMENT_PROPERTY_LABEL
	}
}
