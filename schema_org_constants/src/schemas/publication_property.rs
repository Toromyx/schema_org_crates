/// <https://schema.org/publication>
pub const PUBLICATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/publication";
/// <https://schema.org/publication>
pub const PUBLICATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/publication";
/// <https://schema.org/publication>
pub const PUBLICATION_PROPERTY_LABEL: &str = "publication";
pub struct PublicationPropertyIri;
impl PartialEq<&str> for PublicationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PUBLICATION_PROPERTY_IRI_HTTP || *other == PUBLICATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PublicationPropertyIri> for &str {
	fn eq(&self, other: &PublicationPropertyIri) -> bool {
		*self == PUBLICATION_PROPERTY_IRI_HTTP || *self == PUBLICATION_PROPERTY_IRI_HTTPS
	}
}
pub struct PublicationPropertyIriOrLabel;
impl PartialEq<&str> for PublicationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PublicationPropertyIri || *other == PUBLICATION_PROPERTY_LABEL
	}
}
impl PartialEq<PublicationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PublicationPropertyIriOrLabel) -> bool {
		*self == PublicationPropertyIri || *self == PUBLICATION_PROPERTY_LABEL
	}
}
