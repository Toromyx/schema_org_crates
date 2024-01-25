/// <https://schema.org/webFeed>
pub const WEB_FEED_PROPERTY_IRI_HTTP: &str = "http://schema.org/webFeed";
/// <https://schema.org/webFeed>
pub const WEB_FEED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/webFeed";
/// <https://schema.org/webFeed>
pub const WEB_FEED_PROPERTY_LABEL: &str = "webFeed";
pub struct WebFeedPropertyIri;
impl PartialEq<&str> for WebFeedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEB_FEED_PROPERTY_IRI_HTTP || *other == WEB_FEED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WebFeedPropertyIri> for &str {
	fn eq(&self, other: &WebFeedPropertyIri) -> bool {
		*self == WEB_FEED_PROPERTY_IRI_HTTP || *self == WEB_FEED_PROPERTY_IRI_HTTPS
	}
}
pub struct WebFeedPropertyIriOrLabel;
impl PartialEq<&str> for WebFeedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WebFeedPropertyIri || *other == WEB_FEED_PROPERTY_LABEL
	}
}
impl PartialEq<WebFeedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WebFeedPropertyIriOrLabel) -> bool {
		*self == WebFeedPropertyIri || *self == WEB_FEED_PROPERTY_LABEL
	}
}
