/// <https://schema.org/publicationType>
pub const PUBLICATION_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/publicationType";
/// <https://schema.org/publicationType>
pub const PUBLICATION_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/publicationType";
/// <https://schema.org/publicationType>
pub const PUBLICATION_TYPE_PROPERTY_LABEL: &str = "publicationType";
pub struct PublicationTypePropertyIri;
impl PartialEq<&str> for PublicationTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PUBLICATION_TYPE_PROPERTY_IRI_HTTP
			|| *other == PUBLICATION_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PublicationTypePropertyIri> for &str {
	fn eq(&self, other: &PublicationTypePropertyIri) -> bool {
		*self == PUBLICATION_TYPE_PROPERTY_IRI_HTTP || *self == PUBLICATION_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct PublicationTypePropertyIriOrLabel;
impl PartialEq<&str> for PublicationTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PublicationTypePropertyIri || *other == PUBLICATION_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<PublicationTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PublicationTypePropertyIriOrLabel) -> bool {
		*self == PublicationTypePropertyIri || *self == PUBLICATION_TYPE_PROPERTY_LABEL
	}
}
