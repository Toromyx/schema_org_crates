/// <https://schema.org/publishedBy>
pub const PUBLISHED_BY_PROPERTY_IRI_HTTP: &str = "http://schema.org/publishedBy";
/// <https://schema.org/publishedBy>
pub const PUBLISHED_BY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/publishedBy";
/// <https://schema.org/publishedBy>
pub const PUBLISHED_BY_PROPERTY_LABEL: &str = "publishedBy";
pub struct PublishedByPropertyIri;
impl PartialEq<&str> for PublishedByPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PUBLISHED_BY_PROPERTY_IRI_HTTP || *other == PUBLISHED_BY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PublishedByPropertyIri> for &str {
	fn eq(&self, other: &PublishedByPropertyIri) -> bool {
		*self == PUBLISHED_BY_PROPERTY_IRI_HTTP || *self == PUBLISHED_BY_PROPERTY_IRI_HTTPS
	}
}
pub struct PublishedByPropertyIriOrLabel;
impl PartialEq<&str> for PublishedByPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PublishedByPropertyIri || *other == PUBLISHED_BY_PROPERTY_LABEL
	}
}
impl PartialEq<PublishedByPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PublishedByPropertyIriOrLabel) -> bool {
		*self == PublishedByPropertyIri || *self == PUBLISHED_BY_PROPERTY_LABEL
	}
}
