/// <https://schema.org/PublicationEvent>
pub const PUBLICATION_EVENT_IRI_HTTP: &str = "http://schema.org/PublicationEvent";
/// <https://schema.org/PublicationEvent>
pub const PUBLICATION_EVENT_IRI_HTTPS: &str = "https://schema.org/PublicationEvent";
/// <https://schema.org/PublicationEvent>
pub const PUBLICATION_EVENT_LABEL: &str = "PublicationEvent";
pub struct PublicationEventIri;
impl PartialEq<&str> for PublicationEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PUBLICATION_EVENT_IRI_HTTP || *other == PUBLICATION_EVENT_IRI_HTTPS
	}
}
impl PartialEq<PublicationEventIri> for &str {
	fn eq(&self, other: &PublicationEventIri) -> bool {
		*self == PUBLICATION_EVENT_IRI_HTTP || *self == PUBLICATION_EVENT_IRI_HTTPS
	}
}
pub struct PublicationEventIriOrLabel;
impl PartialEq<&str> for PublicationEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PublicationEventIri || *other == PUBLICATION_EVENT_LABEL
	}
}
impl PartialEq<PublicationEventIriOrLabel> for &str {
	fn eq(&self, other: &PublicationEventIriOrLabel) -> bool {
		*self == PublicationEventIri || *self == PUBLICATION_EVENT_LABEL
	}
}
