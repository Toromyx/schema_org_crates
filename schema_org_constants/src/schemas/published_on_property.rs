/// <https://schema.org/publishedOn>
pub const PUBLISHED_ON_PROPERTY_IRI_HTTP: &str = "http://schema.org/publishedOn";
/// <https://schema.org/publishedOn>
pub const PUBLISHED_ON_PROPERTY_IRI_HTTPS: &str = "https://schema.org/publishedOn";
/// <https://schema.org/publishedOn>
pub const PUBLISHED_ON_PROPERTY_LABEL: &str = "publishedOn";
pub struct PublishedOnPropertyIri;
impl PartialEq<&str> for PublishedOnPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PUBLISHED_ON_PROPERTY_IRI_HTTP || *other == PUBLISHED_ON_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PublishedOnPropertyIri> for &str {
	fn eq(&self, other: &PublishedOnPropertyIri) -> bool {
		*self == PUBLISHED_ON_PROPERTY_IRI_HTTP || *self == PUBLISHED_ON_PROPERTY_IRI_HTTPS
	}
}
pub struct PublishedOnPropertyIriOrLabel;
impl PartialEq<&str> for PublishedOnPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PublishedOnPropertyIri || *other == PUBLISHED_ON_PROPERTY_LABEL
	}
}
impl PartialEq<PublishedOnPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PublishedOnPropertyIriOrLabel) -> bool {
		*self == PublishedOnPropertyIri || *self == PUBLISHED_ON_PROPERTY_LABEL
	}
}
