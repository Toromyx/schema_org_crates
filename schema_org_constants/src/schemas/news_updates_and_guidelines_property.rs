/// <https://schema.org/newsUpdatesAndGuidelines>
pub const NEWS_UPDATES_AND_GUIDELINES_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/newsUpdatesAndGuidelines";
/// <https://schema.org/newsUpdatesAndGuidelines>
pub const NEWS_UPDATES_AND_GUIDELINES_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/newsUpdatesAndGuidelines";
/// <https://schema.org/newsUpdatesAndGuidelines>
pub const NEWS_UPDATES_AND_GUIDELINES_PROPERTY_LABEL: &str = "newsUpdatesAndGuidelines";
pub struct NewsUpdatesAndGuidelinesPropertyIri;
impl PartialEq<&str> for NewsUpdatesAndGuidelinesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NEWS_UPDATES_AND_GUIDELINES_PROPERTY_IRI_HTTP
			|| *other == NEWS_UPDATES_AND_GUIDELINES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NewsUpdatesAndGuidelinesPropertyIri> for &str {
	fn eq(&self, other: &NewsUpdatesAndGuidelinesPropertyIri) -> bool {
		*self == NEWS_UPDATES_AND_GUIDELINES_PROPERTY_IRI_HTTP
			|| *self == NEWS_UPDATES_AND_GUIDELINES_PROPERTY_IRI_HTTPS
	}
}
pub struct NewsUpdatesAndGuidelinesPropertyIriOrLabel;
impl PartialEq<&str> for NewsUpdatesAndGuidelinesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NewsUpdatesAndGuidelinesPropertyIri
			|| *other == NEWS_UPDATES_AND_GUIDELINES_PROPERTY_LABEL
	}
}
impl PartialEq<NewsUpdatesAndGuidelinesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NewsUpdatesAndGuidelinesPropertyIriOrLabel) -> bool {
		*self == NewsUpdatesAndGuidelinesPropertyIri
			|| *self == NEWS_UPDATES_AND_GUIDELINES_PROPERTY_LABEL
	}
}
