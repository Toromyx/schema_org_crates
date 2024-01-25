/// <https://schema.org/publisherImprint>
pub const PUBLISHER_IMPRINT_PROPERTY_IRI_HTTP: &str = "http://schema.org/publisherImprint";
/// <https://schema.org/publisherImprint>
pub const PUBLISHER_IMPRINT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/publisherImprint";
/// <https://schema.org/publisherImprint>
pub const PUBLISHER_IMPRINT_PROPERTY_LABEL: &str = "publisherImprint";
pub struct PublisherImprintPropertyIri;
impl PartialEq<&str> for PublisherImprintPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PUBLISHER_IMPRINT_PROPERTY_IRI_HTTP
			|| *other == PUBLISHER_IMPRINT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PublisherImprintPropertyIri> for &str {
	fn eq(&self, other: &PublisherImprintPropertyIri) -> bool {
		*self == PUBLISHER_IMPRINT_PROPERTY_IRI_HTTP
			|| *self == PUBLISHER_IMPRINT_PROPERTY_IRI_HTTPS
	}
}
pub struct PublisherImprintPropertyIriOrLabel;
impl PartialEq<&str> for PublisherImprintPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PublisherImprintPropertyIri || *other == PUBLISHER_IMPRINT_PROPERTY_LABEL
	}
}
impl PartialEq<PublisherImprintPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PublisherImprintPropertyIriOrLabel) -> bool {
		*self == PublisherImprintPropertyIri || *self == PUBLISHER_IMPRINT_PROPERTY_LABEL
	}
}
