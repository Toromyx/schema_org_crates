/// <https://schema.org/publisher>
pub const PUBLISHER_PROPERTY_IRI_HTTP: &str = "http://schema.org/publisher";
/// <https://schema.org/publisher>
pub const PUBLISHER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/publisher";
/// <https://schema.org/publisher>
pub const PUBLISHER_PROPERTY_LABEL: &str = "publisher";
pub struct PublisherPropertyIri;
impl PartialEq<&str> for PublisherPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PUBLISHER_PROPERTY_IRI_HTTP || *other == PUBLISHER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PublisherPropertyIri> for &str {
	fn eq(&self, other: &PublisherPropertyIri) -> bool {
		*self == PUBLISHER_PROPERTY_IRI_HTTP || *self == PUBLISHER_PROPERTY_IRI_HTTPS
	}
}
pub struct PublisherPropertyIriOrLabel;
impl PartialEq<&str> for PublisherPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PublisherPropertyIri || *other == PUBLISHER_PROPERTY_LABEL
	}
}
impl PartialEq<PublisherPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PublisherPropertyIriOrLabel) -> bool {
		*self == PublisherPropertyIri || *self == PUBLISHER_PROPERTY_LABEL
	}
}
